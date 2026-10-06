use leptonic::{
    atoms::{focus_ring::FocusRingContext, prelude::*},
    components::prelude::Checkbox,
};
use leptos::prelude::*;

#[component]
pub fn FocusRingDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);
    let focused = RwSignal::new(false);

    view! {
        // The CSS draws the ring on `[data-focus-visible]`, which `FocusRing` sets on the button.
        <FocusRing is_disabled=disabled on_focus_change=move |is_focused| focused.set(is_focused)>
            <SaveButton/>
        </FocusRing>

        <p class="demo-status">
            {move || if focused.get() { "The button has focus." } else { "The button doesn\u{2019}t have focus." }}
        </p>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}

/// Reads the focus state of the surrounding `FocusRing` to show a keyboard hint.
#[component]
fn SaveButton() -> impl IntoView {
    let ring = expect_context::<FocusRingContext>();

    view! {
        <button type="button" class="demo-focus-ring">
            "Save"
            <Show when=move || ring.is_focus_visible.get()>
                <span class="demo-focus-hint-inline">" (press Enter)"</span>
            </Show>
        </button>
    }
}
