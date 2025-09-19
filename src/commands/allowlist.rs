use crate::{services::allow_list_service::get_guild_allow_list, types};
use poise::CreateReply;

/// Show list of users allowed on this server. (Server admin only)
#[poise::command(slash_command)]
pub async fn allowlist(ctx: types::Context<'_>) -> Result<(), types::Error> {
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

    let mut output_string = "Allowed user list:\n".to_string();
    let guild = &ctx.guild_id().unwrap();
    let user_list = get_guild_allow_list(guild).await?;

    for user_id in user_list {
        let user = user_id.to_user(ctx).await?;
        output_string =
            output_string + &format!("{} - ({} - {})", user.name, user.tag(), user_id.get());
    }

    output_string =
        output_string + "\n\nThese users will not be removed by Banshee when joining this server.";

    ctx.send(
        CreateReply::default()
            .content(output_string)
            .ephemeral(true),
    )
    .await?;

    Ok(())
}
