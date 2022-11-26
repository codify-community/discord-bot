#![warn(clippy::perf, clippy::pedantic)]
#![allow(
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]

use crate::{
    commands::{
        admin::{ban::ban, unban::unban},
        general::ping::ping,
        information::status::status,
        staff::servidor::servidor,
        utils::userinfo::userinfo,
    },
    primitives::State,
    utils::validations,
};
use anyhow::{Context, Result};
use dotenvy::dotenv;
use poise::{
    builtins::register_in_guild,
    serenity_prelude::{CacheHttp, GatewayIntents, GuildId},
    Framework, FrameworkOptions, Prefix, PrefixFrameworkOptions,
};
use tracing_subscriber::EnvFilter;

use crate::primitives::Database;
use handlers::on_error::on_error;
use std::{env, time::Instant};
use sysinfo::{System, SystemExt};
use tokio::sync::RwLock;
use tracing::log::info;

mod commands;
mod event_handler;
mod handlers;
mod primitives;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive("codify=debug".parse().unwrap()),
        )
        .init();

    dotenv().context("Failed to load `.env` file")?;
    validations::env()?;

    info!("Starting bot...");
    let guild_id: u64 = env::var("GUILD_ID")?.parse()?;

    let commands = vec![ping(), status(), servidor(), userinfo(), ban(), unban()];

    let framework = Framework::builder()
        .token(env::var("DISCORD_TOKEN")?)
        .intents(GatewayIntents::all())
        .options(FrameworkOptions {
            commands,
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("$".into()),
                additional_prefixes: vec![Prefix::Literal(">>"), Prefix::Literal("$ ")],
                ..Default::default()
            },
            on_error: |e| Box::pin(on_error(e)),
            event_handler: |ctx, event, _fw, state| {
                Box::pin(event_handler::handle_event(ctx, event, state))
            },
            ..Default::default()
        })
        .setup(move |ctx, _, f| {
            Box::pin(async move {
                register_in_guild(&ctx.http(), &f.options().commands, GuildId(guild_id)).await?;

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
