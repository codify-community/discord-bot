use crate::primitives::Context;
use anyhow::{Context as _, Result};
use poise::serenity_prelude::Color;

#[poise::command(prefix_command, slash_command)]
/// Informa que música está tocando
pub async fn np(ctx: Context<'_>) -> Result<()> {
    let guild = ctx.guild().context("No Guild!")?;

    let client = songbird::get(ctx.serenity_context())
        .await
        .context("Couldn't start songbird client")?;

    let handler = client
        .get(guild.id)
        .context("Must be in a voice channel to play music!")?;
    let handler = handler.lock().await;

    let queue = handler.queue().current_queue();
    let current = queue.get(0).context("No queue!")?;
    let metadata = current.metadata();

    let play_time = current.get_info().await?.play_time;
    let duration = metadata.duration.as_ref().unwrap();
    let playing = metadata.source_url.as_ref().unwrap();
    let thumb = metadata.thumbnail.as_ref().unwrap();
    let title = metadata.title.as_ref().unwrap();

    ctx.send(|msg| {
        msg.embed(|embed| {
            embed
                .title(title)
                .url(playing)
                .image(thumb)
                .color(Color::RED)
                .footer(|footer| {
                    footer.text(format!("⏱️ Tempo restante: {:.0?}", *duration - play_time))
                })
        })
    })
    .await?;

    Ok(())
}
