use leptonic::atoms::prelude as atoms;
use leptos::prelude::*;

#[component]
pub fn ButtonDemo() -> impl IntoView {
    let presses = RwSignal::new(0u32);
    let disabled = RwSignal::new(false);

    view! {
        <atoms::Button classes="demo-atom-button" is_disabled=disabled on_press=move |_| presses.update(|n| *n += 1)>
            "Press me"
        </atoms::Button>
        <p class="demo-status">
            {move || match presses.get() {
                1 => "Pressed 1 time.".to_owned(),
                n => format!("Pressed {n} times."),
            }}
        </p>
        <div class="demo-controls">
            <atoms::Checkbox is_selected=disabled set_selected=disabled classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Disabled"
            </atoms::Checkbox>
        </div>
    }
}
