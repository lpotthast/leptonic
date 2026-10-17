//! Hook rows (`use_grid_row` on plain elements) in a hook grid, for the performance test of
//! `use_grid_row` (`leptonic/tests/perf_tests/hooks/use_grid_row.rs`): a grid (`use_grid` on a
//! `div`) of [`SMALL`] rows "Row <i>" of three cells, in row focus mode with multiple selection,
//! between two plain buttons (`#perf-before`, `#perf-after`) to tab from and to, mounted at first.
//! Its controls ([`Control`]) mount and unmount the grid (`mount`, `mount-with-actions`: with a row
//! action; `unmount`), set its size while unmounted (`small`, `large`: [`LARGE`] rows), add and
//! remove a row (mounting and unmounting a `use_grid_row`), disable the second row, prepare the
//! live announcer (`prepare-announcer`: its first announcement waits for its live regions) and read
//! how many rows are selected (`selected`) and how often a row action ran (`actions`).

use std::{collections::HashSet, sync::Arc};

use leptonic::{
    Assertiveness, CapturedElement, announce, flag,
    hooks::{
        collections::{
            Collection, CollectionMemo, CollectionOptions, Key, Selection, SelectionMode,
            SelectionOptions,
        },
        grid::{
            CellFocusMode, GridData, GridFocusMode, GridState, UseGridCellInput, UseGridCellReturn,
            UseGridInput, UseGridReturn, UseGridRowInput, UseGridRowReturn, UseGridStateInput,
            use_grid, use_grid_cell, use_grid_row, use_grid_state,
        },
        gridlist::KeyboardNavigationBehavior,
    },
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

/// A row: `use_grid_row` on a `div`, its pressed state in `data-pressed`.
#[component]
fn HookGridRow(grid: GridData, key: Key, children: Children) -> impl IntoView {
    let UseGridRowReturn {
        row: row_props,
        is_pressed,
        ..
    } = use_grid_row(UseGridRowInput {
        grid,
        key,
        on_context_menu: None,
    });
    let (attrs, styles) = row_props.attrs();
    view! {
        <div {..attrs} style=styles data-pressed=flag(is_pressed)>
            {children()}
        </div>
    }
}

/// A cell: `use_grid_cell` on a `div`, its pressed state in `data-pressed`.
#[component]
fn HookGridCell(
    grid: GridData,
    key: Key,
    #[prop(optional)] focus_mode: Option<CellFocusMode>,
    children: Children,
) -> impl IntoView {
    let UseGridCellReturn {
        cell: grid_cell_props,
        is_pressed,
    } = use_grid_cell(UseGridCellInput {
        should_select_on_press_up: grid.should_select_on_press_up,
        grid,
        key,
        id: None,
        focus_mode,
        allows_arrow_navigation: false,
    });
    let (attrs, styles) = grid_cell_props.attrs();
    view! {
        <div {..attrs} style=styles data-pressed=flag(is_pressed)>
            {children()}
        </div>
    }
}

/// Row `i` with its three cells.
#[component]
fn HookGridRowWithCells(grid: GridData, i: usize) -> impl IntoView {
    let row = Key::from(i);
    view! {
        <HookGridRow grid=grid.clone() key=row.clone()>
            <HookGridCell grid=grid.clone() key=Key::cell(&row, 0)>
                {format!("Row {i}")}
            </HookGridCell>
            <HookGridCell grid=grid.clone() key=Key::cell(&row, 1)>
                {format!("Value {i}")}
            </HookGridCell>
            <HookGridCell grid=grid key=Key::cell(&row, 2)>
                {format!("Group {}", i % 3)}
            </HookGridCell>
        </HookGridRow>
    }
}

/// The grid: `use_grid` on a `div`, with its rows; `on_row_action` counts row actions.
#[component]
fn PerfGrid(
    state: GridState,
    rows: RwSignal<usize>,
    on_row_action: Option<Callback<Key>>,
) -> impl IntoView {
    let UseGridReturn { grid: props, data } = use_grid(UseGridInput {
        state,
        element: CapturedElement::new(),
        id: None,
        aria_label: "Perf grid".into(),
        aria_labelledby: Signal::stored(None),
        keyboard_delegate: None,
        options: CollectionOptions::default(),
        keyboard_navigation_behavior: KeyboardNavigationBehavior::Arrow,
        should_select_on_press_up: false,
        on_row_action,
        on_cell_action: None,
    });
    view! {
        <div {..props.attrs()}>
            <For each=move || 0..rows.get() key=|i| *i let:i>
                <HookGridRowWithCells grid=data.clone() i=i />
            </For>
        </div>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let with_actions = RwSignal::new(false);
    let rows = RwSignal::new(SMALL);
    let disabled = RwSignal::new(HashSet::<Key>::new());
    let actions = StoredValue::new(0_u32);
    let state = use_grid_state(UseGridStateInput {
        collection: grid_collection(rows),
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            disabled_keys: disabled.into(),
            ..SelectionOptions::default()
        },
        focus_mode: GridFocusMode::Row,
    });
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
            selected_count(&untrack(|| state.list.selection.raw_selection()))
        }),
        Control::read("actions", move || f64::from(actions.get_value())),
    ];
    let on_row_action = Callback::new(move |_: Key| actions.update_value(|count| *count += 1));
    view! {
        <h1>"use_grid_row performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <PerfGrid
                state
                rows
                on_row_action=with_actions.get_untracked().then_some(on_row_action)
            />
        </Show>
        <button id="perf-after">"After"</button>
    }
}
