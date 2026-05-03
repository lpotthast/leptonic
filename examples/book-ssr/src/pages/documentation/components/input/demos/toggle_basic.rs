use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn ToggleBasicDemo() -> impl IntoView {
    let (state, set_state) = signal(false);

    view! {
        <Toggle state=state set_state=set_state/>
    }
}
