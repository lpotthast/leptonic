//! `use_hidden_select` in the plain select of `use_select`'s performance test ([`SelectFixture`]),
//! for its performance test (`leptonic/tests/perf_tests/hooks/use_hidden_select.rs`): the controls
//! `mount` and `unmount` mount the hidden select, the rest stays.

use leptos::prelude::*;

use super::use_select::{Part, SelectFixture};

#[component]
pub fn PagePerf() -> impl IntoView {
    view! { <SelectFixture title="use_hidden_select performance" part=Part::HiddenSelect /> }
}
