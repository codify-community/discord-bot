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
    serenity_prelude::{CacheHttp, GatewayIntents, GuildId, Role},
    Event, Framework, FrameworkOptions, Prefix, PrefixFrameworkOptions,
};

use crate::primitives::{Database, REGISTRO_ROLE_MARKER};
use poise::serenity_prelude::Interaction;
use std::{env, fs, path::Path, process, time::Instant};
use sysinfo::{System, SystemExt};
use tokio::sync::RwLock;
use tracing::log::info;
use tracing_subscriber::EnvFilter;

mod commands;
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
                Box::pin(async move {
                    let guild_id = state.guild_id;

                    if let Event::InteractionCreate {
                        interaction: Interaction::MessageComponent(component),
                    } = event
                    {
                        if let Some(embed) = component.message.embeds.first() {
                            if let Some(ref description) = embed.description {
                                let roles_id: Vec<Role> = description
                                    .lines()
                                    .filter(|l| l.starts_with(REGISTRO_ROLE_MARKER))
                                    .map(|l| l.replace(REGISTRO_ROLE_MARKER, "").trim().to_string())
                                    .map(|l| {
                                        l.chars().filter(|c| c.is_numeric()).collect::<String>()
                                    })
                                    .flat_map(|s| s.parse::<u64>())
                                    .filter_map(|rid| cx.cache.role(guild_id, rid))
                                    .collect();

                                component
                                    .create_interaction_response(cx.http(), |m| {
                                        m.interaction_response_data(|d| {
                                            d.ephemeral(true).components(|c| {
                                                c.create_action_row(|row| {
                                                    row.create_select_menu(|sm| {
                                                        sm.custom_id("role-resolve")
                                                            .placeholder(
                                                                "Por favor selecione alguma opção",
                                                            )
                                                            .min_values(1)
                                                            .max_values(roles_id.len() as _)
                                                            .options(|opts| {
                                                                for role in roles_id {
                                                                    opts.create_option(|o| {
                                                                        o.label(role.name)
                                                                            .value(role.id)
                                                                    });
                                                                }

                                                                opts
                                                            })
                                                    })
                                                })
                                            })
                                        })
                                    })
                                    .await?;
                            }
                        }
                    }
                    Ok(())
                })
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
