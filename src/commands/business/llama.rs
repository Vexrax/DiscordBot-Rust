use chrono::{NaiveDateTime};
use crate::api::llama_api::{call_llama3_api_await_response, LlamaMessage};

#[derive(Clone)]
pub struct ChatLog {
    pub author: String,
    pub message: String,
    pub unix_timestamp: i64,
    pub message_id: u64,
    pub replying_to_message_id: Option<u64>
}

const SUMMARY_PROMPT: &str = "You have a set of Discord logs in the following format:\n
[2024-05-10 15:20:01] User1: Hey, what's up?\n
[2024-05-10 15:20:05] User2: Not much, just chilling. How about you?\n
[2024-05-10 15:20:12] User1: Same here. Did you see the latest update?\n
[2024-05-10 15:20:18] User2: Yeah, it looks pretty cool. Have you tried it yet?\n
[2024-05-10 15:20:25] User1: Not yet, planning to do it later.\n\
Summarize the provided Discord chat logs. Reference the individuals by name in the conversation and describe what each person is talking about with others.\
Do not comment on the format of the logs.\
Ensure that the summary does not tell the user that you are providing a summary";

pub async fn get_summary_of_logs(chat_logs: Vec<ChatLog>) -> Option<String> {
    let mut log_string: String = "".to_string();

    for log in chat_logs {
        let log_line = format!("[{}] {}: {}", NaiveDateTime::from_timestamp_opt(log.unix_timestamp, 0).unwrap(), log.author, log.message);
        log_string = format!("{} {}\n", log_string, log_line);
    }

    let msgs: Vec<LlamaMessage> = vec![
        LlamaMessage {
            content: SUMMARY_PROMPT.to_string(),
            role: "system".to_string(),
        },
        LlamaMessage {
            content: log_string,
            role: "user".to_string(),
        },
    ];

    return call_llama3_api_await_response(msgs).await;
}
