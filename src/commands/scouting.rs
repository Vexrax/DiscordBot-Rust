use std::collections::HashMap;
use serenity::all::{Color, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, CreateEmbed, CreateMessage, ResolvedOption, ResolvedValue};
use crate::commands::business::league_of_legends::get_recent_match_data;
use crate::utils::discord_message::respond_to_interaction;
use crate::utils::riot_api::{get_profile_icon_url, get_riot_account, get_summoner};
use std::time::{SystemTime, Duration};
use riven::consts::{Champion};
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
struct RiotId {
    name: String,
    tagline: String,
}

pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    let riot_ids_inputs = get_riot_ids_from_options(options);
    let mut failed_riot_ids: Vec<String> = vec![];
    respond_to_interaction(ctx, command, &format!("Building a recent scouting report for {:?}", riot_ids_inputs).to_string()).await;
    for riot_id_input in riot_ids_inputs {
        // TODO there has to be a better way to do this input sanitization right?
        let riot_id;
        match get_riot_id_from_string(&riot_id_input) {
            Some(riot_id_data) => riot_id = riot_id_data,
            None => {
                failed_riot_ids.push(riot_id_input);
                continue;
            }
        }

        let riot_account;
        match get_riot_account(riot_id.name.as_str(), riot_id.tagline.as_str()).await {
            Ok(maybe_riot_account_data) => {
                match maybe_riot_account_data {
                    Some(riot_account_data) => riot_account = riot_account_data,
                    None => {
                        failed_riot_ids.push(riot_id_input);
                        continue;
                    },
                }
            }
            Err(_err) => {
                failed_riot_ids.push(riot_id_input);
                continue;
            },
        }

        let summoner;
        match get_summoner(&riot_account).await {
            Ok(summoner_data) => summoner = summoner_data,
            Err(_) => {
                failed_riot_ids.push(riot_id_input);
                continue;
            },
        }

        let days_ago: u64 = 30;
        let start_time_epoch_seconds = (SystemTime::now() - Duration::from_secs(days_ago * 24 * 60 * 60)).duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

        let match_data = get_recent_match_data(summoner.clone(), start_time_epoch_seconds as i64).await;

        let embed = build_embed_for_summoner(&build_scouting_info_for_player(match_data, riot_account.puuid), &summoner, days_ago);
        command.channel_id.send_message(&ctx.http, CreateMessage::new().tts(false).embed(embed)).await.expect("TODO: panic message");
    }

    command.channel_id.say(&ctx.http, &format!("The following summoners failed {:?}", failed_riot_ids).to_string()).await.expect("TODO: panic message");

}

pub fn register() -> CreateCommand {
    CreateCommand::new("scouting")
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

}

fn build_embed_for_summoner(scouting_info: &HashMap<Champion, ScoutingInfo>, summoner: &Summoner, time_range_days: u64) -> CreateEmbed {
    let mut champs: Vec<(String, String, bool)> = vec![];
    scouting_info.iter().for_each(|champion_info| {
        let wr = format!("{:.2}", (champion_info.1.win as f64 / champion_info.1.games as f64) * 100.0);
        let kda = format!("{:.2}", (champion_info.1.kills as f64 + champion_info.1.assists as f64) / (if champion_info.1.deaths == 0 { 1 } else {champion_info.1.deaths}) as f64);
        let kills_deaths_assists = format!("{:.1}/{:.1}/{:.1}",
                                           champion_info.1.kills as f64 / champion_info.1.games as f64,
                                           champion_info.1.deaths as f64 / champion_info.1.games as f64,
                                           champion_info.1.assists as f64 / champion_info.1.games as f64);

        // INFO: this uses some invisible characters to format the message! be careful
        let formatted =  format!(":regional_indicator_w: {}% ⠀⠀⠀:axe: {} ({}) Customs: {}", wr, kda, kills_deaths_assists, champion_info.1.custom_games);
        let title = format!("{} ({})", champion_info.0.name().expect("Expected Name to exist"), champion_info.1.games);
        champs.push((title.parse().unwrap(), formatted, false));
    });

    return CreateEmbed::new()
        .title(&format!("Scouting report for {} for the last {} days", summoner.name, time_range_days))
        .description(&format!("Scouting report looks at Normals, Ranked and Tournament Draft games"))
        .color(Color::DARK_PURPLE)
        .fields(champs.into_iter())
        .thumbnail(get_profile_icon_url(summoner.profile_icon_id));
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

fn get_riot_ids_from_options(options: &[ResolvedOption<'_>]) -> Vec<String> {
    let mut riot_ids: Vec<String> = vec![];
    options.iter().for_each(|option1| {
        match option1.value {
            ResolvedValue::String(val) => {
                riot_ids.push(val.to_string());
            }
            _ => {}
        }
    });
    return riot_ids;
}

fn get_riot_id_from_string(riot_id: &String) -> Option<RiotId> {
    let mut split_summoner = riot_id.split("#");
    let riot_account_name;
    match split_summoner.next() {
        Some(name) => riot_account_name = name,
        None => return None
    }

    let riot_account_tagline;
    match split_summoner.next() {
        Some(tagline) => riot_account_tagline = tagline,
        None => return None
    }

    Some(RiotId {
       name: riot_account_name.to_string(),
       tagline: riot_account_tagline.to_string()
    })
}