use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use rand::Rng; 
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;

use crate::utils::discord_message::respond_to_interaction;

pub async fn run(_options: &[CommandDataOption], ctx: &Context, command: &ApplicationCommandInteraction) {
    let num: i32 = rand::thread_rng().gen_range(0..6);
    respond_to_interaction(&ctx, &command, &format!("You rolled a {}", num)).await;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("rolldice").description("Rolls a six sides dice")
}