use poise::CreateReply;

use crate::{services::user_service::remove_legacy_bans, types};

/// Remove all legacy bans from Banshee.
#[poise::command(slash_command)]
pub async fn removelegacybans(ctx: types::Context<'_>) -> Result<(), types::Error> {
    ctx.defer_ephemeral().await?;

    if ctx.guild_id().is_none() {
        ctx.send(
            CreateReply::default()
                .content("You can only run this command in a server.")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    let permissions = ctx.author_member().await.unwrap().permissions;
    if permissions.is_none() || !permissions.unwrap().administrator() {
        ctx.send(
            CreateReply::default()
                .content("You do not have admin permissions on this server.")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    remove_legacy_bans(ctx.serenity_context(), &ctx.guild_id().unwrap()).await?;

    ctx.send(
            CreateReply::default()
                .content("Removed all legacy Banshee bans from this server.")
                .ephemeral(true),
        )
        .await?;

    Ok(())
}
