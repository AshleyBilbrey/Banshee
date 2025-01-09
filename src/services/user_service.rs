use crate::entities::user;
use poise::serenity_prelude as serenity;
use sea_orm::{query::*, ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait};

use super::database_service;

pub async fn update_user(user_id: serenity::UserId) -> Result<(), DbErr> {
    let db = database_service::establish_connection().await?;

    let current_user: Option<user::Model> = user::Entity::find()
        .filter(user::Column::Snowflake.eq(user_id.get() as i64))
        .one(&db)
        .await?;

    if let Some(_) = current_user {
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
