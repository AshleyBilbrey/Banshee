use crate::types;
use poise::serenity_prelude as serenity;

/// Report a message
#[poise::command(context_menu_command = "Report to Banshee", slash_command)]
pub async fn report(
    ctx: types::Context<'_>,
    #[description = "Message to Report"] msg: serenity::Message,
) -> Result<(), types::Error> {
    ctx.defer_ephemeral().await?;
    send_report_to_admins(msg).await?;
    send_report_response().await?;
    ctx.say("Reported message.").await?;
    Ok(())
}

async fn send_report_to_admins(_msg: serenity::Message) -> Result<(), types::Error> {
    save_report_to_db().await?;
    Ok(())
}

async fn save_report_to_db() -> Result<(), types::Error> {
    Ok(())
}

async fn send_report_response() -> Result<(), types::Error> {
    Ok(())
}
