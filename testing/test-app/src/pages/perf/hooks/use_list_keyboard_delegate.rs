//! `use_list_keyboard_delegate`, the keyboard delegate of `use_selectable_collection` on the plain
//! list of `use_selectable_list`'s performance test ([`SelectableListFixture`]), for its
//! performance test (`leptonic/tests/perf_tests/hooks/use_list_keyboard_delegate.rs`).

use leptos::prelude::*;

use super::use_selectable_list::{Collection, Part, SelectableListFixture};

#[component]
pub fn PagePerf() -> impl IntoView {
    view! {
        <SelectableListFixture
            title="use_list_keyboard_delegate performance"
            collection=Collection::ListKeyboardDelegate
            part=Part::List
        />
    }
}
