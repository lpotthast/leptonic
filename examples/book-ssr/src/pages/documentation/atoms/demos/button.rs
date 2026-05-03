use leptonic::atoms::prelude as atoms;
use leptos::prelude::*;
use leptos_use::use_window;

#[component]
pub fn ButtonDemo() -> impl IntoView {
    view! {
        <atoms::Button on_press=move |_| {
            if let Some(window) = use_window().as_ref() {
                let _ = window.alert_with_message("Pressed!");
            }
        }>
            "Press me"
        </atoms::Button>
    }
}
