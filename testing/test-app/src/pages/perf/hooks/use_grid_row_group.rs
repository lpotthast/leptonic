//! A hook row group (`use_grid_row_group` on a plain `div`) in a hook grid, for the performance
//! test of `use_grid_row_group` (`leptonic/tests/perf_tests/hooks/use_grid_row_group.rs`): one
//! group of the [`SMALL`] rows "Row <i>" of three cells of a grid (`use_grid` on a `div`), mounted
//! at first. Its controls ([`Control`]) mount and unmount the grid and set its size while unmounted
//! (`small`, `large`: [`LARGE`] rows).

use std::sync::Arc;

use leptonic::{
    CapturedElement, flag,
    hooks::{
        collections::{
            Collection, CollectionMemo, CollectionOptions, Key, SelectionMode, SelectionOptions,
        },
        grid::{
            CellFocusMode, GridData, GridFocusMode, GridState, UseGridCellInput, UseGridCellReturn,
            UseGridInput, UseGridReturn, UseGridRowInput, UseGridRowReturn, UseGridStateInput,
            use_grid, use_grid_cell, use_grid_row, use_grid_row_group, use_grid_state,
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

/// The grid: `use_grid` on a `div`, its rows in a `use_grid_row_group`.
#[component]
fn PerfGrid(state: GridState, rows: RwSignal<usize>) -> impl IntoView {
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
        on_row_action: None,
        on_cell_action: None,
    });
    let row_group = use_grid_row_group();
    view! {
        <div {..props.attrs()}>
            <div {..row_group.row_group.attrs()}>
                <For each=move || 0..rows.get() key=|i| *i let:i>
                    <HookGridRowWithCells grid=data.clone() i=i />
                </For>
            </div>
        </div>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let rows = RwSignal::new(SMALL);
    let state = use_grid_state(UseGridStateInput {
        collection: grid_collection(rows),
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            ..SelectionOptions::default()
        },
        focus_mode: GridFocusMode::Row,
    });
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || rows.set(SMALL)),
        Control::action("large", move || rows.set(LARGE)),
    ];
    view! {
        <h1>"use_grid_row_group performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <PerfGrid state rows />
        </Show>
    }
}
