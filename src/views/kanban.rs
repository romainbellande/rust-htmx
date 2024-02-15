use crate::components::Page;
use crate::utils::renderer;
use axum::response::IntoResponse;
use axum_htmx::HxRequest;
use leptos::*;

pub async fn page(HxRequest(hx_request): HxRequest) -> impl IntoResponse {
    renderer(move || {
        view! {
            <Page use_layout=!hx_request>
                <div>HelloWorld</div>
            </Page>
        }
    })
}
