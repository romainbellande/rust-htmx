use leptos::*;

#[component]
pub fn SidenavLink(children: Children, #[prop(into)] href: String) -> impl IntoView {
    view! {
        <li>
            <a href=href.clone() class="block cursor-pointer px-4 py-2 hover:bg-zinc-100" hx-get=href hx-push-url="true" hx-target="#main">{ children() }</a>
        </li>
    }
}
