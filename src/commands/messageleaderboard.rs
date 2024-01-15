use std::collections::HashMap;
use futures::StreamExt;
use serenity::all::{ChannelId, Color, CommandInteraction, CreateEmbed, CreateEmbedFooter, CreateMessage, User};
use serenity::builder::{CreateCommand};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;

use crate::utils::discord_message::respond_to_interaction;

const AMOUNT_OF_LEADERBOARD_POSITIONS_TO_DISPLAY: i32 = 7;

pub async fn run(_options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {

    let guild = command.guild_id.unwrap();
    let all_channels = guild.channels(&ctx.http).await.unwrap();

    respond_to_interaction(ctx, command, &format!("Calculating Message Counts... this will take a while ({} channels)", all_channels.len()).to_string()).await;

    let mut message_counts: HashMap<User, u32> = HashMap::new();
    for channels in all_channels {
        println!("Starting Channel: {:?}", channels.0.name(&ctx.http).await);
        merge_maps(&mut message_counts, get_all_messages_in_channel(channels.0, ctx).await);
    }

    let embed = build_embed(message_counts);
    let _ = command.channel_id.send_message(&ctx.http, CreateMessage::new().tts(false).embed(embed)).await;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("messageleaderboard").description("Prints out message leaderboard")
}

pub fn build_embed(message_counts: HashMap<User, u32>) -> CreateEmbed {

    let mut message_counts_sorted: Vec<_> = message_counts.into_iter().collect();
    message_counts_sorted.sort_by(|a, b| b.1.cmp(&a.1));

    let mut fields = vec![];
    let mut i = 0;
    for user in message_counts_sorted {

        // Discord has max fields of 25 7*3 = 21
        if i > AMOUNT_OF_LEADERBOARD_POSITIONS_TO_DISPLAY {
            continue;
        }

        fields.push(("Username", user.0.name, true));
        fields.push(("⠀⠀⠀⠀Messages", format!("⠀⠀⠀⠀{}", user.1), true));
        fields.push(("Most Used Channel", format!("TODO"), true));

        i+=1;
    }

    return  CreateEmbed::new()
        .title(&format!("Message Leaderboard"))
        .description(&format!("Top {} yappers in boosted", AMOUNT_OF_LEADERBOARD_POSITIONS_TO_DISPLAY))
        .color(Color::TEAL)
        .fields(fields.into_iter())
        .thumbnail("https://tr.rbxcdn.com/04c6e20f26515ddbcbc5adaf78ce6f09/420/420/Hat/Png")
        .footer(CreateEmbedFooter::new(&format!("{} messages in {} channels", 0, 0)));
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

