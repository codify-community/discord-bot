use crate::{
    commands::{
        general::ping::ping, information::status::status, staff::servidor::servidor,
        utils::userinfo::userinfo,
    },
    primitives::State,
};
use anyhow::{Context, Result};
use dotenvy::dotenv;
use poise::{
    builtins::register_in_guild,
    serenity_prelude::{CacheHttp, GatewayIntents, GuildId},
    Framework, FrameworkOptions, Prefix, PrefixFrameworkOptions,
};

use crate::primitives::Database;
use std::{env, fs, path::Path, process, time::Instant};
use sysinfo::{System, SystemExt};
use tokio::sync::RwLock;
use tracing::log::info;
use tracing_subscriber::EnvFilter;

mod commands;
mod event_handler;
mod handlers;
mod primitives;
mod utils;

fn copy_dotenv() -> Result<()> {
    if !Path::new(".env").exists() {
        info!("Uh, I can't find `.env` file. So i'm copying `.env.example` to `.env`");
        fs::copy(".env.example", ".env").context("Failed to copy `.env` file")?;

        info!("Configure the `.env` then re-run the bot. Please.");
        process::exit(0)
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive("codify=debug".parse().unwrap()),
        )
        .init();

    copy_dotenv()?;
    dotenv().context("Failed to load `.env` file")?;

    info!("Starting bot...");
    let guild_id: u64 = env::var("CODIFY_GUILD_ID")
        .context("Failed to read $DISCORD_GUILD_ID")?
        .parse()
        .context("Failed to parse $DISCORD_GUILD_ID as a valid integer!")?;

    let commands = vec![ping(), status(), servidor(), userinfo()];

    let framework = Framework::builder()
        .token(env::var("DISCORD_TOKEN").context("Failed to read $DISCORD_TOKEN")?)
        .intents(GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT)
        .options(FrameworkOptions {
            commands,
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("$".into()),
                additional_prefixes: vec![Prefix::Literal(">>"), Prefix::Literal("$ ")],
                ..Default::default()
            },
            event_handler: |cx, event, _fw, state| {
                Box::pin(event_handler::handle_event(cx, event, state))
            },
            ..Default::default()
        })
        .setup(move |cx, _, f| {
            Box::pin(async move {
                register_in_guild(&cx.http(), &f.options().commands, GuildId(guild_id)).await?;

                Ok(State {
                    guild_id,
                    database: Database::init_from_directory(
                        &env::var("DATABASE_LOCATION")
                            .context("Failed to read $DATABASE_LOCATION")?,
                    )
                    .await?,
                    uptime: Instant::now(),
                    system: RwLock::new(System::new()),
                })
            })
        });

    framework.run().await?;

    Ok(())
}
