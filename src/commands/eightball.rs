use serenity::all::{CommandInteraction, ResolvedValue, CommandOptionType, CreateEmbed, Color};
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;
use rand::Rng;

use crate::utils::discord_message::{respond_to_interaction, respond_to_interaction_with_embed};

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
    if let Some(ResolvedOption { value: ResolvedValue::String(question), .. }) = options.first() {
        let response = RESPONSE_OPTIONS[rand::thread_rng().gen_range(0..RESPONSE_OPTIONS.len())].to_string();

        let embed = CreateEmbed::new()
            .title(format!("Question: {}", question))
            .description(format!("Answer: {response}").to_string())
            .color(Color::RED);

        respond_to_interaction_with_embed(ctx, command, &"".to_string(), embed).await;
    } else {
        respond_to_interaction(ctx, command, &"Please ask a question".to_string().to_string()).await;
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("eightball").description("Ask the eightball a question").add_option(
        CreateCommandOption::new(CommandOptionType::String, "question", "the question to ask")
            .required(true),
    )
}