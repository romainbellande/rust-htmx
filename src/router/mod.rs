use crate::app_state::AppState;
use crate::oidc::{error::MiddlewareError, EmptyAdditionalClaims, OidcAuthLayer, OidcLoginLayer};
use axum::{error_handling::HandleErrorLayer, http::Uri, response::IntoResponse, Router};
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_sessions::{cookie::SameSite, MemoryStore, SessionManagerLayer};

mod htmx;
mod views;

pub async fn app_router(state: AppState) -> Router {
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
                state.config.base_url.parse::<Uri>().unwrap(),
                state.config.auth0.issuer.clone(),
                state.config.auth0.client_id.clone(),
                Some(state.config.auth0.client_secret.to_owned()),
                scopes,
            )
            .await
            .unwrap(),
        );

    Router::new()
        .nest("/", views::router())
        .nest("/htmx", htmx::router())
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(oidc_login_service)
        .layer(oidc_auth_service)
        .layer(session_service)
        .with_state(state)
}
