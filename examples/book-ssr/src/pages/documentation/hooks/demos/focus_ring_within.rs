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
        <div {..focus_ring_within.props.into_attrs()} class=Classes::from("demo-focus-group")>
            <input type="text" placeholder="Tab here\u{2026}" class=Classes::from(["demo-focus-item", "demo-focus-ring"])/>
            <button class=Classes::from(["demo-focus-item", "demo-focus-ring"])>"Or here"</button>
        </div>

        <p class=Classes::from("demo-mt-1")>
            "Focus within: " <strong>{ move || focus_ring_within.is_focused.get().to_string() }</strong>
            " | Focus ring (within) visible: " <strong>{ move || focus_ring_within.is_focus_visible.get().to_string() }</strong>
        </p>
    }
}
