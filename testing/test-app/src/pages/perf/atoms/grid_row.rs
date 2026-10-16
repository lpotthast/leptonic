//! `GridRow`s in a `Grid` (`[role=grid]`), for the performance test of `GridRow`
//! (`leptonic/tests/perf_tests/atoms/grid_row.rs`): [`SMALL`] rows ("Row <i>", with the cells
//! "Row <i>" and "Value <i>") with multiple selection, between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls
//! ([`Control`]) mount and unmount the grid (`mount`, `mount-with-actions`: with a row action;
//! `unmount`), set its size while unmounted (`small`, `large`: [`LARGE`] rows), add and remove a
//! row (mounting and unmounting a `GridRow`), disable the second row, prepare the live announcer
//! (`prepare-announcer`: its first announcement waits for its live regions) and read how many
//! rows are selected (`selected`) and how often a row action ran (`actions`).

use std::{collections::HashSet, sync::Arc};

use leptonic::{
    Assertiveness, announce,
    atoms::grid::{Grid, GridCell, GridRow},
    hooks::collections::{Collection, CollectionMemo, Key, Selection, SelectionMode},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The rows of the small grid.
pub const SMALL: usize = 10;
/// The rows of the large grid.
pub const LARGE: usize = 1_000;

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let with_actions = RwSignal::new(false);
    let rows = RwSignal::new(SMALL);
    let selection = RwSignal::new(Selection::default());
    let disabled = RwSignal::new(HashSet::<Key>::new());
    let actions = StoredValue::new(0_u32);
    let collection: CollectionMemo = Memo::new(move |_| {
        Arc::new(Collection::build(|b| {
            for i in 0..rows.get() {
                b.row(i, format!("Row {i}"), |r| {
                    r.cell(format!("Row {i}"));
                    r.cell(format!("Value {i}"));
                });
            }
        }))
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
            selection.with_untracked(|selection| match selection {
                Selection::All => -1.0,
                Selection::Keys(keys) => f64::from(u32::try_from(keys.len()).unwrap_or(u32::MAX)),
            })
        }),
        Control::read("actions", move || f64::from(actions.get_value())),
    ];
    let rows_view = move || {
        view! {
            <For each=move || 0..rows.get() key=|i| *i let:i>
                <GridRow key=i>
                    <GridCell key=Key::cell(&Key::from(i), 0)>{format!("Row {i}")}</GridCell>
                    <GridCell key=Key::cell(&Key::from(i), 1)>{format!("Value {i}")}</GridCell>
                </GridRow>
            </For>
        }
    };
    let grid = move || {
        if with_actions.get_untracked() {
            view! {
                <Grid
                    collection=collection
                    selection_mode=SelectionMode::Multiple
                    selection=selection
                    set_selection=selection
                    disabled_keys=disabled
                    on_row_action=move |_| actions.update_value(|count| *count += 1)
                    aria_label="Perf grid"
                >
                    {rows_view()}
                </Grid>
            }
            .into_any()
        } else {
            view! {
                <Grid
                    collection=collection
                    selection_mode=SelectionMode::Multiple
                    selection=selection
                    set_selection=selection
                    disabled_keys=disabled
                    aria_label="Perf grid"
                >
                    {rows_view()}
                </Grid>
            }
            .into_any()
        }
    };
    view! {
        <h1>"GridRow performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>{grid}</Show>
        <button id="perf-after">"After"</button>
    }
}
