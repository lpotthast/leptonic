//! `use_select_state` in the plain select of `use_select`'s performance test ([`SelectFixture`]),
//! for its performance test (`leptonic/tests/perf_tests/hooks/use_select_state.rs`): the controls
//! `mount` and `unmount` mount the state with everything.

use leptos::prelude::*;

use super::use_select::{Part, SelectFixture};

#[component]
pub fn PagePerf() -> impl IntoView {
    view! { <SelectFixture title="use_select_state performance" part=Part::State /> }
}
