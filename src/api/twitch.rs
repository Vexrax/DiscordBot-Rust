use std::env;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Stream {
    id: String,
    user_id: String,
    user_login: String,
    user_name: String,
    game_id: String,
    game_name: String,
    #[serde(rename = "type")]
    stream_type: String,
    title: String,
    viewer_count: u32,
    started_at: String,
    language: String,
    thumbnail_url: String,
    tag_ids: Vec<String>,
    tags: Vec<String>,
    is_mature: bool,
}

#[derive(Debug, Deserialize)]
struct Pagination {
    cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TwitchResponse {
    data: Vec<Stream>,
    pagination: Pagination,
}

#[derive(Deserialize, Debug)]
struct AccessTokenResponse {
    access_token: String,
    expires_in: u64,
    token_type: String,
}

// TODO, SHOULD PROBABLY USE HELIX LIB FOR THIS STUFF
pub async fn call_twitch() {
    let mut auth_token: String;

    if let Some(token_response) = get_access_token().await {
        auth_token = token_response.access_token;
    } else {
        eprintln!("Failed to retrieve access token.");
        return;
    }

    let username = "vexrax_";
    let url = format!("https://api.twitch.tv/helix/streams?user_login={}", username);

    let client_id = env::var("TWITCH_CLIENT_ID").expect("TWITCH_CLIENT_ID not set in environment");

    // Set up headers
    let mut headers = HeaderMap::new();
    headers.insert("Client-ID", HeaderValue::from_str(&client_id).unwrap());
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", auth_token)).unwrap());

    // Send GET request
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .headers(headers)
        .send()
        .await
        .expect("Failed to fetch Twitch API");

    let twitch_response: TwitchResponse = response
        .json()
        .await
        .expect("Failed to deserialize response");

    println!("{:#?}", twitch_response);
}

pub async fn get_access_token() -> Option<AccessTokenResponse> {
    // Initialize the OAuth2 client with your client ID, client secret, authorization URL, and token URL.


    let client_id = env::var("TWITCH_CLIENT_ID").expect("TWITCH_CLIENT_ID not set in environment");
    let client_secret = env::var("TWITCH_CLIENT_SECRET").expect("TWITCH_CLIENT_SECRET not set in environment");

    let url = format!("https://id.twitch.tv/oauth2/token?client_id={}&client_secret={}&grant_type=client_credentials", client_id, client_secret);

    // Send GET request
    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .send()
        .await
        .expect("Failed to fetch Twitch API");

    if !response.status().is_success() {
        return None;
    }

    let json_result = response.json::<AccessTokenResponse>().await;
    match json_result {
        Ok(token_response) => Some(token_response),
        Err(json_error) => None
    }
}