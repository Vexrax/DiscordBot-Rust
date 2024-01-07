use std::collections::HashSet;
use std::string::ToString;
use riven::models::league_v4::LeagueEntry;
use serenity::all::{CommandInteraction};
use serenity::builder::{CreateCommand, CreateEmbed, CreateMessage};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;
use riven::consts::{QueueType, Team, Champion};
use riven::models::summoner_v4::Summoner;
use crate::commands::business::league_of_legends::{get_current_match_by_riot_summoner, get_rank_of_player, get_summoners_by_riot_ids, RiotId};
use queues::*;

use crate::utils::discord_message::respond_to_interaction;
use crate::utils::riot_api::{get_profile_icon_url};

#[derive(Clone)]
struct MatchPlayer {
    rank: Option<LeagueEntry>,
    champion_id: Champion,
    team_id: Team,
    summoner_name: String,
}
pub async fn run(_options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    respond_to_interaction(&ctx, &command, &format!("Checking to see if anyone in boosted is in game...").to_string()).await;

    let players: Vec<RiotId> = vec![
        RiotId {
            name: "Zafa".to_string(),
            tagline: "NA1".to_string(),
        },
        RiotId {
            name: "Vexrax".to_string(),
            tagline: "FAKER".to_string(),
        }
    ];

    let summoners = get_summoners_by_riot_ids(players).await;
    let mut matches_generated = HashSet::new();
    for riot_summoner in summoners {
        let current_match;
        match get_current_match_by_riot_summoner(&riot_summoner).await {
            Some(curr_match) => current_match = curr_match,
            None => continue,
        }

        if matches_generated.contains(&current_match.game_id) {
            continue;
        }

        let mut match_players: Vec<MatchPlayer> = [].to_vec();
        for participant in current_match.participants {
            let rank: Option<LeagueEntry> = get_rank_of_player(participant.summoner_id, QueueType::RANKED_SOLO_5x5).await;
            match_players.push(MatchPlayer { rank, champion_id: participant.champion_id, team_id: participant.team_id, summoner_name: participant.summoner_name })
        }

        let match_embed = build_embed(riot_summoner, match_players, current_match.game_length);
        let _ = command.channel_id.send_message(&ctx.http, CreateMessage::new().tts(false).embed(match_embed)).await;
        matches_generated.insert(current_match.game_id);
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("gamestatus").description("Gets the status of the registered players in the server")
}

fn build_embed(main_player_riot_summoner: Summoner, match_players: Vec<MatchPlayer>, game_length_seconds: i64) -> CreateEmbed {
    let mut fields: Vec<(String, String, bool)> =vec![];

    let mut red_queue: Queue<MatchPlayer> = queue![];
    let mut blue_queue: Queue<MatchPlayer> = queue![];

    for match_player in match_players {
        if match_player.team_id == Team::BLUE {
            red_queue.add(match_player);
        } else if match_player.team_id == Team::RED {
            blue_queue.add(match_player);
        }
    }

    while red_queue.size() > 0 || blue_queue.size() > 0 {
        let red_player = red_queue.remove().expect("Expected playersize to be the same");
        let blue_player = blue_queue.remove().expect("Expected playersize to be the same");

        fields.push(get_fields_for_embed(blue_player));
        fields.push(("[]".to_string(), "[]".to_string(), true));
        fields.push(get_fields_for_embed(red_player));
    }


    let embed = CreateEmbed::new()
        .title(format!("{}'s Game", main_player_riot_summoner.name))
        .description(&format!("In game for {} minutes", game_length_seconds / 60))
        .thumbnail(get_profile_icon_url(main_player_riot_summoner.profile_icon_id))
        .fields(fields.into_iter());
    return embed;
}

fn get_fields_for_embed(match_player: MatchPlayer) -> (String, String, bool) {
    let display_rank = match_player.rank.as_ref()
        .map(|val| format!("{:?} {:?}", val.tier.unwrap(), val.rank.unwrap())) // TODO might need to handle this better
        .unwrap_or_else(|| "UNRANKED".to_string());
    let title = format!("{} ({})", match_player.summoner_name, match_player.champion_id.identifier().unwrap_or_else(|| "Unknown Champ?"));
    return (title, display_rank, true);
}

