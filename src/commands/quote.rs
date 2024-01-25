use mongodb::bson::Document;
use rand::seq::SliceRandom;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use serenity::model::prelude::ChannelId;

use serenity::all::{CommandInteraction, ResolvedValue, CommandOptionType, CreateEmbedFooter, Color};
use serenity::builder::{CreateCommand, CreateCommandOption, CreateMessage, CreateEmbed};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;
use crate::commands::business::quote::{Quote, QUOTE_DB_NAME};

use crate::utils::discord_message::respond_to_interaction;
use crate::utils::mongo::get_mongo_client;
use crate::utils::string_utils::capitalize;

pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    if let Some(ResolvedOption { value: ResolvedValue::String(name), .. }) = options.first() {
        get_quote_from(&name.to_string().clone(), ctx, command).await;
    } else {
        get_random_quote(ctx, command).await;
    }
}
pub fn register() -> CreateCommand {
    CreateCommand::new("quote").description("Gets a quote").add_option(
        CreateCommandOption::new(CommandOptionType::String, "from", "from who")
            .required(false),
    )
}
async fn get_quote_from(name: &String, ctx: &Context, command: &CommandInteraction) {
    respond_to_interaction(&ctx, &command, &format!("getting a quote from {}...", name).to_string()).await;

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

    match optional_quote {
        None => {
            let embed = CreateEmbed::new().title("ERROR").description(&"Couldnt find a quote!".to_string());
            let _msg = channel_id.send_message(&ctx.http, CreateMessage::new().tts(false).embed(embed)).await;
        }
        Some(quote) => {
            let embed = CreateEmbed::new()
                .title("Quote")
                .description(&format!("{}", quote.quote))
                .color(Color::ROHRKATZE_BLUE)
                .footer(CreateEmbedFooter::new(format!("{} {}, {}", quote.author, quote.context, quote.year)));
            let _msg = channel_id.send_message(&ctx.http, CreateMessage::new().tts(false).embed(embed)).await;
        }
    }
}

async fn get_quotes(filter: Document) -> mongodb::error::Result<Vec<Quote>> {
    let database = get_mongo_client().await?;
    let typed_collection = database.collection::<Quote>(QUOTE_DB_NAME);
    let cursor = typed_collection.find(filter, None).await?;
    Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
}

