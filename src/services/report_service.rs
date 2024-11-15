use crate::entities;
use crate::services::database_service;
use entities::prelude::*;
use poise::serenity_prelude as serenity;
use sea_orm::{DbErr, EntityTrait};

async fn save_report(
    message_body: String,
    display_name: String,
    author: serenity::UserId,
    reporter: serenity::UserId,
) -> Result<(), DbErr> {
    let db = database_service::establish_connection().await?;
    let report = entities::report::ActiveModel {
        message_body: sea_orm::ActiveValue::Set(message_body),
        display_name: sea_orm::ActiveValue::Set(display_name),
        author_snowflake: sea_orm::ActiveValue::Set(author.get() as i64),
        reporter_snowflake: sea_orm::ActiveValue::Set(reporter.get() as i64),
        ..Default::default()
    };

    Report::insert(report).exec(&db).await?;

    Ok(())
}

async fn report(
    message_body: String,
    display_name: String,
    author: serenity::UserId,
    reporter: serenity::UserId,
) -> Result<(), DbErr> {
    save_report(message_body, display_name, author, reporter).await?;

    Ok(())
}
