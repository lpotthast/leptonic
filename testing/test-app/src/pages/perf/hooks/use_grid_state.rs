//! The state of a grid (`use_grid_state`) alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_grid_state.rs`): [`SMALL`] rows "Row <i>" of three cells
//! (key `i`) with multiple selection, created by a mounted component (mounted at first) that shows
//! how many rows are selected (`#perf-selected`) and which is focused (`#perf-focused`). Its
//! controls ([`Control`]) mount and unmount it, set its size while unmounted (`small`, `large`:
//! [`LARGE`] rows), add and remove the last row, call its methods (selection, focus, keyboard
//! navigation) and read how many rows are selected (`selected`, -1 for all) and which is focused
//! (`focused`, -1 for none).

use std::sync::Arc;

use leptonic::hooks::{
    collections::{Collection, CollectionMemo, Key, Selection, SelectionMode, SelectionOptions},
    grid::{GridFocusMode, GridState, UseGridStateInput, use_grid_state},
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

/// Creates the state (into `state`, while mounted) and shows it.
#[component]
fn PerfState(rows: RwSignal<usize>, state: StoredValue<Option<GridState>>) -> impl IntoView {
    let grid = use_grid_state(UseGridStateInput {
        collection: grid_collection(rows),
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            ..SelectionOptions::default()
        },
        focus_mode: GridFocusMode::Row,
    });
    state.set_value(Some(grid));
    on_cleanup(move || state.set_value(None));
    let selection = grid.list.selection;
    view! {
        <output id="perf-selected">{move || selected_count(&selection.raw_selection())}</output>
        <output id="perf-focused">
            {move || selection.focused_key().map(|key| key.to_string())}
        </output>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let rows = RwSignal::new(SMALL);
    let state = StoredValue::new(None::<GridState>);
    // Runs `f` with the mounted state.
    let with_state = move |f: fn(GridState, usize)| {
        if let Some(grid) = state.get_value() {
            f(grid, rows.get_untracked());
        }
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || rows.set(SMALL)),
        Control::action("large", move || rows.set(LARGE)),
        Control::action("add-row", move || rows.update(|rows| *rows += 1)),
        Control::action("remove-last", move || rows.update(|rows| *rows -= 1)),
        Control::action("toggle-first", move || {
            with_state(|grid, _| grid.list.selection.toggle_selection(&Key::from(0_usize)));
        }),
        Control::action("replace-with-last", move || {
            with_state(|grid, rows| {
                grid.list.selection.replace_selection(&Key::from(rows - 1));
            });
        }),
        Control::action("extend-to-first", move || {
            with_state(|grid, _| grid.list.selection.extend_selection(&Key::from(0_usize)));
        }),
        Control::action("select-all", move || {
            with_state(|grid, _| grid.list.selection.select_all());
        }),
        Control::action("clear-selection", move || {
            with_state(|grid, _| grid.list.selection.clear_selection());
        }),
        Control::action("focus-first", move || {
            with_state(|grid, _| {
                grid.list
                    .selection
                    .set_focused_key(Some(Key::from(0_usize)), None);
            });
        }),
        Control::action("focus-last", move || {
            with_state(|grid, rows| {
                grid.list
                    .selection
                    .set_focused_key(Some(Key::from(rows - 1)), None);
            });
        }),
        Control::action("disable-navigation", move || {
            with_state(|grid, _| grid.set_keyboard_navigation_disabled(true));
        }),
        Control::action("enable-navigation", move || {
            with_state(|grid, _| grid.set_keyboard_navigation_disabled(false));
        }),
        Control::read("selected", move || {
            state.get_value().map_or(0.0, |grid| {
                selected_count(&untrack(|| grid.list.selection.raw_selection()))
            })
        }),
        Control::read("focused", move || {
            state
                .get_value()
                .and_then(|grid| untrack(|| grid.list.selection.focused_key()))
                .and_then(|key| key.to_string().parse::<f64>().ok())
                .unwrap_or(-1.0)
        }),
        Control::read("navigation-disabled", move || {
            state.get_value().map_or(0.0, |grid| {
                f64::from(u8::from(
                    grid.is_keyboard_navigation_disabled().get_untracked(),
                ))
            })
        }),
    ];
    view! {
        <h1>"use_grid_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <PerfState rows state />
        </Show>
    }
}
