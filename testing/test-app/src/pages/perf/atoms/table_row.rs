//! `TableRow`s in a `Table` (`[role=grid]`), for the performance test of `TableRow`
//! (`leptonic/tests/perf_tests/atoms/table_row.rs`): [`SMALL`] rows "Row <i>" in the columns Name,
//! Value and Group, with multiple selection and selection checkboxes, between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls ([`Control`])
//! mount and unmount the table (`mount`, `mount-with-actions`: with a row and a cell action;
//! `unmount`), set its size while unmounted (`small`, `large`: [`LARGE`] rows), add and remove a
//! row (mounting and unmounting a `TableRow` and its cells), disable the second row, prepare the
//! live announcer (`prepare-announcer`: its first announcement waits for its live regions) and read
//! how many rows are selected (`selected`) and how often a row or cell action ran (`row-actions`,
//! `cell-actions`).

use std::{collections::HashSet, sync::Arc};

use leptonic::{
    Assertiveness, announce,
    atoms::table::{Table, TableBody, TableCell, TableHeader, TableRow},
    hooks::{
        collections::{Key, Selection, SelectionMode},
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
    let with_actions = RwSignal::new(false);
    let rows = RwSignal::new(SMALL);
    let selection = RwSignal::new(Selection::default());
    let disabled = RwSignal::new(HashSet::<Key>::new());
    let row_actions = StoredValue::new(0_u32);
    let cell_actions = StoredValue::new(0_u32);
    let table = table_collection(rows);
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
        Control::read("selected", move || selection.with_untracked(selected_count)),
        Control::read("row-actions", move || f64::from(row_actions.get_value())),
        Control::read("cell-actions", move || f64::from(cell_actions.get_value())),
    ];
    let table_view = move || {
        if with_actions.get_untracked() {
            view! {
                <Table
                    table=table
                    show_selection_checkboxes=true
                    selection_mode=SelectionMode::Multiple
                    selection=selection
                    set_selection=selection
                    disabled_keys=disabled
                    on_row_action=move |_| row_actions.update_value(|count| *count += 1)
                    on_cell_action=move |_| cell_actions.update_value(|count| *count += 1)
                    aria_label="Perf table"
                >
                    <TableHeader />
                    <TableBody>
                        <TableRows rows />
                    </TableBody>
                </Table>
            }
            .into_any()
        } else {
            view! {
                <Table
                    table=table
                    show_selection_checkboxes=true
                    selection_mode=SelectionMode::Multiple
                    selection=selection
                    set_selection=selection
                    disabled_keys=disabled
                    aria_label="Perf table"
                >
                    <TableHeader />
                    <TableBody>
                        <TableRows rows />
                    </TableBody>
                </Table>
            }
            .into_any()
        }
    };
    view! {
        <h1>"TableRow performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>{table_view}</Show>
        <button id="perf-after">"After"</button>
    }
}
