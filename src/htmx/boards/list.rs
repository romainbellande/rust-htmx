use crate::app_state::AppState;
use crate::components::boards::List;
use crate::utils::renderer;
use axum::extract::State;
use axum::response::IntoResponse;
use leptos::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Board {
    pub name: String,
}

pub async fn htmx(State(state): State<AppState>) -> impl IntoResponse {
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
