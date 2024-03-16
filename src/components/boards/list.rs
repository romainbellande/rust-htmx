use crate::prisma::board::Data as Board;
use leptos::*;

#[component]
fn EmptyList() -> impl IntoView {
    view! {
        <div class="text-center">
            "No data yet"
        </div>
    }
}

#[component]
fn BoardCell(#[prop()] board: Board) -> impl IntoView {
    view! {
        <tr>
            <td>{board.name.clone()}</td>
            <td>{board.created_at.clone().to_string()}</td>
            <tr>
                 "view"
            </tr>
        </tr>
    }
}

#[component]
pub fn List(#[prop()] boards: Vec<Board>) -> impl IntoView {
    let is_not_empty = !boards.is_empty();
    view! {
        <div id="board-list">
            <Show
                when=move || { is_not_empty }
                fallback=|| view! { <EmptyList /> }
            >
                <table>
                    <thead></thead>
                    <tbody>
                        {boards.iter().map(|board| {
                            view! {
                                <tr>
                                    <td>{board.name.clone()}</td>
                                </tr>
                            }
                        }).collect_view()}
                    </tbody>
                </table>
            </Show>
        </div>
    }
}
