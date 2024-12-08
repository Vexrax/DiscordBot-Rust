use std::cmp::min;
use std::future::Future;
use riven::models::account_v1::Account;
use serde::{Deserialize, Serialize};
use serenity::all::{CommandInteraction, CommandOptionType, CreateCommandOption, CreateEmbed, CreateMessage, ResolvedValue};
use serenity::builder::{CreateCommand};
use serenity::client::Context;
use serenity::model::application::ResolvedOption;
use crate::commands::business::embed::get_embed_for_current_match;
use crate::utils::discord_message::respond_to_interaction;
use crate::commands::business::league_of_legends::{get_current_match_by_riot_account, get_rank_of_player, get_riot_accounts, get_riot_id_from_string, RiotId};
use crate::commands::inhouse::SubCommand::{REGISTER, STATS};

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
    println!("{:?}", command_options);

    match command_options.sub_command_type {
        REGISTER => {
            let match_embed = register_game(&command_options.riot_id).await;
            command.channel_id.send_message(&ctx.http, CreateMessage::new().tts(false).embed(match_embed.unwrap())).await.expect("TODO: panic message");
        },
        STATS => {}
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

async fn register_game(full_name_and_tagline: &String) -> Option<CreateEmbed> {
    let riot_id: RiotId = get_riot_id_from_string(&full_name_and_tagline.to_string()).unwrap_or_else(|| {
        log::info!("Could not find the player {}", full_name_and_tagline);
        panic!()
    });

    let riot_accounts =   get_riot_accounts(vec![riot_id]).await;
    let riot_account= match riot_accounts.get(0) {
        Some(first_value) => first_value,
        None => return None,
    };


    let current_match;
    match get_current_match_by_riot_account(&riot_account).await {
        Some(curr_match) => current_match = curr_match,
        None => return None,
    }

    // TODO add the match to the DB

    Some(get_embed_for_current_match(&current_match, &riot_account))
}