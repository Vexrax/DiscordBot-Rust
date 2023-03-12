use std::env;

use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::{InteractionResponseType};
use serenity::prelude::*;

pub async fn respond_to_interaction(ctx: &Context, command: &ApplicationCommandInteraction, message_to_send: &String)  {
    if let Err(why) = command
    .create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content(message_to_send))
    })
    .await
    {
        println!("Something went wrong when trying to react to the interaction {}", why)
    }
}