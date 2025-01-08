use std::env;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StreamInfo {
    pub id: String,
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
    pub game_id: String,
    pub game_name: String,
    #[serde(rename = "type")]
    pub stream_type: String,
    pub title: String,
    pub viewer_count: u32,
    pub started_at: String,
    pub language: String,
    pub thumbnail_url: String,
    pub tag_ids: Vec<String>,
    pub tags: Vec<String>,
    pub is_mature: bool,
}

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TwitchResponse {
    pub data: Vec<StreamInfo>,
    pub pagination: Pagination,
}

#[derive(Deserialize, Debug)]
struct AccessTokenResponse {
    access_token: String,
    expires_in: u64,
    token_type: String,
}

const TWITCH_CLIENT_ID_ENV_VAR_NAME: &str = "TWITCH_CLIENT_ID";
const TWITCH_CLIENT_SECRET_ENV_VAR_NAME: &str = "TWITCH_CLIENT_SECRET";

// TODO, SHOULD PROBABLY USE HELIX LIB FOR THIS STUFF
pub async fn get_twitch_channel_status(usernames: &[&str]) -> Option<TwitchResponse> {
    let auth_token: String;

    if let Some(token_response) = get_access_token().await {
        auth_token = token_response.access_token;
    } else {
        eprintln!("Failed to retrieve access token.");
        return None;
    }

    let query_params: String = usernames
        .iter()
        .map(|username| format!("user_login={}", username))
        .collect::<Vec<String>>()
        .join("&");

    let url = format!("https://api.twitch.tv/helix/streams?{}", query_params);

    let client_id = env::var(TWITCH_CLIENT_ID_ENV_VAR_NAME).expect("TWITCH_CLIENT_ID not set in environment");

    let mut headers = HeaderMap::new();
    headers.insert("Client-ID", HeaderValue::from_str(&client_id).unwrap());
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", auth_token)).unwrap());

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .headers(headers)
        .send()
        .await
        .expect("Failed to fetch Twitch API");

    if !response.status().is_success() {
        eprintln!("Failed to fetch streams: {}", response.status());
        return None;
    }

    let json_result = response.json::<TwitchResponse>().await;

    match json_result {
        Ok(token_response) => Some(token_response),
        Err(json_error) => {
            eprintln!("Failed to parse JSON response: {:?}", json_error);
            None
        }
    }
}

pub async fn get_access_token() -> Option<AccessTokenResponse> {
    let client_id = env::var(TWITCH_CLIENT_ID_ENV_VAR_NAME).expect("TWITCH_CLIENT_ID not set in environment");
    let client_secret = env::var(TWITCH_CLIENT_SECRET_ENV_VAR_NAME).expect("TWITCH_CLIENT_SECRET not set in environment");

    let url = format!("https://id.twitch.tv/oauth2/token?client_id={}&client_secret={}&grant_type=client_credentials", client_id, client_secret);

    // Send POST request
    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .send()
        .await
        .expect("Failed to fetch Twitch API");

    if !response.status().is_success() {
        eprintln!("Failed to fetch access token: {}", response.status());
        return None;
    }

    let json_result = response.json::<AccessTokenResponse>().await;
    match json_result {
        Ok(token_response) => Some(token_response),
        Err(json_error) => {
            eprintln!("Failed to parse JSON response: {:?}", json_error);
            None
        }
    }
}
