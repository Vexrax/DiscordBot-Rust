use serenity::all::{CommandInteraction, CommandDataOptionValue, ResolvedValue, CommandOptionType};
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;
use rand::Rng; 

pub async fn run(_options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {

}

pub fn register() -> CreateCommand {
    CreateCommand::new("reminder")
    .description("Sets a reminder")
    .add_option(
        CreateCommandOption::new(CommandOptionType::String, "amount", "Amount")
            .required(true),
    )
    .add_option(
        CreateCommandOption::new(CommandOptionType::String, "unit", "Unit Of Measurement")
            .required(true),
    )
}