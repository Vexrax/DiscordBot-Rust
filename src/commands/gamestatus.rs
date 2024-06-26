use std::collections::HashSet;
use std::string::ToString;
use riven::models::league_v4::LeagueEntry;
use serenity::all::{Color, CommandInteraction};
use serenity::builder::{CreateCommand, CreateEmbed, CreateEmbedFooter, CreateMessage};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;
use riven::consts::{QueueType, Team, Champion};
use riven::consts::Division::IV;
use riven::consts::Tier::UNRANKED;
use riven::models::account_v1::Account;
use riven::models::summoner_v4::Summoner;
use crate::commands::business::league_of_legends::{get_current_match_by_riot_account, get_rank_of_player, get_riot_accounts, get_riot_id_from_string, RiotId};

use crate::utils::discord_message::respond_to_interaction;
use crate::api::riot_api::{get_profile_icon_url, get_summoner};

#[derive(Clone)]
struct MatchPlayer {
    rank: Option<LeagueEntry>,
    champion_id: Champion,
    team_id: Team,
    riot_id: RiotId,
}

const PLAYERS_IDS: [&str; 11] = [
    "Vexrax#FAKER", "Zafa#NA1", "Earleking#NA1", "rgrou2#NA1", "grant#erino", "Perky#GOAT", "LeeSinners#NA1",
    "Koality Player#NA1", "Soulbert#koggy", "ShadyGecko#1313", "Arcadius#NA1"
];
pub async fn run(_options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    respond_to_interaction(&ctx, &command, &"Checking to see if anyone in boosted is in game...".to_string().to_string()).await;
    let mut players: Vec<RiotId> = vec![];

    for player_id_input in PLAYERS_IDS {
        match get_riot_id_from_string(&player_id_input.to_string()) {
            None => {
                log::info!("Could not find the player {}", player_id_input)
            },
            Some(riot_id) => players.push(riot_id)
        }
    }

    let riot_accounts: Vec<Account> = get_riot_accounts(players).await;
    let mut matches_generated = HashSet::new();
    for riot_account in riot_accounts {
        let current_match;
        match get_current_match_by_riot_account(&riot_account).await {
            Some(curr_match) => current_match = curr_match,
            None => continue,
        }

        if matches_generated.contains(&current_match.game_id) {
            continue;
        }

        let mut match_players: Vec<MatchPlayer> = vec![];
        for participant in current_match.participants {

            let riot_id = match get_riot_id_from_string(&participant.riot_id.unwrap_or_else(|| "".to_string())) {
                Some(riot_id) => riot_id,
                None => continue,
            };

            let rank: Option<LeagueEntry> = get_rank_of_player(participant.summoner_id, QueueType::RANKED_SOLO_5x5).await;
            match_players.push(MatchPlayer { rank, champion_id: participant.champion_id, team_id: participant.team_id, riot_id })
        }

        // Need the summoner for thumbnail
        let main_player_summoner = match get_summoner(&riot_account).await {
            Some(summoner) => summoner,
            None => continue,
        };

        let match_embed = build_embed(riot_account, main_player_summoner, match_players, current_match.game_length, current_match.game_id).await;
        let _ = command.channel_id.send_message(&ctx.http, CreateMessage::new().tts(false).embed(match_embed)).await;
        matches_generated.insert(current_match.game_id);
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("gamestatus").description("Gets the status of the registered players in the server")
}

async fn build_embed(main_player_riot_account: Account, main_player_summoner: Summoner, match_players: Vec<MatchPlayer>, game_length_seconds: i64, game_id: i64) -> CreateEmbed {
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

fn build_compact_string_for_embed(match_players: Vec<MatchPlayer>, formatter_fn: &dyn Fn(String, MatchPlayer) -> String) -> String {
    let mut build_string = "".to_string();
    for player in match_players {
        build_string = formatter_fn(build_string, player);
    }
    return build_string;
}


