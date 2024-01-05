use riven::models::league_v4::LeagueEntry;
use serenity::all::{CommandInteraction};
use serenity::builder::{CreateCommand, CreateEmbed, CreateMessage};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;
use riven::consts::{QueueType, Team, Champion};
use riven::models::summoner_v4::Summoner;
use crate::commands::business::league_of_legends::get_rank_of_player;

use crate::utils::discord_message::respond_to_interaction;
use crate::utils::riot_api::{get_current_match, get_profile_icon_url, get_riot_account, get_summoner};

#[derive(Clone)]
struct MatchPlayer {
    rank: Option<LeagueEntry>,
    champion_id: Champion,
    profile_icon_id: i64,
    team_id: Team,
    summoner_name: String,
}

pub async fn run(_options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    respond_to_interaction(&ctx, &command, &format!("Checking to see if anyone in boosted is in game...").to_string()).await;

    // TODO dont hardcode here
    let game_name = "Zafa";
    let tagline = "NA1";

    let riot_account = get_riot_account(game_name, tagline)
        .await
        .expect("Get Summoner Failed")
        .expect("There is no summoner with that name");

    let riot_summoner = get_summoner(&riot_account).await
        .expect("Get Riot Summoner Failed");

    let current_match = get_current_match(&riot_summoner).await
        .expect("Get Match Failed")
        .expect("Summoner not in game");

    let mut match_players: Vec<MatchPlayer> = [].to_vec();
    for participant in current_match.participants {
        let rank: Option<LeagueEntry> = get_rank_of_player(participant.summoner_id, QueueType::RANKED_SOLO_5x5).await;
        match_players.push(MatchPlayer { rank: rank, champion_id: participant.champion_id, profile_icon_id: participant.profile_icon_id, team_id: participant.team_id, summoner_name: participant.summoner_name })
    }

    let match_embed = build_embed(game_name.to_string(), tagline.to_string(), riot_summoner, match_players);
    let _ = command.channel_id.send_message(&ctx.http, CreateMessage::new().tts(false).embed(match_embed)).await;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("gamestatus").description("Gets the status of the registered players in the server")
}

fn build_embed(main_player_game_name: String, main_player_tagline: String, main_player_riot_summoner: Summoner, match_players: Vec<MatchPlayer>) -> CreateEmbed {
    // TODO build cool embed!
    let mut blue_fields: Vec<(String, String, bool)> = vec![];
    let mut red_fields: Vec<(String, String, bool)> = vec![];

    for match_player in match_players {
        let display_rank = match_player.rank.as_ref()
            .map(|val| format!("{:?} {:?}", val.tier.unwrap(), val.rank.unwrap())) // TODO might need to handle this better
            .unwrap_or_else(|| "UNRANKED".to_string());
        let title = format!("{} ({})", match_player.summoner_name, match_player.champion_id.identifier().unwrap_or_else(|| "Unknown Champ?"));

        if match_player.team_id == Team::BLUE {
            blue_fields.push((title, display_rank, true));
        } else if match_player.team_id == Team::RED {
            red_fields.push((title, display_rank, true));
        }
    }

    let embed = CreateEmbed::new()
        .title(format!("{}#{}'s Game", main_player_game_name, main_player_tagline))
        .description(&format!("Some Description"))
        .thumbnail(get_profile_icon_url(main_player_riot_summoner.profile_icon_id))
        .fields(blue_fields.into_iter())
        .fields(red_fields.into_iter());
    return embed;
}