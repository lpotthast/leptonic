//! `use_selectable_item` on the plain items of the list of `use_selectable_list`'s performance test
//! (the controls `mount` and `unmount` mount the items, the list stays mounted)
//! ([`SelectableListFixture`]), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_selectable_item.rs`).

use leptos::prelude::*;

use super::use_selectable_list::{Collection, Part, SelectableListFixture};

#[component]
pub fn PagePerf() -> impl IntoView {
    view! {
        <SelectableListFixture
            title="use_selectable_item performance"
            collection=Collection::SelectableList
            part=Part::Items
        />
    }
}
