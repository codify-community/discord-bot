use crate::primitives::Context;
use anyhow::{Context as _, Result};
use poise::serenity_prelude::Colour;

#[poise::command(prefix_command, slash_command)]
/// Mostra a fila de reprodução
pub async fn queue(ctx: Context<'_>) -> Result<()> {
    let client = songbird::get(ctx.serenity_context())
        .await
        .context("Couldn't start songbird client")?;

    let handler = client
        .get(ctx.guild_id().context("No Guild!")?)
        .context("Must be in a voice channel to play music!")?;

    let handler = handler.lock().await;

    ctx.send(|message| {
        message.embed(|embed| {
            let mut content = String::new();
            let queue = handler.queue().current_queue();

            for (index, video) in queue.iter().enumerate() {
                let title = video.metadata().title.as_ref().unwrap();
                content.push_str(format!("{index} - {title}\n").as_ref());
            }

            if queue.is_empty() {
                embed.title("Fila de reprodução vazia")
            } else {
                embed
                    .title("Fila de reprodução")
                    .description(content)
                    .colour(Colour::DARK_PURPLE)
            }
        })
    })
    .await?;

    Ok(())
}
