use crate::components::ui::{form::Input, Button};
use leptos::*;

#[component]
pub fn Form() -> impl IntoView {
    view! {
        <form
            hx-post="htmx/boards/create"
            hx-target="#board-list"
            hx-swap="outerHTML"
            class="flex flex-col max-w-md space-y-4">
            <Input name="name" />
            <div class="flex justify-end">
                <Button ty="submit">"Create"</Button>
            </div>
        </form>
    }
}
