use mongodb::Collection;

use serenity::all::{CommandInteraction, ResolvedValue, CommandOptionType};
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;
use crate::commands::business::quote::{Quote, QUOTE_DB_NAME};

use crate::utils::discord_message::respond_to_interaction;
use crate::utils::mongo::get_mongo_client;
use crate::utils::string_utils::capitalize;

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
        // Vexrax userId
        if member.user.id != 188313190214533120 {
            respond_to_interaction(&ctx, &command, &format!("No perms").to_string()).await;
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

    let database_result = get_mongo_client().await;

    match database_result {
        Ok(db) => add_quote_to_collection(db.collection::<Quote>(QUOTE_DB_NAME), quote_to_add).await,
        Err(err) => {
            eprintln!("Error: something went wrong when trying to add a quote to the DB: {}", err);
        }
    }
}

async fn add_quote_to_collection(collection: Collection<Quote>, quote_to_add: Quote) {
    collection.insert_one(quote_to_add, None).await.ok();
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
