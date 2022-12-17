use crate::{
    common::messages::{CANT_FIND_GUILD, CANT_START_SONGBIRD, IM_NOT_IN_A_VOICE_CHANNEL},
    primitives::Context,
};
use anyhow::{Context as _, Result};

#[poise::command(prefix_command, slash_command, aliases("skip", "next"))]
/// 「Música」Pula para a proxima música
pub async fn proximo(ctx: Context<'_>) -> Result<()> {
    let client = songbird::get(ctx.serenity_context())
        .await
        .context(CANT_START_SONGBIRD)?;

    let handler = client
        .get(ctx.guild_id().context(CANT_FIND_GUILD)?)
        .context(IM_NOT_IN_A_VOICE_CHANNEL)?;

    let handler = handler.lock().await;

    if let Some(rn) = handler.queue().current_queue().iter().nth(1) {
        handler.queue().skip()?;

        ctx.send(|m| {
            m.ephemeral(true).content(format!(
                ":ok_hand: Feito. Agora estou tocando `{}`",
                rn.metadata().title.as_ref().unwrap(),
            ))
        })
        .await?;
    } else {
        ctx.send(|m| {
            m.ephemeral(true)
                .content(":ok_hand: não tem nenhuma música próxima na fila agora então eu continuarei a tocar a música atual.".to_string())
        })
        .await?;
    }

    Ok(())
}
