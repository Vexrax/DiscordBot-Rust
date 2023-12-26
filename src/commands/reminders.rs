use serenity::all::{ChannelId, CommandInteraction, CommandOptionType, GuildId, ResolvedValue};
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;
use std::time::{SystemTime, UNIX_EPOCH};
use std::str::FromStr;
use mongodb::Collection;
use serde::{Deserialize, Serialize};
use crate::utils::discord_message::respond_to_interaction;
use crate::utils::mongo::get_mongo_client;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Reminder {
    reminder: String,
    guild_id: GuildId,
    channel_id: ChannelId,
    time: u128
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
pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    let reminder: String;
    let amount: i64;
    let unit: TimeUnit;
    let guild_id: GuildId = command.guild_id.expect("Expected this message to be in a guild");
    let channel_id: ChannelId = command.channel_id;

    println!("HEEEEEERRRE");

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


    let current_time_millia = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let time_in_future_millis = i128::from(amount) * get_millisecond_conversion_factor(unit);
    let timestamp_to_remind_at_millis = current_time_millia.as_millis().wrapping_add_signed(time_in_future_millis);

    let reminder = Reminder {
        reminder,
        guild_id,
        channel_id,
        time: timestamp_to_remind_at_millis,
    };

    // TODO?
    // Maybe have a limit of 10 reminders per person?

    println!("{} {} {} {}", reminder.reminder, reminder.time, reminder.channel_id, reminder.guild_id);

    let database_result = get_mongo_client().await;
    match database_result {
        // TODO figure out why this doesnt insert correctly
        Ok(db) => add_reminder_to_collection(db.collection::<Reminder>("Reminders"), reminder).await,
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

pub fn check_for_reminders() {
    // Pull the active reminders
    // Check if any are due
    // Make the call to the channel
    // Delete reminder from DB
    todo!()
}

async fn add_reminder_to_collection(collection: Collection<Reminder>, reminder:  Reminder) {
    collection.insert_one(reminder, None).await.ok();
}

fn get_millisecond_conversion_factor(unit_from_user: TimeUnit) -> i128 {
    return match unit_from_user {
        TimeUnit::Minutes => 60000,
        TimeUnit::Hours => 60000 * 60,
        TimeUnit::Days => 60000 * 60 * 24,
        TimeUnit::Months => 60000 * 60 * 30, // Using a avg here
        TimeUnit::Years => 60000 * 60 * 365,
    }
}