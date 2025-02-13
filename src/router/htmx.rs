use crate::app_state::AppState;
use crate::htmx;
use axum::{
    routing::{get, post},
    Router,
};

fn board_router() -> Router<AppState> {
    Router::new()
        .route("/", get(htmx::boards::list::htmx))
        .route("/create", post(htmx::boards::create::htmx))
}

pub fn router() -> Router<AppState> {
    Router::new().nest("/boards", board_router())
}
