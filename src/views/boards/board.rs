use crate::components::ui::Page;
use crate::utils::renderer;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum_htmx::HxRequest;
use leptos::*;

pub async fn page(
    HxRequest(hx_request): HxRequest,
    Path(board_id): Path<String>,
) -> impl IntoResponse {
    let board_link = format!("/boards/{}", board_id);

    renderer(move || {
        view! {
            <Page use_layout=!hx_request>
                <div id="board" class="h-full py-4 flex justify-center" hx-get=board_link hx-trigger="load">
                </div>
            </Page>
        }
    })
}
