use std::{collections::HashSet, thread::sleep, time::Duration};

use crate::{
    entities::{user, whitelist},
    services::config_service::get_report_server,
    types,
};
use ::serenity::all::{Context, CreateMessage, GuildId, GuildPagination, UserId};
use poise::serenity_prelude as serenity;
use sea_orm::{
    query::*,
    sqlx::types::chrono::{self, Utc},
    ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, ModelTrait, Set,
};

use super::database_service;

pub async fn update_user(user_id: &serenity::UserId) -> Result<(), DbErr> {
    let db = database_service::establish_connection().await?;

    let current_user: Option<user::Model> = user::Entity::find()
        .filter(user::Column::Snowflake.eq(user_id.get() as i64))
        .one(&db)
        .await?;

    if let Some(existing_user) = current_user {
        let mut user: user::ActiveModel = existing_user.into();
        user.updated_at = Set(Some(Utc::now().naive_utc()));
        user.update(&db).await?;

        return Ok(());
    }

    user::ActiveModel {
        snowflake: ActiveValue::Set(user_id.get() as i64),
        banned: ActiveValue::Set(false),
        super_user: ActiveValue::Set(false),
        ban_reason: ActiveValue::Set("".to_string()),
        ..Default::default()
    }
    .insert(&db)
    .await?;

    Ok(())
}

pub async fn is_super_user(user: &UserId) -> Result<bool, types::Error> {
    let db = database_service::establish_connection().await?;

    let current_user = user::Entity::find()
        .filter(user::Column::Snowflake.eq(user.get() as i64))
        .one(&db)
        .await?;

    if let Some(model) = current_user {
        return Ok(model.super_user);
    }

    return Ok(false);
}

pub async fn get_super_users() -> Result<Vec<UserId>, types::Error> {
    let db = database_service::establish_connection().await?;
    let users: Vec<user::Model> = user::Entity::find()
        .filter(user::Column::SuperUser.eq(true))
        .all(&db)
        .await?;

    let user_ids: Vec<UserId> = users
        .into_iter()
        .map(|user| UserId::new(user.snowflake as u64))
        .collect();

    Ok(user_ids)
}

pub async fn is_banshee_bot(user: &UserId, ctx: &Context) -> Result<bool, types::Error> {
    let is_banshee: bool = user.get() == ctx.cache.current_user().id.get();
    Ok(is_banshee)
}

pub async fn make_super(user_id: &UserId) -> Result<(), DbErr> {
    update_user(user_id).await?;

    let db = database_service::establish_connection().await?;

    let current_user = user::Entity::find()
        .filter(user::Column::Snowflake.eq(user_id.get() as i64))
        .one(&db)
        .await?;

    let mut user: user::ActiveModel = current_user.unwrap().into();
    user.super_user = Set(true);
    user.update(&db).await?;

    Ok(())
}

pub async fn un_super(user_id: &UserId) -> Result<(), DbErr> {
    update_user(user_id).await?;

    let db = database_service::establish_connection().await?;

    let current_user = user::Entity::find()
        .filter(user::Column::Snowflake.eq(user_id.get() as i64))
        .one(&db)
        .await?;

    let mut user: user::ActiveModel = current_user.unwrap().into();

    user.super_user = Set(false);
    user.update(&db).await?;

    Ok(())
}

pub async fn is_banned(user: &UserId) -> Result<bool, DbErr> {
    let db = database_service::establish_connection().await?;

    let current_user = user::Entity::find()
        .filter(user::Column::Snowflake.eq(user.get() as i64))
        .one(&db)
        .await?;

    if let Some(model) = current_user {
        return Ok(model.banned);
    }

    return Ok(false);
}

pub async fn ban(
    ctx: &serenity::client::Context,
    user: &UserId,
    reason: Option<String>,
) -> Result<bool, types::Error> {
    let owners: HashSet<UserId> = match ctx.http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else if let Some(owner) = &info.owner {
                owners.insert(owner.id);
            }
            owners
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    if is_super_user(user).await?
        || owners.contains(user)
        || is_banshee_bot(user, ctx).await?
        || is_banned(user).await?
    {
        return Ok(false);
    }

    update_user(user).await?;

    let db = database_service::establish_connection().await?;

    let current_user = user::Entity::find()
        .filter(user::Column::Snowflake.eq(user.get() as i64))
        .one(&db)
        .await?;

    let mut user_model: user::ActiveModel = current_user.unwrap().into();
    user_model.banned = Set(true);
    user_model.ban_reason = Set(reason.clone().unwrap_or("No reason provided.".to_string()));
    user_model.save(&db).await?;

    let mut ban_list = "".to_string();

    let mut guild_count = 200;
    let mut target = GuildId::new(1);
    while guild_count == 200 {
        let guilds = ctx
            .http
            .get_guilds(Some(GuildPagination::After(target)), Some(200))
            .await?;
        guild_count = guilds.len();
        for g in guilds.iter() {
            target = g.id;
            ban_list.push_str("- ");
            ban_list.push_str(&target.to_partial_guild(ctx).await?.name);
            ban_list.push_str("\n");
            match kick_user(ctx, &g.id, user).await {
                Ok(_) => {}
                Err(err) => {
                    eprintln!(
                        "Failed to kick user {} from guild {}: {:?}",
                        user.get(),
                        g.id,
                        err
                    );
                }
            };
        }
    }

    let private_channel = user.create_dm_channel(ctx).await?;
    private_channel.send_message(ctx, CreateMessage::new().content(format!("You've been removed from the following Banshee protected servers for **{}**\n{}\n If you think this is a mistake, contact us at https://discord.gg/b8h9aKsGrT", reason.unwrap_or("Not Specified".to_string()), ban_list))).await?;

    Ok(true)
}

pub async fn unban(user: &UserId) -> Result<bool, types::Error> {
    update_user(user).await?;

    let db = database_service::establish_connection().await?;

    let current_user = user::Entity::find()
        .filter(user::Column::Snowflake.eq(user.get() as i64))
        .one(&db)
        .await?;

    let mut user_model: user::ActiveModel = current_user.unwrap().into();
    user_model.banned = Set(false);
    user_model.ban_reason = Set("".to_string());
    user_model.save(&db).await?;

    Ok(true)
}

pub async fn get_ban_reason(user: &UserId) -> Result<String, types::Error> {
    let db = database_service::establish_connection().await?;

    let current_user = user::Entity::find()
        .filter(user::Column::Snowflake.eq(user.get() as i64))
        .one(&db)
        .await?;

    if let Some(model) = current_user {
        return Ok(model.ban_reason);
    }

    return Ok("".to_string());
}

pub async fn get_update_time(user: &UserId) -> Result<Option<chrono::NaiveDateTime>, types::Error> {
    let db = database_service::establish_connection().await?;

    let current_user = user::Entity::find()
        .filter(user::Column::Snowflake.eq(user.get() as i64))
        .one(&db)
        .await?;

    if let Some(model) = current_user {
        return Ok(model.updated_at);
    }

    return Ok(None);
}

pub async fn kick_user(
    ctx: &serenity::client::Context,
    server_id: &serenity::GuildId,
    user: &UserId,
) -> Result<bool, types::Error> {
    let owners: HashSet<UserId> = match ctx.http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else if let Some(owner) = &info.owner {
                owners.insert(owner.id);
            }
            owners
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    if is_super_user(user).await? || owners.contains(user) || is_banshee_bot(user, ctx).await? {
        return Ok(false);
    }

    if is_whitelisted(*user, *server_id).await? {
        return Ok(false);
    }

    if &get_report_server().await == server_id {
        return Ok(false);
    }

    match server_id.member(ctx, user).await {
        Ok(_member) => {} // Member is in the server, continue
        Err(serenity::Error::Http(_)) => return Ok(false),
        Err(_) => return Ok(false),
    }

    match server_id.ban(ctx, user, 4).await {
        Ok(()) => {}
        Err(_err) => {
            println!("Error banning {:?}", _err);
            return Ok(false);
        }
    }

    sleep(Duration::from_millis(100));

    match server_id.unban(ctx, user).await {
        Ok(()) => {}
        Err(_err) => return Ok(false),
    }

    Ok(true)
}

pub async fn is_whitelisted(user_id: UserId, server_id: GuildId) -> Result<bool, DbErr> {
    let db = database_service::establish_connection().await?;

    let current_whitelist = whitelist::Entity::find()
        .filter(whitelist::Column::ServerSnowflake.eq(server_id.get() as i64))
        .filter(whitelist::Column::UserSnowflake.eq(user_id.get() as i64))
        .one(&db)
        .await?;

    Ok(current_whitelist.is_some())
}

pub async fn whitelist_user(
    server_id: &serenity::GuildId,
    user_id: &UserId,
) -> Result<bool, types::Error> {
    update_user(user_id).await?;

    let db = database_service::establish_connection().await?;

    whitelist::ActiveModel {
        server_snowflake: ActiveValue::Set(server_id.get() as i64),
        user_snowflake: ActiveValue::Set(user_id.get() as i64),
        ..Default::default()
    }
    .insert(&db)
    .await?;

    Ok(true)
}

pub async fn unwhitelist_user(
    server_id: &serenity::GuildId,
    user_id: &UserId,
) -> Result<bool, types::Error> {
    update_user(user_id).await?;

    let db = database_service::establish_connection().await?;

    let current_whitelist = whitelist::Entity::find()
        .filter(whitelist::Column::ServerSnowflake.eq(server_id.get() as i64))
        .filter(whitelist::Column::UserSnowflake.eq(user_id.get() as i64))
        .one(&db)
        .await?;

    let current_whitelist: whitelist::Model = current_whitelist.unwrap();
    current_whitelist.delete(&db).await?;

    Ok(true)
}
