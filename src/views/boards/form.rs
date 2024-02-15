use crate::components::{form::Input, Button};
use leptos::*;

#[component]
pub fn Form() -> impl IntoView {
    view! {
        <form hx-post="boards/create" hx-target="#board-list" class="flex flex-col max-w-md space-y-4">
            <Input name="name" />
            <div class="flex justify-end">
                <Button>"Create"</Button>
            </div>
        </form>
    }
}
