//! Hook rows (`use_grid_row` on plain elements) in a hook grid, for the performance test of
//! `use_grid_row` (`leptonic/tests/perf_tests/hooks/use_grid_row.rs`): the grid of the `use_grid`
//! fixture ([`super::use_grid`]: [`SMALL`] rows "Row <i>" of three cells, in row focus mode with
//! multiple selection), between two plain buttons (`#perf-before`, `#perf-after`) to tab from and
//! to, mounted at first. Its controls ([`Control`]) mount and unmount the grid (`mount`,
//! `mount-with-actions`: with a row action; `unmount`), set its size while unmounted (`small`,
//! `large`: [`LARGE`] rows), add and remove a row (mounting and unmounting a `use_grid_row`),
//! disable the second row, prepare the live announcer (`prepare-announcer`: its first
//! announcement waits for its live regions) and read how many rows are selected (`selected`) and
//! how often a row action ran (`actions`).

use std::collections::HashSet;

use leptonic::{
    Assertiveness, CapturedElement, IntoAttrs, announce,
    hooks::{
        collections::{CollectionOptions, Key, SelectionMode, SelectionOptions},
        grid::{
            GridFocusMode, GridState, UseGridInput, UseGridReturn, UseGridStateInput, use_grid,
            use_grid_state,
        },
        gridlist::KeyboardNavigationBehavior,
    },
};
use leptos::prelude::*;

use super::use_grid::{HookGridRowWithCells, LARGE, SMALL, grid_collection, selected_count};
use crate::pages::perf::{Control, PerfControls};

/// The grid: `use_grid` on a `div`, with its rows; `on_row_action` counts row actions.
#[component]
fn PerfGrid(
    state: GridState,
    rows: RwSignal<usize>,
    on_row_action: Option<Callback<Key>>,
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
        on_row_action,
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
