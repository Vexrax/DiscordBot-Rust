use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption,
    CommandDataOptionValue,
};
use rand::Rng; 
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;

use crate::utils::discord_message::respond_to_interaction;


const RESPONSE_OPTIONS: &[&str] = &[
    "As I see it, yes.",
    "Ask again later",
    "Better not tell you now.",
    "Cannot predict now.",
    "Concentrate and ask again.",
    "Don't count on it.",
    "It is certain.",
    "It is decidedly so.",
    "Yes – definitely",
    "Without a doubt.",
    "Outlook good.",
    "Outlook not so good."
];

pub async fn run(options: &[CommandDataOption], ctx: &Context, command: &ApplicationCommandInteraction) {
    let option = options
        .get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");

    if let CommandDataOptionValue::String(question) = option {
        let response = RESPONSE_OPTIONS[rand::thread_rng().gen_range(0..RESPONSE_OPTIONS.len())].to_string();
        // TODO add the question in here 
        respond_to_interaction(&ctx, &command, &response).await;  
    } else {
        respond_to_interaction(&ctx, &command, &"Please ask a question".to_string()).await;  
    }    
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("eightball").description("Ask the eightball a question").create_option(|option| {
        option
            .name("question")
            .description("the question to ask")
            .kind(CommandOptionType::String)
            .required(true)
    })
}