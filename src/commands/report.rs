use crate::{
    services::{
        allow_list_service::is_allowed,
        config_service,
        report_service::*,
        user_service::{get_ban_reason, is_banned, is_banshee_bot, is_super_user, kick_user},
    },
    types::{self, ReportStatus},
};
use ::serenity::all::{CreateEmbed, CreateMessage};
use poise::serenity_prelude as serenity;

/// Report a message
#[poise::command(context_menu_command = "Report to Banshee")]
pub async fn report(
    ctx: types::Context<'_>,
    #[description = "Message to Report"] msg: serenity::Message,
) -> Result<(), types::Error> {
    ctx.defer_ephemeral().await?;
    let reporter = ctx.author();
    let author = &msg.author;
    let message = msg.content_safe(&ctx.cache());
    let message_guild = ctx.guild_id().unwrap();

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

    if is_allowed(&author.id, &message_guild).await? {
        ctx.send(
            poise::CreateReply::default()
                .content(format!(
                    "You can't report {} because they have been allow listed on this server.",
                    author.name
                ))
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    if is_banned(&author.id).await? {
        ctx.send(
            poise::CreateReply::default()
                .content(format!(
                    "Oops, that user was already supposed to be banned. Sorry!"
                )).ephemeral(true)
        ).await?;
        msg.delete(ctx).await?;
        let ban_reason = get_ban_reason(&author.id).await?;
        kick_user(ctx.serenity_context(), &message_guild, &author.id, Some(ban_reason)).await?;
        return Ok(())
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
