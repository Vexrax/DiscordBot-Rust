use serenity::all::{CommandInteraction};
use serenity::builder::{CreateCommand, CreateInteractionResponseMessage, CreateInteractionResponse};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;
use rand::Rng;
use crate::utils::discord_message::respond_to_interaction;

pub async fn run(_options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    respond_to_interaction(ctx, command, &format!("You rolled a {}", rand::thread_rng().gen_range(0..6))).await;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("rolldice").description("Rolls a six sides dice")
}