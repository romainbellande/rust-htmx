use std::sync::Arc;

use askama::Template;
use axum::extract::State;
use axum_htmx::HxRequest;

use crate::app_state::AppState;
use crate::prisma::board::Data as Board;

#[derive(Template)] // This will generate the code...
#[template(path = "views/boards.html")]
pub struct BoardsTemplate {
    use_layout: bool,
    boards: Vec<Board>,
}

pub async fn page(
    HxRequest(hx_request): HxRequest,
    State(state): State<AppState>,
) -> BoardsTemplate {
    let boards = state.db.board().find_many(vec![]).exec().await.unwrap();

    BoardsTemplate {
        use_layout: !hx_request,
        boards,
    }
}
