use leptonic::hooks::*;
use leptos::prelude::*;
use leptos_classes::Classes;

#[component]
pub fn FocusRingWithinDemo() -> impl IntoView {
    let focus_ring_within = use_focus_ring(UseFocusRingInput {
        within: true,
        ..Default::default()
    });

    view! {
        <div
            {..focus_ring_within.props.into_attrs()}
            style="padding: 1em; border-radius: 8px; border: 2px solid #ccc; display: flex; gap: 0.5em; align-items: center; transition: all 0.2s;"
        >
            <input
                type="text"
                placeholder="Tab here..."
                class=Classes::from("demo-input")
            />
            <button class=Classes::from("demo-btn")>
                "Or here"
            </button>
        </div>

        <p class=Classes::from("demo-mt-1")>
            "Focus within: " <strong>{ move || focus_ring_within.is_focused.get().to_string() }</strong>
            " | Focus ring (within) visible: " <strong>{ move || focus_ring_within.is_focus_visible.get().to_string() }</strong>
        </p>
    }
}
