use axum::{
    error_handling::HandleErrorLayer, http::Uri, response::IntoResponse, routing::get, Router,
};
use oidc::{error::MiddlewareError, EmptyAdditionalClaims, OidcAuthLayer, OidcLoginLayer};
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_sessions::{cookie::SameSite, MemoryStore, SessionManagerLayer};

mod app_state;
mod components;
mod config;
mod icons;
mod oidc;
mod views;

use anyhow::Result;
use app_state::AppState;
use config::CONFIG;
use views::{backlog, kanban};

pub async fn start() -> Result<()> {
    let session_store = MemoryStore::default();
    let session_service = SessionManagerLayer::new(session_store).with_same_site(SameSite::Lax);
    let scopes = "profile email"
        .split(' ')
        .map(|x| x.to_owned())
        .collect::<Vec<_>>();

    let oidc_login_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|e: MiddlewareError| async {
            e.into_response()
        }))
        .layer(OidcLoginLayer::<EmptyAdditionalClaims>::new());

    let oidc_auth_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|e: MiddlewareError| async {
            e.into_response()
        }))
        .layer(
            OidcAuthLayer::<EmptyAdditionalClaims>::discover_client(
                Uri::from_static(CONFIG.base_url.as_str()),
                CONFIG.auth0.issuer.clone(),
                CONFIG.auth0.client_id.clone(),
                Some(CONFIG.auth0.client_secret.to_owned()),
                scopes,
            )
            .await
            .unwrap(),
        );

    let app_state = AppState::try_new().await?;

    // build our application with a single route
    let app = Router::new()
        .route("/", get(kanban::page))
        .route("/backlog", get(backlog::page))
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(oidc_login_service)
        .layer(oidc_auth_service)
        .layer(session_service)
        .with_state(app_state);

    // run it with hyper on localhost:3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
