use std::string::ToString;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::utils::skynet::get_env;
use crate::utils::skynet_constants::Environment;

#[derive(Serialize, Deserialize, Debug)]
pub struct OllamaMessage {
    pub content: String,
    pub role: String
}

#[derive(Serialize, Deserialize)]
struct OllamaAPICall {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: bool,
    options: OllamaAPIOptions
}
#[derive(Serialize, Deserialize)]
struct OllamaAPIOptions {
    seed: Option<i32>,
    temperature: Option<f32>
}

#[derive(Serialize, Deserialize, Debug)]
struct OllamaResponse {
    // model: String,
    created_at: String,
    message: OllamaMessage,
    done: bool,
    total_duration: u64,
    load_duration: u64,
    prompt_eval_duration: u64,
    eval_count: u64,
    eval_duration: u64
}


pub fn get_host() -> String {
    match get_env() {
        Environment::PROD => "http://10.0.0.2",
        Environment::DEV => "http://10.0.0.2"
    }.to_string()
}

pub async fn call_ollama_api_await_response(messages: Vec<OllamaMessage>, model: String) -> Option<String> {
    let source = format!("{}:11434/api/chat", get_host());

    let ollama_api_call = OllamaAPICall {
        model: model,
        messages: messages,
        stream: false,
        options: OllamaAPIOptions {
            temperature: Some(1.0),
            seed: None,
        }
    };

    let client = reqwest::Client::new();
    let res = client.post(source)
        .json(&json!(ollama_api_call))
        .send()
        .await;

    let serialized_result = match res {
        Ok(okay_res) => okay_res.json::<OllamaResponse>().await,
        Err(err) => {
            log::error!("Error occurred while calling ollama {}", err);
            return None;
        }
    };


    let ollama_response = match serialized_result {
        Ok(ok) => ok,
        Err(err) => {
            log::error!("Error occurred while calling deserializing from ollama {}", err);
            return None;
        }
    };

    return Some(ollama_response.message.content);
}