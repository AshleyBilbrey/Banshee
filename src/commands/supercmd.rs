use crate::{
    services::user_service::{is_super_user, make_super},
    types,
};
use poise::{serenity_prelude as serenity, CreateReply};

/// Adds a new super user.
#[poise::command(slash_command, owners_only, rename = "super")]
pub async fn supercmd(
    ctx: types::Context<'_>,
    #[description = "User to make super"] user: Option<serenity::User>,
) -> Result<(), types::Error> {
    ctx.defer_ephemeral().await?;

    let u: &serenity::model::prelude::User = user.as_ref().unwrap_or_else(|| ctx.author());
    if is_super_user(&u.id).await? {
        ctx.send(
            CreateReply::default()
                .ephemeral(true)
                .content(format!("{} already has super powers.", u.name)),
        )
        .await?;

        return Ok(());
    }

    make_super(&u.id).await?;

    ctx.send(
        CreateReply::default()
            .ephemeral(true)
            .content(format!("Gave {} super powers.", u.name)),
    )
    .await?;
    Ok(())
}
