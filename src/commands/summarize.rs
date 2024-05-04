use std::time::{SystemTime, UNIX_EPOCH};
use futures::StreamExt;
use serenity::all::{ChannelId, Color, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, CreateEmbedFooter, CreateMessage, Message, ResolvedOption, ResolvedValue, User};
use serenity::builder::CreateEmbed;
use crate::utils::discord_message::respond_to_interaction;
use crate::api::llama_api::summarize_chat_logs_with_llama;
use crate::utils::skynet_constants::SKYNET_USER_ID;

struct ChatLog {
    author: String,
    message: String,
    timestamp: i64,
    message_id: u64,
    replying_to_message_id: Option<u64>
}

const MAX_HOURS_AGO: i64 = 48;
pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {

    let mut hours;
    if let Some(ResolvedOption { value: ResolvedValue::Integer(amount_option), .. }) = options.get(0) {
        hours = *amount_option;
    } else {
        respond_to_interaction(&ctx, &command, &"Expected amount to be specified".to_string().to_string()).await;
        return;
    }

    let timestamp: u64 = get_unix_timestamp_to_look_for_messages_until(hours);
    // let channel = command.channel_id; // todo uncomment
    let channel = ChannelId::new(187317542283378688);

    let chat_logs = create_chat_log(ctx, channel, timestamp).await;
    // let chat_logs = create_chat_log_by_message_count(ctx, channel, 200).await;
    let channel_name = channel.name(&ctx.http).await.unwrap_or_else(|_| "the channel".to_string());

    respond_to_interaction(ctx, command, &format!("Trying to summarize the conversation in {} ({} messages), this may take a few minutes.", channel_name, chat_logs.len())).await;

    let log_string = create_chat_log_string(chat_logs);

    match summarize_chat_logs_with_llama(log_string).await {
        Some(summary) => {
            let embed= build_embed(summary);
            let _ = command.channel_id.send_message(&ctx.http, CreateMessage::new().tts(false).embed(embed)).await;
        },
        None => {
            let embed = CreateEmbed::new().title("ERROR").description(&"Something happened while trying to generate the summary".to_string());
            let _msg = command.channel_id.send_message(&ctx.http, CreateMessage::new().tts(false).embed(embed)).await;
        }
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("summarize").description("Summarize the conversation in the channel")
        .add_option(
            CreateCommandOption::new(CommandOptionType::Integer, "hours_ago", format!("How many hours ago (max {})", MAX_HOURS_AGO))
                .max_int_value(MAX_HOURS_AGO as u64)
                .required(true),
        )
}

fn create_chat_log_string(chat_logs: Vec<ChatLog>) -> String {
    let mut log_string: String = "".to_string();

    for log in chat_logs {
        // TODO figure out how to intergrate this
        let replying_to = match log.replying_to_message_id {
            None => "NONE".to_string(),
            Some(reply_id) => reply_id.to_string()
        };
        let log_line = format!("<[{}]> ({}) [{}] <{}>", log.message_id, log.timestamp, log.author, log.message);
        log_string = format!("{} {}\n", log_string, log_line);
    }
    log_string
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
                if message.author.id == SKYNET_USER_ID {
                    continue;
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
    let reference_message_id_optional = match message.referenced_message {
        None => None,
        Some(message) => {
            Some(message.id.get())
        }
    };

    // INFO: Some messages contain @s, these are formated like <@111231231232>. This section
    // parses those out and provides the authors name.
    let mut message_content = message.content;
    let mentions = message.mentions;
    for user in mentions {
        message_content = message_content.replace(format!("<@{}>", user.id).as_str(), &*user.name);
    }

    return ChatLog {
        timestamp: message.timestamp.unix_timestamp(),
        author: message.author.clone().name,
        message: message_content,
        message_id: message.id.get(),
        replying_to_message_id: reference_message_id_optional
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
