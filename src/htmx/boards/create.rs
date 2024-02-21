use crate::app_state::AppState;
use crate::components::boards::List;
use crate::prisma::column;
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
    let created_board = state
        .db
        .board()
        .create(board.name, vec![])
        .exec()
        .await
        .expect("Failed to create board");

    let columns: Vec<(String, String, Vec<column::SetParam>)> = vec!["To Do", "Doing", "Done"]
        .into_iter()
        .map(|name| (name.to_string(), created_board.id.clone(), vec![]))
        .collect();

    state
        .db
        .column()
        .create_many(columns)
        .exec()
        .await
        .expect("Failed to create columns");

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
