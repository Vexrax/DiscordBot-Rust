use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use rand::Rng; 

pub fn run(_options: &[CommandDataOption]) -> String {
    let num: i32 = rand::thread_rng().gen_range(0..2);
    
    let mut result = "";
    if num == 1 {
        result = "Heads";
    } else {
        result = "Tails"
    }

    return format!("The coin landed on {}", result);
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("flipcoin").description("Flip A coin")
}