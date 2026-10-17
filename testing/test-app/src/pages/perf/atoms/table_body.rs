//! A `TableBody` in a `Table` (`[role=grid]`), for the performance test of `TableBody`
//! (`leptonic/tests/perf_tests/atoms/table_body.rs`): a table of [`SMALL`] rows "Row <i>" in the
//! columns Name, Value and Group, with multiple selection, mounted at first. Its controls
//! ([`Control`]) mount and unmount the body alone (`mount-body`, `unmount-body`) or the table
//! (`mount`, `unmount`) and set the size (`small`, `large`: [`LARGE`] rows, while unmounted;
//! `empty`: none).

use std::sync::Arc;

use leptonic::{
    atoms::table::{Table, TableBody, TableCell, TableHeader, TableRow},
    hooks::{collections::SelectionMode, table::TableCollection},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The rows of the small table.
const SMALL: usize = 10;
/// The rows of the large table.
const LARGE: usize = 1_000;

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
    let body = RwSignal::new(true);
    let rows = RwSignal::new(SMALL);
    let table = table_collection(rows);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("mount-body", move || body.set(true)),
        Control::action("unmount-body", move || body.set(false)),
        Control::action("small", move || rows.set(SMALL)),
        Control::action("large", move || rows.set(LARGE)),
        Control::action("empty", move || rows.set(0)),
    ];
    view! {
        <h1>"TableBody performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <Table table=table selection_mode=SelectionMode::Multiple aria_label="Perf table">
                <TableHeader />
                <Show when=move || body.get()>
                    <TableBody>
                        <TableRows rows />
                    </TableBody>
                </Show>
            </Table>
        </Show>
    }
}
