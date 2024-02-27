use std::collections::HashMap;
use std::hash::Hash;
use futures::StreamExt;
use serenity::all::{CacheHttp, ChannelId, Color, CommandInteraction, CreateEmbed, CreateEmbedFooter, CreateMessage, User};
use serenity::builder::{CreateCommand};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;

use crate::utils::discord_message::respond_to_interaction;

const AMOUNT_OF_LEADERBOARD_POSITIONS_TO_DISPLAY: i32 = 7;

pub async fn run(_options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {

    let guild = command.guild_id.unwrap();
    let all_channels = guild.channels(&ctx.http).await.unwrap();

    respond_to_interaction(ctx, command, &format!("Calculating Message Counts... this will take a while ({} channels)", all_channels.len()).to_string()).await;

    let mut message_counts_by_user: HashMap<User, u32> = HashMap::new();
    let mut message_counts_by_channel_by_user: HashMap<String, HashMap<User, u32>> = HashMap::new();

    for channels in all_channels {

        log::info!("Starting Channel: {:?}", channels.0.name(&ctx.http).await);
        let messages_in_channel = get_all_messages_in_channel(channels.0, ctx).await;
        match channels.0.name(ctx).await {
            Ok(name) => {
                message_counts_by_channel_by_user.insert(name, messages_in_channel.clone());
            }
            Err(err) => {
                log::info!(format!("Channel did not have a name err: {}", err))
            }
        }
        merge_maps(&mut message_counts_by_user, messages_in_channel.clone());
    }

    let embed = build_embed(&ctx, message_counts_by_user, message_counts_by_channel_by_user);
    let _ = command.channel_id.send_message(&ctx.http, CreateMessage::new().tts(false).embed(embed)).await;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("messageleaderboard").description("Prints out message leaderboard")
}

pub fn build_embed(ctx: &Context, message_counts_by_user: HashMap<User, u32>, message_counts_by_channel_by_user: HashMap<String, HashMap<User, u32>>) -> CreateEmbed {

    let mut message_counts_by_channel_sorted: Vec<_> = get_messages_by_channel(message_counts_by_channel_by_user).into_iter().collect();
    message_counts_by_channel_sorted.sort_by(|a, b| b.1.cmp(&a.1));

    let mut message_counts_by_user_sorted: Vec<_> = message_counts_by_user.into_iter().collect();
    message_counts_by_user_sorted.sort_by(|a, b| b.1.cmp(&a.1));

    let mut fields = vec![];
    let mut i = 0;
    for user in message_counts_by_user_sorted {

        // Discord has max fields of 25 7*3 = 21
        if i > AMOUNT_OF_LEADERBOARD_POSITIONS_TO_DISPLAY {
            continue;
        }

        fields.push(("Username", user.0.name, true));
        fields.push(("⠀⠀⠀⠀Messages", format!("⠀⠀⠀⠀{}", user.1), true));
        fields.push(("⠀⠀⠀⠀", "⠀⠀⠀⠀".to_string(), true));
        i+=1;
    }

    match message_counts_by_channel_sorted.get(0) {
        None => {}
        Some(value) => {
            fields.push(("Most Used Channel", format!("{}", value.clone().0), true));
        }
    }


    return  CreateEmbed::new()
        .title(&"Message Leaderboard".to_string())
        .description(&format!("Top {} yappers in boosted", AMOUNT_OF_LEADERBOARD_POSITIONS_TO_DISPLAY))
        .color(Color::TEAL)
        .fields(fields.into_iter())
        .thumbnail("https://tr.rbxcdn.com/04c6e20f26515ddbcbc5adaf78ce6f09/420/420/Hat/Png")
        .footer(CreateEmbedFooter::new(&format!("{} messages in {} channels", 0, 0)));
}

fn get_messages_by_channel(message_counts_by_channel: HashMap<String, HashMap<User, u32>>) -> HashMap<String, u32>{
    let mut message_counts = HashMap::new();
    for (channel, counts) in &message_counts_by_channel {
        message_counts.insert(channel.clone(), counts.values().sum());
    }

    return message_counts;
}

pub async fn get_all_messages_in_channel(channel_id: ChannelId, ctx: &Context) -> HashMap<User, u32> {
    let mut message_counts: HashMap<User, u32> = HashMap::new();

    let mut messages = channel_id.messages_iter(&ctx).boxed();
    while let Some(message_result) = messages.next().await {
        match message_result {
            Ok(message) => {
                message_counts.insert(message.author.clone(), *message_counts.clone().entry(message.author).or_default() + 1);
            },
            Err(_) => {},
        }
    }
    return message_counts;
}

// TODO this has to be already done by crates right?
fn merge_maps(map1: &mut HashMap<User, u32>, map2: HashMap<User, u32>) {
    for (key, value) in map2 {
        let entry = map1.entry(key).or_insert(0);
        *entry += value;
    }
}

