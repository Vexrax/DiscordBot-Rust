use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use futures::stream::TryStreamExt;
use serde::{Deserialize, Serialize};
use mongodb::{bson::doc};
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::prelude::*;

use crate::utils::mongo::get_mongo_client;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Quote {
    quote: String,
    year: String,
    author: String,
    context: String,  
}

pub async fn run(_options: &[CommandDataOption], ctx: &Context, interaction: &Interaction, command: &ApplicationCommandInteraction) {
    todo!()

    // TODO quote
    // TODO quoteadd
    // TODO quotefrom
    // TODO backupQuotes
}

async fn get_all_quotes() -> mongodb::error::Result<Vec<Quote>> {
    
    let database = get_mongo_client().await?;

    let typed_collection = database.collection::<Quote>("Quotes");

    let cursor = typed_collection.find(None, None).await?;
   
    Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("quote").description("Gets a quote")
}