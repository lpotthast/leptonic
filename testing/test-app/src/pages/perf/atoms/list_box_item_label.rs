//! `ListBoxItemLabel`s in the options of a `ListBox` (a multiple selection), for their performance
//! test (`leptonic/tests/perf_tests/atoms/list_box_item_label.rs`): options "Option 0", "Option 1",
//! ... (`data-perf-option` holding the number), each with a `ListBoxItemLabel` (`data-perf-label`),
//! [`SMALL`] at first, [`LARGE`] after the control `large` (while unmounted), in a 200px high
//! scroll view, between two plain buttons (`#perf-before`, `#perf-after`). The listbox and its
//! options stay mounted; the controls ([`Control`]) mount and unmount the labels, or all options
//! (`large`).

use leptonic::{
    atoms::listbox::{ListBox, ListBoxItem, ListBoxItemLabel},
    hooks::collections::{Key, SelectionMode, UseListCollectionInput, use_list_collection},
    leptos_styles::Styles,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The options at first.
pub const SMALL: u32 = 10;
/// The options after the control `large`.
pub const LARGE: u32 = 1_000;

#[component]
pub fn PagePerf() -> impl IntoView {
    let options_mounted = RwSignal::new(true);
    let mounted = RwSignal::new(true);
    let items = RwSignal::new((0..SMALL).collect::<Vec<u32>>());
    let collection = use_list_collection(UseListCollectionInput {
        items: items.into(),
        key: |i| Key::from(*i),
        text_value: |i| format!("Option {i}"),
    });
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("large", move || {
            options_mounted.set(false);
            items.set((0..LARGE).collect());
            options_mounted.set(true);
        }),
    ];
    view! {
        <h1>"ListBoxItemLabel performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <ListBox
            collection=collection
            selection_mode=SelectionMode::Multiple
            aria_label="Options"
            styles=Styles::new()
                .add_unchecked("max-height", "200px")
                .add_unchecked("overflow", "auto")
        >
            <Show when=move || options_mounted.get()>
                <For each=move || items.get() key=|i| *i let:i>
                    <ListBoxItem key=i attr:data-perf-option=i>
                        <Show when=move || mounted.get()>
                            <ListBoxItemLabel attr:data-perf-label=i>
                                {format!("Option {i}")}
                            </ListBoxItemLabel>
                        </Show>
                    </ListBoxItem>
                </For>
            </Show>
        </ListBox>
        <button id="perf-after">"After"</button>
    }
}
