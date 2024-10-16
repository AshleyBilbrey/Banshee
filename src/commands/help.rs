use crate::types;

/// Show the help menu
#[poise::command(slash_command)]
pub async fn help(
    ctx: types::Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), types::Error> {
    ctx.defer_ephemeral().await?;
    let config = poise::builtins::HelpConfiguration {
        extra_text_at_bottom: get_extra_help_text(),
        ephemeral: true,
        show_context_menu_commands: true,
        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}

fn get_extra_help_text() -> &'static str {
    return "```\n
Banshee automatically bans known spammers from your Discord servers, so you don't have to.
Thanks for using Banshee!
        
Links:
- [Discord Server](https://discord.gg/b8h9aKsGrT) - For report logs, ban logs, and more info.
- [Add to your server](https://www.ashleybilbrey.com)
- [Ashley Bilbrey](https://www.ashleybilbrey.com) - The creator of the bot.";
}
