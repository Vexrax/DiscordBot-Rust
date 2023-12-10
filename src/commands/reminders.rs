use serenity::all::{CommandInteraction, CommandOptionType, ResolvedValue};
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::str::FromStr;
use crate::utils::discord_message::respond_to_interaction;
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

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
}

pub fn register() -> CreateCommand {
    CreateCommand::new("reminder")
        .description("Sets a reminder")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "Reminder", "What")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::Integer, "amount", "Amount")
                .add_string_choice([TimeUnit::Minutes].to_vec().iter(), [TimeUnit::Minutes].to_vec().iter())// TODO need to fix this
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "unit", "Unit Of Measurement")
                .required(true),
        )
}

fn get_millisecond_conversion_factor(unit_from_user: TimeUnit) -> i32 {
    return match unit_from_user {
        TimeUnit::Minutes => 60000,
        TimeUnit::Hours => 60000 * 60,
        TimeUnit::Days => 60000 * 60 * 24,
        TimeUnit::Months => 60000 * 60 * 30, // Using a avg here
        TimeUnit::Years => 60000 * 60 * 365,
    }
}