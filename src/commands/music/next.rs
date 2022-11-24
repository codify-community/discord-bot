use crate::primitives::Context;
use anyhow::{Result, Context as _};

#[poise::command(prefix_command, slash_command)]
pub async fn next(ctx: Context<'_>) -> Result<()> {
    let client = songbird::get(ctx.serenity_context())
        .await
        .context("Couldn't start songbird client")?;

    let handler = client
        .get(ctx.guild_id().context("No Guild!")?)
        .context("Must be in a voice channel to play music!")?;

    let handler = handler.lock().await;

    handler.queue().skip()?;

    Ok(())
}
