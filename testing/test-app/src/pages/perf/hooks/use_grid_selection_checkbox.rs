//! Row selection checkboxes (`use_grid_selection_checkbox`, rendered with `use_checkbox`) for the
//! rows of a grid state, for the performance test of `use_grid_selection_checkbox`
//! (`leptonic/tests/perf_tests/hooks/use_grid_selection_checkbox.rs`): one checkbox per row of
//! the `use_grid` fixture's rows ([`super::use_grid`]: [`SMALL`] rows "Row <i>"), with multiple
//! selection, in `#perf-checkboxes` (a `div` per row), between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls
//! ([`Control`]) mount and unmount the checkboxes, set the size while unmounted (`small`,
//! `large`: [`LARGE`] rows), select all rows and none, disable the first row and enable it, and
//! read how many rows are selected (`selected`, -1 for all).

use std::collections::HashSet;

use leptonic::hooks::{
    collections::{Key, SelectionMode, SelectionOptions},
    form::use_checkbox,
    grid::{
        GridFocusMode, GridState, UseGridSelectionCheckboxInput, UseGridStateInput,
        use_grid_selection_checkbox, use_grid_state,
    },
};
use leptos::prelude::*;

use super::use_grid::{LARGE, SMALL, grid_collection, selected_count};
use crate::pages::perf::{Control, PerfControls};

/// The checkbox selecting row `i`.
#[component]
fn SelectionCheckbox(state: GridState, i: usize) -> impl IntoView {
    let checkbox = use_checkbox(use_grid_selection_checkbox(UseGridSelectionCheckboxInput {
        selection: state.list.selection,
        key: Key::from(i),
    }));
    let (attrs, styles) = checkbox.input_props.into_parts();
    view! {
        <div>
            <input {..attrs} style=styles />
            {format!("Row {i}")}
        </div>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let rows = RwSignal::new(SMALL);
    let disabled = RwSignal::new(HashSet::<Key>::new());
    let state = use_grid_state(UseGridStateInput {
        collection: grid_collection(rows),
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            disabled_keys: disabled.into(),
            ..SelectionOptions::default()
        },
        focus_mode: GridFocusMode::Row,
    });
    let selection = state.list.selection;
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
        Control::read("selected", move || {
            selected_count(&untrack(|| selection.raw_selection()))
        }),
    ];
    view! {
        <h1>"use_grid_selection_checkbox performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <div id="perf-checkboxes">
                <For each=move || 0..rows.get() key=|i| *i let:i>
                    <SelectionCheckbox state i />
                </For>
            </div>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
