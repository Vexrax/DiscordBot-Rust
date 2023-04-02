use mongodb::{Database, Collection};
use mongodb::bson::Document;
use rand::seq::SliceRandom;
use serenity::builder::CreateApplicationCommand;
use futures::stream::TryStreamExt;
use serde::{Deserialize, Serialize};
use mongodb::{bson::doc};
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::{Interaction};
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


// TODO commonize this with the one in quote 
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Quote {
    quote: String,
    year: String,
    author: String,
    context: String,  
}

pub async fn run(_options: &[CommandDataOption], ctx: &Context, interaction: &Interaction, command: &ApplicationCommandInteraction) {

    let quote_option = _options.get(0)
                                                  .expect("Expected quote to be specified")
                                                  .resolved
                                                  .as_ref()
                                                  .expect("Expected string object");
    let author_option = _options.get(1)
                                                .expect("Expected author to be specified")
                                                .resolved
                                                .as_ref()
                                                .expect("Expected string object");
    let year_option = _options.get(2)
                                            .expect("Expected year to be specified")
                                            .resolved
                                            .as_ref()
                                            .expect("Expected string object");

    if let Some(member) = &command.member {
        if member.user.id != 188313190214533120 {
            respond_to_interaction(&ctx, &command, &format!("No.").to_string()).await;
            return;
        }
    }

    let quote = if let CommandDataOptionValue::String(quote) = quote_option { quote } else { todo!() };
    let author = if let CommandDataOptionValue::String(author) = author_option { author } else { todo!() };
    let year = if let CommandDataOptionValue::String(year) = year_option { year } else { todo!() };

    let quote_to_add = Quote {
        quote: capitalize(quote),
        author: author.to_string(),
        year: year.to_string(),
        context: "".to_string()
    };

    respond_to_interaction(&ctx, &command, &format!("Adding the quote: `{}` -{} {}", quote_to_add.quote, quote_to_add.author, quote_to_add.year).to_string()).await;

    let database_result = get_mongo_client().await;

    match database_result {
        Ok(db) => add_quote_to_collection(db.collection::<Quote>("Quotes"), quote_to_add).await,
        Err(err) => {}
    }
}

async fn add_quote_to_collection(collection: Collection<Quote>, quote_to_add: Quote){
    collection.insert_one(quote_to_add, None).await;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("quoteadd").description("add a quote")
        .create_option(|option| {
            option
                .name("quote")
                .description("the quote")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("from")
                .description("from who")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("year")
                .description("year")
                .kind(CommandOptionType::String)
                .required(true)
        })
}

