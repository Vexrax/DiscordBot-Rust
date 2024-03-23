use rand::seq::SliceRandom;
use serenity::model::prelude::ChannelId;

use serenity::all::{CommandInteraction, ResolvedValue, CommandOptionType, CreateEmbedFooter, Color};
use serenity::builder::{CreateCommand, CreateCommandOption, CreateMessage, CreateEmbed};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;
use crate::commands::business::quote::{get_quote_from, get_random_quote, Quote};

use crate::utils::discord_message::respond_to_interaction;

pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    if let Some(ResolvedOption { value: ResolvedValue::String(name), .. }) = options.first() {
        send_quote_from_person_in_channel(&name.to_string().clone(), ctx, command).await;
    } else {
        send_random_quote_in_channel(ctx, command).await;
    }
}
pub fn register() -> CreateCommand {
    CreateCommand::new("quote").description("Gets a quote").add_option(
        CreateCommandOption::new(CommandOptionType::String, "from", "from who")
            .required(false),
    )
}
async fn send_quote_from_person_in_channel(name: &String, ctx: &Context, command: &CommandInteraction) {
    respond_to_interaction(&ctx, &command, &format!("getting a quote from {}...", name).to_string()).await;
    let all_quotes = get_quote_from(name).await;
    send_quote_in_channel(ctx, &command.channel_id, all_quotes).await
}

async fn send_random_quote_in_channel(ctx: &Context, command: &CommandInteraction) {
    respond_to_interaction(&ctx, &command, &"Getting a random quote. . .".to_string()).await;
    let all_quotes: Vec<Quote> = get_random_quote().await;
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

