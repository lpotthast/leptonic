//! The `MenuItems` alone, for its performance test
//! (`leptonic/tests/perf_tests/atoms/menu_items.rs`): rendering the items of a `Menu`
//! (`role="menu"`, not in a popover) from its collection of [`SMALL`] or [`LARGE`] items ("Item 0",
//! "Item 1", ...; key `i`), mounted at first with [`SMALL`]. Its controls ([`Control`]) mount and
//! unmount the items (the menu stays), set the collection's size, and add an item at its end and
//! remove it.

use leptonic::{
    atoms::menu::{Menu, MenuItems},
    hooks::collections::use_collection,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The items of the small collection.
pub const SMALL: i32 = 10;
/// The items of the large collection.
pub const LARGE: i32 = 1_000;

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let count = RwSignal::new(SMALL);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("size-small", move || count.set(SMALL)),
        Control::action("size-large", move || count.set(LARGE)),
        Control::action("add", move || count.update(|count| *count += 1)),
        Control::action("remove", move || count.update(|count| *count -= 1)),
    ];
    let items = use_collection(move |b| {
        for i in 0..count.get() {
            b.item(i, format!("Item {i}"));
        }
    });
    view! {
        <h1>"MenuItems performance"</h1>
        <PerfControls controls />
        <Menu collection=items aria_label="Actions">
            <Show when=move || mounted.get()>
                <MenuItems let:node>{node.text_value.to_string()}</MenuItems>
            </Show>
        </Menu>
    }
}
