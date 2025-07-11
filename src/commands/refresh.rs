use crate::types;
use poise::CreateReply;

// Adds the last 100 users on Banshee's ban list to your server's ban last. (Server admin only)
#[poise::command(slash_command)]
pub async fn refresh(ctx: types::Context<'_>) -> Result<(), types::Error> {
    ctx.defer_ephemeral().await?;

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

    // do that here...

    ctx.send(
        CreateReply::default()
            .content("Added users to your ban list.")
            .ephemeral(true),
    )
    .await?;

    Ok(())
}
