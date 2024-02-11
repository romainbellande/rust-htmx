mod app_state;
mod components;
mod config;
mod icons;
mod oidc;
mod prisma;
mod router;
mod views;

use std::sync::Arc;

use anyhow::Result;
use config::Config;
use prisma::PrismaClient;
use router::app_router;

pub async fn start() -> Result<()> {
    let prisma_client = Arc::new(
        PrismaClient::_builder()
            .build()
            .await
            .expect("Failed to connect to Prisma"),
    );

    let config = Arc::new(Config::new());

    // build our application with a single route
    let router = app_router(config, prisma_client).await;

    // run it with hyper on localhost:3000
    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    println!("Listening on {}", address);

    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
}
