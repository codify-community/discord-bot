use crate::{
    common::messages::{CANT_FIND_GUILD, CANT_START_SONGBIRD},
    primitives::Context,
};
use anyhow::{Context as _, Result};

#[poise::command(prefix_command, slash_command, aliases("join"))]
/// 「Música」Conecta o bot à o canal que você está conectado
pub async fn entrar(ctx: Context<'_>) -> Result<()> {
    let guild = ctx.guild().context(CANT_FIND_GUILD)?;

    let channel = guild
        .voice_states
        .get(&ctx.author().id)
        .and_then(|c| c.channel_id)
        .context("Ei! Não tenho uma bola de cristal. Então eeh, não consigo adivinhar em qual canal você quer que eu toque a musica, Então você pode entrar em um canal de voz por favor?")?;

    let client = songbird::get(ctx.serenity_context())
        .await
        .context(CANT_START_SONGBIRD)?;

    client.join(guild.id, channel).await.1?;
    ctx.send(|m| m.ephemeral(true).content(":ok_hand: Feito."))
        .await?;

    Ok(())
}
