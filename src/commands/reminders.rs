use serenity::all::{CommandInteraction, CommandOptionType, ResolvedValue};
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;
use crate::utils::discord_message::respond_to_interaction;

pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    let reminder: String;
    let amount: i64;
    let unit: String;
    if let Some(ResolvedOption { value: ResolvedValue::String(reminder_option), .. }) = options.get(0) {
        reminder = reminder_option.to_string();
    } else {
        respond_to_interaction(&ctx, &command, &format!("Expected reminder to be specified").to_string()).await;
        return;
    }

    if let Some(ResolvedOption { value: ResolvedValue::Integer(amount_option), .. }) = options.get(0) {
        amount = *amount_option;
    } else {
        respond_to_interaction(&ctx, &command, &format!("Expected amount to be specified").to_string()).await;
        return;
    }

    if let Some(ResolvedOption { value: ResolvedValue::String(unit_option), .. }) = options.get(0) {
        unit = unit_option.to_string();
    } else {
        respond_to_interaction(&ctx, &command, &format!("Expected unit to be specified").to_string()).await;
        return;
    }

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
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "unit", "Unit Of Measurement")
                .required(true),
        )
}