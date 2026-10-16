//! A `ResizableTableContainer` with a `Table` (`[role=grid]`) whose columns can be resized, for
//! the performance test of `ResizableTableContainer`
//! (`leptonic/tests/perf_tests/atoms/resizable_table_container.rs`): the table of the `Table`
//! fixture ([`super::table`]: [`SMALL`] rows "Row <i>" in the columns Name, Value and Group, here
//! resizable, sharing the width equally), the container [`WIDE`] pixels wide (styled by the
//! test-app's `test-resizable-container` class: the resizers 8 pixels wide at the right edge of
//! their column headers), between two plain
//! buttons (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls
//! ([`Control`]) mount and unmount it, set the size while unmounted (`small`, `large`: [`LARGE`]
//! rows), change the container's width (`narrow`: [`NARROW`] pixels, `wide`) and read how often a
//! resize ended (`resize-ends`).

use leptonic::{
    atoms::table::{ResizableTableContainer, Table, TableBody, TableHeader},
    hooks::collections::SelectionMode,
    leptos_styles::Styles,
};
use leptos::prelude::*;

use super::table::{LARGE, SMALL, TableRows, table_collection};
use crate::pages::perf::{Control, PerfControls};

/// The container's width, in pixels.
pub const WIDE: u32 = 900;
/// The container's narrower width, in pixels.
pub const NARROW: u32 = 600;

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let rows = RwSignal::new(SMALL);
    let width = RwSignal::new(WIDE);
    let resize_ends = StoredValue::new(0_u32);
    let table = table_collection(rows, true);
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
