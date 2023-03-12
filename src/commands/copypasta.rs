use std::error::Error;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use futures::stream::TryStreamExt;
use serde::{Deserialize, Serialize};
use mongodb::{bson::doc};
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;

use crate::utils::discord_message::respond_to_interaction;
use crate::utils::mongo::get_mongo_client;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct CopyPasta {
    title: String,
    description: String,  
}

pub async fn run(_options: &[CommandDataOption], ctx: &Context, command: &ApplicationCommandInteraction) {
    respond_to_interaction(&ctx, &command, &"Sending Pastas".to_string()).await;

    let all_copy_pastas: Vec<CopyPasta>;

    match get_copy_pastas().await {
        Ok(pasta) => all_copy_pastas = pasta,
        Err(err) => all_copy_pastas = vec! [get_error_copypasta(&err)]
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

fn get_error_copypasta(err: &dyn Error) -> CopyPasta {
    return CopyPasta {
        title: "Something went wrong".to_string(),
        description: err.to_string()
    }
}
