use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(_options: &[CommandDataOption]) -> String {
    "https://www.google.com/search?client=firefox-b-1-d&q=mental+hospitals+near+me+".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("mentalhelp").description("Used to give mental help")
}