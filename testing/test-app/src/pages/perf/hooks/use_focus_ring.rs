//! `use_focus_ring` on a plain `<button>` (`#perf-focus-ring`, the element target) and on a
//! group of two buttons (`#perf-focus-ring-group`, the within target, with `#perf-group-first`
//! and `#perf-group-second`), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_focus_ring.rs`): between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, both mounted at first, their focus as
//! `data-focused`. Its controls ([`Control`]) mount and unmount the button, and disable and
//! enable its ring.

use leptonic::{
    IntoAttrs, flag,
    hooks::focus::{FocusRingTarget, UseFocusRingInput, use_focus_ring},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
    ];
    view! {
        <h1>"use_focus_ring performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Ringed disabled />
        </Show>
        <RingedGroup />
        <button id="perf-after">"After"</button>
    }
}

/// The button with `use_focus_ring` (the element target).
#[component]
fn Ringed(disabled: RwSignal<bool>) -> impl IntoView {
    let ring = use_focus_ring(UseFocusRingInput {
        is_disabled: disabled.into(),
        ..UseFocusRingInput::default()
    });
    view! {
        <button
            id="perf-focus-ring"
            {..ring.props.into_attrs()}
            data-focused=flag(ring.is_focused)
        >
            "Focus me"
        </button>
    }
}

/// The group with `use_focus_ring` (the within target).
#[component]
fn RingedGroup() -> impl IntoView {
    let ring = use_focus_ring(UseFocusRingInput {
        target: FocusRingTarget::Within,
        ..UseFocusRingInput::default()
    });
    view! {
        <div
            id="perf-focus-ring-group"
            role="group"
            {..ring.props.into_attrs()}
            data-focused=flag(ring.is_focused)
        >
            <button id="perf-group-first">"First"</button>
            <button id="perf-group-second">"Second"</button>
        </div>
    }
}
