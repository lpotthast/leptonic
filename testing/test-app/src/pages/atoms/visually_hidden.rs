use leptonic::atoms::prelude::VisuallyHidden;
use leptos::prelude::*;

/// `VisuallyHidden` (react-aria's `VisuallyHidden.test.tsx` setups): buttons A and C around a
/// hidden button B, once without `is_focusable` (`.test-vh-plain`) and once with it
/// (`.test-vh-focusable`). `#test-vh-toggle` makes the plain one focusable.
#[component]
pub fn PageAtomVisuallyHidden() -> impl IntoView {
    let focusable = RwSignal::new(false);
    view! {
        <div id="test-page-atom-visually-hidden">
            <button id="test-vh-plain-a">"Plain A"</button>
            <VisuallyHidden is_focusable=focusable classes="test-vh-plain">
                <button id="test-vh-plain-b">"Plain B"</button>
            </VisuallyHidden>
            <button id="test-vh-plain-c">"Plain C"</button>

            <button id="test-vh-focusable-a">"Focusable A"</button>
            <VisuallyHidden is_focusable=true classes="test-vh-focusable">
                <button id="test-vh-focusable-b">"Focusable B"</button>
            </VisuallyHidden>
            <button id="test-vh-focusable-c">"Focusable C"</button>

            <button id="test-vh-toggle" on:click=move |_| focusable.update(|f| *f = !*f)>
                "Toggle focusable"
            </button>
        </div>
    }
}
