//! Row selection checkboxes (`use_grid_selection_checkbox`, rendered with `use_checkbox`) for the
//! rows of a grid state, for the performance test of `use_grid_selection_checkbox`
//! (`leptonic/tests/perf_tests/hooks/use_grid_selection_checkbox.rs`): one checkbox per row of
//! [`SMALL`] rows "Row <i>" of three cells, with multiple selection, in `#perf-checkboxes` (a `div`
//! per row), between two plain buttons (`#perf-before`, `#perf-after`) to tab from and to, mounted
//! at first. Its controls ([`Control`]) mount and unmount the checkboxes, set the size while
//! unmounted (`small`, `large`: [`LARGE`] rows), select all rows and none, disable the first row
//! and enable it, and read how many rows are selected (`selected`, -1 for all).

use std::{collections::HashSet, sync::Arc};

use leptonic::{
    GridStrings,
    hooks::{
        collections::{
            Collection, CollectionMemo, Key, Selection, SelectionMode, SelectionOptions,
        },
        form::use_checkbox,
        grid::{
            GridFocusMode, GridState, UseGridSelectionCheckboxInput, UseGridStateInput,
            use_grid_selection_checkbox, use_grid_state,
        },
    },
    use_localized_strings,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The rows of the small grid.
const SMALL: usize = 10;

/// The rows of the large grid.
const LARGE: usize = 1_000;

/// `rows` rows ("Row <i>", key `i`), each with the cells "Row <i>", "Value <i>" and
/// "Group <i % 3>".
fn grid_collection(rows: RwSignal<usize>) -> CollectionMemo {
    Memo::new(move |_| {
        Arc::new(Collection::build(|b| {
            for i in 0..rows.get() {
                b.row(i, format!("Row {i}"), |r| {
                    r.cell(format!("Row {i}"));
                    r.cell(format!("Value {i}"));
                    r.cell(format!("Group {}", i % 3));
                });
            }
        }))
    })
}

/// How many rows of `selection` are selected (-1: all).
fn selected_count(selection: &Selection) -> f64 {
    match selection {
        Selection::All => -1.0,
        Selection::Keys(keys) => f64::from(u32::try_from(keys.len()).unwrap_or(u32::MAX)),
    }
}

/// The checkbox selecting row `i`, with the messages all checkboxes share.
#[component]
fn SelectionCheckbox(state: GridState, strings: Memo<GridStrings>, i: usize) -> impl IntoView {
    let selection = state.list.selection;
    // The row's selection state, as a row hook (`use_grid_row`) would return it.
    let is_selected = Memo::new(move |_| selection.is_selected(&Key::from(i))).into();
    let checkbox = use_checkbox(use_grid_selection_checkbox(UseGridSelectionCheckboxInput {
        selection,
        key: Key::from(i),
        is_selected,
        strings,
    }));
    let (attrs, styles) = checkbox
        .input
        .props()
        .expect("the first input takes its props")
        .into_parts();
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
    let strings = use_localized_strings::<GridStrings>();
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
                    <SelectionCheckbox state strings i />
                </For>
            </div>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
