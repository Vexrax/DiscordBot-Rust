use std::collections::HashMap;
use std::string::ToString;
use reqwest::{Error, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::utils::skynet::get_env;
use crate::utils::skynet_constants::Environment;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub struct LlamaMessage {
    pub content: String,
    pub role: String
}

#[derive(Serialize, Deserialize)]
struct LLamaAPICall {
    model: String,
    messages: Vec<LlamaMessage>,
    stream: bool,
    options: LLamaAPIOptions
}
#[derive(Serialize, Deserialize)]
struct LLamaAPIOptions {
    seed: Option<i32>,
    temperature: Option<f32>
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

const LLAMA3_MODEL: &str = "llama3";

pub fn get_host() -> String {
    return match get_env() {
        Environment::PROD => "http://10.0.0.11",
        Environment::DEV => "http://10.0.0.11"
    }.to_string()
}

pub async fn call_llama3_api_await_response(messages: Vec<LlamaMessage>) -> Option<String> {
    let source = format!("{}:11434/api/chat", get_host());

    let llama_api_call = LLamaAPICall {
        model: LLAMA3_MODEL.to_string(),
        messages: messages,
        stream: false,
        options: LLamaAPIOptions {
            temperature: Some(1.0),
            seed: None,
        }
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