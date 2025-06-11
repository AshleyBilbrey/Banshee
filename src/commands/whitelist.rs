use crate::{services::user_service::{is_whitelisted, whitelist_user}, types};
use poise::{serenity_prelude as serenity, CreateReply};

#[poise::command(slash_command)]
pub async fn whitelist(
    ctx: types::Context<'_>,
    #[description = "User to whitelist"] user: serenity::User,
) -> Result<(), types::Error> {
    ctx.defer_ephemeral().await?;

    let permissions = ctx.author_member().await.unwrap().permissions;
    if permissions.is_none() || !permissions.unwrap().administrator() {
        ctx.send(
            CreateReply::default()
                .content("You do not have permission to whitelist users on this server.")
                .ephemeral(true),
        ).await?;
        return Ok(());
    }

    let server_id = ctx.channel_id().to_channel(ctx).await?.guild().unwrap().guild_id;
    let already_whitelisted = is_whitelisted(user.id, server_id).await?;
    if already_whitelisted {
        ctx.send(
            CreateReply::default()
                .content("That user is already whitelisted on this server.")
                .ephemeral(true),
        ).await?;
        return Ok(());
    }

    whitelist_user(&server_id, &user.id).await?;

    ctx.send(
        CreateReply::default()
            .content(format!("Whitelisted **{}**", user.display_name()))
            .ephemeral(true),
    ).await?;

    Ok(())
}
