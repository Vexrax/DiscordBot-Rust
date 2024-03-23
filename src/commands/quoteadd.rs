use serenity::all::{CommandInteraction, ResolvedValue, CommandOptionType, UserId};
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;
use crate::commands::business::quote::{add_quote_to_collection, Quote};

use crate::utils::discord_message::respond_to_interaction;
use crate::utils::string_utils::capitalize;

const VEXRAX_USER_ID: u64 = 188313190214533120;
pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    let quote: String;
    let author: String;
    let year: String;
    if let Some(ResolvedOption { value: ResolvedValue::String(quote_option), .. }) = options.get(0) {
        quote = quote_option.to_string();
    } else {
        respond_to_interaction(&ctx, &command, &"Expected quote to be specified".to_string()).await;
        return;
    }

    if let Some(ResolvedOption { value: ResolvedValue::String(author_option), .. }) = options.get(1) {
        author = author_option.to_string();
    } else {
        respond_to_interaction(&ctx, &command, &"Expected author to be specified".to_string()).await;
        return;
    }

    if let Some(ResolvedOption { value: ResolvedValue::String(year_option), .. }) = options.get(2) {
        year = year_option.to_string();
    } else {
        respond_to_interaction(&ctx, &command, &"Expected year to be specified".to_string()).await;
        return;
    }


    if let Some(member) = &command.member {
        if member.user.id != UserId::from(VEXRAX_USER_ID) {
            respond_to_interaction(&ctx, &command, &"No permission to run this command".to_string()).await;
            return;
        }
    }

    let quote_to_add = Quote {
        quote: capitalize(&quote),
        author: author.to_string(),
        year: year.to_string(),
        context: "".to_string(),
    };

    respond_to_interaction(&ctx, &command, &format!("Adding the quote: `{}` -{} {}", quote_to_add.quote, quote_to_add.author, quote_to_add.year).to_string()).await;
    add_quote_to_collection(quote_to_add).await;
}


pub fn register() -> CreateCommand {
    CreateCommand::new("quoteadd")
        .description("add a quote")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "quote", "the quote")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "from", "from who")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "year", "year")
                .required(true),
        )
}
