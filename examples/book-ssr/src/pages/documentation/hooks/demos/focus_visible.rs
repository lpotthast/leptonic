use leptonic::hooks::*;
use leptos::prelude::*;
use leptos_classes::Classes;

#[component]
pub fn FocusVisibleDemo() -> impl IntoView {
    let UseFocusVisibleReturn {
        focus_should_be_visible,
        modality,
    } = use_focus_visible(UseFocusVisibleInput::default());

    let modality_display = Memo::new(move |_| match modality.get() {
        Modality::Unknown => "Unknown",
        Modality::Pointer => "Pointer",
        Modality::Keyboard => "Keyboard",
        Modality::Virtual => "Virtual",
    });

    view! {
        <style>
            ".focus-visible-demo:focus { outline: 3px solid var(--brand-color, #e66956); outline-offset: 2px; }"
        </style>

        <button
            tabindex=0
            class="focus-visible-demo"
            style=move || format!(
                "padding: 1em 2em; font-size: 1em; border-radius: 8px; cursor: pointer; transition: all 0.2s; {}",
                if focus_should_be_visible.get() {
                    "border: 2px solid var(--brand-color); background: var(--brand-color-light, rgba(230, 105, 86, 0.1));"
                } else {
                    "border: 2px solid #ccc; background: white;"
                }
            )
        >
            "Interact with me"
        </button>

        <p class=Classes::from("demo-mt-1")>
            "Focus should be visible: "
            <strong class=Classes::builder().with_toggle(focus_should_be_visible.get(), "demo-state-active", "demo-state-inactive").build()>
                { move || focus_should_be_visible.get().to_string() }
            </strong>
        </p>

        <p>
            "Current modality: "
            <strong>{ move || modality_display.get() }</strong>
        </p>
    }
}
