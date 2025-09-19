use crate::{
    entities::user,
    services::{allow_list_service::is_allowed, config_service::get_report_server},
    types,
};
use ::serenity::all::{Context, CreateMessage, GuildId, GuildPagination, UserId, UserPagination};
use poise::serenity_prelude as serenity;
use sea_orm::{
    query::*,
    sqlx::types::chrono::{self, Utc},
    ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, Set,
};
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
    if is_super_user(user).await? || is_banshee_bot(user, ctx).await? || is_banned(user).await? {
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
            match kick_user(ctx, &g.id, user, reason.clone()).await {
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
    reason: Option<String>,
) -> Result<(), types::Error> {
    if is_super_user(user).await? || is_banshee_bot(user, ctx).await? {
        return Ok(());
    }

    if &get_report_server().await == server_id {
        return Ok(());
    }

    if is_allowed(user, server_id).await? {
        return Ok(());
    }

    match server_id.get_ban(ctx, user.clone()).await {
        Ok(Some(_ban)) => return Ok(()), // Skip, user already banned
        Ok(None) => {}
        Err(err) => return Err(Box::new(err)),
    }

    let dm_channel = user.create_dm_channel(ctx).await;
    if dm_channel.is_ok() {
        let _ = dm_channel.unwrap().send_message(ctx, CreateMessage::new().content(format!(
            "You have been removed from **{}** for **{}**. If you believe this is a mistake, please contact us at https://discord.gg/b8h9aKsGrT",
            server_id.name(ctx).unwrap_or("Unknown".to_string()),
            reason.clone().unwrap_or("Unknown".to_string()),
        )));
    }

    let _ = server_id
        .ban_with_reason(
            ctx,
            user,
            7,
            format!(
                "Banned by Banshee: {}",
                reason.unwrap_or("Not Specified".to_string())
            ),
        )
        .await;

    sleep(Duration::from_secs(1)).await;

    let _ = server_id.unban(ctx, user).await;

    Ok(())
}

pub async fn remove_legacy_bans(
    ctx: &serenity::client::Context,
    server_id: &serenity::GuildId,
) -> Result<(), types::Error> {
    let mut ban_count = 200;
    let mut target = UserId::new(0);
    while ban_count == 200 {
        let bans = server_id
            .bans(ctx, Some(UserPagination::After(target)), Some(200))
            .await
            .unwrap();
        ban_count = bans.iter().count();
        for b in bans.iter() {
            target = b.user.id;
            if b.reason.is_some() && b.reason.clone().unwrap().starts_with("Banned by Banshee") {
                let _ = server_id.unban(ctx, b.user.id).await;
            }
        }
    }

    Ok(())
}
