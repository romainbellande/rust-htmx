mod app_state;
mod components;
mod config;
mod htmx;
mod oidc;
mod prisma;
mod router;
mod utils;
mod views;

use anyhow::Result;
use app_state::AppState;
use prisma::PrismaClient;
use router::app_router;

pub async fn start() -> Result<()> {
    let prisma_client = PrismaClient::_builder()
        .build()
        .await
        .expect("Failed to connect to Prisma");

    let state = AppState::new(prisma_client).setup().await?;
    let router = app_router(state).await;
    let address = "0.0.0.0:3000"; // TODO: move port into Config struct
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    println!("Listening on {}", address);

    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
}
