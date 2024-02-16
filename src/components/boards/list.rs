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
