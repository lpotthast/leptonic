use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;

fn times(count: u32) -> String {
    if count == 1 { "1 time".to_string() } else { format!("{count} times") }
}

#[component]
pub fn FocusRingKeyboardDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);
    let (focus_count, set_focus_count) = signal(0_u32);

    let UseFocusRingReturn {
        props,
        is_focused,
        is_focus_visible,
    } = use_focus_ring(UseFocusRingInput {
        is_disabled: disabled.into(),
        on_focus: Some(Callback::new(move |_| set_focus_count.update(|c| *c += 1))),
        ..Default::default()
    });

    view! {
        // The CSS draws the ring on `[data-focus-visible]`, which the hook sets.
        <button type="button" class="demo-focus-ring" {..props.into_attrs()}>
            "Click me, then tab away and back"
        </button>

        <p class="demo-status">
            {move || if is_focused.get() { "Focused" } else { "Not focused" }}
            {move || if is_focus_visible.get() { ", ring visible" } else { ", no ring" }}
            ". Focused " {move || times(focus_count.get())} "."
        </p>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}
