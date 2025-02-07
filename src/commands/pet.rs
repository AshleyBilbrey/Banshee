use poise::CreateReply;
use serenity::all::CreateEmbed;

use crate::types;

/// Pet the bot...
///
/// Or not.
#[poise::command(slash_command)]
pub async fn pet(ctx: types::Context<'_>) -> Result<(), types::Error> {
    ctx.defer_ephemeral().await?;
    ctx.send(
        CreateReply::default()
            .content("What am I, some sort of animal for you to pet?")
            .ephemeral(true)
            .embed(
                CreateEmbed::new()
                    .image("https://media1.tenor.com/m/xc19_U9dSNMAAAAd/chika-fujiwara-hit.gif"),
            ),
    )
    .await?;
    Ok(())
}
