//! The `Button` alone (`#perf-button`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/button.rs`):
//! between two plain buttons (`#perf-before`, `#perf-after`) to tab from and to, mounted at
//! first. Its controls ([`Control`]) mount and unmount it, change its reactive props, and read how
//! often it was pressed (`presses`).
//!
//! The page creates the live announcer on load (`announce("")`): the button announces its pending
//! state, and the first announcement of a page waits 100 ms for the live regions.

use leptonic::{AriaExpanded, AriaPressed, Assertiveness, announce, atoms::button::Button};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    Effect::new(|| announce("", Assertiveness::Polite));
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let pending = RwSignal::new(false);
    let excluded = RwSignal::new(false);
    let aria_pressed = RwSignal::new(None::<AriaPressed>);
    let expanded = RwSignal::new(None::<AriaExpanded>);
    let press_count = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("start-pending", move || pending.set(true)),
        Control::action("end-pending", move || pending.set(false)),
        Control::action("exclude-from-tab-order", move || excluded.set(true)),
        Control::action("include-in-tab-order", move || excluded.set(false)),
        Control::action("set-aria-pressed", move || {
            aria_pressed.set(Some(AriaPressed::True));
        }),
        Control::action("unset-aria-pressed", move || aria_pressed.set(None)),
        Control::action("expand", move || expanded.set(Some(AriaExpanded::True))),
        Control::action("collapse", move || expanded.set(Some(AriaExpanded::False))),
        Control::read("presses", move || f64::from(press_count.get_value())),
    ];
    view! {
        <h1>"Button performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Button
                id="perf-button"
                is_disabled=disabled
                is_pending=pending
                exclude_from_tab_order=excluded
                aria_pressed=aria_pressed
                aria_expanded=expanded
                on_press=move |_| press_count.update_value(|count| *count += 1)
            >
                "Press me"
            </Button>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
