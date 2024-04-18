use serenity::all::{CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, ResolvedOption};
use crate::utils::discord_message::respond_to_interaction;

pub async fn run(_options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    respond_to_interaction(ctx, command, &"This Command is WIP while i figure out LLMs".to_string().to_string()).await;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("summarize").description("Summarize the conversation in the channel")
        .add_option(
            CreateCommandOption::new(CommandOptionType::Number, "minutes ago", "How many minutes in the past to summarize for")
                .required(true),
        )
}