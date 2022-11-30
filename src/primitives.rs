use anyhow::Context as _;
use rustbreak::{deser::Ron, FileDatabase};
use serde::{Deserialize, Serialize};
use std::{path::Path, time::Instant};
use sysinfo::System;
use tokio::{fs, sync::RwLock};
use tracing::{debug, info};
use typemap_rev::TypeMap;

pub const REGISTRO_ROLE_MARKER: &str = " **·**";

#[derive(Clone, Deserialize, Serialize)]
pub struct AutoRole {
    pub category: String,
    pub id: u64,
    pub channel_id: u64,
}

pub struct Database {
    pub auto_rules_messages: FileDatabase<Vec<AutoRole>, Ron>,
}

impl Database {
    pub async fn init_from_directory(directory: &str) -> anyhow::Result<Self> {
        if !Path::new(directory).exists() {
            debug!("Target directory doesn't exists. So creating it.");
            fs::create_dir_all(directory)
                .await
                .context("Failed to create directory for database")?;
        }

        info!("Loading role_selection.ron");
        let auto_rules_messages =
            FileDatabase::load_from_path_or_default(format!("{directory}/role_selection.ron"))
                .context("Failed to open user store file")?;

        Ok(Self {
            auto_rules_messages,
        })
    }
}

pub struct State {
    pub uptime: Instant,
    pub system: RwLock<System>,
    pub database: Database,
    pub jobs: TypeMap,
    pub guild_id: u64,
}

pub type Context<'a> = poise::Context<'a, State, anyhow::Error>;
