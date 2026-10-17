//! The `ComboBoxButton` of a `ComboBox` (`.leptonic-ComboBox`, labelled "Option"), for its
//! performance test (`leptonic/tests/perf_tests/atoms/combo_box_button.rs`): next to the combo
//! box's `Input`, opening its `ComboBoxPopover` with a `ListBox` of the options "Option 0", "Option
//! 1", ... (`ListBoxItems`; an option's content is a span with `data-perf-option` holding the
//! number), [`SMALL`] at first, [`LARGE`] after the control `large`. Before a plain button
//! (`#perf-after`) to hover, mounted at first.
//!
//! Its controls ([`Control`]) mount and unmount the button (the combo box stays mounted), change
//! the options, disable and enable the combo box, and prepare the live announcer
//! (`prepare-announcer`: its first announcement waits for its live regions).

use leptonic::{
    Assertiveness, announce,
    atoms::{
        combobox::{ComboBox, ComboBoxButton, ComboBoxPopover},
        input::Input,
        listbox::{ListBox, ListBoxItems},
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
    let disabled = RwSignal::new(false);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("large", move || items.set((0..LARGE).collect())),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("prepare-announcer", || announce("", Assertiveness::Polite)),
    ];
    view! {
        <h1>"ComboBoxButton performance"</h1>
        <PerfControls controls />
        <ComboBox
            collection=collection
            default_value={None::<u32>}
            is_disabled=disabled
            aria_label="Option"
        >
            <Input />
            <Show when=move || mounted.get()>
                <ComboBoxButton>"▼"</ComboBoxButton>
            </Show>
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
        <button id="perf-after">"After"</button>
    }
}
