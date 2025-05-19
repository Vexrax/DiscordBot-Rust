use chrono::{NaiveDateTime};
use crate::api::ollama_api::{call_ollama_api_await_response, OllamaMessage};
use std::fmt;
use crate::commands::business::ollama::Model::{GEMMA3, LLAMA3};

#[derive(Clone)]
pub struct ChatLog {
    pub author: String,
    pub message: String,
    pub unix_timestamp: i64,
    pub message_id: u64,
    pub replying_to_message_id: Option<u64>
}

pub enum Model {
    GEMMA3,
    LLAMA3,
    LLAMA4,
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Model::GEMMA3 => "gemma3",
            Model::LLAMA3 => "llama3",
            Model::LLAMA4 => "llama4",
        };
        write!(f, "{}", s)
    }
}

pub async fn get_summary_of_logs(chat_logs: Vec<ChatLog>, model: Model) -> Option<String> {

    match model {
        GEMMA3 => get_summary_of_logs_gemma(chat_logs).await,
        LLAMA3 => get_summary_of_logs_llama3(chat_logs).await,
        Model::LLAMA4 => get_summary_of_logs_llama3(chat_logs).await
    }
}

pub async fn get_summary_of_logs_llama3(chat_logs: Vec<ChatLog>) -> Option<String> {
    let mut log_string: String = convert_chat_logs_to_single_string(chat_logs);

    let msgs: Vec<OllamaMessage> = vec![
        OllamaMessage {
            content: include_str!("prompts/summary.md").to_string(),
            role: "system".to_string(),
        },
        OllamaMessage {
            content: log_string,
            role: "user".to_string(),
        },
    ];

    return call_ollama_api_await_response(msgs, Model::LLAMA4.to_string()).await;
}

pub async fn get_summary_of_logs_gemma(chat_logs: Vec<ChatLog>) -> Option<String> {
    let mut log_string: String = convert_chat_logs_to_single_string(chat_logs);

    let msgs: Vec<OllamaMessage> = vec![
        OllamaMessage {
            content: include_str!("prompts/summary.md").to_string(),
            role: "user".to_string(),
        },
        OllamaMessage {
            content: log_string,
            role: "user".to_string(),
        },
    ];

    return call_ollama_api_await_response(msgs, GEMMA3.to_string()).await;
}

fn convert_chat_logs_to_single_string(chat_logs: Vec<ChatLog>) -> String {
    let mut log_string: String = "".to_string();
    for log in chat_logs {
        let log_line = format!("[{}] {}: {}", NaiveDateTime::from_timestamp_opt(log.unix_timestamp, 0).unwrap(), log.author, log.message);
        log_string = format!("{} {}\n", log_string, log_line);
    }
    return log_string;
}