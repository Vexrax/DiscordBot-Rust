use std::env;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use futures::stream::TryStreamExt;
use serde::{Deserialize, Serialize};
use mongodb::{bson::doc, options::ClientOptions, Client, options::FindOptions, Database};
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::id::ChannelId;
use serenity::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct CopyPasta {
    title: String,
    description: String,  
}

/**
 * TODO: Refector to remove duped code
 */
pub async fn run(_options: &[CommandDataOption], ctx: &Context, interaction: &Interaction, command: &ApplicationCommandInteraction) {
    if let Err(why) = command
    .create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content("Sending Pastas".to_string()))
    })
    .await
    {
        // TODO something bad happened
    }

    let all_copy_pastas: Vec<CopyPasta>;

    match get_copy_pastas().await {
        Ok(pasta) => all_copy_pastas = pasta,
        Err(err) => {
            all_copy_pastas = vec! [
                CopyPasta {
                    title: "Something went wrong".to_string(),
                    description: err.to_string()
                }
            ]
        }
    }

    for copypasta in all_copy_pastas {
        let _ = command.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| e.title(copypasta.title)
                                            .description(copypasta.description)                                    
                                        )
        }).await;
    };
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("copypasta").description("Prints out the current pastas")
}

async fn get_copy_pastas() -> mongodb::error::Result<Vec<CopyPasta>> {
    
    let database = get_mongo_client().await?;

    let typed_collection = database.collection::<CopyPasta>("CopyPasta");

    let cursor = typed_collection.find(None, None).await?;
   
    Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
}

async fn get_mongo_client() -> mongodb::error::Result<Database> {
    let mongo_pass = env::var("MONGOPASSWORD").expect("Expected mongopass in environment");
    
    let mongo_connection_string = format!("mongodb+srv://Dueces:{}@cluster0-mzmgc.mongodb.net/test?retryWrites=true&w=majority", mongo_pass);
    let client_options = ClientOptions::parse(mongo_connection_string,).await?;
    let client = Client::with_options(client_options)?;
    let database = client.database("Skynet");
    Ok(database)
}