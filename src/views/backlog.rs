use askama::Template;
use axum_htmx::HxRequest;

#[derive(Template)] // This will generate the code...
#[template(path = "backlog.html")]
pub struct BacklogTemplate {
    use_layout: bool
}

pub async fn page(HxRequest(hx_request): HxRequest) -> BacklogTemplate {
    BacklogTemplate {
        use_layout: !hx_request
    }
}
