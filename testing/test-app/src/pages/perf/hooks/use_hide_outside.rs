//! `use_hide_outside` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_hide_outside.rs`): a plain overlay element
//! (`#perf-overlay`, captured) among [`SMALL`] other elements (`.perf-outside`, [`LARGE`] after
//! the control `large`, set while unmounted), mounted at first, closed, making the page inert
//! while open. The open state is the app's (shown in `#perf-open`). Its controls ([`Control`])
//! mount and unmount it, open and close it, and switch its behavior (`keep-visible`, `inert`).

use leptonic::{
    CapturedElement,
    hooks::overlay::{HideOutsideBehavior, UseHideOutsideInput, use_hide_outside},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The elements outside the overlay at first.
const SMALL: usize = 10;
/// The elements outside the overlay after the control `large`.
const LARGE: usize = 1_000;

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let open = RwSignal::new(false);
    let behavior = RwSignal::new(HideOutsideBehavior::Inert);
    let outside = RwSignal::new(SMALL);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("open", move || open.set(true)),
        Control::action("close", move || open.set(false)),
        Control::action("keep-visible", move || {
            behavior.set(HideOutsideBehavior::KeepVisible);
        }),
        Control::action("inert", move || behavior.set(HideOutsideBehavior::Inert)),
        Control::action("large", move || outside.set(LARGE)),
    ];
    view! {
        <h1>"use_hide_outside performance"</h1>
        <PerfControls controls />
        <span id="perf-open">{move || open.get().to_string()}</span>
        <Show when=move || mounted.get()>
            <Subject open behavior outside=outside.get_untracked() />
        </Show>
    }
}

/// Calls the hook on a plain overlay among `outside` elements.
#[component]
fn Subject(
    open: RwSignal<bool>,
    behavior: RwSignal<HideOutsideBehavior>,
    outside: usize,
) -> impl IntoView {
    let overlay = CapturedElement::new();
    use_hide_outside(UseHideOutsideInput {
        is_open: open.into(),
        overlay,
        behavior: behavior.into(),
    });
    view! {
        <div id="perf-page">
            {(0..outside).map(|i| view! { <p class="perf-outside">{format!("Outside {i}")}</p> }).collect_view()}
            <div id="perf-overlay" {..overlay.attr()}>
                "Overlay"
            </div>
        </div>
    }
}
