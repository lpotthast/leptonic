//! A `TableHeader` in a `Table` (`[role=grid]`), for the performance test of `TableHeader`
//! (`leptonic/tests/perf_tests/atoms/table_header.rs`): the table of the `Table` fixture
//! ([`super::table`]: [`SMALL`] rows "Row <i>" in the sortable columns Name, Value and Group,
//! with multiple selection and selection checkboxes; its column headers, the select all
//! checkbox among them, are the header's), between two plain buttons (`#perf-before`,
//! `#perf-after`) to tab from and to, mounted at first. Its controls ([`Control`]) mount and
//! unmount the header alone (`mount-header`, `unmount-header`) or the table (`mount`,
//! `unmount`), set the size while unmounted (`small`, `large`: [`LARGE`] rows), prepare the live
//! announcer (`prepare-announcer`: its first announcement waits for its live regions) and read
//! how many rows are selected (`selected`, -1 for all).

use leptonic::{
    Assertiveness, announce,
    atoms::table::{Table, TableBody, TableHeader},
    hooks::collections::{Selection, SelectionMode},
};
use leptos::prelude::*;

use super::table::{LARGE, SMALL, TableRows, selected_count, table_collection};
use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let header = RwSignal::new(true);
    let rows = RwSignal::new(SMALL);
    let selection = RwSignal::new(Selection::default());
    let table = table_collection(rows, false);
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
