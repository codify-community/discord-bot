use crate::{
    primitives::Context,
    utils::{process::current_total_memory_usage, time::HumanDate},
};
use anyhow::Result;
use humansize::{format_size, DECIMAL};
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
    💻 Uptime: `{}` 
    💻 Ambiente: `{BUILT_AS}` 
    💻 Sistema: `{} v{}` 
    💻 Uso de memoria: `{}` 
    💻 Uso de memoria por subprocessos: `{}`
    "#,
        env!("CARGO_PKG_VERSION"),
        HumanDate(cx.data().uptime.elapsed(),),
        system.name().unwrap_or_default(),
        system.kernel_version().unwrap_or_default(),
        format_size(used, DECIMAL),
        format_size(used_by_children, DECIMAL),
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
