use crate::types;
use poise::serenity_prelude as serenity;

/// Ban a user
#[poise::command(slash_command)]
pub async fn ban(
    ctx: types::Context<'_>,
    #[description = "User to ban"] user: Option<serenity::User>,
) -> Result<(), types::Error> {
    let u: &serenity::model::prelude::User = user.as_ref().unwrap_or_else(|| ctx.author());
    let response: String = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}
