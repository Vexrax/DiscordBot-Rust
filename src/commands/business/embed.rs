use std::fmt::format;
use std::string::ToString;
use riven::models::league_v4::LeagueEntry;
use serenity::all::{Color};
use serenity::builder::{CreateEmbed, CreateEmbedFooter};
use riven::consts::{QueueType, Team, Champion};
use riven::consts::Division::IV;
use riven::consts::Tier::UNRANKED;
use riven::models::account_v1::Account;
use riven::models::spectator_v5::CurrentGameInfo;
use riven::models::summoner_v4::Summoner;
use crate::commands::business::league_of_legends::{get_rank_of_player, get_riot_id_from_string, RiotId};

use crate::api::riot_api::{get_profile_icon_url, get_summoner};
use crate::commands::business::inhouse::PlayerStat;

#[derive(Clone)]
struct MatchPlayer {
    rank: Option<LeagueEntry>,
    champion_id: Champion,
    team_id: Team,
    riot_id: RiotId,
}

pub fn get_failure_embed(title: String, description: String)  -> CreateEmbed {
    CreateEmbed::new()
        .title(format!("{}", title))
        .description(format!("{}", description))
        .color(Color::DARK_RED)
}

pub fn get_db_add_failure_embed(collection_name: String, failure_message: String) -> CreateEmbed {
    get_failure_embed(format!("there was a failure when trying to add to [{}]", collection_name), failure_message)
}

pub async fn get_embed_for_current_match(current_match: &CurrentGameInfo, riot_account: &Account) -> Option<CreateEmbed> {
    let mut match_players: Vec<MatchPlayer> = vec![];
    for participant in current_match.clone().participants {

        let riot_id = match get_riot_id_from_string(&participant.riot_id.unwrap_or_else(|| "".to_string())) {
            Some(riot_id) => riot_id,
            None => continue,
        };

        let rank: Option<LeagueEntry> = get_rank_of_player(participant.summoner_id, QueueType::RANKED_SOLO_5x5).await;
        match_players.push(MatchPlayer { rank, champion_id: participant.champion_id, team_id: participant.team_id, riot_id })
    }

    let main_player_summoner = match get_summoner(&riot_account).await {
        Some(summoner) => summoner,
        None => return None,
    };

    Some(build_embed_for_current_match(riot_account.clone(), main_player_summoner, match_players, current_match.game_length, current_match.game_id).await)
}

pub async fn get_player_stat_embed(riot_account: Account, player_stat: PlayerStat) -> CreateEmbed {
    let mut fields: Vec<(String, String, bool)> =vec![];
    let player_stat_data = player_stat.player_stat.clone();

    fields.push(("Games Played".to_string(), player_stat_data.games.to_string(), true));
    fields.push(("Win Rate".to_string(), build_rate_string(player_stat_data.wins as i64, player_stat_data.games as i64), true));
    fields.push(("KDA".to_string(), format!("{}", (player_stat_data.total_kills + player_stat_data.total_assists) / player_stat_data.total_deaths), true));

    fields.push(("Damage Per Gold".to_string(), format!("{}", player_stat_data.total_damage / player_stat_data.total_gold), true));
    fields.push(("Damage Per Minute".to_string(), format!("{}", player_stat_data.total_damage / (player_stat_data.total_game_time / 60)) .to_string(), true));
    fields.push(("Damage Share".to_string(), build_rate_string(player_stat_data.total_damage as i64, player_stat_data.total_team_damage as i64).to_string(), true));

    fields.push(("Gold Per Min".to_string(), "a".to_string(), true));
    fields.push(("Gold Share".to_string(), "b".to_string(), true));
    fields.push(("Death Share".to_string(), "c".to_string(), true));

    fields.push(("Average Grubs".to_string(), "a".to_string(), true));
    fields.push(("Average Drags".to_string(), "b".to_string(), true));
    fields.push(("Average Barons".to_string(), "c".to_string(), true));

    fields.push(("Average Turret Plates".to_string(), "a".to_string(), true));
    fields.push(("Average Solo Kills".to_string(), "b".to_string(), true));
    fields.push(("Average Vision Score".to_string(), "c".to_string(), true));

    let embed = CreateEmbed::new()
        .title(format!("{}'s inhouse stats", riot_account.game_name.unwrap_or_default()))
        .description(format!("Last Updated At: {}", player_stat.created_at))
        .footer(CreateEmbedFooter::new(&format!("TODO")))
        // .thumbnail(get_profile_icon_url(main_player_summoner.profile_icon_id).await)
        .color(Color::DARK_ORANGE)
        .fields(fields.into_iter());
    return embed;
}

async fn build_embed_for_current_match(main_player_riot_account: Account, main_player_summoner: Summoner, match_players: Vec<MatchPlayer>, game_length_seconds: i64, game_id: i64) -> CreateEmbed {
    let mut fields: Vec<(String, String, bool)> =vec![];

    let mut red_team: Vec<MatchPlayer> = vec![];
    let mut blue_team: Vec<MatchPlayer> = vec![];

    for match_player in match_players {
        if match_player.team_id == Team::BLUE {
            let _ = red_team.push(match_player);
        } else if match_player.team_id == Team::RED {
            let _ = blue_team.push(match_player);
        }
    }

    fields.push(("Blue Team".to_string(), "".to_string(), false));
    fields.push(("".to_string(), build_compact_string_for_embed(blue_team.clone(), &build_player_string), true));
    fields.push(("".to_string(), build_compact_string_for_embed(blue_team.clone(), &build_champion_string), true));
    fields.push(("".to_string(), build_compact_string_for_embed(blue_team.clone(), &build_rank_string), true));

    fields.push(("Red Team".to_string(), "".to_string(), false));
    fields.push(("".to_string(), build_compact_string_for_embed(red_team.clone(), &build_player_string), true));
    fields.push(("".to_string(), build_compact_string_for_embed(red_team.clone(), &build_champion_string), true));
    fields.push(("".to_string(), build_compact_string_for_embed(red_team.clone(), &build_rank_string), true));

    let embed = CreateEmbed::new()
        .title(format!(":computer: {}'s Game", main_player_riot_account.game_name.unwrap_or_else(|| "Unknown".to_string())))
        .description(format!("Game Id: {}", game_id))
        .footer(CreateEmbedFooter::new(&format!("In game for {} minutes", game_length_seconds / 60)))
        .thumbnail(get_profile_icon_url(main_player_summoner.profile_icon_id).await)
        .color(Color::DARK_ORANGE)
        .fields(fields.into_iter());
    return embed;
}

fn build_champion_string(old: String, player: MatchPlayer) -> String {
    return format!("{}⠀⠀{}\n", old, player.champion_id.identifier().unwrap_or_else(|| "Unknown Champ?"))
}

fn build_player_string(old: String, player: MatchPlayer) -> String {
    return format!("{}{}\n", old, player.riot_id.name)
}

fn build_rank_string(old: String, player: MatchPlayer) -> String {
    return format!("{}⠀⠀⠀⠀⠀⠀{}⠀⠀⠀\n", old, player.rank.as_ref()
        .map(|val| format!("{:?} {:?}", val.tier.unwrap_or_else(|| UNRANKED), val.rank.unwrap_or_else(|| IV)))
        .unwrap_or_else(|| "UNRANKED".to_string()));
}

fn build_rate_string(top: i64, bot: i64) -> String {
    return format!("{}%", (top/bot) * 100);
}

fn build_compact_string_for_embed(match_players: Vec<MatchPlayer>, formatter_fn: &dyn Fn(String, MatchPlayer) -> String) -> String {
    let mut build_string = "".to_string();
    for player in match_players {
        build_string = formatter_fn(build_string, player);
    }
    return build_string;
}
