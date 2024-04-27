use std::cmp;
use std::time::{SystemTime, UNIX_EPOCH};
use futures::StreamExt;
use serenity::all::{ChannelId, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, Message, ResolvedOption, ResolvedValue, User};
use crate::utils::discord_message::respond_to_interaction;
use crate::utils::llama_api::call_llama;

struct ChatLog {
    author: String,
    message: String,
    timestamp: i64,
}
pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    respond_to_interaction(ctx, command, &"This Command is WIP while i figure out LLMs".to_string().to_string()).await;

    let mut mins;
    if let Some(ResolvedOption { value: ResolvedValue::Integer(amount_option), .. }) = options.get(0) {
        mins = cmp::max(*amount_option, 60);
    } else {
        respond_to_interaction(&ctx, &command, &"Expected amount to be specified".to_string().to_string()).await;
        return;
    }

    let timestamp: u64 = get_unix_timestamp_to_look_for_messages_until(mins);
    // let channel = command.channel_id; // todo uncomment
    let channel = ChannelId::new(187317542283378688);

    let chat_logs = create_chat_log(ctx, channel, timestamp).await;


    let mut log_string: String = "".to_string();

    for log in chat_logs {
        let log_line = format!("({}) [{}] <{}>", log.timestamp, log.author, log.message);
        log_string = format!("{} {}\n", log_string, log_line);
    }
    call_llama(log_string).await;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("summarize").description("Summarize the conversation in the channel")
        .add_option(
            CreateCommandOption::new(CommandOptionType::Integer, "minutes_ago", "How many mins ago (max 60)")
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

fn get_unix_timestamp_to_look_for_messages_until(mins_in_past: i64) -> u64 {
    let current_time_seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let time_in_future_seconds = i64::from(mins_in_past) * 60;
    return current_time_seconds.as_secs().wrapping_add_signed(-1 * time_in_future_seconds);
}
