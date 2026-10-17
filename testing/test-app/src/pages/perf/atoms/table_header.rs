//! A `TableHeader` in a `Table` (`[role=grid]`), for the performance test of `TableHeader`
//! (`leptonic/tests/perf_tests/atoms/table_header.rs`): a table of [`SMALL`] rows "Row <i>" in the
//! sortable columns Name, Value and Group, with multiple selection and selection checkboxes (its
//! column headers, the select all checkbox among them, are the header's), between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls ([`Control`])
//! mount and unmount the header alone (`mount-header`, `unmount-header`) or the table (`mount`,
//! `unmount`), set the size while unmounted (`small`, `large`: [`LARGE`] rows), prepare the live
//! announcer (`prepare-announcer`: its first announcement waits for its live regions) and read how
//! many rows are selected (`selected`, -1 for all).

use std::sync::Arc;

use leptonic::{
    Assertiveness, announce,
    atoms::table::{Table, TableBody, TableCell, TableHeader, TableRow},
    hooks::{
        collections::{Selection, SelectionMode},
        table::TableCollection,
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
    let header = RwSignal::new(true);
    let rows = RwSignal::new(SMALL);
    let selection = RwSignal::new(Selection::default());
    let table = table_collection(rows);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("mount-header", move || header.set(true)),
        Control::action("unmount-header", move || header.set(false)),
        Control::action("small", move || rows.set(SMALL)),
        Control::action("large", move || rows.set(LARGE)),
        Control::action("prepare-announcer", || announce("", Assertiveness::Polite)),
        Control::read("selected", move || selection.with_untracked(selected_count)),
    ];
    view! {
        <h1>"TableHeader performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Table
                table=table
                show_selection_checkboxes=true
                selection_mode=SelectionMode::Multiple
                selection=selection
                set_selection=selection
                aria_label="Perf table"
            >
                <Show when=move || header.get()>
                    <TableHeader />
                </Show>
                <TableBody>
                    <TableRows rows />
                </TableBody>
            </Table>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
