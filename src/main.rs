mod commands;

use std::env;

use mongodb::Collection;
use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use mongodb::{bson::doc, options::ClientOptions, Client, options::FindOptions};
use futures::stream::TryStreamExt;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                "id" => commands::id::run(&command.data.options),
                "mentalhelp" => commands::mentalhelp::run(&command.data.options),
                "flipcoin" => commands::flipcoin::run(&command.data.options),
                "copypasta" => commands::copypasta::run(&command.data.options),
                "eightball" => commands::eightball::run(&command.data.options),
                "quote" => commands::quote::run(&command.data.options),
                "rolldice" => commands::roledice::run(&command.data.options),
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::ping::register(command))
                .create_application_command(|command| commands::id::register(command))
                .create_application_command(|command| commands::roledice::register(command))
                .create_application_command(|command| commands::copypasta::register(command))
                .create_application_command(|command| commands::eightball::register(command))
                .create_application_command(|command| commands::quote::register(command))
                .create_application_command(|command| commands::flipcoin::register(command))
                .create_application_command(|command| commands::mentalhelp::register(command))
        }).await;

        let guild_command = Command::create_global_application_command(&ctx.http, |command| {
            commands::wonderful_command::register(command)
        }).await;

    }
}

#[tokio::main]
async fn main() {

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD").expect("Expected a token in the environment");

    // Build our client.
    let mut client = serenity::Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
    connectToMongoAsync();
}

struct CopyPasta {
    title: String,
    description: String,  
}

#[tokio::main]
async fn connectToMongoAsync() -> mongodb::error::Result<()> {
    let mongo_pass = GuildId(
        env::var("mongopass")
            .expect("Expected mongopass in environment")
            .parse()
            .expect("mongopass must be an integer"),
    );
    
    let mongo_connection_string = format!("mongodb+srv://Dueces:{}@cluster0-mzmgc.mongodb.net/test?retryWrites=true&w=majority", mongo_pass);
    let client_options = ClientOptions::parse(mongo_connection_string,).await?;
    let client = Client::with_options(client_options)?;
    let database = client.database("Skynet");

    // let typed_collection = database.collection::<CopyPasta>("CopyPasta");

    // let query = doc! {};
    // let cursor = typed_collection.find(query, None).await?;
    // let x = cursor.deserialize_current()?;
    // println!(x.title);

    // // let mut cursor = typed_collection.find(None, None).await;
    // // cursor.
    Ok(())
}