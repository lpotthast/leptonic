//! `use_tag` on the plain tags of a `use_tag_group` group (the fixture of `use_tag_group`'s
//! performance test, [`TagGroupFixture`]), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_tag.rs`): the controls `mount` and `unmount` mount the
//! tags, the group stays mounted.

use leptos::prelude::*;

use super::use_tag_group::{Part, TagGroupFixture};

#[component]
pub fn PagePerf() -> impl IntoView {
    view! { <TagGroupFixture title="use_tag performance" part=Part::Tags /> }
}
