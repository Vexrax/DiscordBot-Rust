use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::prelude::*;

pub async fn run(_options: &[CommandDataOption], ctx: &Context, interaction: &Interaction, command: &ApplicationCommandInteraction) {
    if let Err(why) = command
    .create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content("Skynet V3 (Rust Version)".to_string()))
    })
    .await
    {
        // TODO something bad happened
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("A ping command")
}