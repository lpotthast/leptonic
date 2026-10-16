//! The hook `use_droppable_collection_state` with its collection, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_droppable_collection_state.rs`): the droppable list of the
//! `use_droppable_collection` fixture ([`DroppableListPage`], see there), whose controls set the
//! drop target through the state (`target-root`, `target-first`, `target-between`, `target-none`)
//! and disable it.

use leptos::prelude::*;

use super::use_droppable_collection::DroppableListPage;

#[component]
pub fn PagePerf() -> impl IntoView {
    view! { <DroppableListPage title="use_droppable_collection_state performance" /> }
}
