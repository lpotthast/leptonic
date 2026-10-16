//! A hook row group (`use_grid_row_group` on a plain `div`) in a hook grid, for the performance
//! test of `use_grid_row_group` (`leptonic/tests/perf_tests/hooks/use_grid_row_group.rs`): one
//! group of the rows of the `use_grid` fixture ([`super::use_grid`]: [`SMALL`] rows "Row <i>" of
//! three cells), mounted at first. Its controls ([`Control`]) mount and unmount the grid and set
//! its size while unmounted (`small`, `large`: [`LARGE`] rows).

use leptonic::{
    CapturedElement, IntoAttrs,
    hooks::{
        collections::{CollectionOptions, SelectionMode, SelectionOptions},
        grid::{
            GridFocusMode, GridState, UseGridInput, UseGridReturn, UseGridStateInput, use_grid,
            use_grid_row_group, use_grid_state,
        },
        gridlist::KeyboardNavigationBehavior,
    },
};
use leptos::prelude::*;

use super::use_grid::{HookGridRowWithCells, LARGE, SMALL, grid_collection};
use crate::pages::perf::{Control, PerfControls};

/// The grid: `use_grid` on a `div`, its rows in a `use_grid_row_group`.
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
    let row_group = use_grid_row_group();
    view! {
        <div {..props.into_attrs()}>
            <div {..row_group.row_group_props.into_attrs()}>
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
