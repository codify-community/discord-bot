use crate::primitives::Context;
use anyhow::{Context as _, Result};

#[poise::command(prefix_command, slash_command)]
/// Desconecta o bot do canal que você está conectado
pub async fn leave(ctx: Context<'_>) -> Result<()> {
    let guild = ctx.guild().context("No Guild!")?;

    let client = songbird::get(ctx.serenity_context())
        .await
        .context("Couldn't start songbird client")?;

    client.remove(guild.id).await?;

    ctx.say("Pronto :+1:!").await?;

    Ok(())
}
