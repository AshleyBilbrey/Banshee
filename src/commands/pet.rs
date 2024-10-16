use crate::types;

/// Pet the bot...
///
/// Or not.
#[poise::command(slash_command)]
pub async fn pet(ctx: types::Context<'_>) -> Result<(), types::Error> {
    ctx.defer_ephemeral().await?;
    let response: String = format!("https://tenor.com/view/chika-fujiwara-hit-cute-kawaii-anime-gif-13583613\nWhat am I to you, a cow to pet?");
    ctx.reply(response).await?;
    Ok(())
}
