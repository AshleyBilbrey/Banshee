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
    let custom_id = &component_interaction.data.custom_id;
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
                CreateInteractionResponseFollowup::new().content(format!(
                    "You can't report {} because they are a super user.",
                    &component_interaction.user.name
                )),
            )
            .await?;
        return Ok(());
    }

    if component_interaction.user.id.get() == ctx.cache.current_user().id.get() {
        component_interaction
            .create_followup(
                ctx,
                CreateInteractionResponseFollowup::new()
                    .content("Nice try, but you can't report me!")
                    .add_file(CreateAttachment::url(
                        ctx,
                        "https://tenor.com/view/chika-fujiwara-hit-cute-kawaii-anime-gif-13583613",
                    ).await?),
            )
            .await?;
        return Ok(());
    }

    component_interaction
        .create_followup(
            ctx,
            CreateInteractionResponseFollowup::new().content(format!("You clicked {}", custom_id)),
        )
        .await?;

    Ok(())
}
