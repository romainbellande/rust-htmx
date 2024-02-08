use askama::Template;
use axum_htmx::HxRequest;

#[derive(Template)] // This will generate the code...
#[template(path = "kanban.html")]
pub struct KanbanTemplate {
    use_layout: bool
}

pub async fn page(HxRequest(hx_request): HxRequest) -> KanbanTemplate {
    KanbanTemplate {
        use_layout: !hx_request
    }
}
