use leptonic::atoms::{
    button::Button,
    checkbox::{CheckboxButton, CheckboxField},
};
use leptos::prelude::*;

#[component]
pub fn ButtonConceptDemo() -> impl IntoView {
    let saves = RwSignal::new(0u32);
    let disabled = RwSignal::new(false);

    view! {
        <Button classes="demo-atom-button" is_disabled=disabled on_press=move |_| saves.update(|n| *n += 1)>
            "Save draft"
        </Button>
        <p class="demo-status">
            {move || match saves.get() {
                0 => "Not saved yet.".to_owned(),
                1 => "Saved 1 time.".to_owned(),
                n => format!("Saved {n} times."),
            }}
        </p>
        <div class="demo-controls">
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>
    }
}
