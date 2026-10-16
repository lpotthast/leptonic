//! The `SelectTrigger` of a `Select` (the fixture of `Select`'s performance test,
//! [`SelectFixture`]), for its performance test
//! (`leptonic/tests/perf_tests/atoms/select_trigger.rs`): the controls `mount` and `unmount` mount
//! the `SelectTrigger`, the rest of the select stays mounted.

use leptos::prelude::*;

use super::select::{Part, SelectFixture};

#[component]
pub fn PagePerf() -> impl IntoView {
    view! { <SelectFixture title="SelectTrigger performance" part=Part::Trigger /> }
}
