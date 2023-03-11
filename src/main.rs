mod commands;

use std::env;

use mongodb::Collection;
use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use mongodb::{bson::doc, options::ClientOptions, Client, options::FindOptions};
use futures::stream::TryStreamExt;
use serde::{Deserialize, Serialize};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = &interaction {
            println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options, &ctx, &interaction, &command).await,
                "id" => commands::id::run(&command.data.options, &ctx, &interaction, &command).await,
                "mentalhelp" => commands::mentalhelp::run(&command.data.options, &ctx, &interaction, &command).await,
                "flipcoin" => commands::flipcoin::run(&command.data.options, &ctx, &interaction, &command).await,
                "copypasta" => commands::copypasta::run(&command.data.options, &ctx, &interaction, &command).await,
                "eightball" => commands::eightball::run(&command.data.options, &ctx, &interaction, &command).await,
                "quote" => commands::quote::run(&command.data.options, &ctx, &interaction, &command).await,
                "rolldice" => commands::roledice::run(&command.data.options, &ctx, &interaction, &command).await,
                _ => (),

            };
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
}