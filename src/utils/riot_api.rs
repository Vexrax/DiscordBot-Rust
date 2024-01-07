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

pub async fn get_riot_account(game_name: &str, tagline: &str) -> Option<Account> {
    let riot_api = get_riot_api();
    match riot_api.account_v1().get_by_riot_id(REGION, game_name, tagline).await {
        Ok(riot_account_maybe) => return riot_account_maybe,
        Err(err) =>  {
            eprintln!("Riot api errored: {}", err);
            None
        }
    }
}

pub async fn get_summoner(riot_account: &Account) -> Option<Summoner> {
    let riot_api = get_riot_api();

    match riot_api.summoner_v4().get_by_puuid(PLATFORM, &riot_account.puuid).await {
        Ok(riot_summoner) => return Some(riot_summoner),
        Err(err) =>  {
            eprintln!("Riot api errored: {}", err);
            None
        }
    }
}

pub async fn get_match_by_id(match_id: &str) -> Option<Match> {
    let riot_api = get_riot_api();
    match riot_api.match_v5().get_match(REGION, match_id).await {
        Ok(league_match) => return league_match,
        Err(err) =>  {
            eprintln!("Riot api errored: {}", err);
            None
        }
    }
}

pub async fn get_match_ids(puuid: &str, queue: Queue, start_time_epoch_seconds: i64, start_index: i32, amount_to_search_for: i32) -> Option<Vec<String>> {
    let riot_api = get_riot_api();
    match riot_api.match_v5().get_match_ids_by_puuid(REGION, puuid, Some(amount_to_search_for), None, Some(queue), Some(start_time_epoch_seconds), Some(start_index), None).await {
        Ok(matches) => Some(matches),
        Err(err) =>  {
            eprintln!("Riot api errored: {}", err);
            None
        }
    }
}

pub async fn get_current_match(summoner: &Summoner) -> Option<CurrentGameInfo> {
    let riot_api = get_riot_api();
    match riot_api.spectator_v4().get_current_game_info_by_summoner(PLATFORM, &summoner.id).await {
        Ok(current_match) => return current_match,
        Err(err) =>  {
            eprintln!("Riot api errored: {}", err);
            None
        }
    }
}

pub async fn get_league_entries(summoner_id: &String) -> Option<Vec<LeagueEntry>> {
    let riot_api = get_riot_api();
    match riot_api.league_v4().get_league_entries_for_summoner(PLATFORM, &summoner_id).await {
        Ok(league_entires) => return Some(league_entires),
        Err(err) =>  {
            eprintln!("Riot api errored: {}", err);
            None
        }
    }
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
