use std::time::Instant;

use crate::{
    primitives::Context,
    utils::{process::me, time::relative_since},
};
use anyhow::{Context as _, Result};
use poise::serenity_prelude::{Colour, ShardId};
use sysinfo::SystemExt;

#[cfg(debug_assertions)]
pub const BUILT_AS: &str = "Debug";
#[cfg(not(debug_assertions))]
pub const BUILT_AS: &str = "Release (Production)";

/// 「FERRAMENTAS」 Veja minhas informações
#[poise::command(prefix_command, slash_command)]
pub async fn status(ctx: Context<'_>) -> Result<()> {
    let (cpu_usage, memory_usage, subprocesses) =
        me(&mut *ctx.data().system.write().await).unwrap();

    let system = ctx.data().system.read().await;

    let shard_manager = ctx.framework().shard_manager();

    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;

    let runner = runners
        .get(&ShardId(ctx.serenity_context().shard_id))
        .context("No shard found")?;
    let time = Instant::now();
    let handle = ctx.say(":stopwatch:").await?;

    let description = format!(
        r#"
    💻 Versão: `{}`
    💻 Uptime: {}
    💻 Ambiente: `{BUILT_AS}`
    💻 Sistema: `{} v{}`
    💻 Uso de CPU: `{:.2}%`
    💻 Uso de memoria: `{} MiB`
    💻 Uso de memoria por subprocessos: `{} MiB`
    🦋 Ping da API: `{:.0?}`
    🔷 Latência do WebSocket: `{:.0?}`
       "#,
        env!("CARGO_PKG_VERSION"),
        relative_since(ctx.data().uptime.elapsed().as_secs()),
        system.name().unwrap_or_default(),
        system.kernel_version().unwrap_or_default(),
        cpu_usage,
        memory_usage / (1024 * 1024),
        subprocesses / 1024 / 1024,
        time.elapsed(),
        runner.latency.unwrap_or_default(),
    )
    .trim_start()
    .to_string();

    handle
        .edit(ctx, |m| {
            m.content("").embed(|e| {
                e.title("Minhas informações")
                    .colour(Colour::BLURPLE)
                    .description(description)
            })
        })
        .await?;
    Ok(())
}
