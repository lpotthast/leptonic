//! A `GridRowGroup` in a `Grid` (`[role=grid]`), for the performance test of `GridRowGroup`
//! (`leptonic/tests/perf_tests/atoms/grid_row_group.rs`): one group of [`SMALL`] rows ("Row
//! <i>", with the cells "Row <i>" and "Value <i>"), mounted at first. Its controls
//! ([`Control`]) mount and unmount the grid and set its size while unmounted (`small`, `large`:
//! [`LARGE`] rows).

use std::sync::Arc;

use leptonic::{
    atoms::grid::{Grid, GridCell, GridRow, GridRowGroup},
    hooks::collections::{Collection, CollectionMemo, Key, SelectionMode},
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
    let rows = RwSignal::new(SMALL);
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
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || rows.set(SMALL)),
        Control::action("large", move || rows.set(LARGE)),
    ];
    view! {
        <h1>"GridRowGroup performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <Grid
                collection=collection
                selection_mode=SelectionMode::Multiple
                aria_label="Perf grid"
            >
                <GridRowGroup>
                    <For each=move || 0..rows.get() key=|i| *i let:i>
                        <GridRow key=i>
                            <GridCell key=Key::cell(&Key::from(i), 0)>
                                {format!("Row {i}")}
                            </GridCell>
                            <GridCell key=Key::cell(&Key::from(i), 1)>
                                {format!("Value {i}")}
                            </GridCell>
                        </GridRow>
                    </For>
                </GridRowGroup>
            </Grid>
        </Show>
    }
}
