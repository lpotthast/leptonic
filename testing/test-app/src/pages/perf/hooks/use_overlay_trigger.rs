//! `use_overlay_trigger` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_overlay_trigger.rs`): its props on a plain button
//! (`#perf-trigger`) of a menu overlay (its id `perf-overlay`; the overlay itself isn't rendered:
//! the hook only describes the trigger), mounted at first, closed. The open state is the app's. Its
//! controls ([`Control`]) mount and unmount it, and open and close the overlay.

use leptonic::hooks::overlay::{OverlayTriggerType, UseOverlayTriggerInput, use_overlay_trigger};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let open = RwSignal::new(false);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("open", move || open.set(true)),
        Control::action("close", move || open.set(false)),
    ];
    view! {
        <h1>"use_overlay_trigger performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <Subject open=open.into() />
        </Show>
    }
}

/// Calls the hook on a plain button.
#[component]
fn Subject(open: Signal<bool>) -> impl IntoView {
    let trigger = use_overlay_trigger(UseOverlayTriggerInput {
        is_open: open,
        overlay_id: "perf-overlay".into(),
        overlay_type: OverlayTriggerType::Menu,
    });
    view! {
        <button id="perf-trigger" {..trigger.trigger.attrs()}>
            "Menu"
        </button>
    }
}
