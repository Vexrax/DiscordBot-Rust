use serenity::all::CommandInteraction;
use serenity::builder::{CreateCommand};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;
use crate::utils::discord_message::respond_to_interaction;

pub async fn run(_options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    respond_to_interaction(ctx, command, &"https://www.google.com/search?client=firefox-b-1-d&q=mental+hospitals+near+me+".to_string()).await;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("mentalhelp").description("Used to give mental help.")
}