use std::error::Error;

use futures::stream::TryStreamExt;
use serde::{Deserialize, Serialize};
use mongodb::{bson::doc};

use serenity::all::{Color, CommandInteraction};
use serenity::builder::{CreateCommand, CreateEmbed, CreateMessage};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;

use crate::utils::discord_message::respond_to_interaction;
use crate::utils::mongo::get_mongo_client;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct CopyPasta {
    title: String,
    description: String,
}

const COPYPASTA_DB_NAME: &str = "CopyPasta";
pub async fn run(_options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    respond_to_interaction(&ctx, &command, &"Sending Pastas".to_string()).await;

    let all_copy_pastas: Vec<CopyPasta> = get_copy_pastas().await.unwrap_or_else(|err| {
        log::error!("Something went wrong while trying to get copypasta: {}", err);
        vec![get_error_copypasta(&err)]
    });

    for copypasta in all_copy_pastas {

        let escaped_copypasta_description = copypasta.description
            .replace("\\n", "\n")
            .replace("\\r", "\r");

        let embed = CreateEmbed::new()
            .color(Color::DARK_RED)
            .title(copypasta.title)
            .description(escaped_copypasta_description);
        let _msg = command.channel_id.send_message(&ctx.http, CreateMessage::new().tts(false).embed(embed)).await;
    };
}

pub fn register() -> CreateCommand {
    return CreateCommand::new("copypasta").description("Prints out the current pastas");
}

async fn get_copy_pastas() -> mongodb::error::Result<Vec<CopyPasta>> {
    let database = get_mongo_client().await?;

    let typed_collection = database.collection::<CopyPasta>(COPYPASTA_DB_NAME);

    let cursor = typed_collection.find(None, None).await?;

    Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
}

fn get_error_copypasta(err: &dyn Error) -> CopyPasta {
    return CopyPasta {
        title: "Something went wrong".to_string(),
        description: err.to_string(),
    };
}
