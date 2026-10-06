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
        <button class=Classes::from(["demo-focus-item", "demo-focus-modality"]).add_reactive("focus-visible", focus_should_be_visible)>
            "Interact with me"
        </button>

        <p class=Classes::from("demo-mt-1")>
            "Focus should be visible: "
            <strong class=Classes::builder().with_toggle(focus_should_be_visible, "demo-state-active", "demo-state-inactive").build()>
                { move || focus_should_be_visible.get().to_string() }
            </strong>
        </p>

        <p>
            "Current modality: "
            <strong>{ move || modality_display.get() }</strong>
        </p>
    }
}
