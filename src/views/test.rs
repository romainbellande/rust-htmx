use crate::utils::renderer;
use axum::response::IntoResponse;
use leptos::*;

pub async fn MyTestPage() -> impl IntoResponse {
    renderer(move || {
        view! {
            <div>HelloWorld</div>
        }
    })
}
