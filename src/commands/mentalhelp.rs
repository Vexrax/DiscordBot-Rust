use serenity::all::CommandInteraction;
use serenity::builder::{CreateCommand, CreateInteractionResponseMessage, CreateInteractionResponse};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;

use crate::utils::discord_message::respond_to_interaction;

pub async fn run(_options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    let data = CreateInteractionResponseMessage::new().content( "https://www.google.com/search?client=firefox-b-1-d&q=mental+hospitals+near+me+".to_string());
    let builder = CreateInteractionResponse::Message(data);
    if let Err(why) = command.create_response(&ctx.http, builder).await {
        println!("Cannot respond to slash command: {why}");
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("mentalhelp").description("Used to give mental help.")
}