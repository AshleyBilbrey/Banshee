use crate::{services::user_service::get_super_users, types};
use poise::CreateReply;

/// List super users
#[poise::command(slash_command)]
pub async fn supers(ctx: types::Context<'_>) -> Result<(), types::Error> {
    ctx.defer_ephemeral().await?;

    let supers = get_super_users().await?;
    let mut supers_string = String::new();
    for user_id in supers {
        let user = user_id.to_user(ctx).await?;
        supers_string = format!("{}- {}\n", supers_string, user.name);
    }

    ctx.send(
        CreateReply::default()
            .content(format!("List of super users:\n{}", supers_string))
            .ephemeral(true),
    )
    .await?;
    Ok(())
}
