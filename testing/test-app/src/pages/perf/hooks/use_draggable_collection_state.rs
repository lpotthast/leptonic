//! The hook `use_draggable_collection_state` in its collection, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_draggable_collection_state.rs`): the draggable list of
//! the `use_draggable_item` fixture ([`DraggableListPage`], see there), whose controls call and
//! read the drag state (`drag-count`, `items`, `dragging`) and change the selection it follows
//! (`select-all`, `clear-selection`).

use leptos::prelude::*;

use super::use_draggable_item::DraggableListPage;

#[component]
pub fn PagePerf() -> impl IntoView {
    view! { <DraggableListPage title="use_draggable_collection_state performance" /> }
}
