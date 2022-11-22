use crate::primitives::{AutoRole, Context, REGISTRO_ROLE_MARKER};
use anyhow::{Context as _, Result};
use poise::serenity_prelude::{ButtonStyle, CacheHttp, ChannelId, Colour, Mentionable, Role};
use std::{env, time::Instant};

#[poise::command(
    prefix_command,
    slash_command,
    aliases("sv", "svctl", "systemctl"),
    subcommands("registro_add_category", "registro_add_role")
)]
pub async fn servidor(_ctx: Context<'_>) -> Result<()> {
    Ok(())
}

///〔🛠️ Staff〕Adiciona uma categoria ao registro
#[poise::command(
    prefix_command,
    slash_command,
    aliases("rac", "registroAddCategory", "regAddCat", "registro-enable-category"),
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
    let registro_id = env::var("CODIFY_REGISTRO_ID")
        .context("Can't get $CODIFY_REGISTRO_ID")?
        .parse()
        .context("Invalid Registro ID!")?;

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
                    .colour(Colour::FOOYOO)
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

///〔🛠️ Staff〕Adiciona um cargo a categoria
#[poise::command(
    prefix_command,
    slash_command,
    aliases("rar", "registroAddRole", "regAddRol", "registro-enable-role"),
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

    embed.description = Some(format!(
        "{}\n{REGISTRO_ROLE_MARKER} {}",
        embed.description.as_ref().context("Mensagem inválida")?,
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
