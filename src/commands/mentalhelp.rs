use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;

use crate::utils::discord_message::respond_to_interaction;

pub async fn run(_options: &[CommandDataOption], ctx: &Context, command: &ApplicationCommandInteraction) {
    respond_to_interaction(&ctx, &command, &"https://www.google.com/search?client=firefox-b-1-d&q=mental+hospitals+near+me+".to_string()).await;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("mentalhelp").description("Used to give mental help.")
}