use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, ModelTrait, QueryFilter};
use serenity::all::{GuildId, UserId};

use crate::{
    entities::allow_list,
    services::{database_service, user_service::update_user},
};

pub async fn is_allowed(user: &UserId, guild: &GuildId) -> Result<bool, DbErr> {
    let db = database_service::establish_connection().await?;

    let current_allow_list = allow_list::Entity::find()
        .filter(allow_list::Column::UserSnowflake.eq(user.get() as i64))
        .filter(allow_list::Column::GuildSnowflake.eq(guild.get() as i64))
        .one(&db)
        .await?;

    Ok(current_allow_list.is_some())
}

pub async fn get_guild_allow_list(guild: &GuildId) -> Result<Vec<UserId>, DbErr> {
    let db = database_service::establish_connection().await?;

    let allow_list_models = allow_list::Entity::find()
        .filter(allow_list::Column::GuildSnowflake.eq(guild.get() as i64))
        .all(&db)
        .await?;

    let allow_list = allow_list_models
        .iter()
        .map(|model| UserId::new(model.user_snowflake as u64))
        .collect::<Vec<UserId>>();
    Ok(allow_list)
}

pub async fn allow(user: &UserId, guild: &GuildId) -> Result<(), DbErr> {
    update_user(user).await?;

    let db = database_service::establish_connection().await?;

    allow_list::ActiveModel {
        user_snowflake: ActiveValue::Set(user.get() as i64),
        guild_snowflake: ActiveValue::Set(guild.get() as i64),
        ..Default::default()
    }
    .insert(&db)
    .await?;

    Ok(())
}

pub async fn unallow(user: &UserId, guild: &GuildId) -> Result<(), DbErr> {
    let db = database_service::establish_connection().await?;

    let current_allow_list = allow_list::Entity::find()
        .filter(allow_list::Column::UserSnowflake.eq(user.get() as i64))
        .filter(allow_list::Column::GuildSnowflake.eq(guild.get() as i64))
        .one(&db)
        .await?;

    if current_allow_list.is_some() {
        current_allow_list.unwrap().delete(&db).await?;
    }

    Ok(())
}
