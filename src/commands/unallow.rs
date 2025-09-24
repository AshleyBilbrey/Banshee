use crate::{services::{allow_list_service::{is_allowed, unallow as service_unallow}, user_service::{get_ban_reason, is_banned, kick_user}}, types};
use poise::{serenity_prelude as serenity, CreateReply};

/// Removes a user from the allow list. (Server admin only)
#[poise::command(slash_command)]
pub async fn unallow(
    ctx: types::Context<'_>,
    #[description = "User to unallow"] user: serenity::User,
) -> Result<(), types::Error> {
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

    if !is_allowed(&user.id, &ctx.guild_id().unwrap()).await? {
        ctx.send(
            CreateReply::default()
                .content("That user isn't on the allow list.")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    service_unallow(&user.id, &ctx.guild_id().unwrap()).await?;

    if is_banned(&user.id).await? {
        kick_user(ctx.serenity_context(), &ctx.guild_id().unwrap(), &user.id, Some(get_ban_reason(&user.id).await?)).await?;
    }

    ctx.send(CreateReply::default().content("Removed user from the allow list.").ephemeral(true))
        .await?;

    Ok(())
}
