use ::serenity::all::{
    ComponentInteraction, CreateInteractionResponse, CreateInteractionResponseFollowup,
    CreateInteractionResponseMessage, CreateQuickModal, FullEvent, Interaction, UserId,
};
use poise::serenity_prelude as serenity;
use std::error::Error;

use crate::types::{self, ReportStatus};

use super::{
    report_service::{ban_report_chat, ban_report_db, dismiss_report_chat, dismiss_report_db},
    user_service::is_super_user,
};

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
    let _custom_id = &component_interaction.data.custom_id;

    if !is_super_user(&component_interaction.user.id).await? {
        component_interaction
            .create_followup(
                ctx,
                CreateInteractionResponseFollowup::new()
                    .content("You must be a super user to action a report.")
                    .ephemeral(true),
            )
            .await?;
        return Ok(());
    }

    if component_interaction.data.custom_id.starts_with("Dismiss") {
        return button_press_dismiss(ctx, component_interaction).await;
    }

    if component_interaction.data.custom_id.starts_with("Ban") {
        return button_press_ban(ctx, component_interaction).await;
    }

    /*component_interaction
    .create_followup(
        ctx,
        CreateInteractionResponseFollowup::new().content(format!(
            "Banning the report, but this hasn't been implemented yet! {}",
            component_interaction.data.custom_id
        )),
    )
    .await?;*/

    Ok(())
}

async fn button_press_dismiss(
    ctx: &serenity::client::Context,
    component_interaction: &ComponentInteraction,
) -> Result<(), types::Error> {
    component_interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Defer(
                CreateInteractionResponseMessage::new().ephemeral(true),
            ),
        )
        .await?;

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

    component_interaction
        .create_followup(
            ctx,
            CreateInteractionResponseFollowup::new()
                .content(format!("Dismissed report number {}.", report_id)),
        )
        .await?;

    Ok(())
}

async fn button_press_ban(
    ctx: &serenity::client::Context,
    component_interaction: &ComponentInteraction,
) -> Result<(), types::Error> {
    let interaction_id_split: Vec<&str> = component_interaction.data.custom_id.split(':').collect();
    let report_id: i32 = interaction_id_split[1].parse().unwrap();
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
            CreateInteractionResponse::Defer(CreateInteractionResponseMessage::new()),
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
    //Actually ban user here.

    Ok(())
}
