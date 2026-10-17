use leptonic::hooks::focus::{
    FocusRingTarget, UseFocusRingInput, UseFocusRingReturn, use_focus_ring,
};
use leptos::prelude::*;

#[component]
pub fn FocusRingWithinDemo() -> impl IntoView {
    let UseFocusRingReturn {
        target,
        is_focus_visible,
        ..
    } = use_focus_ring(UseFocusRingInput {
        target: FocusRingTarget::Within,
        ..Default::default()
    });

    view! {
        // One ring around the group (`[data-focus-visible]`) while its field or button has keyboard focus.
        <div class="demo-focus-group" {..target.attrs()}>
            <input type="search" aria-label="Search" placeholder="Search\u{2026}" class="demo-focus-plain"/>
            <button type="button" class="demo-focus-plain">"Go"</button>
        </div>

        <p class="demo-status">
            {move || if is_focus_visible.get() { "The group shows its ring." } else { "The group shows no ring." }}
        </p>
    }
}
