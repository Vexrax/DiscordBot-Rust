use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::results::InsertOneResult;
use riven::consts::{Queue, QueueType};
use riven::models::account_v1::Account;
use riven::models::league_v4::LeagueEntry;
use riven::models::match_v5::Match;
use riven::models::spectator_v5::CurrentGameInfo;
use riven::models::summoner_v4::Summoner;
use crate::api::riot_api::{get_current_match, get_league_entries, get_match_by_id, get_match_ids, get_riot_account};
use crate::utils::mongo::get_mongo_client;

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
    return get_matches(recent_match_ids).await;
}

pub async fn get_matches(match_ids: Vec<String>) -> Vec<Match> {
    let mut match_data: Vec<Match> = vec![];
    for match_id in match_ids {
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

// Interface for cache vs riot match fetching
pub async fn get_league_matches(match_ids: Vec<i64>) -> Vec<Match> {
    let mut matches = vec![];
    for match_id in match_ids {
        let na_match_id = format!("NA1_{}", match_id);
        let full_league_match = get_full_league_match_from_db(na_match_id.clone()).await;
        match full_league_match {
            None => {
                match get_matches(vec![na_match_id]).await.first() {
                    Some(league_match) => {
                        add_league_match_to_db(league_match.clone()).await;
                        matches.push(league_match.clone());
                    }
                    None => {}
                }
            }
            Some(full_match) => matches.push(full_match)
        }
    }

    return matches;
}

const LEAGUE_MATCH_DB_NAME: &str = "Matches_2";

async fn add_league_match_to_db(league_match: Match) -> Option<InsertOneResult> {
    let database_result = get_mongo_client().await;

    match database_result {
        Ok(db) => {
            let collection = db.collection::<Match>(LEAGUE_MATCH_DB_NAME);
            collection.insert_one(league_match, None).await.ok()
        },
        Err(err) => {
            log::error!("Error: something went wrong when trying to add a league match to the DB: {}", err);
            None
        }
    }
}

async fn get_full_league_match_from_db(match_id: String) -> Option<Match> { // TODO make this an option
    let database = match get_mongo_client().await {
        Ok(db) => db,
        Err(err) => {
            log::error!("An error occurred while trying to get the db: {}", err);
            return None;
        }
    };

    let typed_collection = database.collection::<Match>(LEAGUE_MATCH_DB_NAME);
    let cursor = match typed_collection.find(doc! { "metadata.matchId": match_id }, None).await { // TODO use the correct value here
        Ok(quote_cursor) => quote_cursor,
        Err(err) => {
            log::error!("An error occurred while trying to find the matches: {}", err);
            return None;
        }
    };

    match cursor.try_collect().await.unwrap_or_else(|_| vec![]).first() {
        None => None,
        Some(riot_match) => Some(riot_match.clone())
    }
}