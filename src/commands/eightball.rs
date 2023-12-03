use serenity::all::{CommandInteraction, ResolvedValue, CommandOptionType};
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;
use rand::Rng;

use crate::utils::discord_message::respond_to_interaction;

const RESPONSE_OPTIONS: &[&str] = &[
    "As I see it, yes.",
    "Ask again later",
    "Better not tell you now.",
    "Cannot predict now.",
    "Concentrate and ask again.",
    "Don't count on it.",
    "It is certain.",
    "It is decidedly so.",
    "Yes – definitely",
    "Without a doubt.",
    "Outlook good.",
    "Outlook not so good."
];

pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    if let Some(ResolvedOption { value: ResolvedValue::String(options), .. }) = options.first() {
        let response = RESPONSE_OPTIONS[rand::thread_rng().gen_range(0..RESPONSE_OPTIONS.len())].to_string();
        // TODO add the question in here 
        respond_to_interaction(ctx, command, &format!("{response}").to_string()).await;
    } else {
        respond_to_interaction(ctx, command, &format!("Please ask a question").to_string()).await;
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("eightball").description("Ask the eightball a question").add_option(
        CreateCommandOption::new(CommandOptionType::String, "question", "the question to ask")
            .required(true),
    )
}