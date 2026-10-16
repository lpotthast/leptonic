//! `use_combobox_state` in the plain combo box of `use_combobox`'s performance test
//! ([`ComboBoxFixture`]), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_combobox_state.rs`): the controls `mount` and `unmount`
//! mount the state with the combo box.

use leptos::prelude::*;

use super::use_combobox::{ComboBoxFixture, Part};

#[component]
pub fn PagePerf() -> impl IntoView {
    view! { <ComboBoxFixture title="use_combobox_state performance" part=Part::State /> }
}
