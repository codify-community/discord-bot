use crate::{
    common::messages::{CANT_FIND_GUILD, CANT_START_SONGBIRD, IM_NOT_IN_A_VOICE_CHANNEL},
    primitives::Context,
};
use anyhow::{Context as _, Result};
use songbird::driver::Bitrate;

#[poise::command(prefix_command, slash_command, aliases("play"))]
/// 「Música」Toca uma música
pub async fn tocar(
    ctx: Context<'_>,
    #[description = "URL do youtube ou nome"] song: String,
) -> Result<()> {
    ctx.defer_ephemeral().await?;

    let guild = ctx.guild().context(CANT_FIND_GUILD)?;
    let mut query = song;

    if !query.starts_with("http") {
        query = format!("ytsearch1:{query}");
    }

    let client = songbird::get(ctx.serenity_context())
        .await
        .context(CANT_START_SONGBIRD)?;

    let handler = client.get(guild.id).context(IM_NOT_IN_A_VOICE_CHANNEL)?;

    let mut handler = handler.lock().await;

    let input = songbird::ytdl(query).await?;
    let title = input.metadata.title.clone().unwrap_or_default();

    handler.enqueue_source(input);
    handler.set_bitrate(Bitrate::Max);

    ctx.send(|m| {
        m.ephemeral(true)
            .content(format!(":ok_hand: Adicionado `{title}` a fila."))
    })
    .await?;
    Ok(())
}
