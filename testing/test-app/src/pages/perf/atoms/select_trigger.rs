//! The `SelectTrigger` of a `Select` (`.leptonic-Select`, labelled "Option"), for its performance
//! test (`leptonic/tests/perf_tests/atoms/select_trigger.rs`): the trigger shows a `SelectValue`;
//! the select's `SelectPopover` has a `ListBox` of the options "Option 0", "Option 1", ...
//! (`ListBoxItem`s, `data-perf-option` holding the number; [`SMALL`] at first, [`LARGE`] after the
//! control `large`). The select's open state is controlled. Between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first.
//!
//! Its controls ([`Control`]) mount and unmount the trigger (the select stays mounted), change the
//! options, disable and enable the select, open and close it, and read its value (`value`: the
//! selected option's number, `-1` for none).

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
    let value = RwSignal::new(None::<u32>);
    let open = RwSignal::new(false);
    let disabled = RwSignal::new(false);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("large", move || items.set((0..LARGE).collect())),
        Control::action("open", move || open.set(true)),
        Control::action("close", move || open.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("value", move || {
            value.get_untracked().map_or(-1.0, f64::from)
        }),
    ];
    view! {
        <h1>"SelectTrigger performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Select
            collection=collection
            value=value
            set_value=value
            is_open=open
            set_open=open
            is_disabled=disabled
            aria_label="Option"
        >
            <Show when=move || mounted.get()>
                <SelectTrigger>
                    <SelectValue placeholder="Pick an option" />
                </SelectTrigger>
            </Show>
            <SelectPopover>
                <ListBox>
                    <For each=move || items.get() key=|i| *i let:i>
                        <ListBoxItem key=i attr:data-perf-option=i>
                            {format!("Option {i}")}
                        </ListBoxItem>
                    </For>
                </ListBox>
            </SelectPopover>
        </Select>
        <button id="perf-after">"After"</button>
    }
}
