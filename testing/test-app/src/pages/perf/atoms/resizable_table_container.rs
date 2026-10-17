//! A `ResizableTableContainer` with a `Table` (`[role=grid]`) whose columns can be resized, for the
//! performance test of `ResizableTableContainer`
//! (`leptonic/tests/perf_tests/atoms/resizable_table_container.rs`): [`SMALL`] rows "Row <i>" in
//! the resizable columns Name, Value and Group, sharing the width equally, the container [`WIDE`]
//! pixels wide (styled by the test-app's `test-resizable-container` class: the resizers 8 pixels
//! wide at the right edge of their column headers), between two plain buttons (`#perf-before`,
//! `#perf-after`) to tab from and to, mounted at first. Its controls ([`Control`]) mount and
//! unmount it, set the size while unmounted (`small`, `large`: [`LARGE`] rows), change the
//! container's width (`narrow`: [`NARROW`] pixels, `wide`) and read how often a resize ended
//! (`resize-ends`).

use std::sync::Arc;

use leptonic::{
    atoms::table::{ResizableTableContainer, Table, TableBody, TableCell, TableHeader, TableRow},
    hooks::{collections::SelectionMode, table::TableCollection},
    leptos_styles::Styles,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The rows of the small table.
const SMALL: usize = 10;
/// The rows of the large table.
const LARGE: usize = 1_000;

/// `rows` rows ("Row <i>", key `i`) in the sortable and resizable columns `name` (the row
/// header), `value` and `group`.
fn table_collection(rows: RwSignal<usize>) -> Memo<Arc<TableCollection>> {
    Memo::new(move |_| {
        Arc::new(TableCollection::build(|t| {
            for (key, text) in [("name", "Name"), ("value", "Value"), ("group", "Group")] {
                let mut column = t.column(key, text).allows_sorting();
                if key == "name" {
                    column = column.row_header();
                }
                let _ = column.allows_resizing();
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

/// The container's width, in pixels.
const WIDE: u32 = 900;
/// The container's narrower width, in pixels.
const NARROW: u32 = 600;

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let rows = RwSignal::new(SMALL);
    let width = RwSignal::new(WIDE);
    let resize_ends = StoredValue::new(0_u32);
    let table = table_collection(rows);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || rows.set(SMALL)),
        Control::action("large", move || rows.set(LARGE)),
        Control::action("narrow", move || width.set(NARROW)),
        Control::action("wide", move || width.set(WIDE)),
        Control::read("resize-ends", move || f64::from(resize_ends.get_value())),
    ];
    view! {
        <h1>"ResizableTableContainer performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <ResizableTableContainer
                classes="test-resizable-container"
                styles=Styles::new()
                    .add_optional_unchecked("width", move || Some(format!("{}px", width.get())))
                on_resize_end=move |_| {
                    resize_ends.update_value(|count| *count += 1);
                }
            >
                    <Table
                        table=table
                        selection_mode=SelectionMode::Multiple
                        aria_label="Perf table"
                    >
                        <TableHeader />
                        <TableBody>
                            <TableRows rows />
                        </TableBody>
                    </Table>
            </ResizableTableContainer>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
