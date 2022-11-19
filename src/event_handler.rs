use anyhow::Result;
use poise::{
    serenity_prelude::{Context, Interaction},
    Event,
};

use crate::{handlers::role_selection::on_interaction_component_create, primitives::State};

pub async fn handle_event(cx: &Context, event: &Event<'_>, state: &State) -> Result<()> {
    match event {
        Event::InteractionCreate {
            interaction: Interaction::MessageComponent(component),
        } => {
            on_interaction_component_create(cx, component, state).await?;
        }

        Event::Ready { .. } => {
            tracing::info!("Bot is ready!");
        }
        _ => {}
    }
    Ok(())
}
