mod sidenav_link;
use leptos::*;
use sidenav_link::SidenavLink;

#[component]
pub fn Sidenav() -> impl IntoView {
    view! {
        <nav class="fixed top-0 left-0 w-48 min-h-screen max-h-screen bg-zinc-400 z-50 p-5 shadow">
            <ul class="flex flex-col">
                <SidenavLink href="/">"kanban"</SidenavLink>
                <SidenavLink href="/backlog">"backlog"</SidenavLink>
                <SidenavLink href="/boards">"boards"</SidenavLink>
            </ul>
        </nav>
    }
}
