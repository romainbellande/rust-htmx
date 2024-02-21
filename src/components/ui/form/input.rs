use leptos::*;

#[component]
pub fn Input(
    #[prop(into, default = "text".to_string())] ty: String,
    #[prop(into)] name: String,
) -> impl IntoView {
    view! {
        <label class="form-control w-full max-w-xs">
            <div class="label">
                <span class="label-text">{ name.clone() }</span>
            </div>
            <input type=ty name=name class="input input-bordered w-full max-w-xs" />
        </label>
    }
}
