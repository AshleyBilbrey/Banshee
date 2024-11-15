use crate::types;
use poise::serenity_prelude as serenity;

/// Report a message
#[poise::command(context_menu_command = "Report to Banshee", slash_command)]
pub async fn report(
    ctx: types::Context<'_>,
    #[description = "Message to Report"] msg: serenity::Message,
) -> Result<(), types::Error> {
    ctx.defer_ephemeral().await?;
    let reporter = ctx.author().to_string();
    let author = msg.author.to_string();
    ctx.say(format!(
        "Hello **{}**, you reported a message from **{}**",
        reporter, author
    ))
    .await?;
    Ok(())
}
