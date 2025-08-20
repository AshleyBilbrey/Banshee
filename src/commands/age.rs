use crate::types;
use poise::serenity_prelude as serenity;

/// Displays your or another user's account creation date
#[poise::command(slash_command)]
pub async fn age(
    ctx: types::Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), types::Error> {
    let u: &serenity::model::prelude::User = user.as_ref().unwrap_or_else(|| ctx.author());
    let response: String = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}
