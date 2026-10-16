//! `use_selectable_collection` on the plain list of `use_selectable_list`'s performance test, with
//! a `ListKeyboardDelegate` the fixture builds (without type-ahead) ([`SelectableListFixture`]),
//! for its performance test (`leptonic/tests/perf_tests/hooks/use_selectable_collection.rs`).

use leptos::prelude::*;

use super::use_selectable_list::{Collection, Part, SelectableListFixture};

#[component]
pub fn PagePerf() -> impl IntoView {
    view! {
        <SelectableListFixture
            title="use_selectable_collection performance"
            collection=Collection::SelectableCollection
            part=Part::List
        />
    }
}
