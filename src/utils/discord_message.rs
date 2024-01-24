use serenity::{prelude::*, all::CommandInteraction, builder::{CreateInteractionResponseMessage, CreateInteractionResponse}};
use serenity::all::{ChannelId, CreateEmbed, Http};

pub async fn respond_to_interaction(ctx: &Context, command: &CommandInteraction, message_to_send: &String)  {
    let data = CreateInteractionResponseMessage::new().content(message_to_send);
    let builder = CreateInteractionResponse::Message(data);
    if let Err(why) = command.create_response(&ctx.http, builder).await {
        eprintln!("Cannot respond to slash command: {why}");
    }
}

pub async fn respond_to_interaction_with_embed(ctx: &Context, command: &CommandInteraction, message_to_send: &String, embed: CreateEmbed ) {
    let builder = CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
        .content(message_to_send)
        .embed(embed));
    if let Err(why) = command.create_response(&ctx.http, builder).await {
        eprintln!("Cannot respond to slash command: {why}");
    }
}

pub async fn say_message_in_channel(channel_id: ChannelId, http: &Http, message: &String) {
    match channel_id.say(http, message).await {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Something went wrong when trying to send message {}, err: {}", message, err);
        }
    }
}