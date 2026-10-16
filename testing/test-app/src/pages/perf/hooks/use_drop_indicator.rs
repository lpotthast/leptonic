//! The hook `use_drop_indicator` with its collection, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_drop_indicator.rs`): the droppable list of the
//! `use_droppable_collection` fixture ([`DroppableListPage`], see there), whose drop indicators
//! come before each row, after the last and for the list itself.

use leptos::prelude::*;

use super::use_droppable_collection::DroppableListPage;

#[component]
pub fn PagePerf() -> impl IntoView {
    view! { <DroppableListPage title="use_drop_indicator performance" /> }
}
