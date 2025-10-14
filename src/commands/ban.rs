use crate::{services::user_service, types};
use poise::{serenity_prelude as serenity, CreateReply};

/// Ban a user (Super users only)
#[poise::command(slash_command)]
pub async fn ban(
    ctx: types::Context<'_>,
    #[description = "User to ban"] user: serenity::User,
    #[description = "Reason"] reason: Option<String>,
) -> Result<(), types::Error> {
    ctx.defer_ephemeral().await?;

    if !user_service::is_super_user(&ctx.author().id).await? {
        ctx.send(
            CreateReply::default()
                .ephemeral(true)
                .content("You must be a super user to run this command."),
        ).await?;
        return Ok(());
    }

    if user_service::is_banshee_bot(&user.id, ctx.serenity_context()).await?
        || user_service::is_super_user(&user.id).await?
    {
        ctx.send(
            CreateReply::default()
                .ephemeral(true)
                .content("You can't ban a super user."),
        )
        .await?;
        return Ok(());
    }

    if user_service::is_banned(&user.id).await? {
        ctx.send(
            CreateReply::default()
                .ephemeral(true)
                .content("That user is already banned."),
        )
        .await?;
        return Ok(());
    }

    let response = format!(
            "{}'s account was banned{}.",
            user.name,
            reason
                .as_ref()
                .map(|r| format!(" for {}", r))
                .unwrap_or_default()
        );

    ctx.send(CreateReply::default().ephemeral(true).content(response))
        .await?;

    if user_service::ban(ctx.serenity_context(), &user.id, reason.clone()).await.is_err() {
        ctx.send(CreateReply::default().ephemeral(true).content("There was an issue banning that user.")).await?;
    }
    Ok(())
}
