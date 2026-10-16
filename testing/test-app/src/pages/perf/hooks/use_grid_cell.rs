//! Hook cells (`use_grid_cell` on plain elements, [`HookGridCell`]) in a hook grid in cell focus
//! mode, for the performance test of `use_grid_cell`
//! (`leptonic/tests/perf_tests/hooks/use_grid_cell.rs`): [`SMALL`] rows ("Row <i>") of three
//! cells: "Row <i>", "Value <i>" and one whose children take focus (`CellFocusMode::Child`: the
//! buttons "Edit <i>" and "Delete <i>"), with multiple selection, between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls
//! ([`Control`]) mount and unmount the grid (`mount`, `mount-with-actions`: with a cell action;
//! `unmount`), set its size while unmounted (`small`, `large`: [`LARGE`] rows), add and remove a
//! row (mounting and unmounting its cells), prepare the live announcer (`prepare-announcer`: its
//! first announcement waits for its live regions) and read how often a cell action ran
//! (`actions`).

use leptonic::{
    Assertiveness, CapturedElement, IntoAttrs, announce,
    hooks::{
        collections::{CollectionOptions, Key, SelectionMode, SelectionOptions},
        grid::{
            CellFocusMode, GridFocusMode, GridState, UseGridInput, UseGridReturn,
            UseGridStateInput, use_grid, use_grid_state,
        },
        gridlist::KeyboardNavigationBehavior,
    },
};
use leptos::prelude::*;

use super::use_grid::{HookGridCell, HookGridRow, LARGE, SMALL, grid_collection};
use crate::pages::perf::{Control, PerfControls};

/// The grid: `use_grid` on a `div`, with its rows of cells; `on_cell_action` counts cell
/// actions.
#[component]
fn PerfGrid(
    state: GridState,
    rows: RwSignal<usize>,
    on_cell_action: Option<Callback<Key>>,
) -> impl IntoView {
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
        on_cell_action,
    });
    view! {
        <div {..props.into_attrs()}>
            <For
                each=move || 0..rows.get()
                key=|i| *i
                children=move |i| {
                    let row = Key::from(i);
                    let data = data.clone();
                    view! {
                        <HookGridRow grid=data.clone() key=row.clone()>
                            <HookGridCell grid=data.clone() key=Key::cell(&row, 0)>
                                {format!("Row {i}")}
                            </HookGridCell>
                            <HookGridCell grid=data.clone() key=Key::cell(&row, 1)>
                                {format!("Value {i}")}
                            </HookGridCell>
                            <HookGridCell
                                grid=data.clone()
                                key=Key::cell(&row, 2)
                                focus_mode=CellFocusMode::Child
                            >
                                <button>{format!("Edit {i}")}</button>
                                <button>{format!("Delete {i}")}</button>
                            </HookGridCell>
                        </HookGridRow>
                    }
                }
            />
        </div>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let with_actions = RwSignal::new(false);
    let rows = RwSignal::new(SMALL);
    let actions = StoredValue::new(0_u32);
    let state = use_grid_state(UseGridStateInput {
        collection: grid_collection(rows),
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            ..SelectionOptions::default()
        },
        focus_mode: GridFocusMode::Cell,
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
        Control::action("prepare-announcer", || announce("", Assertiveness::Polite)),
        Control::read("actions", move || f64::from(actions.get_value())),
    ];
    let on_cell_action = Callback::new(move |_: Key| actions.update_value(|count| *count += 1));
    view! {
        <h1>"use_grid_cell performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <PerfGrid
                state
                rows
                on_cell_action=with_actions.get_untracked().then_some(on_cell_action)
            />
        </Show>
        <button id="perf-after">"After"</button>
    }
}
