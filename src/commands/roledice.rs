use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use rand::Rng; 

pub fn run(_options: &[CommandDataOption]) -> String {
    let num: i32 = rand::thread_rng().gen_range(0..6);
    return  format!("You rolled a {}", num)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("rolldice").description("Rolls a six sides dice")
}