use std::collections::HashMap;
use serenity::all::{ChannelId, CommandInteraction, GetMessages, User};
use serenity::builder::{CreateCommand};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;

use crate::utils::discord_message::respond_to_interaction;

pub async fn run(_options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    let guild = command.guild_id.unwrap();
    let all_channels = guild.channels(&ctx.http).await.unwrap();

    let mut message_counts: HashMap<User, u32> = HashMap::new();

    for channels in all_channels {
        message_counts.extend(get_all_messages_in_channel(channels.0, ctx));
    }

    // TODO create embed of top 10

    respond_to_interaction(ctx, command, &format!("Skynet V3 (Rust Version)").to_string()).await;
}

pub async fn get_all_messages_in_channel(channel_id: ChannelId, ctx: &Context) -> HashMap<User, u32> {
    // channel_id.messages(&ctx.http, GetMessages::new().limit(255));
    todo!()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("messageleaderboard").description("Prints out message leaderboard")
}