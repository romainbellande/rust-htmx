use askama::Template;
use axum::extract::State;
use axum_htmx::HxRequest;

use crate::app_state::AppState;
use crate::prisma::board::Data as Board;
use crate::prisma::PrismaClient;

#[derive(Template)] // This will generate the code...
#[template(path = "views/boards.html")]
pub struct BoardsTemplate {
    use_layout: bool,
    boards: Vec<Board>,
}

pub async fn page(
    HxRequest(hx_request): HxRequest,
    State(db): State<PrismaClient>,
) -> BoardsTemplate {
    // let boards =

    BoardsTemplate {
        use_layout: !hx_request,
        boards: vec![],
    }
}
