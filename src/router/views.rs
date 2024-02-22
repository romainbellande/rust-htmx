use crate::app_state::AppState;
use crate::views::{backlog, boards, kanban};
use axum::{routing::get, Router};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(kanban::page))
        .route("/backlog", get(backlog::page))
        .route("/boards", get(boards::page))
        .route("/boards/:board_id", get(boards::board::page))
}
