use crate::primitives::{AutoRole, Context, REGISTRO_ROLE_MARKER};
use anyhow::{bail, Context as _, Result};
use poise::serenity_prelude::{ButtonStyle, CacheHttp, ChannelId, Colour, Mentionable, Role};
use std::{env, time::Instant};

#[poise::command(
    prefix_command,
    slash_command,
    aliases("sv"),
    subcommands(
        "registro_add_category",
        "registro_add_role",
        "registro_remove_role",
        "registro_remove_category"
    )
)]
#[allow(clippy::unused_async)]
pub async fn servidor(_ctx: Context<'_>) -> Result<()> {
    Ok(())
}

/// 「STAFF」Adiciona uma categoria ao registro
#[poise::command(
    prefix_command,
    slash_command,
    aliases("Rac"),
    default_member_permissions = "MANAGE_GUILD"
)]
pub async fn registro_add_category(
    ctx: Context<'_>,
    #[description = "Por favor indique o nome da categoria"] nome: String,
    #[description = "Por favor indique a imagem da categoria"] imagem: String,
    #[description = "Por favor indique a descrição da categoria"] descricao: String,
) -> Result<()> {
    let started = Instant::now();
    let handle = ctx.say(":stopwatch:").await?;
    let registro_id = env::var("REGISTRO_ID")?.parse()?;

    let Some(channel)  = ctx.guild()
        .unwrap()
        .channels
        .iter()
        .find(|it| *it.0 == ChannelId(registro_id)).map(|(_k, v)| v.id()) else {
            ctx.say("Não achei o canal de registro, bad config?").await?;
            return Ok(());
        };

    let message = channel
        .send_message(ctx.http(), |m| {
            m.embed(|e| {
                e.title(&nome)
                    .image(imagem)
                    .colour(Colour::BLURPLE)
                    .description(descricao)
            })
            .components(|c| {
                c.create_action_row(|row| {
                    row.create_button(|b| {
                        b.label("Selecionar cargos")
                            .style(ButtonStyle::Secondary)
                            .custom_id("registro-select-roles")
                    })
                })
            })
        })
        .await?;

    ctx.data().database.auto_rules_messages.write(|ar| {
        ar.push(AutoRole {
            category: nome,
            id: message.id.0,
            channel_id: message.channel_id.0,
        });
    })?;
    ctx.data().database.auto_rules_messages.save()?;

    handle
        .edit(ctx, |m| {
            m.content(format!("OK in {:.2?}", started.elapsed()))
        })
        .await?;
    Ok(())
}

/// 「STAFF」Adiciona um cargo a categoria
#[poise::command(
    prefix_command,
    slash_command,
    aliases("Rar"),
    default_member_permissions = "MANAGE_GUILD"
)]
pub async fn registro_add_role(
    ctx: Context<'_>,
    #[description = "Por favor indique o nome da categoria"] nome: String,
    #[description = "Por favor indique um cargo"] cargo: Role,
) -> Result<()> {
    let started = Instant::now();
    let handle = ctx.say(":stopwatch:").await?;
    let ar_message = ctx
        .data()
        .database
        .auto_rules_messages
        .read(move |ar| ar.clone().into_iter().find(|i| i.category == nome))?
        .context("Não foi possivel encontrar a categoria.")?;

    let mut message = ctx
        .http()
        .get_message(ar_message.channel_id, ar_message.id)
        .await?;

    let embed = message.embeds.first_mut().context("Mensagem inválida")?;
    let description = embed.description.as_ref().context("Mensagem inválida")?;

    if description.contains(&cargo.mention().to_string()) {
        handle.delete(ctx).await?;
        bail!(":x: Esse cargo já existe nessa categoria.");
    }

    embed.description = Some(format!(
        "{description}\n{REGISTRO_ROLE_MARKER} {}",
        cargo.mention()
    ));

    let embed = embed.clone();
    message
        .edit(ctx.http(), |m| m.set_embed(embed.into()))
        .await?;

    handle
        .edit(ctx, |m| {
            m.content(format!("OK in {:.2?}", started.elapsed()))
        })
        .await?;
    Ok(())
}

/// 「STAFF」Remove um cargo da categoria
#[poise::command(
    prefix_command,
    slash_command,
    aliases("Rrr"),
    default_member_permissions = "MANAGE_GUILD"
)]
pub async fn registro_remove_role(
    ctx: Context<'_>,
    #[description = "Por favor indique o nome da categoria"] nome: String,
    #[description = "Por favor indique um cargo"] cargo: Role,
) -> Result<()> {
    let started = Instant::now();
    let handle = ctx.say(":stopwatch:").await?;
    let ar_message = ctx
        .data()
        .database
        .auto_rules_messages
        .read(move |ar| ar.clone().into_iter().find(|i| i.category == nome))?
        .context("Não foi possivel encontrar a categoria.")?;

    let mut message = ctx
        .http()
        .get_message(ar_message.channel_id, ar_message.id)
        .await?;

    let mut embed = message
        .embeds
        .first_mut()
        .context("Mensagem inválida")?
        .clone();

    let description = embed.description.as_ref().context("Invalid message")?;

    if !description.contains(&cargo.mention().to_string()) {
        handle.delete(ctx).await?;
        bail!(":x: Esse cargo não existe na categoria.");
    }

    embed.description =
        Some(description.replace(&format!("\n{REGISTRO_ROLE_MARKER} {}", cargo.mention()), ""));

    message
        .edit(ctx.http(), |m| m.set_embed(embed.into()))
        .await?;

    handle
        .edit(ctx, |m| {
            m.content(format!("OK in {:.2?}", started.elapsed()))
        })
        .await?;
    Ok(())
}

/// 「STAFF」Remove uma categoria
#[poise::command(
    prefix_command,
    slash_command,
    aliases("Rrc"),
    default_member_permissions = "MANAGE_GUILD"
)]
pub async fn registro_remove_category(
    ctx: Context<'_>,
    #[description = "Por favor indique o nome da categoria"] nome: String,
) -> Result<()> {
    let started = Instant::now();
    let handle = ctx.say(":stopwatch:").await?;
    let ar_message = ctx
        .data()
        .database
        .auto_rules_messages
        .read(move |ar| ar.clone().into_iter().find(|i| i.category == nome))?
        .context("Não foi possivel encontrar a categoria.")?;

    let message = ctx
        .http()
        .get_message(ar_message.channel_id, ar_message.id)
        .await?;

    message.delete(&ctx).await?;

    ctx.data()
        .database
        .auto_rules_messages
        .write(move |ar| ar.retain(|m| m.category != ar_message.category))?;

    ctx.data().database.auto_rules_messages.save()?;

    handle
        .edit(ctx, |m| {
            m.content(format!("OK in {:.2?}", started.elapsed()))
        })
        .await?;

    Ok(())
}
