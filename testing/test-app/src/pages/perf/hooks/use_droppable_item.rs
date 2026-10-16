//! The hook `use_droppable_item` with its collection, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_droppable_item.rs`): the droppable list of the
//! `use_droppable_collection` fixture ([`DroppableListPage`], see there), whose rows are the
//! droppable items.

use leptos::prelude::*;

use super::use_droppable_collection::DroppableListPage;

#[component]
pub fn PagePerf() -> impl IntoView {
    view! { <DroppableListPage title="use_droppable_item performance" /> }
}
