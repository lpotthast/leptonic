//! The `Tag` of a `TagGroup` (the fixture of `TagGroup`'s performance test, [`TagGroupFixture`]),
//! for its performance test (`leptonic/tests/perf_tests/atoms/tag.rs`): the controls `mount` and
//! `unmount` mount every `Tag` (the fixture renders them with a `For`), the rest of the tag group
//! stays mounted.

use leptos::prelude::*;

use super::tag_group::{Part, TagGroupFixture};

#[component]
pub fn PagePerf() -> impl IntoView {
    view! { <TagGroupFixture title="Tag performance" part=Part::Tags /> }
}
