use crate::{entities::user, types};
use ::serenity::all::{Context, UserId};
use poise::serenity_prelude as serenity;
use sea_orm::{
    prelude::DateTime, query::*, sqlx::types::chrono::Utc, ActiveModelTrait, ActiveValue,
    ColumnTrait, DbErr, EntityTrait, Set,
};

use super::database_service;

pub async fn update_user(user_id: serenity::UserId) -> Result<(), DbErr> {
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
    update_user(*user_id).await?;

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
    update_user(*user_id).await?;

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
