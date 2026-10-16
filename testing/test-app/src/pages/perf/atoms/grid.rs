//! The `Grid` alone (`[role=grid]`, labelled "Perf grid"), for its performance test
//! (`leptonic/tests/perf_tests/atoms/grid.rs`): [`SMALL`] rows ("Row <i>", with the cells
//! "Row <i>", "Value <i>" and "Group <i % 3>") in row focus mode with multiple selection, between
//! two plain buttons (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its
//! controls ([`Control`]) mount and unmount it, set its size while unmounted (`small`, `large`:
//! [`LARGE`] rows), add and remove a row, change its controlled selection and its reactive props
//! (selection mode and behavior, disabled rows), prepare the live announcer
//! (`prepare-announcer`: its first announcement waits for its live regions) and read how many
//! rows are selected (`selected`, -1 for all).

use std::{collections::HashSet, sync::Arc};

use leptonic::{
    Assertiveness, announce,
    atoms::grid::{Grid, GridCell, GridRow},
    hooks::collections::{
        Collection, CollectionMemo, Key, Selection, SelectionBehavior, SelectionMode,
    },
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
    let selection = RwSignal::new(Selection::default());
    let selection_mode = RwSignal::new(SelectionMode::Multiple);
    let selection_behavior = RwSignal::new(SelectionBehavior::Toggle);
    let disabled = RwSignal::new(HashSet::<Key>::new());
    let collection: CollectionMemo = Memo::new(move |_| {
        Arc::new(Collection::build(|b| {
            for i in 0..rows.get() {
                b.row(i, format!("Row {i}"), |r| {
                    r.cell(format!("Row {i}"));
                    r.cell(format!("Value {i}"));
                    r.cell(format!("Group {}", i % 3));
                });
            }
        }))
    });
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || rows.set(SMALL)),
        Control::action("large", move || rows.set(LARGE)),
        Control::action("add-row", move || rows.update(|rows| *rows += 1)),
        Control::action("remove-row", move || rows.update(|rows| *rows -= 1)),
        Control::action("select-first", move || {
            selection.set(Selection::keys([Key::from(0_usize)]));
        }),
        Control::action("select-all", move || selection.set(Selection::All)),
        Control::action("clear-selection", move || {
            selection.set(Selection::default());
        }),
        Control::action("selection-none", move || {
            selection_mode.set(SelectionMode::None);
        }),
        Control::action("selection-multiple", move || {
            selection_mode.set(SelectionMode::Multiple);
        }),
        Control::action("behavior-replace", move || {
            selection_behavior.set(SelectionBehavior::Replace);
        }),
        Control::action("behavior-toggle", move || {
            selection_behavior.set(SelectionBehavior::Toggle);
        }),
        Control::action("disable-first", move || {
            disabled.set(HashSet::from([Key::from(0_usize)]));
        }),
        Control::action("enable-all", move || disabled.set(HashSet::new())),
        Control::action("prepare-announcer", || announce("", Assertiveness::Polite)),
        Control::read("selected", move || {
            selection.with_untracked(|selection| match selection {
                Selection::All => -1.0,
                Selection::Keys(keys) => f64::from(u32::try_from(keys.len()).unwrap_or(u32::MAX)),
            })
        }),
    ];
    view! {
        <h1>"Grid performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Grid
                collection=collection
                selection_mode=selection_mode
                selection_behavior=selection_behavior
                selection=selection
                set_selection=selection
                disabled_keys=disabled
                aria_label="Perf grid"
            >
                <For each=move || 0..rows.get() key=|i| *i let:i>
                    <GridRow key=i>
                        <GridCell key=Key::cell(&Key::from(i), 0)>{format!("Row {i}")}</GridCell>
                        <GridCell key=Key::cell(&Key::from(i), 1)>{format!("Value {i}")}</GridCell>
                        <GridCell key=Key::cell(&Key::from(i), 2)>
                            {format!("Group {}", i % 3)}
                        </GridCell>
                    </GridRow>
                </For>
            </Grid>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
