//! The `SelectPopover` of a `Select` (the fixture of `Select`'s performance test,
//! [`SelectFixture`]), for its performance test
//! (`leptonic/tests/perf_tests/atoms/select_popover.rs`): the controls `mount` and `unmount` mount
//! the `SelectPopover`, the rest of the select stays mounted.

use leptos::prelude::*;

use super::select::{Part, SelectFixture};

#[component]
pub fn PagePerf() -> impl IntoView {
    view! { <SelectFixture title="SelectPopover performance" part=Part::Popover /> }
}
