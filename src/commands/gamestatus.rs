use std::env;
use std::fmt::format;

use riven::models::league_v4::LeagueEntry;
use serenity::all::{CommandInteraction};
use serenity::builder::{CreateCommand, CreateEmbed, CreateMessage};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;
use riven::RiotApi;
use riven::consts::{PlatformRoute, QueueType, Team, Champion};
use riven::models::summoner_v4::Summoner;

use crate::utils::discord_message::respond_to_interaction;

#[derive(Clone)]
struct MatchPlayer {
    rank: Option<LeagueEntry>,
    champion_id: Champion,
    profile_icon_id: i64,
    team_id: Team,
    summoner_name: String
}

pub async fn run(_options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {

    respond_to_interaction(&ctx, &command, &format!("Checking to see if anyone in boosted is in game...").to_string()).await;

    // TODO dont hardcode here
    let game_name = "Zafa";
    let tagline = "NA1";

    let api_key = env::var("RIOT").expect("Expected riotAPI KEY in environment");
    let riot_api = RiotApi::new(api_key);    

    let riot_account = riot_api.account_v1().get_by_riot_id(riven::consts::RegionalRoute::AMERICAS, game_name, tagline)
    .await
    .expect("Get Summoner Failed")
    .expect("There is no summoner with that name");

    let riot_summoner = riot_api.summoner_v4().get_by_puuid(PlatformRoute::NA1, &riot_account.puuid).await
    .expect("Get Riot Summoner Failed");

    let current_match = riot_api.spectator_v4().get_current_game_info_by_summoner(PlatformRoute::NA1, &riot_summoner.id).await
    .expect("Get Match Failed")
    .expect("Summoner not in game");

    let mut match_players: Vec<MatchPlayer> = [].to_vec();
    for participant in current_match.participants {
        let rank: Option<LeagueEntry> = get_rank_of_player(participant.summoner_id, QueueType::RANKED_SOLO_5x5, &riot_api).await;  
        match_players.push(MatchPlayer { rank: rank, champion_id: participant.champion_id, profile_icon_id: participant.profile_icon_id, team_id: participant.team_id, summoner_name: participant.summoner_name })
    }

    let match_embed = build_embed(game_name.to_string(), tagline.to_string(), riot_summoner, match_players);
    let _ = command.channel_id.send_message(&ctx.http,CreateMessage::new().tts(false).embed(match_embed)).await;
}

fn build_embed(main_player_game_name: String, main_player_tagline: String, main_player_riot_summoner: Summoner, match_players: Vec<MatchPlayer>) -> CreateEmbed {
    // TODO build cool embed!
    let mut blue_fields: Vec<(String, String, bool)> = vec![];
    let mut red_fields: Vec<(String, String, bool)> = vec![];

    for match_player in match_players {
        let display_rank = match_player.rank.as_ref()
            .map(|val| format!("{:?} {:?}", val.tier.unwrap(), val.rank.unwrap())) // TODO might need to handle this better
            .unwrap_or_else(|| "UNRANKED".to_string());
        let title = format!("{} ({})",match_player.summoner_name, match_player.champion_id.identifier().unwrap_or_else(|| "Unknown Champ?"));

        if (match_player.team_id == Team::BLUE) {
            blue_fields.push((title, display_rank, true));
        } else if (match_player.team_id == Team::RED) {
            red_fields.push((title, display_rank, true));
        }
    }

    // TODO move this out
    let ddragon_base: &str = "http://ddragon.leagueoflegends.com/cdn/10.25.1";
    let ddragon_base_icon: String = format!("{}/img/profileicon/", ddragon_base);

    let embed = CreateEmbed::new()
        .title(format!("{}#{}'s Game", main_player_game_name, main_player_tagline))
        .description(&format!("Some Description"))
        .thumbnail(format!("{}{}.png", ddragon_base_icon, main_player_riot_summoner.profile_icon_id))
        .fields(blue_fields.into_iter())
        .fields(red_fields.into_iter());
    return embed;
}

pub async fn get_rank_of_player(ecrypted_summoner_id: String, queue_type: QueueType, riot_api: &RiotApi) -> Option<LeagueEntry> {
    let league_entries = riot_api.league_v4().get_league_entries_for_summoner(PlatformRoute::NA1, &ecrypted_summoner_id).await
        .expect("Could not find");

        for league in league_entries.iter() {
            if league.queue_type == queue_type {
                return Some(league.clone());
            }
        }
    
        None 
}

pub fn register() -> CreateCommand {
    CreateCommand::new("gamestatus").description("Gets the status of the registered players in the server")
}