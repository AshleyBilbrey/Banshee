use ::serenity::all::{
    ComponentInteraction, CreateInteractionResponse, CreateInteractionResponseFollowup,
    CreateInteractionResponseMessage, FullEvent, Interaction,
};
use poise::serenity_prelude as serenity;
use std::error::Error;

use crate::types;

use super::{report_service::dismiss_report_db, user_service::is_super_user};

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
    component_interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Defer(
                CreateInteractionResponseMessage::new().ephemeral(true),
            ),
        )
        .await?;

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

    component_interaction
        .create_followup(
            ctx,
            CreateInteractionResponseFollowup::new().content(format!(
                "Banning the report, but this hasn't been implemented yet! {}",
                component_interaction.data.custom_id
            )),
        )
        .await?;

    Ok(())
}

async fn button_press_dismiss(
    ctx: &serenity::client::Context,
    component_interaction: &ComponentInteraction,
) -> Result<(), types::Error> {
    let interaction_id_split: Vec<&str> = component_interaction.data.custom_id.split(':').collect();
    let report_id: i32 = interaction_id_split[1].parse().unwrap();
    dismiss_report_db(report_id).await?;

    component_interaction
        .create_followup(
            ctx,
            CreateInteractionResponseFollowup::new().content(format!(
                "Dismissing the report, but this hasn't been implemented yet! {}",
                report_id
            )),
        )
        .await?;

    Ok(())
}
