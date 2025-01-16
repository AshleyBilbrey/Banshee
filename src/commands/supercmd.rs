use crate::types;
use poise::serenity_prelude as serenity;

/// Adds a new super user.
#[poise::command(slash_command, owners_only, rename = "super")]
pub async fn supercmd(
    ctx: types::Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), types::Error> {
    let u: &serenity::model::prelude::User = user.as_ref().unwrap_or_else(|| ctx.author());
    let response: String = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}
