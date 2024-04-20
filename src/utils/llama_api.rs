use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct LlamaMessage {
    content: String,
    Role: String
}

#[derive(Serialize, Deserialize)]
struct LLamaAPICall {
    messages: Vec<LlamaMessage>,
    temperature: f64,
    max_tokens: i32
}

pub async fn call_llama(logs_as_string_with_newlines: String) {
    let source = format!("http://10.0.0.11:3001/v1/chat/completions");
    
    let msgs: Vec<LlamaMessage> = vec![
        LlamaMessage {
            content: "Summarize chat logs that you are provided with, every newline begins with (unix timestamp) and then [author] then the <message>. Do not tell the user what you are doing, just provide the summary".to_string(),
            Role: "system".to_string(),
        },
        LlamaMessage {
            content: logs_as_string_with_newlines,
            Role: "user".to_string(),
        },
    ];

    let llama_api_call = LLamaAPICall {
        messages: msgs,
        temperature: 0.7,
        max_tokens: 4069,
    };

    let serialized_api_call = match serde_json::to_string(&llama_api_call) {
        Ok(str) => str,
        Err(_) => return,
    };

    println!("{}", serialized_api_call);

    let client = reqwest::Client::new();
    let res = client.post(source)
        .header("accept", "application/json")
        .header("Content-Type", "application/json")
        .body(serialized_api_call)
        .send()
        .await;
    println!("{:?}", res)
}