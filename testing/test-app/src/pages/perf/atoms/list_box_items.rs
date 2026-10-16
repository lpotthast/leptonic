//! `ListBoxItems` in a `ListBox` (a multiple selection), for its performance test
//! (`leptonic/tests/perf_tests/atoms/list_box_items.rs`): one option per collection item, "Option
//! 0", "Option 1", ... (the option's content is a span with `data-perf-text` holding the number),
//! [`SMALL`] at first, [`LARGE`] after the control `large` (while unmounted), in a 200px high
//! scroll view, between two plain buttons (`#perf-before`, `#perf-after`). The listbox stays
//! mounted; the controls ([`Control`]) mount and unmount the `ListBoxItems` and change the
//! collection (append, remove, rename, reverse).

use leptonic::{
    atoms::listbox::{ListBox, ListBoxItems},
    hooks::collections::{SelectionMode, use_collection},
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
    // The item shown as "Renamed <i>".
    let renamed = RwSignal::new(None::<u32>);
    let collection = use_collection(move |b| {
        let renamed = renamed.get();
        items.with(|items| {
            for &i in items {
                let text = if renamed == Some(i) {
                    format!("Renamed {i}")
                } else {
                    format!("Option {i}")
                };
                b.item(i, text);
            }
        });
    });
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("large", move || items.set((0..LARGE).collect())),
        Control::action("append", move || {
            items.update(|items| {
                let next = items.iter().max().map_or(0, |last| last + 1);
                items.push(next);
            });
        }),
        Control::action("remove-last", move || {
            items.update(|items| {
                items.pop();
            });
        }),
        Control::action("rename-first", move || renamed.set(Some(0))),
        Control::action("restore-names", move || renamed.set(None)),
        Control::action("reverse", move || items.update(|items| items.reverse())),
    ];
    view! {
        <h1>"ListBoxItems performance"</h1>
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
            <Show when=move || mounted.get()>
                <ListBoxItems let:node>
                    <span data-perf-text=node.key.to_string()>{node.text_value.to_string()}</span>
                </ListBoxItems>
            </Show>
        </ListBox>
        <button id="perf-after">"After"</button>
    }
}
