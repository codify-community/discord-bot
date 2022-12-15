use crate::{jobs::browser::RequestKind, primitives::Context, utils::web as browser};
use anyhow::Result;
use poise::serenity_prelude::AttachmentType;

#[poise::command(prefix_command, slash_command, subcommands("google"))]
#[allow(clippy::unused_async)]
pub async fn web(_ctx: Context<'_>) -> Result<()> {
    Ok(())
}

/// 「FERRAMENTAS」Procura alguma coisa no google
#[poise::command(prefix_command, slash_command)]
pub async fn google(
    ctx: Context<'_>,
    #[description = "O quê pesquisar"] query: String,
) -> Result<()> {
    ctx.defer_or_broadcast().await?;
    let result = browser::request(&ctx, RequestKind::Google { query }).await?;

    ctx.send(|m| {
        m.attachment(AttachmentType::Bytes {
            data: result.into(),
            filename: "google.png".to_string(),
        })
    })
    .await?;

    Ok(())
}
