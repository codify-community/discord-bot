use anyhow::Context as _;
use poise::serenity_prelude::{
    ActionRowComponent, Context, Embed, MessageComponentInteraction, Role, RoleId,
};

use crate::primitives::{State, REGISTRO_ROLE_MARKER};

fn get_roles_from_embed(cx: &Context, guild_id: u64, embed: &Embed) -> Option<Vec<Role>> {
    let description = embed.description.clone()?;

    let roles: Vec<Role> = description
        .lines()
        .filter(|l| l.starts_with(REGISTRO_ROLE_MARKER))
        .map(|l| l.replace(REGISTRO_ROLE_MARKER, "").trim().to_string())
        .map(|l| l.chars().filter(|c| c.is_numeric()).collect::<String>())
        .flat_map(|s| s.parse::<u64>())
        .filter_map(|rid| cx.cache.role(guild_id, rid))
        .collect();

    Some(roles)
}

pub async fn on_interaction_component_create(
    cx: &Context,
    component: &MessageComponentInteraction,
    state: &State,
) -> anyhow::Result<()> {
    let guild_id = state.guild_id;

    let roles_from_embed = || get_roles_from_embed(cx, guild_id, component.message.embeds.first()?);

    match component.data.custom_id.as_str() {
        "registro-select-roles" => {
            tracing::debug!(user = ?component.user, "Sending select menu to user.");

            let roles = roles_from_embed().context("Failed do get roles from embed")?;

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

        "role-resolve" => {
            component.defer(&cx.http).await?;
            tracing::debug!("Received role-resolve event");
            let to_add: Vec<RoleId> = component
                .data
                .values
                .iter()
                .flat_map(|s| s.parse::<u64>())
                .map(RoleId)
                .collect();

            tracing::info!(user = ?component.user, "Add {to_add:?}");

            let mut member = cx
                .http
                .get_member(guild_id, component.user.id.0)
                .await
                .context("Can't find member!")?;

            let member_roles: Vec<RoleId> = member
                .roles(&cx.cache)
                .context("Failed to get roles")?
                .into_iter()
                .map(|p| p.id)
                .collect();

            let ActionRowComponent::SelectMenu(ref sm) = component
                .message
                .components
                .first()
                .context("Invalid message")?
                .components[0] else {
                    anyhow::bail!("Unexpected component");
                };

            let roles: Vec<_> = sm
                .options
                .iter()
                .map(|it| it.value.clone())
                .flat_map(|s| s.parse::<u64>())
                .filter_map(|rid| cx.cache.role(guild_id, rid))
                .map(|p| p.id)
                .collect();

            let to_remove: Vec<_> = roles
                .iter()
                .filter(|m| member_roles.contains(m))
                .copied()
                .collect();

            if !to_remove.is_empty() {
                tracing::info!("Removing {to_remove:?} roles from {}.", component.user.id);

                member.remove_roles(&cx.http, &to_remove).await?;
            }

            if !to_add.is_empty() {
                tracing::info!("Adding {to_add:?} roles to {}.", component.user.id);

                member.add_roles(&cx.http, &to_add).await?;
            }

            component
                    .create_followup_message(&cx.http, |m| {
                        m.ephemeral(true).content(
                            "Prontinho! Caso você queira remover ou adicionar novos cargos é só clicar no `Selecionar Cargos` novamente.",
                        )
                    })
                    .await?;
        }

        _ => {
            tracing::warn!("Event not handled: {}", component.data.custom_id);
        }
    }

    Ok(())
}
