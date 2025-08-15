use crate::{services::{config_service::get_report_server, user_service::refresh_bans}, types};
use poise::CreateReply;

// Adds the last 100 users on Banshee's ban list to your server's ban last. (Server admin only)
#[poise::command(slash_command)]
pub async fn refresh(ctx: types::Context<'_>) -> Result<(), types::Error> {
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

    if get_report_server().await == ctx.guild_id().unwrap() {
        ctx.send(
            CreateReply::default()
                .content("Cannot ban users on report server.")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    refresh_bans(ctx.serenity_context(), &ctx.guild_id().unwrap()).await?;

    ctx.send(
        CreateReply::default()
            .content("Added users to your ban list.")
            .ephemeral(true),
    )
    .await?;

    Ok(())
}
