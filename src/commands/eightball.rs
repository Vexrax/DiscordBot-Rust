use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption,
    CommandDataOptionValue,
};
use rand::Rng; 

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
    " Without a doubt.",
    " Outlook good.",
    "Outlook not so good."
];

pub fn run(options: &[CommandDataOption]) -> String {
    let option = options
        .get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");

    if let CommandDataOptionValue::String(question) = option {
        return RESPONSE_OPTIONS[rand::thread_rng().gen_range(0..RESPONSE_OPTIONS.len())].to_string();
    } else {
        return "Please ask a question".to_string();
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