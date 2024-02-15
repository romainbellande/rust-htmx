use crate::prisma::board::Data as Board;
use leptos::*;

#[component]
pub fn List(#[prop()] boards: Vec<Board>) -> impl IntoView {
    view! {
        <table id="board-list">
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
    }
}
