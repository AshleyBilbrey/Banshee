use crate::{
    services::user_service::{is_super_user, un_super},
    types,
};
use poise::{serenity_prelude as serenity, CreateReply};

/// Removes a super user (Super users only)
#[poise::command(slash_command, owners_only)]
pub async fn unsuper(
    ctx: types::Context<'_>,
    #[description = "User to unsuper"] user: Option<serenity::User>,
) -> Result<(), types::Error> {
    ctx.defer_ephemeral().await?;

    let u: &serenity::model::prelude::User = user.as_ref().unwrap_or_else(|| ctx.author());

    if !is_super_user(&u.id).await? {
        ctx.send(
            CreateReply::default()
                .ephemeral(true)
                .content(format!("{} doesn't have super powers to remove.", u.name)),
        )
        .await?;

        return Ok(());
    }

    un_super(&u.id).await?;

    ctx.send(
        CreateReply::default()
            .ephemeral(true)
            .content(format!("Took away super powers from {}.", u.name)),
    )
    .await?;
    Ok(())
}
