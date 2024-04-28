use std::collections::HashMap;
use reqwest::{Error, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;

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

pub async fn summarize_chat_logs_with_llama(logs_as_string_with_newlines: String) -> Option<String> {
    let dev_api = "http://10.0.0.11"; // lmao
    let prod_api = "http://localhost"; // Lmao
    let source = format!("http://10.0.0.11:11434/api/chat");
    let msgs: Vec<LlamaMessage> = vec![
        LlamaMessage {
            content: "Summarize chat logs that you are provided with, every newline begins with (unix timestamp) and then [author] then the <message>. Do not tell the user what you are doing, just provide the summary".to_string(),
            role: "system".to_string(),
        },
        LlamaMessage {
            content: logs_as_string_with_newlines,
            role: "user".to_string(),
        },
    ];

    let llama_api_call = LLamaAPICall {
        model: "llama3".to_string(),
        messages: msgs,
        stream: false
    };

    let serialized_api_call = match serde_json::to_string(&llama_api_call) {
        Ok(str) => str,
        Err(_) => return None,
    };

    println!("{}", serialized_api_call);

    let client = reqwest::Client::new();
    let res = client.post(source)
        .json(&json!(llama_api_call))
        .send()
        .await;

    let serialized_result = match res {
        Ok(okay_res) => okay_res.json::<LlamaResponse>().await,
        Err(err) => {
            log::info!("Error occured while calling llama {}", err);
            return None;
        }
    };

    let llama_response = match serialized_result {
        Ok(ok) => ok,
        Err(err) => {
            log::info!("Error occured while calling deserializing from llama {}", err);
            return None;
        }
    };

    return Some(llama_response.message.content);
}