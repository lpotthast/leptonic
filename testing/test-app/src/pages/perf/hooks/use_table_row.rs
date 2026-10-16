//! Hook rows and cells (`use_table_row`, `use_table_cell` on plain elements) in a hook table,
//! for the performance tests of `use_table_row` and `use_table_cell`
//! (`leptonic/tests/perf_tests/hooks/use_table_row.rs`, `.../use_table_cell.rs`, whose fixture
//! this is too): the table of the `use_table` fixture ([`super::use_table`]: [`SMALL`] rows "Row
//! <i>" in the columns Name, Value and Group, with multiple selection), between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls
//! ([`Control`]) mount and unmount the table (`mount`, `mount-with-actions`: with a row and a
//! cell action; `unmount`), set its size while unmounted (`small`, `large`: [`LARGE`] rows), add
//! and remove a row (mounting and unmounting a `use_table_row` and its cells), disable the second
//! row, prepare the live announcer (`prepare-announcer`: its first announcement waits for its
//! live regions) and read how many rows are selected (`selected`) and how often a row or cell
//! action ran (`row-actions`, `cell-actions`).

use std::collections::HashSet;

use leptonic::{Assertiveness, announce, hooks::collections::Key};
use leptos::prelude::*;

use super::use_table::{HookTable, LARGE, SMALL, hook_table_state, selected_count};
use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let with_actions = RwSignal::new(false);
    let rows = RwSignal::new(SMALL);
    let disabled = RwSignal::new(HashSet::<Key>::new());
    let row_actions = StoredValue::new(0_u32);
    let cell_actions = StoredValue::new(0_u32);
    let state = hook_table_state(rows, false, disabled.into());
    let controls = vec![
        Control::action("mount", move || {
            with_actions.set(false);
            mounted.set(true);
        }),
        Control::action("mount-with-actions", move || {
            with_actions.set(true);
            mounted.set(true);
        }),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || rows.set(SMALL)),
        Control::action("large", move || rows.set(LARGE)),
        Control::action("add-row", move || rows.update(|rows| *rows += 1)),
        Control::action("remove-row", move || rows.update(|rows| *rows -= 1)),
        Control::action("disable-second", move || {
            disabled.set(HashSet::from([Key::from(1_usize)]));
        }),
        Control::action("prepare-announcer", || announce("", Assertiveness::Polite)),
        Control::read("selected", move || {
            selected_count(&untrack(|| state.grid.list.selection.raw_selection()))
        }),
        Control::read("row-actions", move || f64::from(row_actions.get_value())),
        Control::read("cell-actions", move || f64::from(cell_actions.get_value())),
    ];
    let on_row_action = Callback::new(move |_: Key| row_actions.update_value(|count| *count += 1));
    let on_cell_action =
        Callback::new(move |_: Key| cell_actions.update_value(|count| *count += 1));
    view! {
        <h1>"use_table_row performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <HookTable
                state
                rows
                on_row_action=with_actions.get_untracked().then_some(on_row_action)
                on_cell_action=with_actions.get_untracked().then_some(on_cell_action)
            />
        </Show>
        <button id="perf-after">"After"</button>
    }
}
