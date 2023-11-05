use mongodb::bson::Document;
use rand::seq::SliceRandom;
use serenity::builder::CreateApplicationCommand;
use futures::stream::TryStreamExt;
use serde::{Deserialize, Serialize};
use mongodb::bson::doc;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::Interaction;
use serenity::model::prelude::ChannelId;
use serenity::prelude::*;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption,
    CommandDataOptionValue,
    
};
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

pub async fn run(_options: &[CommandDataOption], ctx: &Context, _interaction: &Interaction, command: &ApplicationCommandInteraction) {

    if _options.get(0).is_none() {
        get_random_quote(ctx, command).await;
        return;
    }

    let option = _options.get(0)
                                                  .expect("Expected auther to be specified")
                                                  .resolved
                                                  .as_ref()
                                                  .expect("Expected string object");

    if let CommandDataOptionValue::String(name) = option {
        get_quote_from(name, ctx, command).await;
    } 
}

async fn get_quote_from(name: &String, ctx: &Context, command: &ApplicationCommandInteraction) {
    respond_to_interaction(&ctx, &command, &format!("getting a quote from {}. . .", name).to_string()).await;

    let all_quotes: Vec<Quote>;

    match get_quotes(doc! { "author": capitalize(name) }).await {
        Ok(quote) => all_quotes = quote,
        Err(err) => {
            all_quotes = vec! [];
            eprintln!("Error occured while getting quote from, error: {}", err)
        }
    }

    send_quote_in_channel(ctx, &command.channel_id, all_quotes).await
}

async fn get_random_quote(ctx: &Context, command: &ApplicationCommandInteraction) {

    respond_to_interaction(&ctx, &command, &"Getting a random quote. . .".to_string()).await;

    let all_quotes: Vec<Quote>;

    match get_quotes(doc! {  }).await {
        Ok(quote) => all_quotes = quote,
        Err(err) => {
            all_quotes = vec! [];
            eprintln!("Error occured while getting qute {}", err)
        } 
    }

    send_quote_in_channel(ctx, &command.channel_id, all_quotes).await
}


async fn send_quote_in_channel(ctx: &Context, channel_id: &ChannelId, quotes: Vec<Quote>) {
    let optional_quote: Option<&Quote> = quotes.choose(&mut rand::thread_rng());

    if optional_quote.is_none() {
        let _ = channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| e.title("ERROR")
                                            .description(&format!("Couldnt find a quote!"))                                    
                                        )
        }).await;
        return;
    }

    let chosen_quote = optional_quote.unwrap();

    let _ = channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| e.title("")
                                        .description(&format!("{} -{} {} {}", chosen_quote.quote, chosen_quote.author, chosen_quote.context, chosen_quote.year))                                    
                                    )
    }).await;
}

async fn get_quotes(filter: Document) -> mongodb::error::Result<Vec<Quote>> {
    let database = get_mongo_client().await?;
    let typed_collection = database.collection::<Quote>("Quotes");
    let cursor = typed_collection.find(filter, None).await?;
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