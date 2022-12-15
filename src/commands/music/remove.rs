use crate::{
    common::messages::{CANT_FIND_GUILD, CANT_START_SONGBIRD, IM_NOT_IN_A_VOICE_CHANNEL},
    primitives::Context,
};
use anyhow::{Context as _, Result};

#[poise::command(prefix_command, slash_command, aliases("remove"))]
/// 「Música」Pula para a proxima música
pub async fn remover(ctx: Context<'_>, id: usize) -> Result<()> {
    let client = songbird::get(ctx.serenity_context())
        .await
        .context(CANT_START_SONGBIRD)?;

    let handler = client
        .get(ctx.guild_id().context(CANT_FIND_GUILD)?)
        .context(IM_NOT_IN_A_VOICE_CHANNEL)?;

    let handler = handler.lock().await;

    let removed = handler.queue().modify_queue(|q| q.remove(id));

    if let Some(removed) = removed {
        ctx.send(|m| {
            m.ephemeral(true).content(format!(
                ":ok_hand: Feito. A Música `{}` foi removida.",
                removed.metadata().title.clone().unwrap_or_default()
            ))
        })
        .await?;
    } else {
        ctx.send(|m| {
            m.ephemeral(true)
                .content(":x: Não foi possível encontrar a musica com o ID desejado.".to_string())
        })
        .await?;

    }

    Ok(())
}
