use std::sync::Arc;

use crate::{config::Config, prisma::PrismaClient};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
}

impl AppState {
    pub fn new() -> Self {
        let config = Arc::new(Config::new());
        Self { config }
    }
}
