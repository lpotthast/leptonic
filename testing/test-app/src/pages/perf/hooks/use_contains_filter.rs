//! `use_contains_filter` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_contains_filter.rs`): the texts "Option 0", "Option 1",
//! ... ([`SMALL`] at first, [`LARGE`] after the control `large`) filtered by a query, the matches
//! shown in a plain list (`#perf-list`, one `<li>` per match). Mounted at first (mounting creates
//! the filter). The controls ([`Control`]) set the query: `1`, `12`, `opt` (matching every text,
//! ignoring case), `ö` (accents ignored: every text) and none.

use leptonic::hooks::combobox::use_contains_filter;
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The texts at first.
pub const SMALL: u32 = 10;
/// The texts after the control `large`.
pub const LARGE: u32 = 1_000;

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let count = RwSignal::new(SMALL);
    let query = RwSignal::new(String::new());
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("large", move || count.set(LARGE)),
        Control::action("one", move || query.set("1".to_owned())),
        Control::action("twelve", move || query.set("12".to_owned())),
        Control::action("opt", move || query.set("opt".to_owned())),
        Control::action("umlaut", move || query.set("ö".to_owned())),
        Control::action("clear", move || query.set(String::new())),
    ];
    view! {
        <h1>"use_contains_filter performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <Matches count query />
        </Show>
    }
}

/// The texts matching the query (the filter created while mounted).
#[component]
fn Matches(count: RwSignal<u32>, query: RwSignal<String>) -> impl IntoView {
    let filter = use_contains_filter();
    let matches = move || {
        let matches = query.with(|query| filter(query));
        (0..count.get())
            .map(|i| format!("Option {i}"))
            .filter(|text| matches(text))
            .collect::<Vec<String>>()
    };
    view! {
        <ul id="perf-list">
            <For each=matches key=Clone::clone let:text>
                <li>{text}</li>
            </For>
        </ul>
    }
}
