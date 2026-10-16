//! `ListBoxItem`s in a `ListBox` (a multiple selection, controlled), for their performance test
//! (`leptonic/tests/perf_tests/atoms/list_box_item.rs`): options "Option 0", "Option 1", ...
//! (`data-perf-option` holding the number), [`SMALL`] at first, [`LARGE`] after the control `large`
//! (while unmounted), in a 200px high scroll view, between two plain buttons (`#perf-before`,
//! `#perf-after`). The listbox stays mounted; the controls ([`Control`]) mount and unmount its
//! items, disable the first one and set the selection.

use std::collections::HashSet;

use leptonic::{
    atoms::listbox::{ListBox, ListBoxItem},
    hooks::collections::{
        Key, Selection, SelectionMode, UseListCollectionInput, use_list_collection,
    },
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
    let mounted = RwSignal::new(true);
    let items = RwSignal::new((0..SMALL).collect::<Vec<u32>>());
    let selection = RwSignal::new(Selection::default());
    let disabled_keys = RwSignal::new(HashSet::<Key>::new());
    let collection = use_list_collection(UseListCollectionInput {
        items: items.into(),
        key: |i| Key::from(*i),
        text_value: |i| format!("Option {i}"),
    });
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("large", move || items.set((0..LARGE).collect())),
        Control::action("select-first", move || {
            selection.set(Selection::keys([Key::from(0_u32)]));
        }),
        Control::action("clear-selection", move || {
            selection.set(Selection::default());
        }),
        Control::action("disable-first", move || {
            disabled_keys.set(HashSet::from([Key::from(0_u32)]));
        }),
        Control::action("enable-all", move || disabled_keys.set(HashSet::new())),
    ];
    view! {
        <h1>"ListBoxItem performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <ListBox
            collection=collection
            selection=selection
            set_selection=selection
            selection_mode=SelectionMode::Multiple
            disabled_keys=disabled_keys
            aria_label="Options"
            styles=Styles::new()
                .add_unchecked("max-height", "200px")
                .add_unchecked("overflow", "auto")
        >
            <Show when=move || mounted.get()>
                <For each=move || items.get() key=|i| *i let:i>
                    <ListBoxItem key=i attr:data-perf-option=i>
                        {format!("Option {i}")}
                    </ListBoxItem>
                </For>
            </Show>
        </ListBox>
        <button id="perf-after">"After"</button>
    }
}
