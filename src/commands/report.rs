use crate::{
    services::report_service::*,
    types::{self, ReportStatus},
};
use poise::serenity_prelude as serenity;

/// Report a message
#[poise::command(context_menu_command = "Report to Banshee", slash_command)]
pub async fn report(
    ctx: types::Context<'_>,
    #[description = "Message to Report"] msg: serenity::Message,
) -> Result<(), types::Error> {
    ctx.defer_ephemeral().await?;
    let reporter = ctx.author();
    let author = &msg.author;
    let message = msg.content_safe(&ctx.cache());

    let report_number =
        save_report(&message, reporter.name.clone(), author.id, reporter.id).await?;

    let report_embed = generate_report_embed(
        &message,
        author,
        reporter,
        report_number,
        ReportStatus::Open,
        serenity::Timestamp::now(),
    )
    .await?;

    let report_buttons = generate_report_buttons();

    ctx.send(
        poise::CreateReply::default()
            .content("Submitted your report!")
            .embed(report_embed)
            .ephemeral(true),
    )
    .await?;

    Ok(())
}
