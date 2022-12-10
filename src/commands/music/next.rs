use crate::primitives::Context;
use anyhow::{Context as _, Result};

#[poise::command(prefix_command, slash_command, aliases("skip", "next"))]
/// 「Música」Pula para a proxima música
pub async fn proximo(ctx: Context<'_>) -> Result<()> {
    let client = songbird::get(ctx.serenity_context())
        .await
        .context("Couldn't start songbird client")?;

    let handler = client
        .get(ctx.guild_id().context("No Guild!")?)
        .context("Must be in a voice channel to play music!")?;

    let handler = handler.lock().await;

    handler.queue().skip()?;

    ctx.say("Ok!").await?;

    Ok(())
}
