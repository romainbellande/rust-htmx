use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <nav class="top-0 left-0 border-b border-slate-400 h-16 fixed w-screen flex justify-between">
            <div>"Cortex"</div>
            <div>
                <a href="/projects">"Project"</a>
            </div>
            <div>
                <div>"Search"</div>
                <div>"User"</div>
            </div>
        </nav>
    }
}
