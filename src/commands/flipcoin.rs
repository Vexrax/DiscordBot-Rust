use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use rand::Rng; 
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

pub async fn run(_options: &[CommandDataOption], ctx: &Context, interaction: &Interaction, command: &ApplicationCommandInteraction) {
    let num: i32 = rand::thread_rng().gen_range(0..2);
    
    let mut result = "";
    if num == 1 {
        result = "Heads";
    } else {
        result = "Tails"
    }

    if let Err(why) = command
    .create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content(format!("The coin landed on {}", result)))
    })
    .await
    {
        // TODO something bad happened
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("flipcoin").description("Flip A coin")
}