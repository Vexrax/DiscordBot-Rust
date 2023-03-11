use std::env;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use futures::stream::TryStreamExt;
use serde::{Deserialize, Serialize};
use mongodb::{bson::doc, options::ClientOptions, Client, options::FindOptions, Database};
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct CopyPasta {
    title: String,
    description: String,  
}

/**
 * TODO: Figure out how to do async functions with run
 * TODO: Return all copy pastas
 * TODO: format the copypastas nicely with embeds
 */
pub async fn run(_options: &[CommandDataOption], ctx: &Context, interaction: &Interaction, command: &ApplicationCommandInteraction) {
    let retpasta: String;
    match get_copy_pastas().await {
        Ok(pasta) => retpasta = pasta.title,
        Err(err) => retpasta = "something bad happened".to_string()
    }

    println!("{}", retpasta);
    if let Err(why) = command
    .create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content(format!("Pasta {}", retpasta)))
    })
    .await
    {
        // TODO something bad happened
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("copypasta").description("Prints out the current pastas")
}

async fn get_copy_pastas() -> mongodb::error::Result<CopyPasta> {
    
    let database = get_mongo_client().await?;

    let typed_collection = database.collection::<CopyPasta>("CopyPasta");

    let cursor = typed_collection.find(None, None).await?;
    let pasta = cursor.deserialize_current()?;

    Ok(pasta)
}

async fn get_mongo_client() -> mongodb::error::Result<Database> {
    let mongo_pass = env::var("MONGOPASSWORD").expect("Expected mongopass in environment");
    
    let mongo_connection_string = format!("mongodb+srv://Dueces:{}@cluster0-mzmgc.mongodb.net/test?retryWrites=true&w=majority", mongo_pass);
    let client_options = ClientOptions::parse(mongo_connection_string,).await?;
    let client = Client::with_options(client_options)?;
    let database = client.database("Skynet");
    Ok(database)
}