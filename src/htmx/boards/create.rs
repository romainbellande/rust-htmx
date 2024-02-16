use crate::app_state::AppState;
use crate::components::boards::List;
use crate::utils::renderer;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Form;
use leptos::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Board {
    pub name: String,
}

pub async fn htmx(State(state): State<AppState>, Form(board): Form<Board>) -> impl IntoResponse {
    state
        .db
        .board()
        .create(board.name, vec![])
        .exec()
        .await
        .expect("Failed to create board");

    let boards = state
        .db
        .board()
        .find_many(vec![])
        .exec()
        .await
        .expect("Failed to fetch boards");
    renderer(move || {
        view! {
            <List boards=boards />
        }
    })
}
