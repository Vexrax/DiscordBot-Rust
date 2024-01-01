use std::env;
use riven::consts::{PlatformRoute, Queue, RegionalRoute};
use riven::models::account_v1::Account;
use riven::models::league_v4::LeagueEntry;
use riven::models::match_v5::Match;
use riven::models::spectator_v4::CurrentGameInfo;
use riven::models::summoner_v4::Summoner;
use riven::RiotApi;

const REGION: RegionalRoute = RegionalRoute::AMERICAS;
const PLATFORM: PlatformRoute = PlatformRoute::NA1;

pub async fn get_riot_account(game_name: &str, tagline: &str) -> riven::Result<Option<Account>> {
    let riot_api = get_riot_api();
    return riot_api.account_v1().get_by_riot_id(REGION, game_name, tagline).await
}

pub async fn get_summoner(riot_account: &Account) -> riven::Result<Summoner> {
    let riot_api = get_riot_api();
    return riot_api.summoner_v4().get_by_puuid(PLATFORM, &riot_account.puuid).await;
}

pub async fn get_match_by_id(match_id: &str) -> riven::Result<Option<Match>> {
    let riot_api = get_riot_api();
    return riot_api.match_v5().get_match(REGION, match_id).await;
}

pub async fn get_match_ids(puuid: &str, queue: Queue, start_time_epoch_seconds: i64, start_index: i32, amount_to_search_for: i32) -> riven::Result<Vec<String>> {
    let riot_api = get_riot_api();
    return riot_api.match_v5().get_match_ids_by_puuid(REGION, puuid, Some(amount_to_search_for), None, Some(queue), Some(start_time_epoch_seconds), Some(start_index), None).await
}

pub async fn get_current_match(summoner: &Summoner) -> riven::Result<Option<CurrentGameInfo>> {
    let riot_api = get_riot_api();
    return riot_api.spectator_v4().get_current_game_info_by_summoner(PLATFORM, &summoner.id).await;
}

pub async fn get_league_entries(summoner_id: &String) -> riven::Result<Vec<LeagueEntry>> {
    let riot_api = get_riot_api();
    return riot_api.league_v4().get_league_entries_for_summoner(PLATFORM, &summoner_id).await
}

pub fn get_profile_icon_url(profile_icon_id: i32) -> String {
    let ddragon_base = "http://ddragon.leagueoflegends.com/cdn/13.24.1"; //TODO prob need to get this dynamically
    let ddragon_base_icon = format!("{}/img/profileicon/", ddragon_base);
    return format!("{}{}.png", ddragon_base_icon, profile_icon_id)
}

fn get_riot_api() -> RiotApi {
    let api_key = env::var("RIOT").expect("Expected riotAPI KEY in environment");
    return RiotApi::new(api_key);
}
