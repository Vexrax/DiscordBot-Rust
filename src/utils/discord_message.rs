use serenity::{prelude::*, all::CommandInteraction, builder::{CreateInteractionResponseMessage, CreateInteractionResponse}};

pub async fn respond_to_interaction(ctx: &Context, command: &CommandInteraction, message_to_send: &String)  {
    // TODO figure out why this doesnt work
    let data = CreateInteractionResponseMessage::new().content(message_to_send);
    let builder = CreateInteractionResponse::Message(data);
    if let Err(why) = command.create_response(&ctx.http, builder).await {
        println!("Cannot respond to slash command: {why}");
    }
}