use crate::app_state::AppState;
use crate::components::boards::Board;
use crate::prisma;
use crate::utils::renderer;
use axum::extract::Path;
use axum::extract::State;
use axum::response::IntoResponse;
use leptos::*;

pub async fn htmx(
    State(state): State<AppState>,
    Path(board_id): Path<String>,
) -> impl IntoResponse {
    let board = state
        .db
        .board()
        .find_unique(prisma::board::id::equals(board_id))
        .with(prisma::board::columns::fetch(vec![]).with(prisma::column::tasks::fetch(vec![])))
        .exec()
        .await
        .expect("Failed to fetch board");

    let html = match board {
        Some(board) => view! {
            <>
                <Board board=board />
            </>
        },
        None => view! {
            <>
                <p>"board not found"</p>
            </>
        },
    };

    renderer(move || html)
}
