use crate::{
    services::{ban_service, user_service},
    types,
};
use poise::{serenity_prelude as serenity, CreateReply};

/// Ban a user
#[poise::command(slash_command)]
pub async fn unban(
    ctx: types::Context<'_>,
    #[description = "User to unban"] user: serenity::User,
) -> Result<(), types::Error> {
    // Check perms

    ctx.defer_ephemeral().await?;

    if !user_service::is_banned(&user.id).await? {
        ctx.reply_builder(
            CreateReply::default()
                .ephemeral(true)
                .content("That user isn't banned."),
        );
        return Ok(());
    }

    let result = ban_service::unban(ctx.serenity_context(), user.to_owned()).await?;
    let response: String;
    if result {
        response = format!("{} was unbanned.", user.name);
    } else {
        response = "There was a problem unbanning banning that user.".to_string();
    }

    ctx.reply_builder(CreateReply::default().ephemeral(true).content(response));
    Ok(())
}
