//! The `Focusable` alone, on a `<span role="button">` (`#perf-focusable`), for its performance
//! test (`leptonic/tests/perf_tests/atoms/focusable.rs`): between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first, its focus (from
//! `on_focus_change`) as `data-focused` on the span, with a key handler counting key downs. Its
//! controls ([`Control`]) mount and unmount it, change its reactive props, and read the count
//! (`key-downs`).

use leptonic::atoms::focusable::Focusable;
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let excluded = RwSignal::new(false);
    let focused = RwSignal::new(false);
    let key_down_count = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("exclude-from-tab-order", move || excluded.set(true)),
        Control::action("include-in-tab-order", move || excluded.set(false)),
        Control::read("key-downs", move || f64::from(key_down_count.get_value())),
    ];
    view! {
        <h1>"Focusable performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Focusable
                is_disabled=disabled
                exclude_from_tab_order=excluded
                on_focus_change=move |is_focused| focused.set(is_focused)
                on_key_down=move |_| key_down_count.update_value(|count| *count += 1)
            >
                <span
                    id="perf-focusable"
                    role="button"
                    data-focused=move || focused.get().then_some("true")
                >
                    "Focus me"
                </span>
            </Focusable>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
