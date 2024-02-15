use crate::app_state::AppState;
use crate::components::Page;
use crate::prisma;
use crate::utils::renderer;
use axum::extract::State;
use axum::response::IntoResponse;
use axum_htmx::HxRequest;
use leptos::*;
mod form;
mod list;

use form::Form;
use list::List;

pub async fn page(
    HxRequest(hx_request): HxRequest,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let boards = state
        .db
        .board()
        .find_many(vec![])
        .exec()
        .await
        .expect("Failed to fetch boards");

    renderer(move || {
        view! {
            <Page use_layout=!hx_request>
                <div id="boards" class="h-full py-4 flex justify-center">
                    <Form />
                    <List boards=boards />
                </div>
            </Page>
        }
    })
}
