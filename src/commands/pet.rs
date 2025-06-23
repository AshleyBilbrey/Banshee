use poise::CreateReply;

use crate::types;

/// Pet the bot... Or not.
#[poise::command(slash_command)]
pub async fn pet(ctx: types::Context<'_>) -> Result<(), types::Error> {
    ctx.defer_ephemeral().await?;
    ctx.send(
        CreateReply::default()
            .content("What am I, some sort of animal for you to pet?\nhttps://media1.tenor.com/m/xc19_U9dSNMAAAAd/chika-fujiwara-hit.gif")
            .ephemeral(true),
    )
    .await?;
    Ok(())
}
