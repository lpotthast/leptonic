//! The `Table` alone (`[role=grid]`, labelled "Perf table"), for its performance test
//! (`leptonic/tests/perf_tests/atoms/table.rs`): a `TableHeader` and a `TableBody` of [`SMALL`]
//! `TableRow`s ("Row <i>", key `i`) with the sortable columns Name (the row header), Value and
//! Group, with multiple selection and selection checkboxes, between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls
//! ([`Control`]) mount and unmount it, set its size while unmounted (`small`, `large`:
//! [`LARGE`] rows), add and remove a row, change its controlled selection and sort and its
//! reactive props (selection mode, selection checkboxes, disabled rows), prepare the live
//! announcer (`prepare-announcer`: its first announcement waits for its live regions) and read
//! how many rows are selected (`selected`, -1 for all).

use std::{collections::HashSet, sync::Arc};

use leptonic::{
    Assertiveness, announce,
    atoms::table::{Table, TableBody, TableCell, TableHeader, TableRow},
    hooks::{
        collections::{Key, Selection, SelectionMode},
        table::{SortDescriptor, SortDirection, TableCollection},
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The rows of the small table.
const SMALL: usize = 10;
/// The rows of the large table.
const LARGE: usize = 1_000;

/// How many rows of `selection` are selected (-1: all).
fn selected_count(selection: &Selection) -> f64 {
    match selection {
        Selection::All => -1.0,
        Selection::Keys(keys) => f64::from(u32::try_from(keys.len()).unwrap_or(u32::MAX)),
    }
}

/// `rows` rows ("Row <i>", key `i`) in the sortable columns `name` (the row header), `value` and
/// `group`.
fn table_collection(rows: RwSignal<usize>) -> Memo<Arc<TableCollection>> {
    Memo::new(move |_| {
        Arc::new(TableCollection::build(|t| {
            for (key, text) in [("name", "Name"), ("value", "Value"), ("group", "Group")] {
                let column = t.column(key, text).allows_sorting();
                if key == "name" {
                    let _ = column.row_header();
                }
            }
            for i in 0..rows.get() {
                t.row(i, format!("Row {i}"), |r| {
                    r.cell(format!("Row {i}"));
                    r.cell(format!("Value {i}"));
                    r.cell(format!("Group {}", i % 3));
                });
            }
        }))
    })
}

/// The table's `rows` rows (`TableRow`s of three `TableCell`s).
#[component]
fn TableRows(rows: RwSignal<usize>) -> impl IntoView {
    view! {
        <For each=move || 0..rows.get() key=|i| *i let:i>
            <TableRow key=i>
                <TableCell column="name">{format!("Row {i}")}</TableCell>
                <TableCell column="value">{format!("Value {i}")}</TableCell>
                <TableCell column="group">{format!("Group {}", i % 3)}</TableCell>
            </TableRow>
        </For>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let rows = RwSignal::new(SMALL);
    let selection = RwSignal::new(Selection::default());
    let selection_mode = RwSignal::new(SelectionMode::Multiple);
    let checkboxes = RwSignal::new(true);
    let disabled = RwSignal::new(HashSet::<Key>::new());
    let sort = RwSignal::new(None::<SortDescriptor>);
    let table = table_collection(rows);
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
        Control::action("hide-checkboxes", move || checkboxes.set(false)),
        Control::action("show-checkboxes", move || checkboxes.set(true)),
        Control::action("disable-first", move || {
            disabled.set(HashSet::from([Key::from(0_usize)]));
        }),
        Control::action("enable-all", move || disabled.set(HashSet::new())),
        Control::action("sort-ascending", move || {
            sort.set(Some(SortDescriptor {
                column: Key::from("value"),
                direction: SortDirection::Ascending,
            }));
        }),
        Control::action("sort-descending", move || {
            sort.set(Some(SortDescriptor {
                column: Key::from("value"),
                direction: SortDirection::Descending,
            }));
        }),
        Control::action("prepare-announcer", || announce("", Assertiveness::Polite)),
        Control::read("selected", move || selection.with_untracked(selected_count)),
    ];
    view! {
        <h1>"Table performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Table
                table=table
                show_selection_checkboxes=checkboxes
                selection_mode=selection_mode
                selection=selection
                set_selection=selection
                disabled_keys=disabled
                sort_descriptor=sort
                set_sort_descriptor=sort
                aria_label="Perf table"
            >
                <TableHeader />
                <TableBody>
                    <TableRows rows />
                </TableBody>
            </Table>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
