use serenity::all::{ChannelId, Context, CreateMessage};
use crate::api::twitch::get_twitch_channel_status;
use crate::commands::business::embed::{get_failure_embed, get_live_twitch_embed};

const STREAM_USERNAMES: [&str; 5] = [
    "Vexrax_", "koality_player", "helper08", "earleking", "granterino",
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
        let embed = get_live_twitch_embed(live_channel.user_name, live_channel.thumbnail_url, live_channel.title);

        let channel = match ctx.http.get_channel(ChannelId::from(CHANNEL_ID_FOR_STREAM_ANNOUNCEMENT)).await {
            Ok(channel) => channel,
            Err(err) => {
                log::error!("Could not find the channel {}, err: {}", CHANNEL_ID_FOR_STREAM_ANNOUNCEMENT, err);
                continue;
            }
        };

        // TODO make this better
        let _ = channel.id().send_message(&ctx.http, CreateMessage::new().tts(false).embed(embed)).await;

    }



    // TODO create embeds
}
