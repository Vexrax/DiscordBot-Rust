use serenity::all::CommandInteraction;
use serenity::builder::CreateCommand;
use serenity::client::Context;
use serenity::model::application::ResolvedOption;
use rand::Rng;

use crate::utils::discord_message::respond_to_interaction;

pub async fn run(_options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    let num: i32 = rand::thread_rng().gen_range(0..2);

    let result;
    if num == 1 {
        result = "heads";
    } else {
        result = "tails"
    }

    respond_to_interaction(ctx, command, &format!("The coin landed on {}", result).to_string()).await;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("flipcoin").description("Flip A coin")
}