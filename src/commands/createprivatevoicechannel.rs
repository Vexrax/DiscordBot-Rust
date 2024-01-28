use serde::{Deserialize, Serialize};
use serenity::all::{CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, EditChannel, PermissionOverwrite, PermissionOverwriteType, Permissions, PRESET_VOICE, ResolvedOption, ResolvedValue, Role, RoleId, User, UserId};
use crate::utils::discord_message::respond_to_interaction;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct ChannelParameters {
    name: String,
    r#type: i32,
    user_limit: i32,
    parent_id: i64,
    permission_overwrites: Vec<PermissionOverwrite>,
}

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

    // TODO check if channel for user already exists

    let channel_params = ChannelParameters {
        name: format!("{}'s private channel", command.user.name),
        r#type: 2,
        user_limit: 1,
        parent_id: 974072802200125460, // Private channel snowflake
        permission_overwrites: build_permissions(users.clone(), roles.clone())
    };

    let audit_log_message = format!("Creating a private channel for {} vai skynet", command.user.name);
    let guild_id = command.guild_id.expect("Expected GuildId");
    let channel = ctx.http.create_channel(guild_id, &channel_params, Some(&*audit_log_message)).await;
    match channel {
        Ok(mut created_channel) => {
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
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::empty(),
            kind,
        },
        PermissionOverwrite {
            allow: Permissions::CONNECT,
            deny: Permissions::empty(),
            kind,
        },
        PermissionOverwrite {
            allow: Permissions::SPEAK,
            deny: Permissions::empty(),
            kind,
        },
        PermissionOverwrite {
            allow: Permissions::STREAM,
            deny: Permissions::empty(),
            kind,
        },
    ];

}

fn get_permissions_for_everyone_role() -> Vec<PermissionOverwrite> {
    let everyone_role_id = RoleId::new(187317542283378688);
    return vec![
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::VIEW_CHANNEL,
            kind: PermissionOverwriteType::Role(everyone_role_id)
        },
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::CONNECT,
            kind: PermissionOverwriteType::Role(everyone_role_id),
        }
    ];
}