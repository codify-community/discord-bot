use anyhow::Result;
use poise::{
    serenity_prelude::{Context, Interaction, Role},
    Event,
};

use crate::primitives::{State, REGISTRO_ROLE_MARKER};

pub async fn handle_event(cx: &Context, event: &Event<'_>, state: &State) -> Result<()> {
    let guild_id = state.guild_id;

    if let Event::InteractionCreate {
        interaction: Interaction::MessageComponent(component),
    } = event
    {
        if let Some(embed) = component.message.embeds.first() {
            if let Some(ref description) = embed.description {
                let roles_id: Vec<Role> = description
                    .lines()
                    .filter(|l| l.starts_with(REGISTRO_ROLE_MARKER))
                    .map(|l| l.replace(REGISTRO_ROLE_MARKER, "").trim().to_string())
                    .map(|l| l.chars().filter(|c| c.is_numeric()).collect::<String>())
                    .flat_map(|s| s.parse::<u64>())
                    .filter_map(|rid| cx.cache.role(guild_id, rid))
                    .collect();

                component
                    .create_interaction_response(&cx.http, |m| {
                        m.interaction_response_data(|d| {
                            d.ephemeral(true).components(|c| {
                                c.create_action_row(|row| {
                                    row.create_select_menu(|sm| {
                                        sm.custom_id("role-resolve")
                                            .placeholder("Por favor selecione alguma opção")
                                            .min_values(1)
                                            .max_values(roles_id.len() as _)
                                            .options(|opts| {
                                                for role in roles_id {
                                                    opts.create_option(|o| {
                                                        o.label(role.name).value(role.id)
                                                    });
                                                }

                                                opts
                                            })
                                    })
                                })
                            })
                        })
                    })
                    .await?;
            }
        }
    }
    Ok(())
}
