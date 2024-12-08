use std::cmp;
use std::cmp::min;
use std::collections::HashMap;
use serenity::all::{Color, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, CreateEmbed, CreateEmbedFooter, CreateMessage, ResolvedOption, ResolvedValue};
use crate::commands::business::league_of_legends::{get_recent_match_data, get_riot_id_from_string};
use crate::utils::discord_message::{respond_to_interaction, say_message_in_channel};
use crate::api::riot_api::{get_profile_icon_url, get_riot_account, get_summoner};
use std::time::{SystemTime, Duration};
use riven::consts::{Champion, Queue};
use riven::models::account_v1::Account;
use riven::models::match_v5::{Match, Participant};
use riven::models::summoner_v4::Summoner;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct ScoutingInfo {
    games: i32,
    win: i32,
    loss: i32,
    kills: i32,
    deaths: i32,
    assists: i32,
    custom_games: i32,
}

struct CommandOptions {
    riot_ids: Vec<String>,
    days_ago: u64
}
const DEFAULT_DAYS_AGO: u64 = 30;
const MAX_DAYS_AGO: u64 = 365;
const VALID_QUEUES_FOR_SCOUTING: [Queue; 5]= [
    Queue::SUMMONERS_RIFT_NORMAL_QUICKPLAY,
    Queue::SUMMONERS_RIFT_5V5_DRAFT_PICK,
    Queue::SUMMONERS_RIFT_5V5_RANKED_FLEX,
    Queue::SUMMONERS_RIFT_5V5_RANKED_SOLO,
    Queue::CUSTOM
];

const VALID_QUEUES_FOR_SCOUTING_COMP: [Queue; 1]= [
    Queue::CUSTOM
];

pub async fn run_scouting(options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    run(options, ctx, command, VALID_QUEUES_FOR_SCOUTING.to_vec()).await;
}

pub async fn run_scouting_for_comp(options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    run(options, ctx, command, VALID_QUEUES_FOR_SCOUTING_COMP.to_vec()).await;
}

async fn run(options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction, queues: Vec<Queue>) {
    let command_options = get_command_options(options);
    let mut failed_riot_ids: Vec<String> = vec![];
    respond_to_interaction(ctx, command, &format!("Building a recent scouting report for {}", command_options.riot_ids.join(", ")).to_string()).await;
    for riot_id_input in command_options.riot_ids {
        let riot_id = match get_riot_id_from_string(&riot_id_input) {
            Some(riot_id_data) => riot_id_data,
            None => {
                failed_riot_ids.push(riot_id_input);
                continue;
            }
        };

        let riot_account = match get_riot_account(riot_id.name.as_str(), riot_id.tagline.as_str()).await {
            Some(riot_account_data) => riot_account_data,
            None => {
                failed_riot_ids.push(riot_id_input);
                continue;
            },
        };

        let summoner = match get_summoner(&riot_account).await {
            Some(summoner_data) => summoner_data,
            None => {
                failed_riot_ids.push(riot_id_input);
                continue;
            },
        };

        let start_time_epoch_seconds = (SystemTime::now() - Duration::from_secs(command_options.days_ago * 24 * 60 * 60)).duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

        let match_data = get_recent_match_data(summoner.clone(), start_time_epoch_seconds as i64, queues.clone()).await;

        let embed = build_embed_for_summoner(&build_scouting_info_for_player(match_data, riot_account.clone().puuid), summoner.clone(), riot_account.clone(), command_options.days_ago).await;
        let _ = command.channel_id.send_message(&ctx.http, CreateMessage::new().tts(false).embed(embed)).await;
    }

    if failed_riot_ids.len() > 0 {
        say_message_in_channel(command.channel_id, &ctx.http, &format!("The following summoners failed {:?}", failed_riot_ids).to_string()).await;
    }
}

pub fn register(command_name: String) -> CreateCommand {
    CreateCommand::new(command_name)
        .description("Scouting command to fetch info about players recent champions in soloq and normal")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "summoner1", "summoner1")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "summoner2", "summoner2")
                .required(false),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "summoner3", "summoner3")
                .required(false),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "summoner4", "summoner4")
                .required(false),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "summoner5", "summoner5")
                .required(false),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::Integer, "days_ago", "amount of days ago to look at")
                .required(false),
        )
}

async fn build_embed_for_summoner(scouting_info: &HashMap<Champion, ScoutingInfo>, summoner: Summoner, riot_account: Account, time_range_days: u64) -> CreateEmbed {
    let mut fields: Vec<(String, String, bool)> = vec![];
    let mut total_games: i32 = 0;
    let mut total_wins: i32 = 0;
    let mut scouting_vec: Vec<_> = scouting_info.into_iter().collect();
    scouting_vec.sort_by_key(|&(_, ref info)| std::cmp::Reverse(info.games));


    let mut champs_build_string = "".to_string();
    let mut winrate_build_string = "".to_string();
    let mut kda_build_string = "".to_string();

    scouting_vec.iter().for_each(|champion_info| {
        let wr = format!("{:.2}", (champion_info.1.win as f64 / champion_info.1.games as f64) * 100.0);
        let kda = format!("{:.2}", (champion_info.1.kills as f64 + champion_info.1.assists as f64) / cmp::max(champion_info.1.deaths, 1) as f64);
        let kills_deaths_assists = format!("{:.1}/{:.1}/{:.1}",
                                           champion_info.1.kills as f64 / champion_info.1.games as f64,
                                           champion_info.1.deaths as f64 / champion_info.1.games as f64,
                                           champion_info.1.assists as f64 / champion_info.1.games as f64);
        total_games += champion_info.1.games;
        total_wins += champion_info.1.win;

        champs_build_string = format!("{}{}\n", champs_build_string, champion_info.0.name().expect("Expected Name to exist").to_string());
        winrate_build_string = format!("{}{}\n", winrate_build_string, format!("{}% WR ({})", wr, champion_info.1.games));
        kda_build_string = format!("{}{}\n", kda_build_string, format!("{} ({})", kda, kills_deaths_assists));
    });

    // Headers
    fields.push(("Champions".to_string(), "".to_string(), false));
    fields.push(("".to_string(), "Champion".to_string(), true));
    fields.push(("".to_string(), "Winrate".to_string(), true));
    fields.push(("".to_string(), "KDA".to_string(), true));

    fields.push(("".to_string(), champs_build_string, true));
    fields.push(("".to_string(), winrate_build_string, true));
    fields.push(("".to_string(), kda_build_string, true));

    let name =  riot_account.game_name.unwrap_or_else(||"Unknown".to_string());
    let tagline =  riot_account.tag_line.unwrap_or_else(||"Unknown".to_string());

    return CreateEmbed::new()
        .title(&format!("Scouting report for {}#{} for the last {} days", name, tagline, time_range_days))
        .description(&format!("Games: {}. Report looks at Normals, Ranked and Tournament Draft games", total_games))
        .color(Color::DARK_PURPLE)
        .fields(fields.into_iter())
        .footer(CreateEmbedFooter::new( format!("Total WR: {:.2}%", (total_wins as f64 / total_games as f64) * 100.0)))
        .thumbnail(get_profile_icon_url(summoner.profile_icon_id).await);
}

fn get_command_options(options: &[ResolvedOption<'_>]) -> CommandOptions {
    let mut riot_ids: Vec<String> = vec![];
    let mut days_ago = DEFAULT_DAYS_AGO;
    options.iter().for_each(|option1| {
        match option1.value {
            ResolvedValue::String(val) => {
                riot_ids.push(val.to_string());
            }
            ResolvedValue::Integer(val) => {
                days_ago = min(val as u64, MAX_DAYS_AGO);
            }
            _ => {}
        }
    });
    return CommandOptions {
        riot_ids,
        days_ago,
    };
}

fn build_scouting_info_for_player(match_data: Vec<Match>, puuid: String) -> HashMap<Champion, ScoutingInfo> {
    return match_data
        .into_iter()
        .flat_map(|matches| build_scouting_info_for_single_match(matches.info.participants, puuid.clone()))
        .fold(HashMap::new(), |mut merged_map, (champion_name, scouting_info)| {
            let entry = merged_map
                .entry(champion_name)
                .or_insert_with(|| ScoutingInfo {
                    games: 0,
                    win: 0,
                    loss: 0,
                    kills: 0,
                    deaths: 0,
                    assists: 0,
                    custom_games: 0,
                });

            entry.games += scouting_info.games;
            entry.win += scouting_info.win;
            entry.loss += scouting_info.loss;
            entry.kills += scouting_info.kills;
            entry.deaths += scouting_info.deaths;
            entry.assists += scouting_info.assists;
            entry.custom_games += scouting_info.custom_games;

            merged_map
        })
}

fn build_scouting_info_for_single_match(participants: Vec<Participant>, puuid: String) -> Option<(Champion, ScoutingInfo)> {
    return participants.into_iter().find_map(|participant| {
        if participant.puuid == puuid {
            let scouting_info = ScoutingInfo {
                games: 1,
                win: if participant.win { 1 } else { 0 },
                loss: if participant.win { 0 } else { 1 },
                kills: participant.kills,
                deaths: participant.deaths,
                assists: participant.assists,
                custom_games: 0,
            };
            Some((participant.champion().expect("Expected champion to exist"), scouting_info))
        } else {
            None
        }
    })
}

