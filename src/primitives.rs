use anyhow::Context as _;
use poise::serenity_prelude::MessageId;
use rustbreak::{deser::Ron, FileDatabase};
use serde::{Deserialize, Serialize};
use std::{path::Path, time::Instant};
use sysinfo::System;
use tokio::{fs, sync::RwLock};
use tracing::{debug, info};

#[derive(Clone, Deserialize, Serialize)]
pub struct AutoRole {
    pub category: String,
    pub id: MessageId,
}

pub struct Database {
    pub auto_rules_messages: FileDatabase<Vec<AutoRole>, Ron>,
}

impl Database {
    #[tracing::instrument]
    pub async fn init_from_directory(directory: &str) -> anyhow::Result<Self> {
        if !Path::new(directory).exists() {
            debug!("Target directory doesn't exists. So creating it.");
            fs::create_dir_all(directory)
                .await
                .context("Failed to create directory for database")?;
        }

        info!("Loading autoRulesMessages.ron");
        let auto_rules_messages =
            FileDatabase::load_from_path_or_default(format!("{directory}/autoRulesMessages.ron"))
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
}

pub type Context<'a> = poise::Context<'a, State, anyhow::Error>;
