use std::collections::HashMap;
use serenity::all::{ChannelId, CommandInteraction, GetMessages, User};
use serenity::builder::{CreateCommand};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;

use crate::utils::discord_message::respond_to_interaction;

pub async fn run(_options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    respond_to_interaction(ctx, command, &format!("Calculating Message Counts").to_string()).await;

    let guild = command.guild_id.unwrap();
    let all_channels = guild.channels(&ctx.http).await.unwrap();

    let mut message_counts: HashMap<User, u32> = HashMap::new();

    for channels in all_channels {
        message_counts = message_counts.into_iter().chain(get_all_messages_in_channel(channels.0, ctx).await).collect();
    }

    for x in message_counts {
        println!("{} - {}", x.0.name, x.1);
    }
}

// TODO this function is super inefficent, need to find a faster way to do this
pub async fn get_all_messages_in_channel(channel_id: ChannelId, ctx: &Context) -> HashMap<User, u32> {

    let mut message_counts: HashMap<User, u32> = HashMap::new();

    let mut before = 255;
    let mut after = 1;
    let mut messages_in_channel = channel_id.messages(&ctx.http, GetMessages::new().before(before).after(after)).await.unwrap();

    while messages_in_channel.len() > 0 {
        for message in messages_in_channel.clone() {
            let x = message_counts.entry(message.author.clone()).or_insert(0);
            let y  = x.clone();
            message_counts.insert(message.author, y + 1);
        }
        before += 255;
        after += 255;
        messages_in_channel = channel_id.messages(&ctx.http,  GetMessages::new().before(before).after(after)).await.unwrap();
    }
    return HashMap::new();
}

pub fn register() -> CreateCommand {
    CreateCommand::new("messageleaderboard").description("Prints out message leaderboard")
}