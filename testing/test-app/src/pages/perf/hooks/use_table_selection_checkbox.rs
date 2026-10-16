//! Row selection checkboxes (`use_table_selection_checkbox`, rendered with `use_checkbox`) in the
//! selection column of a hook table, for the performance test of `use_table_selection_checkbox`
//! (`leptonic/tests/perf_tests/hooks/use_table_selection_checkbox.rs`): the table of the
//! `use_table` fixture ([`super::use_table`]: [`SMALL`] rows "Row <i>" in the columns Name, Value
//! and Group, with multiple selection) with selection checkboxes, between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls
//! ([`Control`]) mount and unmount the table, set its size while unmounted (`small`, `large`:
//! [`LARGE`] rows), select all rows and none, disable the first row and enable it, prepare the
//! live announcer (`prepare-announcer`: its first announcement waits for its live regions) and
//! read how many rows are selected (`selected`, -1 for all).

use std::collections::HashSet;

use leptonic::{Assertiveness, announce, hooks::collections::Key};
use leptos::prelude::*;

use super::use_table::{HookTable, LARGE, SMALL, hook_table_state, selected_count};
use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let rows = RwSignal::new(SMALL);
    let disabled = RwSignal::new(HashSet::<Key>::new());
    let state = hook_table_state(rows, true, disabled.into());
    let selection = state.grid.list.selection;
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || rows.set(SMALL)),
        Control::action("large", move || rows.set(LARGE)),
        Control::action("select-all", move || selection.select_all()),
        Control::action("clear-selection", move || selection.clear_selection()),
        Control::action("disable-first", move || {
            disabled.set(HashSet::from([Key::from(0_usize)]));
        }),
        Control::action("enable-all", move || disabled.set(HashSet::new())),
        Control::action("prepare-announcer", || announce("", Assertiveness::Polite)),
        Control::read("selected", move || {
            selected_count(&untrack(|| selection.raw_selection()))
        }),
    ];
    view! {
        <h1>"use_table_selection_checkbox performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <HookTable state rows on_row_action=None on_cell_action=None />
        </Show>
        <button id="perf-after">"After"</button>
    }
}
