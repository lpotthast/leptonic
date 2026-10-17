//! The `ComboBox` alone (`.leptonic-ComboBox`, labelled "Option"), for its performance test
//! (`leptonic/tests/perf_tests/atoms/combo_box.rs`), with its parts: an `Input`, a
//! `ComboBoxButton`, a `ComboBoxValue` and a `ComboBoxPopover` with a `ListBox` of the options
//! "Option 0", "Option 1", ... (`ListBoxItems`, filtered by "contains"; an option's content is a
//! span with `data-perf-option` holding the number), [`SMALL`] at first, [`LARGE`] after the
//! control `large`. Its value and input text are controlled. Between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first.
//!
//! Its controls ([`Control`]) mount and unmount it, change the options and its reactive props, set
//! its value and input text, prepare the live announcer (`prepare-announcer`: its first
//! announcement waits for its live regions), and read its value (`value`: the selected option's
//! number, `-1` for none).

use std::collections::HashSet;

use leptonic::{
    Assertiveness, announce,
    atoms::{
        combobox::{ComboBox, ComboBoxButton, ComboBoxPopover, ComboBoxValue},
        input::Input,
        listbox::{ListBox, ListBoxItems},
    },
    hooks::{
        collections::{Key, UseListCollectionInput, use_list_collection},
        combobox::{ComboBoxMenuTrigger, use_contains_filter},
    },
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
    let text = RwSignal::new(String::new());
    let disabled = RwSignal::new(false);
    let read_only = RwSignal::new(false);
    let invalid = RwSignal::new(false);
    let required = RwSignal::new(false);
    let menu_trigger = RwSignal::new(ComboBoxMenuTrigger::Input);
    let disabled_keys = RwSignal::new(HashSet::<Key>::new());
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("large", move || items.set((0..LARGE).collect())),
        Control::action("select-first", move || value.set(Some(0))),
        Control::action("select-last", move || {
            value.set(items.with_untracked(|items| items.last().copied()));
        }),
        Control::action("clear-value", move || value.set(None)),
        Control::action("set-text", move || text.set("Option 5".to_owned())),
        Control::action("clear-text", move || text.set(String::new())),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("make-read-only", move || read_only.set(true)),
        Control::action("make-editable", move || read_only.set(false)),
        Control::action("invalidate", move || invalid.set(true)),
        Control::action("validate", move || invalid.set(false)),
        Control::action("require", move || required.set(true)),
        Control::action("unrequire", move || required.set(false)),
        Control::action("open-on-focus", move || {
            menu_trigger.set(ComboBoxMenuTrigger::Focus);
        }),
        Control::action("open-on-input", move || {
            menu_trigger.set(ComboBoxMenuTrigger::Input);
        }),
        Control::action("disable-first", move || {
            disabled_keys.set(HashSet::from([Key::from(0_u32)]));
        }),
        Control::action("enable-all", move || disabled_keys.set(HashSet::new())),
        Control::action("prepare-announcer", || announce("", Assertiveness::Polite)),
        Control::read("value", move || {
            value.get_untracked().map_or(-1.0, f64::from)
        }),
    ];
    view! {
        <h1>"ComboBox performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <ComboBox
                collection=collection
                filter=use_contains_filter()
                value=value
                set_value=value
                input_value=text
                set_input_value=text
                is_disabled=disabled
                is_read_only=read_only
                is_invalid=invalid
                is_required=required
                menu_trigger=menu_trigger
                disabled_keys=disabled_keys
                aria_label="Option"
            >
                <Input />
                <ComboBoxButton>"▼"</ComboBoxButton>
                <ComboBoxValue placeholder="Nothing selected" />
                <ComboBoxPopover>
                    <ListBox>
                        <ListBoxItems let:node>
                            <span data-perf-option=node.key.to_string()>
                                {node.text_value.to_string()}
                            </span>
                        </ListBoxItems>
                    </ListBox>
                </ComboBoxPopover>
            </ComboBox>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
