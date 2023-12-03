use serenity::all::{CommandInteraction};
use serenity::builder::{CreateCommand, CreateInteractionResponseMessage, CreateInteractionResponse};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;
use rand::Rng;

pub async fn run(_options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    let num: i32 = rand::thread_rng().gen_range(0..6);

    let data = CreateInteractionResponseMessage::new().content(format!("You rolled a {}", num));
    let builder = CreateInteractionResponse::Message(data);
    if let Err(why) = command.create_response(&ctx.http, builder).await {
        println!("Cannot respond to slash command: {why}");
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("rolldice").description("Rolls a six sides dice")
}