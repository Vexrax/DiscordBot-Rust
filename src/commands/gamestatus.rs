use std::collections::HashSet;
use std::string::ToString;
use riven::models::league_v4::LeagueEntry;
use serenity::all::{Color, CommandInteraction};
use serenity::builder::{CreateCommand, CreateEmbed, CreateEmbedFooter, CreateMessage};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;
use riven::consts::{QueueType, Team, Champion};
use riven::consts::Division::IV;
use riven::consts::Tier::UNRANKED;
use riven::models::account_v1::Account;
use riven::models::summoner_v4::Summoner;
use crate::commands::business::league_of_legends::{get_current_match_by_riot_account, get_rank_of_player, get_riot_accounts, get_riot_id_from_string, RiotId};

use crate::utils::discord_message::respond_to_interaction;
use crate::api::riot_api::{get_profile_icon_url, get_summoner};
use crate::commands::business::embed::get_embed_for_current_match;


const PLAYERS_IDS: [&str; 11] = [
    "Vexrax#FAKER", "Zafa#NA1", "Earleking#NA1", "rgrou2#NA1", "grant#erino", "Perky#GOAT", "LeeSinners#NA1",
    "Koality Player#NA1", "Soulbert#koggy", "ShadyGecko#1313", "Arcadius#NA1"
];
pub async fn run(_options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    respond_to_interaction(&ctx, &command, &"Checking to see if anyone in boosted is in game...".to_string().to_string()).await;
    let mut players: Vec<RiotId> = vec![];

    for player_id_input in PLAYERS_IDS {
        match get_riot_id_from_string(&player_id_input.to_string()) {
            None => {
                log::info!("Could not find the player {}", player_id_input)
            },
            Some(riot_id) => players.push(riot_id)
        }
    }

    let riot_accounts: Vec<Account> = get_riot_accounts(players).await;
    let mut matches_generated = HashSet::new();
    for riot_account in riot_accounts {
        let current_match;
        match get_current_match_by_riot_account(&riot_account).await {
            Some(curr_match) => current_match = curr_match,
            None => continue,
        }

        if matches_generated.contains(&current_match.game_id) {
            continue;
        }

        let match_embed = match get_embed_for_current_match(&current_match, &riot_account).await {
            None => continue,
            Some(match_embed) => match_embed
        };

        let _ = command.channel_id.send_message(&ctx.http, CreateMessage::new().tts(false).embed(match_embed)).await;
        matches_generated.insert(current_match.game_id);
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("gamestatus").description("Gets the status of the registered players in the server")
}

