use leptonic::{atoms::prelude as atoms, components::prelude::Checkbox};
use leptos::prelude::*;

#[component]
pub fn ButtonDemo() -> impl IntoView {
    let presses = RwSignal::new(0u32);
    let disabled = RwSignal::new(false);

    view! {
        <div class="demo-flex-center-row">
            <atoms::Button classes="demo-btn" is_disabled=disabled on_press=move |_| presses.update(|p| *p += 1)>
                "Press me"
            </atoms::Button>
            <Checkbox state=disabled>"Disabled"</Checkbox>
        </div>
        <p>{move || format!("Pressed {} times.", presses.get())}</p>
    }
}
