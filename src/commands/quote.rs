use mongodb::bson::Document;
use rand::seq::SliceRandom;
use futures::stream::TryStreamExt;
use serde::{Deserialize, Serialize};
use mongodb::bson::doc;
use serenity::model::prelude::ChannelId;

use serenity::all::{CommandInteraction, ResolvedValue, CommandOptionType};
use serenity::builder::{CreateCommand, CreateCommandOption, CreateMessage, CreateEmbed};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;

use crate::utils::discord_message::respond_to_interaction;
use crate::utils::mongo::get_mongo_client;
use crate::utils::string_utils::capitalize;

// TODO commonize this with the one in quoteadd
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Quote {
    quote: String,
    year: String,
    author: String,
    context: String,
}

pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    if let Some(ResolvedOption { value: ResolvedValue::String(name), .. }) = options.first() {
        get_quote_from(&name.to_string().clone(), ctx, command).await;
    } else {
        get_random_quote(ctx, command).await;
    }
}

async fn get_quote_from(name: &String, ctx: &Context, command: &CommandInteraction) {
    respond_to_interaction(&ctx, &command, &format!("getting a quote from {}. . .", name).to_string()).await;

    let all_quotes: Vec<Quote>;

    match get_quotes(doc! { "author": capitalize(name) }).await {
        Ok(quote) => all_quotes = quote,
        Err(err) => {
            all_quotes = vec![];
            eprintln!("Error occured while getting quote from, error: {}", err)
        }
    }

    send_quote_in_channel(ctx, &command.channel_id, all_quotes).await
}

async fn get_random_quote(ctx: &Context, command: &CommandInteraction) {
    respond_to_interaction(&ctx, &command, &"Getting a random quote. . .".to_string()).await;

    let all_quotes: Vec<Quote>;

    match get_quotes(doc! {  }).await {
        Ok(quote) => all_quotes = quote,
        Err(err) => {
            all_quotes = vec![];
            eprintln!("Error occured while getting qute {}", err)
        }
    }

    send_quote_in_channel(ctx, &command.channel_id, all_quotes).await
}


async fn send_quote_in_channel(ctx: &Context, channel_id: &ChannelId, quotes: Vec<Quote>) {
    let optional_quote: Option<&Quote> = quotes.choose(&mut rand::thread_rng());

    // TODO clean this up so we dont have dupe code for sending the embed
    if optional_quote.is_none() {
        let embed = CreateEmbed::new().title("ERROR").description(&format!("Couldnt find a quote!"));
        let _msg = channel_id.send_message(&ctx.http, CreateMessage::new().tts(false).embed(embed)).await;
        return;
    }

    let chosen_quote = optional_quote.unwrap();

    let embed = CreateEmbed::new().title("").description(&format!("{} -{} {} {}", chosen_quote.quote, chosen_quote.author, chosen_quote.context, chosen_quote.year));
    let _msg = channel_id.send_message(&ctx.http, CreateMessage::new().tts(false).embed(embed)).await;
}

async fn get_quotes(filter: Document) -> mongodb::error::Result<Vec<Quote>> {
    let database = get_mongo_client().await?;
    let typed_collection = database.collection::<Quote>("Quotes");
    let cursor = typed_collection.find(filter, None).await?;
    Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
}

pub fn register() -> CreateCommand {
    CreateCommand::new("quote").description("Gets a quote").add_option(
        CreateCommandOption::new(CommandOptionType::String, "from", "from who")
            .required(false),
    )
}