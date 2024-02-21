use super::Dropdown;
use crate::icons;
use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <nav class="px-8 top-0 left-0 border-b border-slate-200 h-16 fixed w-screen flex justify-between items-center">
            <div class="text-xl font-bold">"Cortex"</div>
            <div>
                <a href="/boards">"board"</a>
            </div>
            <div class="flex space-x-4">
                <div>"Search"</div>
                <Dropdown>
                    <icons::User />
                </Dropdown>
            </div>
        </nav>
    }
}
