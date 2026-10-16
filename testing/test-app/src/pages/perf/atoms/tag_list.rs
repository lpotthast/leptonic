//! The `TagList` of a `TagGroup` (the fixture of `TagGroup`'s performance test,
//! [`TagGroupFixture`]), for its performance test (`leptonic/tests/perf_tests/atoms/tag_list.rs`):
//! the controls `mount` and `unmount` mount the `TagList` (with its tags), the rest of the tag
//! group stays mounted.

use leptos::prelude::*;

use super::tag_group::{Part, TagGroupFixture};

#[component]
pub fn PagePerf() -> impl IntoView {
    view! { <TagGroupFixture title="TagList performance" part=Part::TagList /> }
}
