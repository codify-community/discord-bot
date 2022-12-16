use crate::primitives::Context;
use anyhow::Result;

#[poise::command(prefix_command, slash_command, subcommands("permita_me"))]
#[allow(clippy::unused_async)]
pub async fn web(_ctx: Context<'_>) -> Result<()> {
    Ok(())
}

/// 「FERRAMENTAS」Manda um link de pesquisa do `permita.me`
#[poise::command(prefix_command, slash_command)]
pub async fn permita_me(
    ctx: Context<'_>,
    #[description = "O quê pesquisar"] query: String,
) -> Result<()> {
    ctx.defer_or_broadcast().await?;

    ctx.send(|m| m.content(format!("https://permita.me/?q={}", query.replace(' ', "+"))))
        .await?;

    Ok(())
}
