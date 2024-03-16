use leptos::*;

#[component]
pub fn Dropdown(children: Children) -> impl IntoView {
    view! {
        <details class="dropdown">
            <summary class="m-1 btn">{children()}</summary>
            <ul class="p-2 shadow menu dropdown-content z-[1] bg-base-100 rounded-box w-52">
                <li><a>Item 1</a></li>
                <li><a>Item 2</a></li>
            </ul>
        </details>
    }
}
