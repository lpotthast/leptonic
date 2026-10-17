//! The `ComboBoxPopover` of a `ComboBox` (`.leptonic-ComboBox`, labelled "Option"), for its
//! performance test (`leptonic/tests/perf_tests/atoms/combo_box_popover.rs`): a `ListBox` of the
//! options "Option 0", "Option 1", ... (`ListBoxItems`, filtered by "contains"; an option's content
//! is a span with `data-perf-option` holding the number), [`SMALL`] at first, [`LARGE`] after the
//! control `large`, opened by the combo box's `Input` and `ComboBoxButton`. The combo box's value
//! is controlled. Mounted at first.
//!
//! Its controls ([`Control`]) mount and unmount the popover (the combo box stays mounted), change
//! the options, prepare the live announcer (`prepare-announcer`: its first announcement waits for
//! its live regions), and read the combo box's value (`value`: the selected option's number, `-1`
//! for none).

use leptonic::{
    Assertiveness, announce,
    atoms::{
        combobox::{ComboBox, ComboBoxButton, ComboBoxPopover},
        input::Input,
        listbox::{ListBox, ListBoxItems},
    },
    hooks::{
        collections::{Key, UseListCollectionInput, use_list_collection},
        combobox::use_contains_filter,
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
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("large", move || items.set((0..LARGE).collect())),
        Control::action("prepare-announcer", || announce("", Assertiveness::Polite)),
        Control::read("value", move || {
            value.get_untracked().map_or(-1.0, f64::from)
        }),
    ];
    view! {
        <h1>"ComboBoxPopover performance"</h1>
        <PerfControls controls />
        <ComboBox
            collection=collection
            filter=use_contains_filter()
            value=value
            set_value=value
            aria_label="Option"
        >
            <Input />
            <ComboBoxButton>"▼"</ComboBoxButton>
            <Show when=move || mounted.get()>
                <ComboBoxPopover>
                    <ListBox>
                        <ListBoxItems let:node>
                            <span data-perf-option=node.key.to_string()>
                                {node.text_value.to_string()}
                            </span>
                        </ListBoxItems>
                    </ListBox>
                </ComboBoxPopover>
            </Show>
        </ComboBox>
    }
}
