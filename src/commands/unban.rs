use crate::{services::user_service, types};
use poise::{serenity_prelude as serenity, CreateReply};

/// Ban a user
#[poise::command(slash_command)]
pub async fn unban(
    ctx: types::Context<'_>,
    #[description = "User to ban"] user: serenity::User,
    #[description = "Reason"] reason: Option<String>,
) -> Result<(), types::Error> {
    // check perms

    ctx.defer_ephemeral().await?;

    if !user_service::is_banned(&user.id).await? {
        ctx.send(
            CreateReply::default()
                .ephemeral(true)
                .content("That user is not banned."),
        )
        .await?;
        return Ok(());
    }

    let result = user_service::unban(&user.id).await?;
    let response: String;
    if result {
        response = format!(
            "{}'s account was unbanned{}.",
            user.name,
            reason
                .as_ref()
                .map(|r| format!(" for {}", r))
                .unwrap_or_default()
        );
    } else {
        response = "There was a problem unbanning that user.".to_string();
    }

    ctx.send(CreateReply::default().ephemeral(true).content(response))
        .await?;
    Ok(())
}
