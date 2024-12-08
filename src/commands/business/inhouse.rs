use std::collections::{HashMap, HashSet};
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::results::{DeleteResult, InsertOneResult};
use riven::consts::Team;
use serde::{Deserialize, Serialize};
use serenity::all::CreateEmbed;
use crate::api::riot_api::get_riot_account_by_puuid;
use crate::commands::business::embed::{get_db_add_failure_embed, get_embed_for_current_match};
use crate::commands::business::league_of_legends::{get_current_match_by_riot_account, get_league_matches, get_riot_accounts, get_riot_id_from_string, RiotId};
use crate::utils::mongo::get_mongo_client;

const INHOUSE_MATCH_DB_NAME: &str = "InHouses";
const PLAYER_STAT_DB_NAME: &str = "Stats";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InhouseMatch {
    match_id: i64,
    added_by: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayerStat {
    created_at: i64,
    riot_full_tag: String,
    player_stat: PlayerStatData
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayerStatData {
    games: i32,
    wins: i32,
    losses: i32,
    champions: HashSet<i16>, // Use the champion ids

    total_damage: i32,
    total_game_time: i32,
    total_gold: i32,
    total_damage_taken: i32,
    total_solo_kills: i32,
    total_kills: i32,
    total_deaths: i32,
    total_assists: i32,
    total_cs: i32,
    total_vs: i32,

    total_turrets: i32,
    total_plates: i32,
    total_grubs: i32,
    total_dragons: i32,
    total_barons: i32,

    total_team_damage: i32,
    total_team_game_time: i32,
    total_team_gold: i32,
    total_team_damage_taken: i32,
    total_team_solo_kills: i32,
    total_team_kills: i32,
    total_team_deaths: i32,
    total_team_assists: i32,
    total_team_cs: i32,
    total_team_vs: i32,

    // cs_diff_10: i64,
    // gold_diff_10: i64,
    // xp_diff_10: i64,
    // cs_diff_20: i64,
    // gold_diff_20: i64,
    // xp_diff_20: i64,
}

impl PlayerStatData {
    fn update(&mut self, other: &PlayerStatData) {
        self.games += other.games;
        self.wins += other.wins;
        self.losses += other.losses;
        self.total_damage += other.total_damage;
        self.total_game_time += other.total_game_time;
        self.total_gold += other.total_gold;
        self.total_damage_taken += other.total_damage_taken;
        self.total_solo_kills += other.total_solo_kills;
        self.total_kills += other.total_kills;
        self.total_deaths += other.total_deaths;
        self.total_assists += other.total_assists;
        self.total_cs += other.total_cs;
        self.total_vs += other.total_vs;
        self.total_turrets += other.total_turrets;
        self.total_plates += other.total_plates;
        self.total_grubs += other.total_grubs;
        self.total_dragons += other.total_dragons;
        self.total_barons += other.total_barons;
        self.total_team_damage += other.total_team_damage;
        self.total_team_game_time += other.total_team_game_time;
        self.total_team_gold += other.total_team_gold;
        self.total_team_damage_taken += other.total_team_damage_taken;
        self.total_team_solo_kills += other.total_team_solo_kills;
        self.total_team_kills += other.total_team_kills;
        self.total_team_deaths += other.total_team_deaths;
        self.total_team_assists += other.total_team_assists;
        self.total_team_cs += other.total_team_cs;
        self.total_team_vs += other.total_team_vs;
        self.champions.extend(other.champions.clone());
    }
}

async fn get_stat(full_name_and_tagline: &String) {
    // Check if theres a new match available thats not in the cache
    // run thru each players stats and add in the new values from the new match
    // add the new match to the cache
}

pub async fn full_refresh_stat() {
    let inhouse_matches = get_all_inhouse_matches().await;
    let mut match_ids = vec![];
    println!("{:?}", match_ids);
    for inhouse_match in inhouse_matches {
        match_ids.push(inhouse_match.match_id);
    }

    let league_matches = get_league_matches(match_ids).await;
    let mut stats: HashMap<String, PlayerStatData> = HashMap::new(); // PUUID to PlayerStat
    for league_match in league_matches {
        let winning_team_id = if league_match.info.teams.first().unwrap().team_id == Team::BLUE
            && league_match.info.teams.first().unwrap().win {
            Team::BLUE
        } else {
            Team::RED
        };

        let mut stats_to_add: HashMap<String, PlayerStatData> = HashMap::new(); // PUUID to PlayerStat

        let mut total_team_damage: i32 = 0;
        let mut total_team_game_time: i32 = 0;
        let mut total_team_gold: i32 = 0;
        let mut total_team_damage_taken: i32 = 0;
        let mut total_team_solo_kills: i32 = 0;
        let mut total_team_kills: i32 = 0;
        let mut total_team_deaths: i32 = 0;
        let mut total_team_assists: i32 = 0;
        let mut total_team_cs: i32 = 0;
        let mut total_team_vs: i32 = 0;

        for participant in league_match.info.participants {
            let challenges = participant.challenges.unwrap();

            let puuid = participant.puuid.clone();
            let win = if participant.team_id == winning_team_id { 1 } else { 0 };
            let loss = if participant.team_id != winning_team_id { 1 } else { 0 };
            let games = 1;
            let champions = HashSet::from([participant.champion_id.unwrap().0]);
            let damage = participant.physical_damage_dealt_to_champions + participant.magic_damage_dealt_to_champions + participant.true_damage_dealt_to_champions;
            let game_time = participant.time_played;
            let gold = participant.gold_earned;
            let damage_taken = participant.total_damage_taken;
            let solo_kills = challenges.solo_kills.unwrap_or_default();
            let kills = participant.kills;
            let deaths = participant.deaths;
            let assists = participant.assists;
            let cs = participant.total_ally_jungle_minions_killed.unwrap_or_default() + participant.total_enemy_jungle_minions_killed.unwrap_or_default() + participant.total_minions_killed;
            let vs = participant.vision_score;

            let turrets = participant.turret_takedowns.unwrap_or_default();
            let plates = challenges.turret_plates_taken.unwrap_or_default();
            let grubs = challenges.void_monster_kill.unwrap_or_default();
            let dragons = challenges.dragon_takedowns.unwrap_or_default();
            let barons = challenges.baron_takedowns.unwrap_or_default();

            let player_stat_data = PlayerStatData {
                games,
                wins: win,
                losses: loss,
                champions,
                total_damage: damage,
                total_game_time: game_time,
                total_gold: gold,
                total_damage_taken: damage_taken,
                total_solo_kills: solo_kills,
                total_kills: kills,
                total_deaths: deaths,
                total_assists: assists,
                total_cs: cs,
                total_vs: vs,
                total_turrets: turrets,
                total_plates: plates,
                total_grubs: grubs,
                total_dragons: dragons,
                total_barons: barons,
                total_team_damage,
                total_team_game_time,
                total_team_gold,
                total_team_damage_taken,
                total_team_solo_kills,
                total_team_kills,
                total_team_deaths,
                total_team_assists,
                total_team_cs,
                total_team_vs,
            };

            // Update team totals
            total_team_damage += damage;
            total_team_game_time += game_time;
            total_team_gold += gold;
            total_team_damage_taken += damage_taken;
            total_team_solo_kills += solo_kills;
            total_team_kills += kills;
            total_team_deaths += deaths;
            total_team_assists += assists;
            total_team_cs += cs;
            total_team_vs += vs;

            // Add the PlayerStat to the stats_to_add map
            stats_to_add.insert(puuid, player_stat_data);
        }

        // Add team totals to all PlayerStat objects
        for stat in stats_to_add.values_mut() {
            stat.total_team_damage = total_team_damage;
            stat.total_team_game_time = total_team_game_time;
            stat.total_team_gold = total_team_gold;
            stat.total_team_damage_taken = total_team_damage_taken;
            stat.total_team_solo_kills = total_team_solo_kills;
            stat.total_team_kills = total_team_kills;
            stat.total_team_deaths = total_team_deaths;
            stat.total_team_assists = total_team_assists;
            stat.total_team_cs = total_team_cs;
            stat.total_team_vs = total_team_vs;
        }

        for stat_to_add in stats_to_add {
            stats.entry(stat_to_add.0)
                .and_modify(|existing_stat| {
                    existing_stat.update(&stat_to_add.1);
                })
                .or_insert(stat_to_add.1);
        }
    }

    update_player_stat_in_db(stats).await;
}

pub async fn register_game(full_name_and_tagline: &String) -> Option<CreateEmbed> {
    let riot_id: RiotId = get_riot_id_from_string(&full_name_and_tagline.to_string()).unwrap_or_else(|| {
        log::info!("Could not find the player {}", full_name_and_tagline);
        panic!()
    });

    let riot_accounts =   get_riot_accounts(vec![riot_id.clone()]).await;
    let riot_account= match riot_accounts.get(0) {
        Some(first_value) => first_value,
        None => return None,
    };


    let current_match;
    match get_current_match_by_riot_account(&riot_account).await {
        Some(curr_match) => current_match = curr_match,
        None => return None,
    }


    let result = add_inhouse_match_to_db(InhouseMatch {
        match_id: current_match.game_id,
        added_by: format!("{}#{}", riot_id.name, riot_id.tagline)
    }).await;

    match result {
        None => Some(get_db_add_failure_embed(INHOUSE_MATCH_DB_NAME.to_string(), format!("failed to add {}", current_match.game_id))),
        Some(result) => Some(get_embed_for_current_match(&current_match, &riot_account).await.expect("TODO panic"))
    }

}

async fn update_player_stat_in_db(stats: HashMap<String, PlayerStatData>) {
    let mut player_stats = vec![];
    for (puuid, stat_data) in stats {
        if let Some(account) = get_riot_account_by_puuid(&puuid).await {
            let game_name = account.game_name.clone().unwrap_or_else(|| {
                panic!("Account with PUUID {} has a missing game_name!", puuid)
            });
            let tagline = account.tag_line.clone().unwrap_or_else(|| {
                panic!("Account with PUUID {} has a missing tagline!", puuid)
            });

            let riot_full_tag = format!("{}#{}", game_name, tagline);
            let player_stat = PlayerStat {
                created_at: chrono::Utc::now().timestamp(),
                riot_full_tag,
                player_stat: stat_data,
            };
            player_stats.push(player_stat);
        }
    }

    // delete the old one and replace with new one
    for player_stat in player_stats {
        match delete_player_stat_from_db(player_stat.riot_full_tag.clone()).await {
            None => {
                panic!("TODO couldnt delete")
            }
            Some(result) => {
                add_player_stat_to_db(player_stat).await;
            }
        }
    }
}

async fn add_player_stat_to_db(stat: PlayerStat) -> Option<InsertOneResult> {
    let database_result = get_mongo_client().await;

    match database_result {
        Ok(db) => {
            let collection = db.collection::<PlayerStat>(PLAYER_STAT_DB_NAME);
            collection.insert_one(stat, None).await.ok()
        },
        Err(err) => {
            log::error!("Error: something went wrong when trying to add a stat match to the DB: {}", err);
            None
        }
    }
}

async fn delete_player_stat_from_db(riot_full_id:  String) -> Option<DeleteResult> {
    let database_result = get_mongo_client().await;

    match database_result {
        Ok(db) => {
            let collection = db.collection::<PlayerStat>(PLAYER_STAT_DB_NAME);
            collection.delete_many(doc! { "riot_full_tag": riot_full_id }, None).await.ok()
        },
        Err(err) => {
            log::error!("Error: something went wrong when trying to delete a stat match to the DB: {}", err);
            None
        }
    }
}

async fn add_inhouse_match_to_db(inhouse_match: InhouseMatch) -> Option<InsertOneResult> {
    let database_result = get_mongo_client().await;

    match database_result {
        Ok(db) => {
            let collection = db.collection::<InhouseMatch>(INHOUSE_MATCH_DB_NAME);
            collection.insert_one(inhouse_match, None).await.ok()
        },
        Err(err) => {
            log::error!("Error: something went wrong when trying to add a inhouse match to the DB: {}", err);
            None
        }
    }
}

async fn get_all_inhouse_matches() -> Vec<InhouseMatch> {
    let database = match get_mongo_client().await {
        Ok(db) => db,
        Err(err) => {
            log::error!("An error occurred while trying to get the db: {}", err);
            return vec![];
        }
    };

    let typed_collection = database.collection::<InhouseMatch>(INHOUSE_MATCH_DB_NAME);
    let cursor = match typed_collection.find(doc! {  }, None).await {
        Ok(quote_cursor) => quote_cursor,
        Err(err) => {
            log::error!("An error occurred while trying to find the matches: {}", err);
            return vec![];
        }
    };

    return cursor.try_collect().await.unwrap_or_else(|_| vec![]);
}

