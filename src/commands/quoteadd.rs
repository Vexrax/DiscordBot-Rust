use mongodb::Collection;
use serde::{Deserialize, Serialize};
use mongodb::bson::doc;
use serenity::prelude::*;

use serenity::all::{CommandInteraction, CommandDataOptionValue, ResolvedValue, CommandOptionType};
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;

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

pub async fn run(_options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    // return "Reimpliment this later".to_string();
}

// pub async fn run(_options: &[CommandDataOption], ctx: &Context, _interaction: &Interaction, command: &ApplicationCommandInteraction) {

//     let quote_option = _options .get(0)
//                                 .expect("Expected quote to be specified")
//                                 .resolved
//                                 .as_ref()
//                                 .expect("Expected string object");
//     let author_option = _options.get(1)
//                                 .expect("Expected author to be specified")
//                                 .resolved
//                                 .as_ref()
//                                 .expect("Expected string object");
//     let year_option = _options.get(2)
//                                 .expect("Expected year to be specified")
//                                 .resolved
//                                 .as_ref()
//                                 .expect("Expected string object");

//     if let Some(member) = &command.member {
//         // Vexrax userId
//         if member.user.id != 188313190214533120 {
//             respond_to_interaction(&ctx, &command, &format!("No perms").to_string()).await;
//             return;
//         }
//     }

//     let quote = if let CommandDataOptionValue::String(quote) = quote_option { quote } else { todo!() };
//     let author = if let CommandDataOptionValue::String(author) = author_option { author } else { todo!() };
//     let year = if let CommandDataOptionValue::String(year) = year_option { year } else { todo!() };

//     let quote_to_add = Quote {
//         quote: capitalize(quote),
//         author: author.to_string(),
//         year: year.to_string(),
//         context: "".to_string()
//     };

//     respond_to_interaction(&ctx, &command, &format!("Adding the quote: `{}` -{} {}", quote_to_add.quote, quote_to_add.author, quote_to_add.year).to_string()).await;

//     let database_result = get_mongo_client().await;

//     match database_result {
//         Ok(db) => add_quote_to_collection(db.collection::<Quote>("Quotes"), quote_to_add).await,
//         Err(err) => {
//           eprintln!("Error: something went wrong when trying to add a quote to the DB: {}", err);  
//         }
//     }
// }

// async fn add_quote_to_collection(collection: Collection<Quote>, quote_to_add: Quote){
//     collection.insert_one(quote_to_add, None).await.ok();
// }

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
