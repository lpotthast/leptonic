use leptonic::{atoms::checkbox::{CheckboxButton, CheckboxField}, hooks::*};
use leptos::prelude::*;

fn modality_name(modality: Modality) -> &'static str {
    match modality {
        Modality::Unknown => "Unknown",
        Modality::Pointer => "Pointer",
        Modality::Keyboard => "Keyboard",
        Modality::Virtual => "Virtual",
    }
}

#[component]
pub fn FocusVisibleDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);

    let UseFocusVisibleReturn {
        focus_should_be_visible,
        modality,
    } = use_focus_visible(UseFocusVisibleInput {
        is_disabled: disabled.into(),
        ..Default::default()
    });

    view! {
        // The hook sets no attribute: the demo exposes its state, and the CSS outlines the focused button
        // only while `data-focus-visible` is present.
        <button
            type="button"
            class="demo-focus-modality"
            data-focus-visible=move || focus_should_be_visible.get().then_some("true")
        >
            "Click me, then tab away and back"
        </button>

        <p class="demo-status">
            "Modality: " {move || modality_name(modality.get())} ". Focus should be visible: "
            {move || if focus_should_be_visible.get() { "yes" } else { "no" }} "."
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
