use anyhow::Context;
use poise::serenity_prelude::validate_token;
use std::{env::var, path::Path};

pub fn env() -> anyhow::Result<()> {
    let evar = |e| var(e).context(format!("Can't find ${e}"));
    let try_id = |var| {
        evar(var).and_then(|x| {
            x.parse::<u64>()
                .context(format!("Failed to parse ${var} as unsigned integer (u64)"))
        })
    };

    validate_token(evar("DISCORD_TOKEN")?)?;

    try_id("REGISTRO_ID")?;
    try_id("LOG_CHANNEL_ID")?;

    println!("{}", evar("DATABASE_LOCATION")?);

    if !evar("DATABASE_LOCATION").map(|x| Path::new(&(x + "/")).exists())? {
        anyhow::bail!("Invalid database location");
    }

    Ok(())
}
