use std::env;
use riven::consts::{PlatformRoute, Queue, RegionalRoute};
use riven::models::account_v1::Account;
use riven::models::league_v4::LeagueEntry;
use riven::models::match_v5::Match;
use riven::models::spectator_v5::CurrentGameInfo;
use riven::models::summoner_v4::Summoner;
use riven::RiotApi;

const REGION: RegionalRoute = RegionalRoute::AMERICAS;
const PLATFORM: PlatformRoute = PlatformRoute::NA1;
const DDRAGON_BASE: &str = "http://ddragon.leagueoflegends.com";

pub async fn get_riot_account(game_name: &str, tagline: &str) -> Option<Account> {
    let riot_api = get_riot_api();
    match riot_api.account_v1().get_by_riot_id(REGION, game_name, tagline).await {
        Ok(riot_account_maybe) => return riot_account_maybe,
        Err(err) =>  {
            log::error!("Riot api errored, game name: {}, tagline: {}, err {}", game_name, tagline, err);
            None
        }
    }
}

pub async fn get_riot_account_by_puuid(puuid: &str) -> Option<Account> {
    let riot_api = get_riot_api();
    match riot_api.account_v1().get_by_puuid(REGION, puuid).await {
        Ok(riot_account_maybe) => Some(riot_account_maybe),
        Err(err) =>  {
            log::error!("Riot api errored, puuid {}, err {}", puuid, err);
            None
        }
    }
}

pub async fn get_summoner(riot_account: &Account) -> Option<Summoner> {
    let riot_api = get_riot_api();

    match riot_api.summoner_v4().get_by_puuid(PLATFORM, &riot_account.puuid).await {
        Ok(riot_summoner) => return Some(riot_summoner),
        Err(err) =>  {
            log::error!("Riot api errored, err: {}", err);
            None
        }
    }
}

pub async fn get_match_by_id(match_id: &str) -> Option<Match> {
    let riot_api = get_riot_api();
    match riot_api.match_v5().get_match(REGION, match_id).await {
        Ok(league_match) => return league_match,
        Err(err) =>  {
            log::error!("Riot api errored: matchId: {}, err: {}", match_id, err);
            None
        }
    }
}

pub async fn get_match_ids(puuid: &str, queue: Queue, start_time_epoch_seconds: i64, start_index: i32, amount_to_search_for: i32) -> Option<Vec<String>> {
    let riot_api = get_riot_api();
    match riot_api.match_v5().get_match_ids_by_puuid(REGION, puuid, Some(amount_to_search_for), None, Some(queue), Some(start_time_epoch_seconds), Some(start_index), None).await {
        Ok(matches) => Some(matches),
        Err(err) =>  {
            log::error!("Riot api errored: puuid: {}, err {}", puuid, err);
            None
        }
    }
}

pub async fn get_current_match(puuid: &String) -> Option<CurrentGameInfo> {
    let riot_api = get_riot_api();
    match riot_api.spectator_v5().get_current_game_info_by_puuid(PLATFORM, &puuid).await {
        Ok(current_match) => return current_match,
        Err(err) =>  {
            log::error!("Riot api errored: {}", err);
            None
        }
    }
}

pub async fn get_league_entries(encrypted_summoner_id: &String) -> Option<Vec<LeagueEntry>> {
    let riot_api = get_riot_api();
    match riot_api.league_v4().get_league_entries_for_summoner(PLATFORM, &encrypted_summoner_id).await {
        Ok(league_entires) => return Some(league_entires),
        Err(err) =>  {
            log::error!("Riot api errored: {}", err);
            None
        }
    }
}

pub async fn get_current_patch() -> String {
    let source = format!("{}/api/versions.json", DDRAGON_BASE);
    return match reqwest::get(source).await {
        Ok(response) => {
            response.json::<serde_json::Value>().await.unwrap().get(0).unwrap().to_string().replace('"', "")
        }
        Err(err) => {
            eprintln!("Something went wrong while trying to find current patch: {}", err);
            "14.1.1".to_string()
        }
    }
}

pub async fn get_profile_icon_url(profile_icon_id: i32) -> String {
    let cdn_base: String = format!("{}/cdn/{}", DDRAGON_BASE, get_current_patch().await.as_str());
    let ddragon_base_icon: String = format!("{}/img/profileicon/", cdn_base);
    return format!("{}{}.png", ddragon_base_icon, profile_icon_id)
}

fn get_riot_api() -> RiotApi {
    let api_key = env::var("RIOT").expect("Expected riotAPI KEY in environment");
    return RiotApi::new(api_key);
}
