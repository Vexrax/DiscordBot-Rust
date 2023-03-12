use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption,
    CommandDataOptionValue,
};
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;

use crate::utils::discord_message::respond_to_interaction;

pub async fn run(options: &[CommandDataOption], ctx: &Context, command: &ApplicationCommandInteraction) {
    let option = options
        .get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");


    if let CommandDataOptionValue::User(user, _member) = option {
        respond_to_interaction(&ctx, &command,  &format!("{}'s id is {}", user.tag(), user.id)).await;
    } else {
        respond_to_interaction(&ctx, &command,  &  "Please provide a valid user".to_string()).await;
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("id").description("Get a user id").create_option(|option| {
        option
            .name("id")
            .description("The user to lookup")
            .kind(CommandOptionType::User)
            .required(true)
    })
}