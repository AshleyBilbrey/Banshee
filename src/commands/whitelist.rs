use crate::types;
use ::serenity::all::Permissions;
use poise::{serenity_prelude as serenity, CreateReply};

#[poise::command(slash_command)]
pub async fn user(
    ctx: types::Context<'_>,
    #[description = "User to whitelist"] user: Option<serenity::User>,
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


    // todo whitelist user
    Ok(())
}
