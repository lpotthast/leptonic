use leptos::prelude::*;

pub mod anchor_link;
pub mod button;
pub mod focus;
pub mod hover;
pub mod r#move;
pub mod overlay;
pub mod press;

#[component]
pub fn PageHooks() -> impl IntoView {
    view! {
        <div>
            <h1>Hooks</h1>
            <p>
                Hooks are a way to extend the functionality of Leptos components.
                They are a way to add functionality to components without having to
                modify the component itself.
            </p>
        </div>
    }
}