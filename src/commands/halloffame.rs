use std::collections::HashMap;
use serenity::all::{ChannelId, CommandInteraction, Context, CreateCommand, CreateMessage, ResolvedOption, User};
use crate::commands::messageleaderboard::{build_embed, get_all_messages_in_channel};
use crate::utils::discord_message::respond_to_interaction;

pub async fn run(_options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {

    let guild = command.guild_id.unwrap();
    let all_channels = guild.channels(&ctx.http).await.unwrap();

    respond_to_interaction(ctx, command, &format!("Calculating Message Counts... this will take a while ({} channels)", all_channels.len()).to_string()).await;

    for channels in all_channels {
        log::info!("Starting Channel: {:?}", channels.0.name(&ctx.http).await);
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("halloffame").description("Gets the most reacted to messages of all time")
}

pub async fn get_most_reactions_in_channel(_channel_id: ChannelId, _ctx: &Context) -> HashMap<User, u32> {
    // let mut message_counts: HashMap<User, u32> = HashMap::new();
    //
    // let mut messages = channel_id.messages_iter(&ctx).boxed();
    // while let Some(message_result) = messages.next().await {
    //     match message_result {
    //         Ok(message) => {
    //             message_counts.insert(message.author.clone(), *message_counts.clone().entry(message.author).or_default() + 1);
    //         },
    //         Err(_) => {},
    //     }
    // }
    // return message_counts;
    todo!()
}