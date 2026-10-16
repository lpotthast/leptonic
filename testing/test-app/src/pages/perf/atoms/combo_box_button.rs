//! The `ComboBoxButton` of a `ComboBox` (the fixture of `ComboBox`'s performance test,
//! [`ComboBoxFixture`]), for its performance test
//! (`leptonic/tests/perf_tests/atoms/combo_box_button.rs`): the controls `mount` and `unmount`
//! mount the `ComboBoxButton`, the rest of the combo box stays mounted.

use leptos::prelude::*;

use super::combo_box::{ComboBoxFixture, Part};

#[component]
pub fn PagePerf() -> impl IntoView {
    view! { <ComboBoxFixture title="ComboBoxButton performance" part=Part::Button /> }
}
