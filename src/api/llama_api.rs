use std::collections::HashMap;
use std::string::ToString;
use reqwest::{Error, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::utils::skynet::get_env;
use crate::utils::skynet_constants::Environment;

#[derive(Serialize, Deserialize, Debug)]
struct LlamaMessage {
    content: String,
    role: String
}

#[derive(Serialize, Deserialize)]
struct LLamaAPICall {
    model: String,
    messages: Vec<LlamaMessage>,
    stream: bool
}

#[derive(Serialize, Deserialize, Debug)]
struct LlamaResponse {
    model: String,
    created_at: String,
    message: LlamaMessage,
    done: bool,
    total_duration: u64,
    load_duration: u64,
    prompt_eval_duration: u64,
    eval_count: u64,
    eval_duration: u64
}

const PROMPT: &str = "Summarize the discord chat logs that you are provided with, every newline begins with <[message id]> and then the (unix timestamp) and then [author] then the <message>. \
    The summary should reference the individuals in the conversation by name and what they are talking about with other individuals.\
    Do not tell the user what you are doing, just provide the summary.";

const LLAMA3_MODEL: &str = "llama3";

pub fn get_host() -> String {
    return match get_env() {
        Environment::PROD => "http://localhost",
        Environment::DEV => "http://10.0.0.11"
    }.to_string()
}

pub async fn summarize_chat_logs_with_llama(logs_as_string_with_newlines: String) -> Option<String> {
    let source = format!("{}:11434/api/chat", get_host());

    let msgs: Vec<LlamaMessage> = vec![
        LlamaMessage {
            content: PROMPT.to_string(),
            role: "system".to_string(),
        },
        LlamaMessage {
            content: logs_as_string_with_newlines,
            role: "user".to_string(),
        },
    ];

    let llama_api_call = LLamaAPICall {
        model: LLAMA3_MODEL.to_string(),
        messages: msgs,
        stream: false
    };

    let client = reqwest::Client::new();
    let res = client.post(source)
        .json(&json!(llama_api_call))
        .send()
        .await;

    let serialized_result = match res {
        Ok(okay_res) => okay_res.json::<LlamaResponse>().await,
        Err(err) => {
            log::error!("Error occurred while calling llama {}", err);
            return None;
        }
    };

    let llama_response = match serialized_result {
        Ok(ok) => ok,
        Err(err) => {
            log::error!("Error occurred while calling deserializing from llama {}", err);
            return None;
        }
    };

    return Some(llama_response.message.content);
}