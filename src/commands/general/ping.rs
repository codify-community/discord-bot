use std::time::Instant;

use anyhow::Result;

use crate::primitives::Context;

/// 「GERAL」Veja meu ping
#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<()> {
    let time = Instant::now();
    let handle = ctx.say(":stopwatch: um momento...").await?;

    handle
        .edit(ctx, |m| {
            m.content(format!(
                ":butterfly: Atualmente estou com `{:.2?}` de delay com a API do discord.",
                time.elapsed()
            ))
        })
        .await?;
    Ok(())
}
