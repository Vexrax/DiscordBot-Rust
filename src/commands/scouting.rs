use serenity::all::{CommandInteraction, Context, CreateCommand, ResolvedOption};
use crate::utils::discord_message::respond_to_interaction;

// TODO pass in a bunch of summoners and scout their latest games
pub async fn run(_options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    respond_to_interaction(ctx, command, &format!("Scouting command (WIP)").to_string()).await;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("scout").description("Scouting command to fetch info about summoners")
}
