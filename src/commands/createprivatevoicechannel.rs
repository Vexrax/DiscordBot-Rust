use serde::{Deserialize, Serialize};
use serenity::all::{Channel, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, GuildId, PermissionOverwrite, PermissionOverwriteType, Permissions, ResolvedOption, ResolvedValue, Role, RoleId, User};
use serenity::all::Change::ChannelId;
use serenity::all::ChannelType::Voice;
use crate::utils::discord_message::respond_to_interaction;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct ChannelParameters {
    name: String,
    r#type: i32,
    user_limit: i32,
    parent_id: u64,
    permission_overwrites: Vec<PermissionOverwrite>,
}

const CUSTOM_PRIVATE_VC_SNOWFLAKE_ID: u64 = 1201272633572995163;
const CHANNEL_SUFFIX: &str = "'s private channel";

pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction) {
    let mut users: Vec<User> = vec![];
    let mut roles: Vec<Role> = vec![];

    options.iter().for_each(|option1| {
        match option1.value {
            ResolvedValue::User(user, ..) => users.push(user.clone()),
            ResolvedValue::Role(role, ..) => roles.push(role.clone()),
            _ => {}
        }
    });

    let channel_name =  format!("{}{}", command.user.name, CHANNEL_SUFFIX,);
    let guild_id = command.guild_id.expect("Expected GuildId");
    let private_channel_section_snowflake_id = CUSTOM_PRIVATE_VC_SNOWFLAKE_ID;

    let channel_params = ChannelParameters {
        name: channel_name,
        r#type: 2,
        user_limit: (users.len() + options.len() + 1) as i32,
        parent_id: private_channel_section_snowflake_id,
        permission_overwrites: build_permissions(users.clone(), roles.clone())
    };

    let audit_log_message = format!("Creating a private channel for {} via Skynet", command.user.name);
    let channel = ctx.http.create_channel(guild_id, &channel_params, Some(&*audit_log_message)).await;
    match channel {
        Ok(mut created_channel) => {
            // TODO lets store the channels we create in mongo and periodically clean them up
            respond_to_interaction(&ctx, &command, &audit_log_message).await;
        }
        Err(err) => {
            respond_to_interaction(&ctx, &command, &"Something went wrong when creating channel!".to_string()).await;
        }
    }

}

pub fn register() -> CreateCommand {
    CreateCommand::new("createprivatevc")
        .description("Create a private VC")
        .add_option(CreateCommandOption::new(CommandOptionType::User, "user1", "user")
            .required(true))
        .add_option(CreateCommandOption::new(CommandOptionType::User, "user2", "user")
            .required(false))
        .add_option(CreateCommandOption::new(CommandOptionType::User, "user3", "user")
            .required(false))
        .add_option(CreateCommandOption::new(CommandOptionType::Role, "role", "user")
            .required(false))
}

pub async  fn cleanup_unused_channels(ctx: &Context, guild_id: GuildId) {
    let channels;
    match ctx.http.get_channels(guild_id).await {
        Ok(guild_channels) => {
            channels = guild_channels.clone();
        }
        Err(_) => {
            // todo some logging
            return;
        }
    }

    channels.iter()
        .filter(|channel| {
            // todo check if anyone is connected
            return channel.kind == Voice // && channel.name.contains(CHANNEL_SUFFIX)
        })
        .for_each(|channel| {
            // todo actually delete the channel
            println!("Would have deleted {}", channel.name);
        });
}

fn build_permissions(users: Vec<User>, roles: Vec<Role>) -> Vec<PermissionOverwrite> {
    let mut permissions: Vec<PermissionOverwrite> = vec![];
    users.iter()
        .for_each(|user|permissions.extend(get_permissions_for_users(PermissionOverwriteType::Member(user.id))));

    roles.iter()
        .for_each(|role| permissions.extend(get_permissions_for_users(PermissionOverwriteType::Role(role.id))));
    permissions.extend(get_permissions_for_everyone_role());
    return permissions;
}

fn get_permissions_for_users(kind: PermissionOverwriteType) -> Vec<PermissionOverwrite> {
    return vec![
        PermissionOverwrite {
            allow: Permissions::from_bits_truncate(Permissions::STREAM.bits() | Permissions::SPEAK.bits() | Permissions::VIEW_CHANNEL.bits() | Permissions::CONNECT.bits()),
            deny: Permissions::empty(),
            kind,
        },
    ];

}

fn get_permissions_for_everyone_role() -> Vec<PermissionOverwrite> {
    let everyone_role_id = RoleId::new(187317542283378688);
    return vec![
        PermissionOverwrite {
            allow: Permissions::from_bits_truncate(Permissions::VIEW_CHANNEL.bits()),
            deny: Permissions::from_bits_truncate(Permissions::CONNECT.bits()),
            kind: PermissionOverwriteType::Role(everyone_role_id)
        },
    ];
}