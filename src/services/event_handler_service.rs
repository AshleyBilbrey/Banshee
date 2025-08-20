use super::{
    report_service::{
        self, ban_report_chat, ban_report_db, dismiss_report_chat, dismiss_report_db,
    },
    user_service::{is_banned, is_super_user},
};
use crate::{
    services::user_service::ban,
    types::{self, ReportStatus},
};
use ::serenity::all::{
    ComponentInteraction, CreateInteractionResponse, CreateInteractionResponseFollowup,
    CreateInteractionResponseMessage, CreateQuickModal, FullEvent, Interaction, UserId,
};
use poise::serenity_prelude as serenity;
use std::error::Error;

pub async fn event_handler(
    ctx: &serenity::client::Context,
    event: &serenity::FullEvent,
    _framework_ctx: poise::FrameworkContext<'_, types::Data, Box<dyn Error + Send + Sync>>,
) -> Result<(), types::Error> {
    if let FullEvent::InteractionCreate { interaction } = event {
        if let Interaction::Component(component_interaction) = interaction {
            button_press(ctx, component_interaction).await?;
        }
    }

    Ok(())
}

async fn button_press(
    ctx: &serenity::client::Context,
    component_interaction: &ComponentInteraction,
) -> Result<(), types::Error> {
    if !is_super_user(&component_interaction.user.id).await? {
        component_interaction
            .create_response(
                ctx,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("You must be a super user to action a report.")
                        .ephemeral(true),
                ),
            )
            .await?;
        return Ok(());
    }

    if component_interaction.data.custom_id.starts_with("Dismiss") {
        return button_press_dismiss(ctx, component_interaction, true).await;
    }

    if component_interaction.data.custom_id.starts_with("Ban") {
        return button_press_ban(ctx, component_interaction).await;
    }

    component_interaction
        .create_followup(
            ctx,
            CreateInteractionResponseFollowup::new().content(format!(
                "Sorry, there was an issue processing your request. Interaction ID: {}",
                component_interaction.data.custom_id
            )),
        )
        .await?;

    Ok(())
}

async fn button_press_dismiss(
    ctx: &serenity::client::Context,
    component_interaction: &ComponentInteraction,
    respond: bool,
) -> Result<(), types::Error> {
    component_interaction.defer(ctx).await?;

    let interaction_id_split: Vec<&str> = component_interaction.data.custom_id.split(':').collect();
    let report_id: i32 = interaction_id_split[1].parse().unwrap();
    let report = dismiss_report_db(report_id).await?;
    dismiss_report_chat(
        ctx,
        component_interaction.message.clone(),
        &report.message_body,
        &UserId::new(report.author_snowflake as u64)
            .to_user(ctx)
            .await?,
        &UserId::new(report.reporter_snowflake as u64)
            .to_user(ctx)
            .await?,
        report_id,
        types::ReportStatus::Dismissed,
    )
    .await?;

    if respond {
        component_interaction
            .create_followup(
                ctx,
                CreateInteractionResponseFollowup::new()
                    .content(format!("Dismissed report number {}.", report_id)),
            )
            .await?;
    }

    Ok(())
}

async fn button_press_ban(
    ctx: &serenity::client::Context,
    component_interaction: &ComponentInteraction,
) -> Result<(), types::Error> {
    let interaction_id_split: Vec<&str> = component_interaction.data.custom_id.split(':').collect();
    let report_id: i32 = interaction_id_split[1].parse().unwrap();

    let reported_user = report_service::get_reported_user(report_id).await?;
    if is_super_user(&reported_user).await?
        || ctx.http.get_current_user().await?.id == reported_user
    {
        component_interaction
            .create_followup(
                ctx,
                CreateInteractionResponseFollowup::new()
                    .ephemeral(true)
                    .content("Oops, this user can't be banned."),
            )
            .await?;

        return Ok(button_press_dismiss(ctx, component_interaction, false).await?);
    }

    if is_banned(&reported_user).await? {
        component_interaction
            .create_followup(
                ctx,
                CreateInteractionResponseFollowup::new()
                    .ephemeral(true)
                    .content("Oops, this user is already banned."),
            )
            .await?;

        return Ok(button_press_dismiss(ctx, component_interaction, false).await?);
    }

    let modal = CreateQuickModal::new("Are you sure you want to ban?")
        .timeout(std::time::Duration::from_secs(600))
        .short_field("Ban Reason");

    let response = component_interaction
        .quick_modal(ctx, modal)
        .await?
        .unwrap();
    response
        .interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Defer(
                CreateInteractionResponseMessage::new().ephemeral(true),
            ),
        )
        .await?;
    let ban_reason = &response.inputs[0];
    let report = ban_report_db(report_id).await?;
    ban_report_chat(
        ctx,
        component_interaction.message.clone(),
        &report.message_body,
        &UserId::new(report.author_snowflake as u64)
            .to_user(ctx)
            .await?,
        &UserId::new(report.reporter_snowflake as u64)
            .to_user(ctx)
            .await?,
        report_id,
        ReportStatus::Banned,
    )
    .await?;

    response
        .interaction
        .create_followup(
            ctx,
            CreateInteractionResponseFollowup::new()
                .content(format!(
                    "Banned {}{}.",
                    report_id,
                    (response
                        .inputs
                        .get(0)
                        .map(|input| format!(" for {}", input))
                        .unwrap_or_default())
                ))
                .ephemeral(true),
        )
        .await?;

    ban(ctx, &reported_user, Some(ban_reason.clone())).await?;

    Ok(())
}
