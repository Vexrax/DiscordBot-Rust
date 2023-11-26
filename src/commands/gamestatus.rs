use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption
};
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;

pub async fn run(_options: &[CommandDataOption], _ctx: &Context, _command: &ApplicationCommandInteraction) {
    unimplemented!()
    // Loop over all the players in the server 
    // generate a card to show their current status of their game
    // Print those to console
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("game status").description("Gets the status of the registered players in the server")
}