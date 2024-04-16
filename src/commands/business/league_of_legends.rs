use riven::consts::{Queue, QueueType};
use riven::models::account_v1::Account;
use riven::models::league_v4::LeagueEntry;
use riven::models::match_v5::Match;
use riven::models::spectator_v5::CurrentGameInfo;
use riven::models::summoner_v4::Summoner;
use crate::utils::riot_api::{get_current_match, get_league_entries, get_match_by_id, get_match_ids, get_riot_account, get_summoner};

#[derive(Clone)]
pub struct RiotId {
    pub(crate) name: String,
    pub(crate) tagline: String,
}

pub async fn get_recent_match_ids(summoner: Summoner, start_time_epoch_seconds: i64, valid_queues: Vec<Queue>) -> Vec<String> {
    let mut match_ids_for_all_valid_queues = vec![];
    for queue in valid_queues {
        match_ids_for_all_valid_queues.extend(get_recent_match_ids_for_queue(&*summoner.puuid, queue, start_time_epoch_seconds).await);
    }
    return match_ids_for_all_valid_queues;
}

pub async fn get_recent_match_ids_for_queue(puuid: &str, queue: Queue, start_time_epoch_seconds: i64) -> Vec<String> {
    let mut matches = vec![];
    let mut start_index = 0;
    let amount_to_return = 100;
    let mut data_from_api = get_match_ids_for(puuid, queue, start_time_epoch_seconds, start_index, amount_to_return).await;
    while data_from_api.len() > 0 {
        matches.extend(data_from_api.clone());
        start_index += amount_to_return;
        data_from_api = get_match_ids_for(puuid, queue, start_time_epoch_seconds, start_index, amount_to_return).await;
    }
    return matches;
}

async fn get_match_ids_for(puuid: &str, queue: Queue, start_time_epoch_seconds: i64, start_index: i32, amount_to_search_for: i32) -> Vec<String> {
    return match get_match_ids(puuid, queue, start_time_epoch_seconds, start_index, amount_to_search_for).await {
        Some(matches) => { matches }
        None => { vec![] }
    }
}

pub async fn get_recent_match_data(summoner: Summoner, start_time_epoch_seconds: i64, valid_queues: Vec<Queue>) -> Vec<Match>  {
    let recent_match_ids = get_recent_match_ids(summoner, start_time_epoch_seconds, valid_queues).await;
    let mut match_data: Vec<Match> = vec![];
    for match_id in recent_match_ids {
        match get_match_by_id(&*match_id).await {
            Some(match_data_from_api) => match_data.push(match_data_from_api),
            None => {}
        }
    }

    return match_data;
}

pub async fn get_rank_of_player(ecrypted_summoner_id: String, queue_type: QueueType) -> Option<LeagueEntry> {
    let league_entries= match get_league_entries(&ecrypted_summoner_id).await {
        Some(league_entries_from_api) =>  league_entries_from_api,
        None => return None
    };

    for league in league_entries.iter() {
        if league.queue_type == queue_type {
            return Some(league.clone());
        }
    }

    None
}

pub async fn get_riot_accounts(riot_ids: Vec<RiotId>) -> Vec<Account> {
    let mut accounts: Vec<Account> = vec![];
    for riot_id in riot_ids {
        match get_riot_account(&riot_id.name, &riot_id.tagline).await {
            Some(account) => accounts.push(account),
            None => {}
        }
    }
    return accounts;
}

pub async fn get_current_match_by_riot_account(riot_account: &Account) -> Option<CurrentGameInfo> {
    return get_current_match(&riot_account.puuid).await
}

pub fn get_riot_id_from_string(riot_id: &String) -> Option<RiotId> {
    let mut split_summoner = riot_id.split("#");
    let riot_account_name= match split_summoner.next() {
        Some(riot_account_name ) => riot_account_name,
        None => return None
    };

    let riot_account_tagline= match split_summoner.next() {
        Some(riot_account_tagline) => riot_account_tagline,
        None => return None
    };

    Some(RiotId {
        name: riot_account_name.to_string(),
        tagline: riot_account_tagline.to_string()
    })
}