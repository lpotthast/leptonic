use leptonic::atoms::prelude::*;
use leptos::prelude::*;

#[component]
pub fn FocusRingDemo() -> impl IntoView {
    view! {
        <FocusRing>
            <button class="demo-btn">"Focus me with Tab"</button>
        </FocusRing>
    }
}
