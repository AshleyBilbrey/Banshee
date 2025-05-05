use crate::{
    services::{report_service, user_service},
    types,
};
use poise::serenity_prelude as serenity;

/// Show information about a user
#[poise::command(slash_command)]
pub async fn user(
    ctx: types::Context<'_>,
    #[description = "User to show info on"] user: Option<serenity::User>,
) -> Result<(), types::Error> {
    ctx.defer_ephemeral().await?;

    let user = user.unwrap_or_else(|| ctx.author().clone());
    let account_creation = user.created_at();

    let is_banned = user_service::is_banned(&user.id).await.unwrap_or(false);
    let is_super_user = user_service::is_super_user(&user.id).await.unwrap_or(false);

    let mut description = format!(
        "📅 **Account Created:** <t:{}:F>\n",
        account_creation.timestamp()
    );

    if is_banned {
        description.push_str("🚫 **Banned:** Yes\n");
    }

    if is_super_user {
        description.push_str("⭐ **Super User:** Yes\n");
    }

    let color = if is_banned {
        report_service::report_status_color(&crate::types::ReportStatus::Banned)
    } else if is_super_user {
        report_service::report_status_color(&crate::types::ReportStatus::Open)
    } else {
        serenity::Color::default()
    };

    let embed = serenity::CreateEmbed::default()
        .title(format!("User Info: {}", user.tag()))
        .thumbnail(user.face())
        .description(description)
        .color(color);

    ctx.send(poise::CreateReply::default().ephemeral(true).embed(embed))
        .await?;

    Ok(())
}
