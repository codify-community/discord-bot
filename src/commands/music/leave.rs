use crate::primitives::Context;
use anyhow::{Context as _, Result};
use std::time::Instant;

#[poise::command(prefix_command, slash_command)]
/// Desconecta o bot do canal que você está conectado
pub async fn leave(ctx: Context<'_>) -> Result<()> {
    let guild = ctx.guild().context("No Guild!")?;

    let ts = Instant::now();
    let handler = ctx.say("Saindo...").await?;

    let client = songbird::get(ctx.serenity_context())
        .await
        .context("Couldn't start songbird client")?;

    client.remove(guild.id).await?;

    handler
        .edit(ctx, |e| {
            e.content(format!("Saiu em `{:.2?}`", ts.elapsed()))
        })
        .await?;

    Ok(())
}
