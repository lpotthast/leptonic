//! The `SelectValue` of a `Select` (`.leptonic-Select`, labelled "Option"), for its performance
//! test (`leptonic/tests/perf_tests/atoms/select_value.rs`): in the select's `SelectTrigger`,
//! showing the selected one of the options "Option 0", "Option 1", ... ([`SMALL`] at first,
//! [`LARGE`] after the control `large`) or its placeholder. The select's value is controlled.
//! Mounted at first.
//!
//! Its controls ([`Control`]) mount and unmount the value (the select and its trigger stay
//! mounted), change the options, set the select's value and rename the placeholder.

use leptonic::{
    atoms::select::{Select, SelectTrigger, SelectValue},
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
    let placeholder = RwSignal::new("Pick an option".to_owned());
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("large", move || items.set((0..LARGE).collect())),
        Control::action("select-first", move || value.set(Some(0))),
        Control::action("select-last", move || {
            value.set(items.with_untracked(|items| items.last().copied()));
        }),
        Control::action("clear-value", move || value.set(None)),
        Control::action("rename-placeholder", move || {
            placeholder.set("Choose an option".to_owned());
        }),
    ];
    view! {
        <h1>"SelectValue performance"</h1>
        <PerfControls controls />
        <Select collection=collection value=value set_value=value aria_label="Option">
            <SelectTrigger>
                <Show when=move || mounted.get()>
                    <SelectValue placeholder=placeholder />
                </Show>
            </SelectTrigger>
        </Select>
    }
}
