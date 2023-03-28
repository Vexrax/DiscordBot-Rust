use rand::seq::SliceRandom;
use serenity::builder::CreateApplicationCommand;
use futures::stream::TryStreamExt;
use serde::{Deserialize, Serialize};
use mongodb::{bson::doc};
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::prelude::*;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption,
    CommandDataOptionValue,
};

use crate::utils::discord_message::respond_to_interaction;
use crate::utils::mongo::get_mongo_client;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Quote {
    quote: String,
    year: String,
    author: String,
    context: String,  
}

pub async fn run(_options: &[CommandDataOption], ctx: &Context, interaction: &Interaction, command: &ApplicationCommandInteraction) {
    get_random_quote(ctx, interaction, command).await;
    // TODO quote
    // TODO quoteadd
    // TODO quotefrom
    // TODO backupQuotes
}

async fn get_random_quote(ctx: &Context, interaction: &Interaction, command: &ApplicationCommandInteraction) {

    respond_to_interaction(&ctx, &command, &"Some message here until i figure out why mongodb takes years to resp".to_string()).await;

    let all_quotes: Vec<Quote>;

    match get_all_quotes().await {
        Ok(quote) => all_quotes = quote,
        Err(err) => all_quotes = vec! []
    }

    let chosen_quote: &Quote = all_quotes.choose(&mut rand::thread_rng()).unwrap();
    let _ = command.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| e.title("")
                                        .description(&format!("{} -{} {} {}", chosen_quote.quote, chosen_quote.author, chosen_quote.context, chosen_quote.year))                                    
                                    )
    }).await;
}

async fn get_all_quotes() -> mongodb::error::Result<Vec<Quote>> {
    
    let database = get_mongo_client().await?;

    let typed_collection = database.collection::<Quote>("Quotes");

    let cursor = typed_collection.find(None, None).await?;
   
    Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("quote").description("Gets a quote")
        .create_option(|option| {
            option
                .name("from")
                .description("from who")
                .kind(CommandOptionType::String)
                .required(false)
        })
}