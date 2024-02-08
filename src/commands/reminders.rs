use serenity::all::{ChannelId, CommandInteraction, CommandOptionType, CreateEmbed, CreateMessage, GuildId, ResolvedValue, User, UserId};
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use chrono::prelude::DateTime;
use chrono::{Datelike, Local, Month};
use futures::TryStreamExt;
use mongodb::bson::{Bson, doc, Document};
use mongodb::results::{DeleteResult, InsertOneResult};
use serenity::model::Color;
use crate::utils::discord_message::{respond_to_interaction, respond_to_interaction_with_embed};
use crate::utils::mongo::get_mongo_client;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Reminder {
    reminder: String,
    guild_id: u64,
    channel_id: u64,
    user_id: u64,
    time: u64
}

#[derive(Clone)]
enum TimeUnit {
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

impl FromStr for TimeUnit {
    type Err = ();

    fn from_str(input: &str) -> Result<TimeUnit, Self::Err> {
        match input {
            "Minutes"  => Ok(TimeUnit::Minutes),
            "Hours"  => Ok(TimeUnit::Hours),
            "Days"  => Ok(TimeUnit::Days),
            "Months" => Ok(TimeUnit::Months),
            "Years" => Ok(TimeUnit::Years),
            _      => Err(()),
        }
    }
}

const REMINDER_DB_NAME: &str = "Reminders";

pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    let reminder: String;
    let amount: i64;
    let unit: TimeUnit;
    let guild_id: GuildId = command.guild_id.expect("Expected this message to be in a guild");
    let channel_id: ChannelId = command.channel_id;
    let user_id: UserId = command.user.id;

    if let Some(ResolvedOption { value: ResolvedValue::String(reminder_option), .. }) = options.get(0) {
        reminder = reminder_option.to_string();
    } else {
        respond_to_interaction(&ctx, &command, &"Expected reminder to be specified".to_string().to_string()).await;
        return;
    }

    if let Some(ResolvedOption { value: ResolvedValue::Integer(amount_option), .. }) = options.get(1) {
        amount = *amount_option;
    } else {
        respond_to_interaction(&ctx, &command, &"Expected amount to be specified".to_string().to_string()).await;
        return;
    }

    if let Some(ResolvedOption { value: ResolvedValue::String(unit_option), .. }) = options.get(2) {
        unit = TimeUnit::from_str(&unit_option).unwrap();
    } else {
        respond_to_interaction(&ctx, &command, &"Expected unit to be specified".to_string().to_string()).await;
        return;
    }

    let current_time_seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let time_in_future_seconds = i64::from(amount) * get_second_conversion_factor(unit);
    let timestamp_to_remind_at_seconds = current_time_seconds.as_secs().wrapping_add_signed(time_in_future_seconds);

    let reminder = Reminder {
        reminder,
        guild_id: guild_id.get(),
        channel_id: channel_id.get(),
        user_id: user_id.get(),
        time: timestamp_to_remind_at_seconds,
    };

    // TODO: Maybe have a limit of 10 reminders per person?

    let add_reminder_result = add_reminder_to_collection(reminder.clone()).await;

    match add_reminder_result {
        Some(_result) => {
            respond_to_interaction_with_embed(&ctx,
                                              &command,
                                              &"I am creating the following reminder:".to_string(),
                                              get_reminder_creation_embed(&command.user, &reminder, time_in_future_seconds as u64))
                .await;
        }
        None => {
            respond_to_interaction(&ctx, &command, &"Something went wrong when trying to save the reminder!".to_string())
                .await;
        }
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("reminder")
        .description("Sets a reminder")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "reminder", "What")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::Integer, "amount", "Amount")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "unit", "Unit Of Measurement")
                .add_string_choice("Minutes", "Minutes")
                .add_string_choice("Hours", "Hours")
                .add_string_choice("Days", "Days")
                .add_string_choice("Months", "Months")
                .add_string_choice("Years", "Years")
                .required(true),
        )
}

pub async fn check_for_reminders(ctx: &Context) {

    let all_reminders: Vec<Reminder> = get_reminders_from_collection(doc! {  }).await.unwrap_or_else(|err| {
        eprintln!("Error occurred while getting reminders {}", err);
        vec![]
    });

    for reminder in all_reminders {
        if!(SystemTime::now() > SystemTime::UNIX_EPOCH + Duration::from_secs(reminder.time)) {
            continue;
        }

        let channel = match ctx.http.get_channel(ChannelId::from(reminder.channel_id)).await {
            Ok(channel) => channel,
            Err(err) => {
                eprintln!("Could not find the channel {}, err: {}", reminder.channel_id, err);
                delete_reminder_from_collection(reminder).await;
                continue;
            }
        };

        let user= match ctx.http.get_user(UserId::from(reminder.user_id)).await {
            Ok(user) => user,
            Err(err) => {
                eprintln!("Could not find the user {}, err: {}", reminder.user_id, err);
                delete_reminder_from_collection(reminder).await;
                continue;
            }
        };

        let embed: CreateEmbed = CreateEmbed::new()
            .title(&format!("Reminder for {}", user.name))
            .description(&format!("{}", reminder.reminder))
            .color(Color::DARK_TEAL)
            .thumbnail(user.avatar_url().unwrap_or_else(|| "".to_string()));

        let _ = channel.id().send_message(&ctx.http, CreateMessage::new().tts(false).embed(embed)).await;

        delete_reminder_from_collection(reminder).await;
    }
}

async fn get_reminders_from_collection(filter: Document) -> mongodb::error::Result<Vec<Reminder>> {
    let database = get_mongo_client().await?;
    let typed_collection = database.collection::<Reminder>(REMINDER_DB_NAME);
    let cursor = typed_collection.find(filter, None).await?;
    Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
}

async fn add_reminder_to_collection(reminder:  Reminder) -> Option<InsertOneResult> {
    let database_result = get_mongo_client().await;

    return match database_result {
        Ok(db) => {
            let collection = db.collection::<Reminder>(REMINDER_DB_NAME);
            collection.insert_one(reminder, None).await.ok()
        },
        Err(err) => {
            eprintln!("Error: something went wrong when trying to add a reminder to the DB: {}", err);
            None
        }
    };
}

async fn delete_reminder_from_collection(reminder: Reminder) -> Option<DeleteResult> {
    let database = get_mongo_client().await.expect("Expected to be able to find DB");
    let typed_collection = database.collection::<Reminder>(REMINDER_DB_NAME);

    return match typed_collection.delete_one(doc! { "reminder" : reminder.reminder, "user_id": Bson::Int64(reminder.user_id as i64)}, None).await {
        Ok(delete_result) => {
            println!("Deleted {} from the {} db", delete_result.deleted_count, typed_collection.name());
            Some(delete_result)
        },
        Err(err) => {
            eprintln!("Error occurred when deleting reminder: {}", err);
            None
        }
    }
}

fn get_reminder_creation_embed(user: &User, reminder: &Reminder, time_in_future_seconds: u64) -> CreateEmbed {
    // Format to human readable
    let timestamp_in_future = SystemTime::now().checked_add(Duration::from_secs(time_in_future_seconds))
        .expect("Expected the time to be a u64");
    let datetime = DateTime::<Local>::from(timestamp_in_future); // using locale for now, should find a way to use EST in the future
    let month = Month::try_from(u8::try_from(datetime.month()).unwrap()).expect("Expected Month to be formatted correctly");
    let day = datetime.day();
    let year = datetime.year();

    return CreateEmbed::new()
        .title(&format!("Reminder for {} on {} {}, {} EST", user.name, month.name(), day, year))
        .description(&format!("{}", reminder.reminder))
        .color(Color::DARK_BLUE)
        .thumbnail(user.avatar_url().unwrap_or_else(|| "".to_string()));
}

fn get_second_conversion_factor(unit_from_user: TimeUnit) -> i64 {
    return match unit_from_user {
        TimeUnit::Minutes => 60,
        TimeUnit::Hours => 60 * 60,
        TimeUnit::Days => 60 * 60 * 24,
        TimeUnit::Months => 60 * 60 * 30, // Using a avg here
        TimeUnit::Years => 60 * 60 * 365,
    }
}