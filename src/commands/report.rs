use crate::{
    services::{
        config_service,
        report_service::*,
        user_service::{is_banshee_bot, is_super_user},
    },
    types::{self, ReportStatus},
};
use ::serenity::all::{CreateEmbed, CreateMessage};
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

    if is_super_user(&author.id).await? {
        ctx.send(
            poise::CreateReply::default()
                .content(format!(
                    "You can't report {} because they are a super user.",
                    author.name
                ))
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    if is_banshee_bot(&author.id, ctx.serenity_context()).await? {
        ctx.send(
            poise::CreateReply::default()
                .content("Nice try, you can't report me!")
                .ephemeral(true)
                .embed(
                    CreateEmbed::new().image(
                        "https://media1.tenor.com/m/xc19_U9dSNMAAAAd/chika-fujiwara-hit.gif",
                    ),
                ),
        )
        .await?;
        return Ok(());
    }

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

    ctx.send(
        poise::CreateReply::default()
            .content("Submitted your report!")
            .embed(report_embed.clone())
            .ephemeral(true),
    )
    .await?;

    let report_buttons = generate_report_buttons(report_number, msg.link()).await;
    let report_channel = config_service::get_report_channel().await;
    report_channel
        .send_message(
            ctx,
            CreateMessage::new()
                .add_embed(report_embed.clone())
                .components(report_buttons),
        )
        .await?;

    Ok(())
}
