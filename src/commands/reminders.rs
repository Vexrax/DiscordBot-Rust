use serenity::all::{Channel, ChannelId, CommandInteraction, CommandOptionType, CreateEmbed, CreateMessage, GuildId, ResolvedValue, User, UserId};
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::str::FromStr;
use mongodb::Collection;
use serde::{Deserialize, Serialize};
use chrono::prelude::DateTime;
use chrono::{Local};
use futures::TryStreamExt;
use mongodb::bson::{Bson, doc, Document};
use serenity::model::Color;
use crate::utils::discord_message::respond_to_interaction;
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
        respond_to_interaction(&ctx, &command, &format!("Expected reminder to be specified").to_string()).await;
        return;
    }

    if let Some(ResolvedOption { value: ResolvedValue::Integer(amount_option), .. }) = options.get(1) {
        amount = *amount_option;
    } else {
        respond_to_interaction(&ctx, &command, &format!("Expected amount to be specified").to_string()).await;
        return;
    }

    if let Some(ResolvedOption { value: ResolvedValue::String(unit_option), .. }) = options.get(2) {
        unit = TimeUnit::from_str(&unit_option).unwrap();
    } else {
        respond_to_interaction(&ctx, &command, &format!("Expected unit to be specified").to_string()).await;
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

    // Format to human readable
    let timestamp_in_future = SystemTime::now().checked_add(Duration::from_secs(time_in_future_seconds as u64)).expect("Expected the time to be a u64");
    let datetime = DateTime::<Local>::from(timestamp_in_future); // using locale for now, should find a way to use EST in the future

    // TODO use month names
    respond_to_interaction(&ctx, &command, &format!("I will remind {} about: '{}' on {} EST", command.user.name, &reminder.reminder, datetime.format("%Y-%m-%d %H:%M").to_string())).await;

    let database_result = get_mongo_client().await;

    match database_result {
        Ok(db) => add_reminder_to_collection(db.collection::<Reminder>(REMINDER_DB_NAME), reminder).await,
        Err(err) => {
            eprintln!("Error: something went wrong when trying to add a reminder to the DB: {}", err);
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
                .required(true),
        )
}

pub async fn check_for_reminders(ctx: &Context) {

    let all_reminders: Vec<Reminder>;

    match get_reminders(doc! {  }).await {
        Ok(quote) => all_reminders = quote,
        Err(err) => {
            all_reminders = vec![];
            eprintln!("Error occurred while getting reminders {}", err)
        }
    }

    for reminder in all_reminders {
        if!(SystemTime::now() > SystemTime::UNIX_EPOCH + Duration::from_secs(reminder.time)) {
            continue;
        }

        let channel: Channel = ctx.http.get_channel(ChannelId::from(reminder.channel_id))
            .await
            .expect("Expected to be able to get a channel");
        let user: User = ctx.http.get_user(UserId::from(reminder.user_id))
            .await
            .expect("Expected the user to exist");
        let embed: CreateEmbed = CreateEmbed::new()
            .title(&format!("Reminder for {}", user.name))
            .description(&format!("{}", reminder.reminder))
            .color(Color::DARK_TEAL)
            .thumbnail(user.avatar_url().expect("Expected URL"));

        let _ = channel.id().send_message(&ctx.http, CreateMessage::new().tts(false).embed(embed)).await;

        delete_reminder_from_collection(reminder).await;
    }
}

async fn add_reminder_to_collection(collection: Collection<Reminder>, reminder:  Reminder) {
   collection.insert_one(reminder, None).await.ok();
}

async fn get_reminders(filter: Document) -> mongodb::error::Result<Vec<Reminder>> {
    let database = get_mongo_client().await?;
    let typed_collection = database.collection::<Reminder>(REMINDER_DB_NAME);
    let cursor = typed_collection.find(filter, None).await?;
    Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
}

async fn delete_reminder_from_collection(reminder: Reminder) {
    let database = get_mongo_client().await.expect("Expected to be able to find DB");
    let typed_collection = database.collection::<Reminder>(REMINDER_DB_NAME);

    match typed_collection.delete_one(doc! { "reminder" : reminder.reminder, "user_id": Bson::Int64(reminder.user_id as i64)}, None).await {
        Ok(delete_result) => {
            println!("Deleted {} from the {} db", delete_result.deleted_count, typed_collection.name());
        },
        Err(err) => {
            eprintln!("Error occurred when deleting reminder: {}", err)
        }
    }
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