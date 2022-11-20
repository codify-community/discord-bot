use crate::{
    primitives::Context,
    utils::{process::current_total_memory_usage, time::relative_time},
};
use anyhow::Result;
use poise::serenity_prelude::Colour;
use sysinfo::SystemExt;

#[cfg(debug_assertions)]
pub const BUILT_AS: &str = "Debug";
#[cfg(not(debug_assertions))]
pub const BUILT_AS: &str = "Release (Production)";

///〔🛠️ Depuração〕Veja minhas informações
#[poise::command(prefix_command, slash_command)]
pub async fn status(cx: Context<'_>) -> Result<()> {
    let (used, used_by_children) =
        current_total_memory_usage(&mut *cx.data().system.write().await).unwrap_or((0, 0));

    let system = cx.data().system.read().await;

    let description = format!(
        r#"
    💻 Versão: `{}`
    💻 Uptime: {}
    💻 Ambiente: `{BUILT_AS}`
    💻 Sistema: `{} v{}`
    💻 Uso de memoria: `{} MiB`
    💻 Uso de memoria por subprocessos: `{:.1} MiB`
    "#,
        env!("CARGO_PKG_VERSION"),
        relative_time(cx.data().uptime.elapsed().as_secs()),
        system.name().unwrap_or_default(),
        system.kernel_version().unwrap_or_default(),
        used / (1024 * 1024),
        used_by_children as f64 / (1024.0 * 1024.0),
    )
    .trim_start()
    .to_string();

    cx.send(|m| {
        m.embed(|e| {
            e.title("Minhas informações")
                .colour(Colour::BLURPLE)
                .description(description)
        })
    })
    .await?;
    Ok(())
}
