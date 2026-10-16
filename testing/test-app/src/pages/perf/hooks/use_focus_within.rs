//! `use_focus_within` on a group of two buttons (`#perf-group`, with `#perf-first` and
//! `#perf-second`), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_focus_within.rs`): between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first, whether it has focus
//! within as `data-focus-within`. Its controls ([`Control`]) mount and unmount it, disable and
//! enable it, and read how often focus entered it (`entries`).

use leptonic::{
    IntoAttrs, flag,
    hooks::focus::{UseFocusWithinInput, use_focus_within},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let entry_count = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("entries", move || f64::from(entry_count.get_value())),
    ];
    view! {
        <h1>"use_focus_within performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Group disabled entry_count />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// The group with `use_focus_within`.
#[component]
fn Group(disabled: RwSignal<bool>, entry_count: StoredValue<u32>) -> impl IntoView {
    let within = use_focus_within(UseFocusWithinInput {
        is_disabled: disabled.into(),
        on_focus_within: Some(Callback::new(move |_| {
            entry_count.update_value(|count| *count += 1);
        })),
        ..UseFocusWithinInput::default()
    });
    view! {
        <div
            id="perf-group"
            role="group"
            {..within.props.into_attrs()}
            data-focus-within=flag(within.is_focus_within)
        >
            <button id="perf-first">"First"</button>
            <button id="perf-second">"Second"</button>
        </div>
    }
}
