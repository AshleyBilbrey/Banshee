use crate::types;
use poise::samples::register_application_commands_buttons;

/// Register command menu (Super users only)
#[poise::command(slash_command, owners_only)]
pub async fn register<'a>(ctx: types::Context<'a>) -> Result<(), types::Error> {
    register_application_commands_buttons(ctx).await?;
    Ok(())
}
