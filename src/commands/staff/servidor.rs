use crate::primitives::{AutoRole, Context};
use anyhow::{Context as _, Result};
use poise::serenity_prelude::{CacheHttp, ChannelId, Colour};
use std::{env, time::Instant};

#[poise::command(
    prefix_command,
    slash_command,
    aliases("sv", "svctl", "systemctl"),
    subcommands("registro_add_category")
)]
pub async fn servidor(_cx: Context<'_>) -> Result<()> {
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
    cx: Context<'_>,
    #[description = "Por favor indique o nome da categoria"] nome: String,
    #[description = "Por favor indique a imagem da categoria"] imagem: String,
    #[description = "Por favor indique a descrição da categoria"] descricao: String,
) -> Result<()> {
    let started = Instant::now();
    let handle = cx.say(":stopwatch:").await?;
    let registro_id = env::var("CODIFY_REGISTRO_ID")
        .context("Can't get $CODIFY_REGISTRO_ID")?
        .parse()
        .context("Invalid Registro ID!")?;

    let Some(channel)  = cx.guild()
        .unwrap()
        .channels
        .iter()
        .find(|it| *it.0 == ChannelId(registro_id)).map(|(_k, v)| v.id()) else {
            cx.say("Não achei o canal de registro, bad config?").await?;
            return Ok(());
        };

    let message = channel
        .send_message(cx.http(), |m| {
            m.embed(|e| {
                e.title(&nome)
                    .image(imagem)
                    .colour(Colour::FOOYOO)
                    .description(descricao)
            })
        })
        .await?;

    cx.data().database.auto_rules_messages.write(|ar| {
        ar.push(AutoRole {
            category: nome,
            id: message.id,
        });
    })?;
    cx.data().database.auto_rules_messages.save()?;

    handle
        .edit(cx, |m| {
            m.content(format!("OK in {:.2?}", started.elapsed()))
        })
        .await?;
    Ok(())
}
