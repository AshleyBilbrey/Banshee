use crate::{services::allow_list_service::{allow as service_allow, is_allowed}, types};
use poise::{serenity_prelude as serenity, CreateReply};

/// Prevent Banshee from removing a user. (Server admin only)
#[poise::command(slash_command)]
pub async fn allow(
    ctx: types::Context<'_>,
    #[description = "User to allow"] user: serenity::User,
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

    if is_allowed(&user.id, &ctx.guild_id().unwrap()).await? {
        ctx.send(
            CreateReply::default()
                .content("That user is already on the allow list.")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    service_allow(&user.id, &ctx.guild_id().unwrap()).await?;

    ctx.send(CreateReply::default().content("Placed user on allow list.").ephemeral(true))
        .await?;

    Ok(())
}
