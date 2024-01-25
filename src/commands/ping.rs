use serenity::all::{CommandInteraction};
use serenity::builder::{CreateCommand};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;

use crate::utils::discord_message::respond_to_interaction;

pub async fn run(_options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    respond_to_interaction(ctx, command, &"Skynet V3 (Rust Version)".to_string().to_string()).await;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("A ping command")
}