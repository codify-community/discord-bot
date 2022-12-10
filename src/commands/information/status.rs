use crate::{
    primitives::Context,
    utils::{process::me, time::relative_since},
};
use anyhow::Result;
use poise::serenity_prelude::Colour;
use sysinfo::SystemExt;

#[cfg(debug_assertions)]
pub const BUILT_AS: &str = "Debug";
#[cfg(not(debug_assertions))]
pub const BUILT_AS: &str = "Release (Production)";

/// 「FERRAMENTAS」 Veja minhas informações
#[poise::command(prefix_command, slash_command)]
pub async fn status(ctx: Context<'_>) -> Result<()> {
    let (cpu_usage, memory_usage) = me(&mut *ctx.data().system.write().await).unwrap();

    let system = ctx.data().system.read().await;

    let description = format!(
        r#"
    💻 Versão: `{}`
    💻 Uptime: {}
    💻 Ambiente: `{BUILT_AS}`
    💻 Sistema: `{} v{}`
    💻 Uso de CPU: `{:.2}%`
    💻 Uso de memoria: `{} MiB`
       "#,
        env!("CARGO_PKG_VERSION"),
        relative_since(ctx.data().uptime.elapsed().as_secs()),
        system.name().unwrap_or_default(),
        system.kernel_version().unwrap_or_default(),
        cpu_usage,
        memory_usage / (1024 * 1024),
    )
    .trim_start()
    .to_string();

    ctx.send(|m| {
        m.embed(|e| {
            e.title("Minhas informações")
                .colour(Colour::BLURPLE)
                .description(description)
        })
    })
    .await?;
    Ok(())
}
