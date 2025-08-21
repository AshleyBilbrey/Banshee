use crate::{entities::user, services::config_service::get_report_server, types};
use ::serenity::all::{Context, CreateMessage, GuildId, GuildPagination, User, UserId};
use poise::serenity_prelude as serenity;
use sea_orm::{
    query::*,
    sqlx::types::chrono::{self, Utc},
    ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, Set,
};
use std::collections::HashSet;
use tokio::time::{sleep, Duration};

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
    if is_super_user(user).await?
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
            match process_ban(ctx, &g.id, user, reason.clone()).await {
                Ok(_) => {}
                Err(err) => {
                    eprintln!(
                        "Failed to ban user {} from guild {}: {:?}",
                        user.get(),
                        g.id,
                        err
                    );
                }
            };
            sleep(Duration::from_millis(100)).await;
            
        }
    }

    let private_channel = user.create_dm_channel(ctx).await?;
    private_channel.send_message(ctx, CreateMessage::new().content(format!("You've been removed from Banshee protected servers for **{}**\n If you think this is a mistake, contact us at https://discord.gg/b8h9aKsGrT", reason.unwrap_or("Not Specified".to_string())))).await?;

    Ok(true)
}

pub async fn unban(ctx: &serenity::client::Context, user: &UserId) -> Result<bool, types::Error> {
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
            match process_unban(ctx, &g.id, user).await {
                Ok(_) => {}
                Err(err) => {
                    eprintln!(
                        "Failed to unban user {} from guild {}: {:?}",
                        user.get(),
                        g.id,
                        err
                    );
                }
            };
            sleep(Duration::from_millis(100)).await;
        }
    }

    Ok(true)
}

pub async fn refresh_bans(
    ctx: &serenity::client::Context,
    server_id: &serenity::GuildId,
) -> Result<(), types::Error> {
    let db = database_service::establish_connection().await?;

    let most_recent_users = user::Entity::find()
        .filter(user::Column::Banned.eq(true))
        .order_by_desc(user::Column::Id)
        .limit(100)
        .all(&db)
        .await?;

    for user in most_recent_users.iter() {
        process_ban(ctx, server_id, &UserId::new(user.snowflake as u64), Some(user.ban_reason.clone())).await?;
        sleep(Duration::from_millis(100)).await;
    }

    Ok(())
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

pub async fn process_ban(
    ctx: &serenity::client::Context,
    server_id: &serenity::GuildId,
    user: &UserId,
    reason: Option<String>,
) -> Result<bool, types::Error> {
    if is_super_user(user).await? || is_banshee_bot(user, ctx).await? {
        return Ok(false);
    }

    if &get_report_server().await == server_id {
        return Ok(false);
    }

    match server_id.get_ban(ctx, user.clone()).await {
        Ok(Some(_ban)) => return Ok(false), // Skip, user already banned
        Ok(None) => {}
        Err(err) => return Err(Box::new(err)),
    }

    match server_id
        .ban_with_reason(
            ctx,
            user,
            4,
            if reason.is_some() {
                format!("Banned by Banshee: {}", reason.unwrap())
            } else {
                "Banned by Banshee".to_string()
            },
        )
        .await
    {
        Ok(()) => {}
        Err(err) => return Err(Box::new(err)),
    }

    Ok(true)
}

pub async fn process_unban(
    ctx: &serenity::client::Context,
    server_id: &serenity::GuildId,
    user: &UserId,
) -> Result<bool, types::Error> {
    if &get_report_server().await == server_id {
        return Ok(false);
    }

    let ban = match server_id.get_ban(ctx, user.clone()).await {
        Ok(Some(ban)) => ban,
        Ok(None) => return Ok(false), // No ban found
        Err(err) => return Err(Box::new(err)),
    };

    if ban.reason.is_none() || !ban.reason.unwrap().starts_with("Banned by Banshee") {
        return Ok(false); // Not banned by Banshee, so skip.
    }

    server_id.unban(ctx, user).await?;

    return Ok(true);
}
