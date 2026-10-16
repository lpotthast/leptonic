//! `use_option` on the plain `<div>` options of a `use_listbox` listbox (the fixture of
//! `use_listbox`'s performance test, [`ListBoxFixture`]), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_option.rs`): the controls `mount` and `unmount` mount the
//! options, the listbox stays mounted.

use leptos::prelude::*;

use super::use_listbox::{ListBoxFixture, Part};

#[component]
pub fn PagePerf() -> impl IntoView {
    view! { <ListBoxFixture title="use_option performance" part=Part::Options /> }
}
