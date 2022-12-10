use std::time::Duration;

use crate::{primitives::Context, utils::time::HumanTime};
use anyhow::{Context as _, Result};
use poise::serenity_prelude::Color;

#[poise::command(prefix_command, slash_command, guild_only)]
/// Informa que música está tocando
pub async fn np(ctx: Context<'_>) -> Result<()> {
    let guild = ctx
        .guild()
        .context("Whoar! Não estou em uma guilda. Esse comando só funciona em uma guidla :>")?;

    let client = songbird::get(ctx.serenity_context())
        .await
        .context("Couldn't start songbird client")?;

    let handler = client
        .get(guild.id)
        .context("Yo, Não estou em um canal de voz. Eu estou tocando algo?")?;
    let handler = handler.lock().await;

    let queue = handler.queue().current_queue();
    let current = queue.get(0).context("Não tem nada tocando agora.")?;
    let metadata = current.metadata();

    let play_time = current.get_info().await?.play_time;
    let duration = metadata.duration.unwrap();
    let playing = metadata.source_url.as_ref().unwrap();
    let thumb = metadata.thumbnail.as_ref().unwrap();
    let title = metadata.title.as_ref().unwrap();

    let description = format!(
        r#"
    Tocando em: <#{}>
    "#,
        handler
            .current_channel()
            .context("Estranho, parece que não estou em um canal de voz")?,
    );

    ctx.send(|msg| {
        msg.embed(|embed| {
            embed
                .title(title)
                .url(playing)
                .image(thumb)
                .color(Color::RED)
                .description(description)
                .author(|a| {
                    a.name(
                        metadata
                            .artist
                            .clone()
                            .or(metadata.channel.clone())
                            .unwrap_or_default(),
                    )
                })
                .footer(|f| {
                    let remaining = (duration - play_time).as_secs();

                    f.text(format!(
                        "⏱️ Tempo restante: {} / {}",
                        HumanTime(Duration::from_secs(remaining)),
                        HumanTime(metadata.duration.unwrap())
                    ))
                })
        })
    })
    .await?;

    Ok(())
}
