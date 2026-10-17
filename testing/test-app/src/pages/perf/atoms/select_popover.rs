//! The `SelectPopover` of a `Select` (`.leptonic-Select`, labelled "Option"), for its performance
//! test (`leptonic/tests/perf_tests/atoms/select_popover.rs`): a `ListBox` of the options "Option
//! 0", "Option 1", ... (`ListBoxItem`s, `data-perf-option` holding the number; [`SMALL`] at first,
//! [`LARGE`] after the control `large`), opened by the select's `SelectTrigger` (showing a
//! `SelectValue`). The select's open state is controlled. Mounted at first.
//!
//! Its controls ([`Control`]) mount and unmount the popover (the select and its trigger stay
//! mounted), change the options, and open and close the select.

use leptonic::{
    atoms::{
        listbox::{ListBox, ListBoxItem},
        select::{Select, SelectPopover, SelectTrigger, SelectValue},
    },
    hooks::collections::{Key, UseListCollectionInput, use_list_collection},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The options at first.
const SMALL: u32 = 10;
/// The options after the control `large`.
const LARGE: u32 = 1_000;

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let items = RwSignal::new((0..SMALL).collect::<Vec<u32>>());
    let collection = use_list_collection(UseListCollectionInput {
        items: items.into(),
        key: |i| Key::from(*i),
        text_value: |i| format!("Option {i}"),
    });
    let open = RwSignal::new(false);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("large", move || items.set((0..LARGE).collect())),
        Control::action("open", move || open.set(true)),
        Control::action("close", move || open.set(false)),
    ];
    view! {
        <h1>"SelectPopover performance"</h1>
        <PerfControls controls />
        <Select
            collection=collection
            default_value={None::<u32>}
            is_open=open
            set_open=open
            aria_label="Option"
        >
            <SelectTrigger>
                <SelectValue placeholder="Pick an option" />
            </SelectTrigger>
            <Show when=move || mounted.get()>
                <SelectPopover>
                    <ListBox>
                        <For each=move || items.get() key=|i| *i let:i>
                            <ListBoxItem key=i attr:data-perf-option=i>
                                {format!("Option {i}")}
                            </ListBoxItem>
                        </For>
                    </ListBox>
                </SelectPopover>
            </Show>
        </Select>
    }
}
