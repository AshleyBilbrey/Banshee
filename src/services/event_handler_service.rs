use ::serenity::all::{
    ComponentInteraction, CreateAttachment, CreateInteractionResponse,
    CreateInteractionResponseFollowup, CreateInteractionResponseMessage, FullEvent, Interaction,
};
use poise::serenity_prelude as serenity;
use std::error::Error;

use crate::types;

use super::user_service::is_super_user;

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
            CreateInteractionResponse::Defer(CreateInteractionResponseMessage::new()),
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

    component_interaction
        .create_followup(
            ctx,
            CreateInteractionResponseFollowup::new()
                .content("Actioning the report, but this hasn't been implemented yet!"),
        )
        .await?;

    Ok(())
}
