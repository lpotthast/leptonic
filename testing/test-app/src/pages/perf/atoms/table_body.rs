//! A `TableBody` in a `Table` (`[role=grid]`), for the performance test of `TableBody`
//! (`leptonic/tests/perf_tests/atoms/table_body.rs`): the table of the `Table` fixture
//! ([`super::table`]: [`SMALL`] rows "Row <i>" in the columns Name, Value and Group, with
//! multiple selection), mounted at first. Its controls ([`Control`]) mount and unmount the body
//! alone (`mount-body`, `unmount-body`) or the table (`mount`, `unmount`) and set the size
//! (`small`, `large`: [`LARGE`] rows, while unmounted; `empty`: none).

use leptonic::{
    atoms::table::{Table, TableBody, TableHeader},
    hooks::collections::SelectionMode,
};
use leptos::prelude::*;

use super::table::{LARGE, SMALL, TableRows, table_collection};
use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let body = RwSignal::new(true);
    let rows = RwSignal::new(SMALL);
    let table = table_collection(rows, false);
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
