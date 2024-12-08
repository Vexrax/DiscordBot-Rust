use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::future::Future;
use futures::TryStreamExt;
use mongodb::bson::{doc, Document};
use mongodb::results::InsertOneResult;
use riven::consts::Team;
use riven::models::account_v1::Account;
use riven::models::match_v5::Match;
use serde::{Deserialize, Serialize};
use serenity::all::{CommandInteraction, CommandOptionType, CreateCommandOption, CreateEmbed, CreateMessage, ResolvedValue, User};
use serenity::builder::{CreateCommand};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;
use crate::api::riot_api::get_riot_account_by_puuid;
use crate::commands::business::embed::{get_db_add_failure_embed, get_embed_for_current_match};
use crate::utils::discord_message::respond_to_interaction;
use crate::commands::business::league_of_legends::{get_current_match_by_riot_account, get_league_matches, get_matches, get_rank_of_player, get_riot_accounts, get_riot_id_from_string, RiotId};
use crate::commands::inhouse::SubCommand::{REGISTER, STATS};
use crate::utils::mongo::get_mongo_client;

const INHOUSE_MATCH_DB_NAME: &str = "InHouses";

#[derive(Clone, Debug, Deserialize, Serialize)]
struct InhouseMatch {
    match_id: i64,
    added_by: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct PlayerStat {
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

#[derive(Clone, Debug, Deserialize, Serialize)]
enum SubCommand {
    REGISTER,
    STATS,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct CommandOptions {
    riot_id: String,
    sub_command_type: SubCommand
}

pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    let command_options = get_command_options(options);

    match command_options.sub_command_type {
        REGISTER => {
            let match_embed = register_game(&command_options.riot_id).await;
            command.channel_id.send_message(&ctx.http, CreateMessage::new().tts(false).embed(match_embed.unwrap())).await.expect("TODO: panic message");
        },
        STATS => {
            full_refresh_stat().await;
        }
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("inhouse")
        .description("Add a game for inhouse stats")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "type", "either REGISTER or STATS")
                .required(true)
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "riot_tag", "the riot name + tageline eg: vexrax#FAKER")
                .required(true)
        )
}


// TODO MAKE THIS NOT DEPEND ON ORDER
fn get_command_options(options: &[ResolvedOption<'_>]) -> CommandOptions {
    let sub_command = match options.get(0) {
        Some(val) =>  val,
        None => {
            panic!() // TODO
        }
    };

    let riot_id = match options.get(1) {
        Some(val) =>  val,
        None => {
            panic!() // TODO
        }
    };


    let sub_command: SubCommand = match sub_command.value {
        ResolvedValue::String(val) =>  {
            let result = match val.to_string().as_str() {
                "REGISTER" => REGISTER,
                "STATS" => STATS,
                other => panic!(), // TODO
            };
            result
        }
        _ => {
            panic!() // TODO
        }
    };

    let riot_id: String = match riot_id.value {
        ResolvedValue::String(val) => val.to_string(),
        _ => {
            panic!() // TODO
        }
    };

    return CommandOptions {
        sub_command_type: sub_command,
        riot_id: riot_id,
    }
}


async fn get_stat(full_name_and_tagline: &String) {
    // Check if theres a new match available thats not in the cache
    // run thru each players stats and add in the new values from the new match
    // add the new match to the cache
}

async fn full_refresh_stat() {
    let inhouse_matches = get_all_inhouse_matches().await;
    let mut match_ids = vec![];
    println!("{:?}", match_ids);
    for inhouse_match in inhouse_matches {
        match_ids.push(inhouse_match.match_id);
    }

    let league_matches = get_league_matches(match_ids).await;
    let mut stats: HashMap<String, PlayerStat> = HashMap::new(); // PUUID to PlayerStat
    for league_match in league_matches {
        let winning_team_id = if league_match.info.teams.first().unwrap().team_id == Team::BLUE
            && league_match.info.teams.first().unwrap().win {
            Team::BLUE
        } else {
            Team::RED
        };

        let mut stats_to_add: HashMap<String, PlayerStat> = HashMap::new(); // PUUID to PlayerStat

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

            let player_stat = PlayerStat {
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
            stats_to_add.insert(puuid, player_stat);
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

        // Add stats_to_add to the main stats HashMap
        stats.extend(stats_to_add);
    }
    println!("{:?}", stats);
}

async fn register_game(full_name_and_tagline: &String) -> Option<CreateEmbed> {
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

async fn add_inhouse_match_to_db(inhouse_match:  InhouseMatch) -> Option<InsertOneResult> {
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