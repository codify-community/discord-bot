use std::time::Instant;

use anyhow::Result;

use crate::primitives::Context;

///〔🦁 Geral〕 Veja meu ping
#[poise::command(prefix_command, slash_command)]
pub async fn ping(cx: Context<'_>) -> Result<()> {
    let time = Instant::now();
    let handle = cx.say(":stopwatch: um momento...").await?;

    handle
        .edit(cx, |m| {
            m.content(format!(
                ":butterfly: Atualmente estou com `{:.2?}` de delay com a API do discord.",
                time.elapsed()
            ))
        })
        .await?;
    Ok(())
}
