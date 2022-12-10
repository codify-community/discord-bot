use crate::{
    common::messages::{CANT_FIND_GUILD, CANT_START_SONGBIRD, IM_NOT_IN_A_VOICE_CHANNEL},
    primitives::Context,
};
use anyhow::{bail, Context as _, Result};

#[poise::command(prefix_command, slash_command, aliases("leave"))]
/// 「Música」Desconecta o bot do canal que você está conectado
pub async fn sair(ctx: Context<'_>) -> Result<()> {
    let guild = ctx.guild().context(CANT_FIND_GUILD)?;

    let client = songbird::get(ctx.serenity_context())
        .await
        .context(CANT_START_SONGBIRD)?;

    if client.get(guild.id).is_some() {
        client.remove(guild.id).await?;
        ctx.say("Pronto :+1:!").await?;
    } else {
        bail!(IM_NOT_IN_A_VOICE_CHANNEL)
    }

    Ok(())
}
