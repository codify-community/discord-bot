use crate::primitives::Context;
use anyhow::{Context as _, Result};
use std::time::Instant;

#[poise::command(prefix_command, slash_command)]
/// Conecta o bot à o canal que você está conectado
pub async fn join(ctx: Context<'_>) -> Result<()> {
    let guild = ctx.guild().context("No Guild!")?;

    let ts = Instant::now();
    let handler = ctx.say("Entrando...").await?;

    let channel = guild
        .voice_states
        .get(&ctx.author().id)
        .and_then(|c| c.channel_id)
        .context("Can't find a voice channel")?;

    let client = songbird::get(ctx.serenity_context())
        .await
        .context("Couldn't start songbird client")?;

    client.join(guild.id, channel).await.1?;

    handler
        .edit(ctx, |e| {
            e.content(format!("Entrou em `{:.2?}`", ts.elapsed()))
        })
        .await?;

    Ok(())
}
