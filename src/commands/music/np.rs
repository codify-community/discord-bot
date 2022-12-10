use std::time::Duration;

use crate::{
    common::messages::{CANT_FIND_GUILD, CANT_START_SONGBIRD, IM_NOT_IN_A_VOICE_CHANNEL},
    primitives::Context,
    utils::time::HumanTime,
};
use anyhow::{Context as _, Result};
use poise::serenity_prelude::Color;

#[poise::command(prefix_command, slash_command, guild_only, aliases("np"))]
/// 「Música」Informa a música está tocando agora
pub async fn tocando(ctx: Context<'_>) -> Result<()> {
    let guild = ctx.guild().context(CANT_FIND_GUILD)?;

    let client = songbird::get(ctx.serenity_context())
        .await
        .context(CANT_START_SONGBIRD)?;

    let handler = client.get(guild.id).context(IM_NOT_IN_A_VOICE_CHANNEL)?;
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
            .context(IM_NOT_IN_A_VOICE_CHANNEL)?,
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
