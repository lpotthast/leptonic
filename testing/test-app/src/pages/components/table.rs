use leptonic::{
    components::table::{Table, TableBody, TableCell, TableHeader, TableHeaderCell, TableRow},
    utils::aria::AriaSort,
};
use leptos::prelude::*;

/// The styled table: a sortable header (focusable, pressed by keyboard too, `aria-sort`).
#[component]
pub fn PageComponentTable() -> impl IntoView {
    let sort = RwSignal::new(AriaSort::None);
    let toggle = move |_| {
        sort.update(|s| {
            *s = if *s == AriaSort::Ascending {
                AriaSort::Descending
            } else {
                AriaSort::Ascending
            };
        });
    };
    view! {
        <div id="test-page-component-table">
            <h1>"Table components"</h1>
            <button id="test-ctable-before">"Before"</button>
            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHeaderCell on_press=toggle aria_sort=sort>
                            "Name"
                        </TableHeaderCell>
                        <TableHeaderCell>"Age"</TableHeaderCell>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    <TableRow>
                        <TableCell>"Ada"</TableCell>
                        <TableCell>"36"</TableCell>
                    </TableRow>
                </TableBody>
            </Table>
        </div>
    }
}
