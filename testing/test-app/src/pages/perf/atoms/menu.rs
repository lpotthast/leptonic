//! The `Menu` alone (`role="menu"`, not in a popover), for its performance test
//! (`leptonic/tests/perf_tests/atoms/menu.rs`): [`SMALL`] or [`LARGE`] items ("Item 0", "Item 1",
//! ..., and the last one "Zebra"; key `i`, rendered by `MenuItems`), between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first with [`SMALL`], without
//! selection. Its controls ([`Control`]) mount it with either size and unmount it, switch to single
//! selection and back, disable and enable the first item, and read how many actions it reported
//! (`actions`).

use std::collections::HashSet;

use leptonic::{
    atoms::menu::{Menu, MenuItems},
    hooks::collections::{Key, SelectionMode, use_collection},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The items of the small menu.
pub const SMALL: i32 = 10;
/// The items of the large menu.
pub const LARGE: i32 = 1_000;

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let size = RwSignal::new(SMALL);
    let selection_mode = RwSignal::new(SelectionMode::None);
    let disabled_keys = RwSignal::new(HashSet::<Key>::new());
    let actions = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount-small", move || {
            size.set(SMALL);
            mounted.set(true);
        }),
        Control::action("mount-large", move || {
            size.set(LARGE);
            mounted.set(true);
        }),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("single-selection", move || {
            selection_mode.set(SelectionMode::Single);
        }),
        Control::action("no-selection", move || {
            selection_mode.set(SelectionMode::None);
        }),
        Control::action("disable-first", move || {
            disabled_keys.set(HashSet::from([Key::from(0)]));
        }),
        Control::action("enable-first", move || disabled_keys.set(HashSet::new())),
        Control::read("actions", move || f64::from(actions.get_value())),
    ];
    view! {
        <h1>"Menu performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Subject
                size=size.get_untracked()
                selection_mode=selection_mode.into()
                disabled_keys=disabled_keys.into()
                actions
            />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// The menu with `size` items.
#[component]
#[allow(clippy::implicit_hasher)]
fn Subject(
    size: i32,
    selection_mode: Signal<SelectionMode>,
    disabled_keys: Signal<HashSet<Key>>,
    actions: StoredValue<u32>,
) -> impl IntoView {
    let items = use_collection(move |b| {
        for i in 0..size - 1 {
            b.item(i, format!("Item {i}"));
        }
        // The only item starting with "z", for type-ahead.
        b.item(size - 1, "Zebra");
    });
    view! {
        <Menu
            collection=items
            aria_label="Actions"
            selection_mode=selection_mode
            disabled_keys=disabled_keys
            on_action=move |_| actions.update_value(|count| *count += 1)
        >
            <MenuItems let:node>{node.text_value.to_string()}</MenuItems>
        </Menu>
    }
}
