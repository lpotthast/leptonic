use leptonic::hooks::focus::{
    Modality, UseFocusVisibleInput, UseFocusVisibleReturn, use_focus_visible,
    use_interaction_modality,
};
use leptos::prelude::*;

fn modality_name(modality: Option<Modality>) -> &'static str {
    match modality {
        None => "None yet",
        Some(Modality::Pointer) => "Pointer",
        Some(Modality::Keyboard) => "Keyboard",
        Some(Modality::Virtual) => "Virtual",
    }
}

#[component]
pub fn FocusVisibleDemo() -> impl IntoView {
    let UseFocusVisibleReturn {
        focus_should_be_visible,
    } = use_focus_visible(UseFocusVisibleInput::default());
    let modality = use_interaction_modality();

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
    }
}
