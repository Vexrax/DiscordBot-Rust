use serenity::{prelude::*, all::CommandInteraction, builder::{CreateInteractionResponseMessage, CreateInteractionResponse}};
use serenity::all::CreateEmbed;

pub async fn respond_to_interaction(ctx: &Context, command: &CommandInteraction, message_to_send: &String)  {
    let data = CreateInteractionResponseMessage::new().content(message_to_send);
    let builder = CreateInteractionResponse::Message(data);
    if let Err(why) = command.create_response(&ctx.http, builder).await {
        println!("Cannot respond to slash command: {why}");
    }
}

pub async fn respond_to_interaction_with_embed(ctx: &Context, command: &CommandInteraction, message_to_send: &String, embed: CreateEmbed ) {
    let builder = CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
        .content(message_to_send)
        .embed(embed));
    if let Err(why) = command.create_response(&ctx.http, builder).await {
        println!("Cannot respond to slash command: {why}");
    }
}