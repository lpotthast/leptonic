//! A grid built from the grid hooks on plain elements, for the performance test of `use_grid`
//! (`leptonic/tests/perf_tests/hooks/use_grid.rs`): `use_grid` on a `div` (`[role=grid]`), its
//! [`SMALL`] rows ("Row <i>") `use_grid_row`s ([`HookGridRow`]) with the cells "Row <i>",
//! "Value <i>" and "Group <i % 3>" (`use_grid_cell`s, [`HookGridCell`]), in row focus mode with
//! multiple selection, between two plain buttons (`#perf-before`, `#perf-after`) to tab from and
//! to, mounted at first. The state (`use_grid_state`) lives outside the mounted part, so mounting
//! measures `use_grid` and its rows.
//!
//! Its controls ([`Control`]) mount and unmount the grid, set its size (`small`, `large`:
//! [`LARGE`] rows; `empty`: none), change the selection mode, disable keyboard navigation and
//! enable it again, prepare the live announcer (`prepare-announcer`: its first announcement waits
//! for its live regions) and read how many rows are selected (`selected`, -1 for all).
//!
//! [`HookGridRow`] and [`HookGridCell`] also build the grids of the fixtures of the other grid
//! hooks.

use std::sync::Arc;

use leptonic::{
    Assertiveness, CapturedElement, IntoAttrs, announce, flag,
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
pub const SMALL: usize = 10;
/// The rows of the large grid.
pub const LARGE: usize = 1_000;

/// `rows` rows ("Row <i>", key `i`), each with the cells "Row <i>", "Value <i>" and
/// "Group <i % 3>".
pub fn grid_collection(rows: RwSignal<usize>) -> CollectionMemo {
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
pub fn selected_count(selection: &Selection) -> f64 {
    match selection {
        Selection::All => -1.0,
        Selection::Keys(keys) => f64::from(u32::try_from(keys.len()).unwrap_or(u32::MAX)),
    }
}

/// A row: `use_grid_row` on a `div`, its pressed state in `data-pressed`.
#[component]
pub fn HookGridRow(grid: GridData, key: Key, children: Children) -> impl IntoView {
    let UseGridRowReturn {
        row_props,
        is_pressed,
        ..
    } = use_grid_row(UseGridRowInput {
        grid,
        key,
        on_context_menu: None,
    });
    let (attrs, styles) = row_props.into_parts();
    view! {
        <div {..attrs} style=styles data-pressed=flag(is_pressed)>
            {children()}
        </div>
    }
}

/// A cell: `use_grid_cell` on a `div`, its pressed state in `data-pressed`.
#[component]
pub fn HookGridCell(
    grid: GridData,
    key: Key,
    #[prop(optional)] focus_mode: Option<CellFocusMode>,
    children: Children,
) -> impl IntoView {
    let UseGridCellReturn {
        grid_cell_props,
        is_pressed,
    } = use_grid_cell(UseGridCellInput {
        should_select_on_press_up: grid.should_select_on_press_up,
        grid,
        key,
        id: None,
        focus_mode,
        allows_arrow_navigation: false,
    });
    let (attrs, styles) = grid_cell_props.into_parts();
    view! {
        <div {..attrs} style=styles data-pressed=flag(is_pressed)>
            {children()}
        </div>
    }
}

/// Row `i` with its three cells.
#[component]
pub fn HookGridRowWithCells(grid: GridData, i: usize) -> impl IntoView {
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

/// The grid: `use_grid` on a `div`, with its rows.
#[component]
fn PerfGrid(state: GridState, rows: RwSignal<usize>) -> impl IntoView {
    let UseGridReturn { props, data } = use_grid(UseGridInput {
        state,
        element: CapturedElement::new(),
        id: None,
        aria_label: "Perf grid".into(),
        aria_labelledby: Signal::stored(None),
        keyboard_delegate: None,
        options: CollectionOptions::default(),
        keyboard_navigation_behavior: KeyboardNavigationBehavior::Arrow,
        should_select_on_press_up: false,
        on_row_action: None,
        on_cell_action: None,
    });
    view! {
        <div {..props.into_attrs()}>
            <For each=move || 0..rows.get() key=|i| *i let:i>
                <HookGridRowWithCells grid=data.clone() i=i />
            </For>
        </div>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let rows = RwSignal::new(SMALL);
    let selection_mode = RwSignal::new(SelectionMode::Multiple);
    let state = use_grid_state(UseGridStateInput {
        collection: grid_collection(rows),
        selection: SelectionOptions {
            selection_mode: selection_mode.into(),
            ..SelectionOptions::default()
        },
        focus_mode: GridFocusMode::Row,
    });
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || rows.set(SMALL)),
        Control::action("large", move || rows.set(LARGE)),
        Control::action("empty", move || rows.set(0)),
        Control::action("selection-none", move || {
            selection_mode.set(SelectionMode::None);
        }),
        Control::action("selection-multiple", move || {
            selection_mode.set(SelectionMode::Multiple);
        }),
        Control::action("disable-navigation", move || {
            state.set_keyboard_navigation_disabled(true);
        }),
        Control::action("enable-navigation", move || {
            state.set_keyboard_navigation_disabled(false);
        }),
        Control::action("prepare-announcer", || announce("", Assertiveness::Polite)),
        Control::read("selected", move || {
            selected_count(&untrack(|| state.list.selection.raw_selection()))
        }),
    ];
    view! {
        <h1>"use_grid performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <PerfGrid state rows />
        </Show>
        <button id="perf-after">"After"</button>
    }
}
