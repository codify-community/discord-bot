#![warn(clippy::perf, clippy::pedantic)]
#![allow(
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]

use crate::{
    commands::{
        information::status::status,
        music::musica,
        staff::{ban::ban, kick::kick, servidor::servidor, unban::unban},
        utils::{nppp::nppp, userinfo::userinfo, web::web},
    },
    primitives::State,
    utils::validations,
};
use anyhow::{Context as _, Result};
use dotenvy::dotenv;
use poise::{
    builtins::register_in_guild,
    serenity_prelude::{CacheHttp, Context, GatewayIntents, GuildId},
    Framework, FrameworkOptions, PrefixFrameworkOptions,
};

#[cfg(debug_assertions)]
use poise::Prefix;

use songbird::SerenityInit;
use tracing_subscriber::EnvFilter;

use crate::primitives::Database;
use handlers::on_error::on_error;
use std::{env, time::Instant};
use sysinfo::{System, SystemExt};
use tokio::sync::RwLock;
use tracing::log::info;

mod commands;
mod common;
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

    let commands = vec![
        status(),
        servidor(),
        userinfo(),
        ban(),
        unban(),
        kick(),
        musica(),
        web(),
        nppp(),
    ];

    #[cfg(debug_assertions)]
    let prefixes = vec![Prefix::Literal(">>")];

    #[cfg(not(debug_assertions))]
    let prefixes = vec![];

    let framework = Framework::builder()
        .token(env::var("DISCORD_TOKEN")?)
        .intents(GatewayIntents::all())
        .options(FrameworkOptions {
            commands,
            prefix_options: PrefixFrameworkOptions {
                prefix: Some(String::from(".")),
                additional_prefixes: prefixes,
                ..Default::default()
            },
            on_error: |e| Box::pin(on_error(e)),
            event_handler: |ctx, event, _fw, state| {
                Box::pin(event_handler::handle_event(ctx, event, state))
            },
            ..Default::default()
        })
        .setup(move |ctx, _, f| Box::pin(setup(ctx, f, guild_id)))
        .client_settings(SerenityInit::register_songbird);

    framework.run().await?;

    Ok(())
}

async fn setup(ctx: &Context, f: &Framework<State, anyhow::Error>, guild_id: u64) -> Result<State> {
    register_in_guild(&ctx.http(), &f.options().commands, GuildId(guild_id)).await?;

    Ok(State {
        guild_id,
        database: Database::init_from_directory(&env::var("DATABASE_LOCATION")?).await?,
        uptime: Instant::now(),
        system: RwLock::new(System::new()),
    })
}
