//! The `HiddenSelect` of a `Select` (`.leptonic-Select`, labelled "Option", the field `option`),
//! for its performance test (`leptonic/tests/perf_tests/atoms/hidden_select.rs`): the select's
//! options "Option 0", "Option 1", ... ([`SMALL`] at first: a native `<select>`; [`LARGE`] after
//! the control `large`: a hidden input). The select's value is controlled. Mounted at first.
//!
//! Its controls ([`Control`]) mount and unmount the hidden select (the select stays mounted),
//! change the options and the select's reactive props, set its value, and read it (`value`: the
//! selected option's number, `-1` for none).

use leptonic::{
    atoms::select::{HiddenSelect, Select},
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
    let disabled = RwSignal::new(false);
    let required = RwSignal::new(false);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("large", move || items.set((0..LARGE).collect())),
        Control::action("select-first", move || value.set(Some(0))),
        Control::action("clear-value", move || value.set(None)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("require", move || required.set(true)),
        Control::action("unrequire", move || required.set(false)),
        Control::read("value", move || {
            value.get_untracked().map_or(-1.0, f64::from)
        }),
    ];
    view! {
        <h1>"HiddenSelect performance"</h1>
        <PerfControls controls />
        <Select
            collection=collection
            value=value
            set_value=value
            is_disabled=disabled
            is_required=required
            aria_label="Option"
            name="option"
        >
            <Show when=move || mounted.get()>
                <HiddenSelect />
            </Show>
        </Select>
    }
}
