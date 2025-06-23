use crate::{
    services::user_service::{
        get_ban_reason, is_banned, is_whitelisted, kick_user, unwhitelist_user,
    },
    types,
};
use ::serenity::all::CreateMessage;
use poise::{serenity_prelude as serenity, CreateReply};

// Remove user from a whitelist, allowing them to be banned on your server by Banshee. (Server admin only)
#[poise::command(slash_command)]
pub async fn unwhitelist(
    ctx: types::Context<'_>,
    #[description = "User to unwhitelist"] user: serenity::User,
) -> Result<(), types::Error> {
    ctx.defer_ephemeral().await?;

    let permissions = ctx.author_member().await.unwrap().permissions;
    if permissions.is_none() || !permissions.unwrap().administrator() {
        ctx.send(
            CreateReply::default()
                .content("You do not have permission to whitelist users on this server.")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    let guild_channel = ctx.channel_id().to_channel(ctx).await?.guild().unwrap();
    let server_id = guild_channel.guild_id;
    let already_whitelisted = is_whitelisted(user.id, server_id).await?;
    if !already_whitelisted {
        ctx.send(
            CreateReply::default()
                .content("That user is not whitelisted on this server.")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    unwhitelist_user(&server_id, &user.id).await?;

    if is_banned(&user.id).await? {
        let private_channel = user.create_dm_channel(ctx).await?;
        let _ = private_channel.send_message(ctx, CreateMessage::new().content(format!("You've been removed from **{}**, a Banshee protected server, for **{}**. If you think this is a mistake, contact us at https://discord.gg/b8h9aKsGrT", server_id.to_partial_guild(ctx).await?.name, get_ban_reason(&user.id).await?))).await;
        kick_user(ctx.serenity_context(), &server_id, &user.id).await?;
    }

    ctx.send(
        CreateReply::default()
            .content(format!(
                "Removed **{}** from whitelist.",
                user.display_name()
            ))
            .ephemeral(true),
    )
    .await?;

    Ok(())
}
