use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;
use serenity::model::prelude::command::CommandOptionType;

use crate::utils::discord_message::respond_to_interaction;

pub async fn run(_options: &[CommandDataOption], ctx: &Context, command: &ApplicationCommandInteraction) {
    respond_to_interaction(&ctx, &command, &"Reminders (TODO)".to_string()).await;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("remindme").description("remind yourself about something")
    .create_option(|option| {
        option
            .name("amount")
            .description("amount of time")
            .kind(CommandOptionType::Number)
            .required(true)
    })
    .create_option(|option| {
        option
            .name("unit")
            .description("unit of measurement")
            .kind(CommandOptionType::String)
            .required(true)
    })
}