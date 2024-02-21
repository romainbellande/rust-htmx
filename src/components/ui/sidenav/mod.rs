mod sidenav_link;
use leptos::*;
use sidenav_link::SidenavLink;

#[component]
pub fn Sidenav() -> impl IntoView {
    view! {
        <nav class="fixed left-0 w-48 min-h-screen max-h-screen z-50 p-5 shadow">
            <ul class="flex flex-col">
                <SidenavLink href="/">"kanban"</SidenavLink>
                <SidenavLink href="/backlog">"backlog"</SidenavLink>
                <SidenavLink href="/boards">"boards"</SidenavLink>
            </ul>
        </nav>
    }
}
