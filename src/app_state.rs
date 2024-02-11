use std::sync::Arc;

use crate::{config::Config, prisma::PrismaClient};
use anyhow::Result;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: Arc<PrismaClient>,
}

impl AppState {
    pub fn new(db: PrismaClient) -> Self {
        let config = Arc::new(Config::new());
        let db = Arc::new(db);

        Self { config, db }
    }

    pub async fn setup(&self) -> Result<Self> {
        self.db._db_push().accept_data_loss().force_reset().await?;
        Ok(self.clone())
    }
}
