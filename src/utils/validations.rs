use anyhow::Context;
use std::{env::var, path::Path};

pub fn env() -> anyhow::Result<()> {
    find_var("DISCORD_TOKEN")?;

    try_id("REGISTRO_ID")?;
    try_id("LOG_CHANNEL_ID")?;

    if !find_var("DATABASE_LOCATION").map(|x| Path::new(&(x + "/")).exists())? {
        anyhow::bail!("Invalid database location");
    }

    Ok(())
}

#[inline]
fn find_var(env: &str) -> anyhow::Result<String> {
    var(env).context(format!("Can't find ${env}"))
}

#[inline]
fn try_id(var: &str) -> anyhow::Result<u64> {
    find_var(var).and_then(|n| {
        n.parse::<u64>()
            .context(format!("Failed to parse ${var} as unsigned integer (u64)"))
    })
}
