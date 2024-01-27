use serenity::all::CommandInteraction;
use serenity::builder::CreateCommand;
use serenity::client::Context;
use serenity::model::application::ResolvedOption;
use rand::Rng;

use crate::utils::discord_message::respond_to_interaction;

pub async fn run(_options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    let num: i32 = rand::thread_rng().gen_range(0..2);

    match num {
        1 => respond_to_interaction(ctx, command, &"The coin landed on heads".to_string().to_string()).await,
        2 => respond_to_interaction(ctx, command, &"The coin landed on tails".to_string().to_string()).await,
        _ => {}
    };
}

pub fn register() -> CreateCommand {
    CreateCommand::new("flipcoin").description("Flip A coin")
}