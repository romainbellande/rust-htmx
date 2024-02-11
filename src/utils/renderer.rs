use axum::response::{Html, IntoResponse};
use leptos::ssr::render_to_string;
use leptos::*;

pub fn renderer<F, N>(f: F) -> impl IntoResponse
where
    F: FnOnce() -> N + 'static,
    N: IntoView,
{
    let html = render_to_string(f);
    Html(html.to_string())
}
