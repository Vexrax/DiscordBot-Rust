mod commands;
mod utils;
mod api;

use std::env;
use std::time::Duration;

use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

use serenity::model::application::{Interaction};
use tokio::{task, time};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        log::info!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        // ctx.set_presence(Some(Activity::playing("Taking Over The World")), OnlineStatus::Online).await;

        let _commands = guild_id
            .set_commands(&ctx.http, vec![
                commands::roledice::register(),
                commands::reminders::register(),
                commands::quoteadd::register(),
                commands::quote::register(),
                commands::ping::register(),
                commands::mentalhelp::register(),
                commands::id::register(),
                commands::gamestatus::register(),
                commands::flipcoin::register(),
                commands::eightball::register(),
                commands::copypasta::register(),
                commands::messageleaderboard::register(),
                commands::scouting::register("scouting".to_string()),
                commands::scouting::register("scouting_comp".to_string()),
                commands::createprivatevoicechannel::register(),
                commands::summarize::register(),
                commands::inhouse::register()

            ])
            .await;

        // Keep this for debugging when adding new commands
        // println!("I now have the following guild slash commands: {_commands:#?}");

        // Do a check every 5 mins to async tasks. This is nonblocking
        task::spawn(async move{
            let mut interval = time::interval(Duration::from_secs(300));

            loop {
                interval.tick().await;
                commands::reminders::check_for_reminders(&ctx).await;
                commands::createprivatevoicechannel::cleanup_unused_channels(&ctx, guild_id).await;
            }
        });
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let _content = match command.data.name.as_str() {
                "rolldice" => {
                    commands::roledice::run(&command.data.options(), &ctx, &command).await;
                    None
                }
                "reminder" => {
                    commands::reminders::run(&command.data.options(), &ctx, &command).await;
                    None
                }
                "quoteadd" => {
                    commands::quoteadd::run(&command.data.options(), &ctx, &command).await;
                    None
                }
                "quote" => {
                    commands::quote::run(&command.data.options(), &ctx, &command).await;
                    None
                }
                "ping" => {
                    commands::ping::run(&command.data.options(), &ctx, &command).await;
                    None
                }
                "mentalhelp" => {
                    commands::mentalhelp::run(&command.data.options(), &ctx, &command).await;
                    None
                }
                "id" => {
                    commands::id::run(&command.data.options(), &ctx, &command).await;
                    None
                }
                "gamestatus" => {
                    commands::gamestatus::run(&command.data.options(), &ctx, &command).await;
                    None
                }
                "flipcoin" => {
                    commands::flipcoin::run(&command.data.options(), &ctx, &command).await;
                    None
                }
                "eightball" => {
                    commands::eightball::run(&command.data.options(), &ctx, &command).await;
                    None
                }
                "copypasta" => {
                    commands::copypasta::run(&command.data.options(), &ctx, &command).await;
                    None
                }
                "messageleaderboard" => {
                    commands::messageleaderboard::run(&command.data.options(), &ctx, &command).await;
                    None
                }
                "scouting" => {
                    commands::scouting::run_scouting(&command.data.options(), &ctx, &command).await;
                    None
                },
                "scouting_comp" => {
                    commands::scouting::run_scouting_for_comp(&command.data.options(), &ctx, &command).await;
                    None
                }
                "createprivatevc" => {
                    commands::createprivatevoicechannel::run(&command.data.options(), &ctx, &command).await;
                    None
                },
                "summarize" => {
                    commands::summarize::run(&command.data.options(), &ctx, &command).await;
                    None
                }
                "inhouse" => {
                    commands::inhouse::run(&command.data.options(), &ctx, &command).await;
                    None
                }
                _ => Some("not implemented :(".to_string()),
            };
        }
    }
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Warn).unwrap();
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD").expect("Expected a token in the environment");

    let intents = GatewayIntents::MESSAGE_CONTENT | GatewayIntents::non_privileged();
    // Build our client.
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        log::error!("Client error: {:?}", why);
    }
}