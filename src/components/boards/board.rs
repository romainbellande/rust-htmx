use crate::prisma::board::Data as BoardData;
use crate::prisma::column::Data as Column;
use crate::prisma::task::Data as Task;
use leptos::*;

#[component]
fn BoardHeader(columns: Vec<Column>) -> impl IntoView {
    view! {
        <div>
            { columns.iter().map(|column| view! {
                <div>{ column.name.clone() }</div>
            }).collect_view() }
        </div>
    }
}

#[component]
fn BoardTask(task: Task) -> impl IntoView {
    view! {
        <div>
            { task.name.clone() }
        </div>
    }
}

#[component]
fn BoardColumn(column: Column) -> impl IntoView {
    view! {
        <div>
            {
                column.tasks().cloned().unwrap_or_default().iter().map(|task| view! {
                    <BoardTask task=task.clone() />
                }).collect_view()
            }
        </div>
    }
}

#[component]
pub fn Board(board: BoardData) -> impl IntoView {
    let columns = board.columns.unwrap_or_default();
    view! {
        <div>
            <BoardHeader columns=columns.clone() />
            <div>
                { columns.iter().map(|column| view! {
                    <BoardColumn column=column.clone() />
                }).collect_view() }
            </div>
        </div>
    }
}
