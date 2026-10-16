//! The hook `use_draggable_collection` in its collection, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_draggable_collection.rs`): the draggable list of the
//! `use_draggable_item` fixture ([`DraggableListPage`], see there), which records itself as the
//! collection drags come from while its rows are dragged.

use leptos::prelude::*;

use super::use_draggable_item::DraggableListPage;

#[component]
pub fn PagePerf() -> impl IntoView {
    view! { <DraggableListPage title="use_draggable_collection performance" /> }
}
