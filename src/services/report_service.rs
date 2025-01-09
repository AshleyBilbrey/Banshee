use crate::entities::report;
use crate::types::{Error, ReportStatus};
use ::serenity::all::{CreateActionRow, CreateButton};
use poise::serenity_prelude as serenity;
use sea_orm::{ActiveModelTrait, DbErr};

use super::database_service;
use super::user_service;

pub async fn save_report(
    message_body: &String,
    display_name: String,
    author: serenity::UserId,
    reporter: serenity::UserId,
) -> Result<i32, DbErr> {
    user_service::update_user(author).await?;
    user_service::update_user(reporter).await?;

    let db = database_service::establish_connection().await?;
    let report = report::ActiveModel {
        message_body: sea_orm::ActiveValue::Set(message_body.to_string()),
        display_name: sea_orm::ActiveValue::Set(display_name),
        author_snowflake: sea_orm::ActiveValue::Set(author.get() as i64),
        reporter_snowflake: sea_orm::ActiveValue::Set(reporter.get() as i64),
        ..Default::default()
    }
    .insert(&db)
    .await?;

    Ok(report.id)
}

fn report_status_color(status: &ReportStatus) -> serenity::Color {
    match status {
        ReportStatus::Open => serenity::Color::new(0x4dfffe),
        ReportStatus::Banned => serenity::Color::new(0xfe60fb),
        ReportStatus::Rejected => serenity::Color::new(0x6c757d),
    }
}

fn report_status_string(status: &ReportStatus) -> String {
    match status {
        ReportStatus::Open => "Open".to_string(),
        ReportStatus::Banned => "Banned".to_string(),
        ReportStatus::Rejected => "Rejected".to_string(),
    }
}

pub async fn generate_report_embed(
    message_body: &String,
    author: &serenity::User,
    reporter: &serenity::User,
    report_number: i32,
    status: ReportStatus,
    timestamp: serenity::Timestamp,
) -> Result<serenity::CreateEmbed, Error> {
    let embed = serenity::CreateEmbed::new()
        .author(
            serenity::CreateEmbedAuthor::new(format!("{} ({})", author.name, author.tag()))
                .icon_url(author.avatar_url().unwrap()),
        )
        .title(format!(
            "Report #{} - {}",
            report_number,
            report_status_string(&status)
        ))
        .color(report_status_color(&status))
        .description(format!("**Message Content:**\n{}", message_body))
        .footer(
            serenity::CreateEmbedFooter::new(format!(
                "Reported by {} ({})",
                reporter.name,
                reporter.tag()
            ))
            .icon_url(reporter.avatar_url().unwrap()),
        )
        .timestamp(timestamp);
    Ok(embed)
}

pub async fn generate_report_buttons(report_id: i32, link: String) -> Vec<CreateActionRow> {
    let ban = CreateButton::new("Ban:".to_owned() + &report_id.to_string())
        .label("Ban")
        .style(serenity::ButtonStyle::Danger);
    let dismiss = CreateButton::new("Dismiss:".to_owned() + &report_id.to_string())
        .label("Dismiss")
        .style(serenity::ButtonStyle::Primary);
    let link = CreateButton::new_link(link).label("View Original");

    let buttons = vec![ban, dismiss, link];

    return vec![CreateActionRow::Buttons(buttons)];
}
