use std::cmp;
use std::time::{SystemTime, UNIX_EPOCH};
use futures::StreamExt;
use serenity::all::{ChannelId, Color, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, CreateEmbedFooter, CreateMessage, Message, ResolvedOption, ResolvedValue, User};
use serenity::builder::CreateEmbed;
use crate::utils::discord_message::respond_to_interaction;
use crate::utils::llama_api::summarize_chat_logs_with_llama;

struct ChatLog {
    author: String,
    message: String,
    timestamp: i64,
}
pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {

    let mut hours;
    if let Some(ResolvedOption { value: ResolvedValue::Integer(amount_option), .. }) = options.get(0) {
        hours = cmp::max(*amount_option, 24);
    } else {
        respond_to_interaction(&ctx, &command, &"Expected amount to be specified".to_string().to_string()).await;
        return;
    }

    let timestamp: u64 = get_unix_timestamp_to_look_for_messages_until(hours);
    // let channel = command.channel_id; // todo uncomment
    let channel = ChannelId::new(187317542283378688);

    let chat_logs = create_chat_log(ctx, channel, timestamp).await; // todo uncomment
    // let chat_logs = create_chat_log_by_message_count(ctx, channel, 200).await;

    respond_to_interaction(ctx, command, &format!("Trying to summarize the conversation ({} messages), this may take a few minutes.", chat_logs.len())).await;

    let mut log_string: String = "".to_string();

    for log in chat_logs {
        let log_line = format!("({}) [{}] <{}>", log.timestamp, log.author, log.message);
        log_string = format!("{} {}\n", log_string, log_line);
    }
    match summarize_chat_logs_with_llama(log_string).await {
        Some(summary) => {
            let embed= build_embed(summary);
            let _ = command.channel_id.send_message(&ctx.http, CreateMessage::new().tts(false).embed(embed)).await;
        },
        None => {
            println!("Todo")
        }
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("summarize").description("Summarize the conversation in the channel")
        .add_option(
            CreateCommandOption::new(CommandOptionType::Integer, "hours_ago", "How many hours ago (max 24)")
                .required(true),
        )
}

async fn create_chat_log(ctx: &Context, channel_id: ChannelId, unix_time_to_look_until: u64) -> Vec<ChatLog> {
    let mut chat_logs: Vec<ChatLog> = vec![];
    let mut messages = channel_id.messages_iter(&ctx).boxed();
    while let Some(message_result) = messages.next().await {
        match message_result {
            Ok(message) => {
                if message.timestamp.unix_timestamp() < unix_time_to_look_until as i64 {
                    break;
                }
                chat_logs.push(create_single_chat_log_from_message(message));
            },
            Err(_) => {},
        }
    }
    return chat_logs;
}

async fn create_chat_log_by_message_count(ctx: &Context, channel_id: ChannelId, amount_of_messages_to_find: i32) -> Vec<ChatLog>{
    let mut chat_logs: Vec<ChatLog> = vec![];
    let mut messages = channel_id.messages_iter(&ctx).boxed();
    let mut i = 0;
    while i < amount_of_messages_to_find {
        let Some(message_result) = messages.next().await else { break };
        match message_result {
            Ok(message) => chat_logs.push(create_single_chat_log_from_message(message)),
            Err(_) => {},
        }
        i+=1;
    }

    return chat_logs;
}

fn create_single_chat_log_from_message(message: Message) -> ChatLog {
    return ChatLog {
        timestamp: message.timestamp.unix_timestamp(),
        author: message.author.clone().name,
        message: message.content.clone()
    };
}

fn get_unix_timestamp_to_look_for_messages_until(hours_in_past: i64) -> u64 {
    let current_time_seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let time_in_future_seconds = i64::from(hours_in_past) * 60 * 60;
    return current_time_seconds.as_secs().wrapping_add_signed(-1 * time_in_future_seconds);
}

fn build_embed(summary: String) -> CreateEmbed {
    return CreateEmbed::new()
        .title(&format!("Summary"))
        .description(&format!("{}", summary))
        .color(Color::TEAL)
        .footer(CreateEmbedFooter::new( format!("Summary powered by LLAMA3")));
}
