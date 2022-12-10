use crate::primitives::Context;
use anyhow::{Context as _, Result};

#[poise::command(prefix_command, slash_command, aliases("join"))]
/// 「Música」Conecta o bot à o canal que você está conectado
pub async fn entrar(ctx: Context<'_>) -> Result<()> {
    let guild = ctx.guild().context("No Guild!")?;

    let channel = guild
        .voice_states
        .get(&ctx.author().id)
        .and_then(|c| c.channel_id)
        .context("Can't find a voice channel")?;

    let client = songbird::get(ctx.serenity_context())
        .await
        .context("Couldn't start songbird client")?;

    client.join(guild.id, channel).await.1?;

    ctx.say("Pronto :+1:!").await?;

    Ok(())
}
