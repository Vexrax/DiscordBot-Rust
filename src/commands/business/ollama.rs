use chrono::{NaiveDateTime};
use crate::api::ollama_api::{call_llama3_api_await_response, OllamaMessage};

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
    LLAMA3
}

pub async fn get_summary_of_logs(chat_logs: Vec<ChatLog>, model: Model) -> Option<String> {
    match model {
        Model::GEMMA3 => get_summary_of_logs_gemma(chat_logs).await,
        Model::LLAMA3 => get_summary_of_logs_llama3(chat_logs).await
    }
}

fn fetch_prompt_by_model() -> String {
    let md_string: &'static str = include_str!("prompts/summary.md");
    return md_string.to_string();
}

pub async fn get_summary_of_logs_llama3(chat_logs: Vec<ChatLog>) -> Option<String> {
    let mut log_string: String = convert_chat_logs_to_single_string(chat_logs);

    let msgs: Vec<OllamaMessage> = vec![
        OllamaMessage {
            content: fetch_prompt_by_model(),
            role: "system".to_string(),
        },
        OllamaMessage {
            content: log_string,
            role: "user".to_string(),
        },
    ];

    return call_llama3_api_await_response(msgs).await;
}

pub async fn get_summary_of_logs_gemma(chat_logs: Vec<ChatLog>) -> Option<String> {
    let mut log_string: String = convert_chat_logs_to_single_string(chat_logs);

    let msgs: Vec<OllamaMessage> = vec![
        OllamaMessage {
            content: fetch_prompt_by_model(),
            role: "user".to_string(),
        },
        OllamaMessage {
            content: log_string,
            role: "user".to_string(),
        },
    ];

    return call_llama3_api_await_response(msgs).await;
}

fn convert_chat_logs_to_single_string(chat_logs: Vec<ChatLog>) -> String {
    let mut log_string: String = "".to_string();
    for log in chat_logs {
        let log_line = format!("[{}] {}: {}", NaiveDateTime::from_timestamp_opt(log.unix_timestamp, 0).unwrap(), log.author, log.message);
        log_string = format!("{} {}\n", log_string, log_line);
    }
    return log_string;
}