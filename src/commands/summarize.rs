use std::time::{SystemTime, UNIX_EPOCH};
use futures::StreamExt;
use serenity::all::{ChannelId, Color, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, CreateEmbedFooter, CreateMessage, Message, ResolvedOption, ResolvedValue};
use serenity::builder::CreateEmbed;
use crate::utils::discord_message::respond_to_interaction;
use crate::api::ollama_api::call_ollama_api_await_response;
use crate::commands::business::ollama::{ChatLog, get_summary_of_logs, Model};
use crate::commands::business::ollama::Model::{GEMMA3, LLAMA3, LLAMA4};
use crate::utils::discord_message::say_message_in_channel;
use crate::utils::skynet_constants::SKYNET_USER_ID;

#[derive(Debug)]
struct CommandParams {
    hours_ago: Option<i64>,
    messages_ago: Option<i64>,
    channel: Option<ChannelId>
}

const MAX_HOURS_AGO: i64 = 24 * 3;
const MAX_MESSAGES: i64 = 200;
const CURRENT_MODEL: Model = LLAMA3;
pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    let command_params = get_command_params(options);
    let channel = command_params.channel.unwrap_or_else(|| command.channel_id);

    let chat_logs;
    if command_params.hours_ago.is_some() {
        let hours = command_params.hours_ago.unwrap();
        respond_to_interaction(ctx, command, &format!("Collecting messages for the past {} hours...", hours)).await;
        let timestamp: u64 = get_unix_timestamp_to_look_for_messages_until(hours);
        chat_logs = create_chat_log(ctx, channel, timestamp).await;
    }
    else if command_params.messages_ago.is_some() {
        let amount_messages_to_look_at = command_params.messages_ago.unwrap();
        respond_to_interaction(ctx, command, &format!("Collecting the last {} messages...", amount_messages_to_look_at)).await;
        chat_logs = create_chat_log_by_message_count(ctx, channel, amount_messages_to_look_at).await;
    }
    else {
        respond_to_interaction(&ctx, &command, &"Expected amount to be specified".to_string()).await;
        return;
    }

    let channel_name = channel.name(&ctx.http).await.unwrap_or_else(|_| "the channel".to_string());
    say_message_in_channel(command.channel_id, &ctx.http, &format!("Trying to summarize the conversation in {} ({} messages), this may take a few minutes.", channel_name, chat_logs.len())).await;

    match get_summary_of_logs(chat_logs, CURRENT_MODEL).await {
        Some(summary) => {
            let embed= build_embed(summary, channel_name, "Gemma".to_string());
            let _ = command.channel_id.send_message(&ctx.http, CreateMessage::new().tts(false).embed(embed)).await;
        },
        None => {
            let embed = CreateEmbed::new()
                .title("ERROR")
                .description(&"Something happened while trying to generate the summary".to_string());
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
        .add_option(
            CreateCommandOption::new(CommandOptionType::Integer, "messages", format!("How many messages to look at (max {})", MAX_MESSAGES))
                .max_int_value(MAX_MESSAGES as u64)
                .required(false),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::Channel, "channel", "Which channel to summarize")
                .required(false),
        )
}

// TODO, need to make this expandable to all other commands
fn get_command_params(options: &[ResolvedOption<'_>]) -> CommandParams {
    let mut command_parms = CommandParams {
        hours_ago: None,
        messages_ago: None,
        channel: None,
    };
    for resolved_option in options {
        match resolved_option.value {
            ResolvedValue::Integer(val, ..) => {
                if resolved_option.name.eq("messages") {
                    command_parms.messages_ago = Some(val)
                }
                if resolved_option.name.eq("hours_ago") {
                    command_parms.hours_ago = Some(val)
                }
            },
            ResolvedValue::Channel(channel, ..) => {
                command_parms.channel = Some(channel.id);
            },
            _ => {}
        }
    }

    return command_parms;
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
            Err(err) => {
                log::error!("Error occured while trying to create a singular log line: {}", err);
            },
        }
    }
    return chat_logs.iter().rev().cloned().collect();
}

async fn create_chat_log_by_message_count(ctx: &Context, channel_id: ChannelId, amount_of_messages_to_find: i64) -> Vec<ChatLog>{
    let mut chat_logs: Vec<ChatLog> = vec![];
    let mut messages = channel_id.messages_iter(&ctx).boxed();
    let mut i = 0;
    while i < amount_of_messages_to_find {
        let Some(message_result) = messages.next().await else { break };
        match message_result {
            Ok(message) => chat_logs.push(create_single_chat_log_from_message(message)),
            Err(err) => {
                log::error!("An error occured while trying to make the chat log: {}", err)
            },
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
        unix_timestamp: message.timestamp.unix_timestamp(),
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

fn build_embed(summary: String, channel_name: String, model_name: String) -> CreateEmbed {
    return CreateEmbed::new()
        .title(&format!("Summary for {}", channel_name))
        .description(&format!("{}", summary))
        .color(Color::TEAL)
        .footer(CreateEmbedFooter::new( format!("Summary powered by {}", model_name)));
}
