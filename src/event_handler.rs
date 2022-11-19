use anyhow::{Context as _, Result};
use poise::{
    serenity_prelude::{Context, Interaction, Role, RoleId},
    Event,
};

use crate::primitives::{State, REGISTRO_ROLE_MARKER};

pub async fn handle_event(cx: &Context, event: &Event<'_>, state: &State) -> Result<()> {
    let guild_id = state.guild_id;

    if let Event::InteractionCreate {
        interaction: Interaction::MessageComponent(component),
    } = event
    {
        match component.data.custom_id.as_str() {
            "registro-select-roles" => {
                tracing::debug!(user = ?component.user, "Sending select menu to user.");

                if let Some(embed) = component.message.embeds.first() {
                    if let Some(ref description) = embed.description {
                        let roles: Vec<Role> = description
                            .lines()
                            .filter(|l| l.starts_with(REGISTRO_ROLE_MARKER))
                            .map(|l| l.replace(REGISTRO_ROLE_MARKER, "").trim().to_string())
                            .map(|l| l.chars().filter(|c| c.is_numeric()).collect::<String>())
                            .flat_map(|s| s.parse::<u64>())
                            .filter_map(|rid| cx.cache.role(guild_id, rid))
                            .collect();

                        if roles.is_empty() {
                            component.create_interaction_response(&cx.http, |m| {
                                m.interaction_response_data(|d| {
                                    d.ephemeral(true).content("A direção do servidor ainda não configurou essa seção. Por favor aguarde até que eles configurem. ")
                                })
                            }).await?;

                            return Ok(());
                        }

                        tracing::trace!("Found {roles:?} in message.");

                        component
                            .create_interaction_response(&cx.http, |m| {
                                m.interaction_response_data(|d| {
                                    d.ephemeral(true).components(|c| {
                                        c.create_action_row(|row| {
                                            row.create_select_menu(|sm| {
                                                sm.custom_id("role-resolve")
                                                    .placeholder("Por favor selecione alguma opção")
                                                    .min_values(1)
                                                    .max_values(roles.len() as _)
                                                    .options(|opts| {
                                                        for role in roles {
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

            "role-resolve" => {
                tracing::debug!("Received role-resolve event");
                let mut roles: Vec<RoleId> = component
                    .data
                    .values
                    .iter()
                    .flat_map(|s| s.parse::<u64>())
                    .map(RoleId)
                    .collect();

                tracing::info!(user = ?component.user, "Add {roles:?}");

                let mut member = cx
                    .http
                    .get_member(guild_id, component.user.id.0)
                    .await
                    .context("Can't find member!")?;

                let mut to_remove = vec![];

                for role in &roles {
                    if let Some(roles) = member.roles(&cx.cache) {
                        if roles.iter().any(|r| r.id == *role) {
                            to_remove.push(*role);
                        }
                    }
                }

                roles.retain(|x| !to_remove.contains(x));

                if !to_remove.is_empty() {
                    tracing::info!("Removing {to_remove:?} roles from {}.", component.user.id);
                    member.remove_roles(&cx.http, &to_remove).await?;
                }

                if !roles.is_empty() {
                    tracing::info!("Adding {roles:?} roles to {}.", component.user.id);
                    member.add_roles(&cx.http, &roles).await?;
                }

                component
                    .create_interaction_response(&cx.http, |m| {
                        m.interaction_response_data(|d|
                        d.ephemeral(true).content(
                            "Prontinho! Caso você queira remover ou adicionar novos cargos é só clicar no `Selecionar Cargos` novamente.",
                        ))
                    })
                    .await?;
            }

            _ => {
                tracing::warn!("Event not handled: {}", component.data.custom_id);
            }
        }
    }
    Ok(())
}
