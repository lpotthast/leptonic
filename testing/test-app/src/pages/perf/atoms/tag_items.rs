//! The `TagItems` of a `TagGroup` (the fixture of `TagGroup`'s performance test,
//! [`TagGroupFixture`]), for its performance test (`leptonic/tests/perf_tests/atoms/tag_items.rs`):
//! the controls `mount` and `unmount` mount the `TagItems` (rendering the tags), the rest of the
//! tag group stays mounted.

use leptos::prelude::*;

use super::tag_group::{Part, TagGroupFixture};

#[component]
pub fn PagePerf() -> impl IntoView {
    view! { <TagGroupFixture title="TagItems performance" part=Part::TagItems /> }
}
