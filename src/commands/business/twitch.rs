use serenity::all::{ChannelId, Context, CreateMessage};
use crate::api::twitch::{get_twitch_channel_status, StreamInfo};
use crate::commands::business::embed::{get_failure_embed};

const STREAM_USERNAMES: [&str; 5] = [
    "Vexrax_", "koality_player", "helper08", "earleking", "corejj",
];

const CHANNEL_ID_FOR_STREAM_ANNOUNCEMENT: u64 =  373234281708912643;

// TODO, SHOULD PROBABLY USE HELIX LIB FOR THIS STUFF
pub async fn check_if_channels_are_live(ctx: &Context) {
    let twitch_response;
    match get_twitch_channel_status(&STREAM_USERNAMES).await {
        None => return,
        Some(data) => twitch_response = data,
    }

    // TODO filter out channels based on if we posted about them recently
    // TODO filter out live_channels, if they have been live for over 10 mins, filter them out

    for live_channel in twitch_response.data {
        let channel = match ctx.http.get_channel(ChannelId::from(CHANNEL_ID_FOR_STREAM_ANNOUNCEMENT)).await {
            Ok(channel) => channel,
            Err(err) => {
                log::error!("Could not find the channel {}, err: {}", CHANNEL_ID_FOR_STREAM_ANNOUNCEMENT, err);
                continue;
            }
        };

        let _ = channel.id().send_message(&ctx.http, CreateMessage::new().content(format!("A Boosted member is LIVE: https://twitch.tv/{}", &live_channel.user_name))).await;
    }
}

// TODO function impl
pub async fn has_posted_about_member_recently(stream_info: StreamInfo) -> bool {
    return false;
}

// TODO function impl
pub async fn has_channel_been_live_for_atleast_ten_minutes(stream_info: StreamInfo) -> bool {
    return true;
}
