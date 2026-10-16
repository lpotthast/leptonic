//! The `Virtualizer` alone (around a `ListBox`, the collection atom it virtualizes), for its
//! performance test (`leptonic/tests/perf_tests/atoms/virtualizer.rs`): `#perf-virtualizer` holds
//! a list box of [`SMALL`] or [`LARGE`] options ("Item 0", ...; keys `item-0`, ...) in fixed
//! 25px rows, 100 × 100 px, single selection, between two plain buttons (`#perf-before`,
//! `#perf-after`) to tab from and to, mounted at first (with [`SMALL`] options).
//!
//! Its controls ([`Control`]) mount and unmount it, switch the size (`small`, `large`; while
//! unmounted: `#perf-count` builds the collection outside the measurements), change its row size
//! (`taller-rows`, `default-rows`, through `layout_options`), append and remove options
//! (`append`, `remove`) and hide and show it (`display: none`).

use leptonic::{
    atoms::{
        listbox::{ListBox, ListBoxItems},
        virtualizer::Virtualizer,
    },
    hooks::{
        collections::{Key, SelectionMode, UseListCollectionInput, use_list_collection},
        virtualizer::{ItemSize, ListLayout, ListLayoutOptions},
    },
    leptos_styles::Styles,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The options of the small list.
pub const SMALL: usize = 10;
/// The options of the large list.
pub const LARGE: usize = 10_000;
/// Options per `append`.
pub const APPENDED: usize = 10;

/// The layout options with rows of `row_size` px.
fn rows_of(row_size: f64) -> ListLayoutOptions {
    ListLayoutOptions {
        row_size: ItemSize::Fixed(row_size),
        ..ListLayoutOptions::default()
    }
}

/// The list box's size: 100 × 100 px.
fn square() -> Styles {
    Styles::new()
        .add_unchecked("width", "100px")
        .add_unchecked("height", "100px")
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let count = RwSignal::new(SMALL);
    let hidden = RwSignal::new(false);
    let options = RwSignal::new(rows_of(25.0));
    let collection = use_list_collection(UseListCollectionInput {
        items: Signal::derive(move || (0..count.get()).collect::<Vec<usize>>()),
        key: |i| Key::from(format!("item-{i}")),
        text_value: |i| format!("Item {i}"),
    });
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || count.set(SMALL)),
        Control::action("large", move || count.set(LARGE)),
        Control::action("append", move || count.update(|count| *count += APPENDED)),
        Control::action("remove", move || count.update(|count| *count -= APPENDED)),
        Control::action("taller-rows", move || options.set(rows_of(40.0))),
        Control::action("default-rows", move || options.set(rows_of(25.0))),
        Control::action("hide", move || hidden.set(true)),
        Control::action("show", move || hidden.set(false)),
    ];
    view! {
        <h1>"Virtualizer performance"</h1>
        <PerfControls controls />
        // Reads the collection: it is built when the size changes, not when the list mounts.
        <p id="perf-count" data-count=move || collection.with(|c| c.size())></p>
        <button id="perf-before">"Before"</button>
        <div
            id="perf-virtualizer"
            style:display=move || if hidden.get() { "none" } else { "block" }
        >
            <Show when=move || mounted.get()>
                <Virtualizer layout=ListLayout::new(rows_of(25.0)) layout_options=options>
                    <ListBox
                        collection=collection
                        aria_label="Items"
                        selection_mode=SelectionMode::Single
                        styles=square()
                    >
                        <ListBoxItems let:node>{node.text_value.to_string()}</ListBoxItems>
                    </ListBox>
                </Virtualizer>
            </Show>
        </div>
        <button id="perf-after">"After"</button>
    }
}
