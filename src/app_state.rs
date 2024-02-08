use std::sync::Arc;

use crate::config::Config;
use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
    config: Arc<Config>,
    db: Pool<Postgres>,
}

impl AppState {
    pub async fn try_new() -> Result<Self> {
        let config = Arc::new(Config::new());
        let db = PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.database_url)
            .await?;
        Ok(Self { config, db })
    }
    pub fn config(&self) -> &Config {
        &self.config
    }
    pub fn db(&self) -> &Pool<Postgres> {
        &self.db
    }
}
