use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::all::{CommandInteraction, CommandDataOptionValue, ResolvedValue, CommandOptionType};
use serenity::builder::{CreateCommand, CreateCommandOption, CreateMessage};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;
use rand::Rng;

use crate::utils::discord_message::respond_to_interaction; 

pub async fn run(_options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    respond_to_interaction(ctx, command, &format!("Skynet V3 (Rust Version)").to_string()).await;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("A ping command")
}