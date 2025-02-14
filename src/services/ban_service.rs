use crate::{
    entities::{ban, user},
    types,
};
use poise::serenity_prelude as serenity;
use sea_orm::{query::*, ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, Set};
use serenity::all::{User, UserId};
use std::collections::HashSet;

use super::{
    database_service,
    user_service::{is_banned, is_super_user, update_user},
};

pub async fn ban(
    ctx: &serenity::client::Context,
    user: User,
    reason: Option<String>,
    report_id: Option<i64>,
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

    if is_super_user(&user.id).await?
        || owners.contains(&user.id)
        || ctx.http.get_current_user().await?.id == user.id
        || is_banned(&user.id).await?
    {
        return Ok(false);
    }

    ban_user_db(&user.id, reason, report_id).await?;

    Ok(true)
}

async fn ban_user_db(
    user_id: &UserId,
    reason: Option<String>,
    report_id: Option<i64>,
) -> Result<(), DbErr> {
    update_user(*user_id).await?;

    let db = database_service::establish_connection().await?;

    let current_user = user::Entity::find()
        .filter(user::Column::Snowflake.eq(user_id.get() as i64))
        .one(&db)
        .await?;

    let mut user: user::ActiveModel = current_user.unwrap().into();
    user.banned = Set(true);
    user.update(&db).await?;

    ban::ActiveModel {
        user_snowflake: ActiveValue::Set(user_id.get() as i64),
        reason: ActiveValue::Set(reason),
        report_id: ActiveValue::Set(report_id),
        active: ActiveValue::Set(true),
        ..Default::default()
    }
    .insert(&db)
    .await?;

    Ok(())
}
