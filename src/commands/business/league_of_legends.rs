use riven::consts::Queue;
use riven::models::match_v5::Match;
use riven::models::summoner_v4::Summoner;
use crate::utils::riot_api::{get_match_by_id, get_match_ids};

pub async fn get_recent_match_ids(summoner: Summoner, start_time_epoch_seconds: i64) -> Vec<String> {
    // These are valid queues for the scouting usecase
    let valid_queues = [Queue::SUMMONERS_RIFT_NORMAL_QUICKPLAY_, Queue::SUMMONERS_RIFT_5V5_DRAFT_PICK, Queue::SUMMONERS_RIFT_5V5_RANKED_FLEX, Queue::SUMMONERS_RIFT_5V5_RANKED_SOLO, Queue::CUSTOM];
    let mut matches_for_all_valid_queues = vec![];
    for queue in valid_queues {
        matches_for_all_valid_queues.extend(get_recent_match_ids_for_queue(&*summoner.puuid, queue, start_time_epoch_seconds).await);
    }
    return matches_for_all_valid_queues;
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
        Ok(matches) => { matches }
        Err(err) => { vec![] }
    }
}

pub async fn get_recent_match_data(summoner: Summoner, start_time_epoch_seconds: i64) -> Vec<Match>  {
    let recent_match_ids = get_recent_match_ids(summoner, start_time_epoch_seconds).await;
    let mut match_data: Vec<Match> = vec![];
    for match_id in recent_match_ids {
        match get_match_by_id(&*match_id).await {
            Ok(match_data_from_api) => { match_data.push(match_data_from_api.unwrap()); }
            Err(err) => {
                // Do nothing, the match doesnt exist;
            }
        }
    }

    return match_data;
}