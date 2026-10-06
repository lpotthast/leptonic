use std::collections::HashSet;

use leptonic::{
    atoms::{
        combobox::{ComboBox, ComboBoxButton, ComboBoxPopover},
        field::Label,
        input::Input,
        listbox::{ListBox, ListBoxItems},
    },
    hooks::{
        collections::{Key, use_list_collection},
        use_contains_filter,
    },
};
use leptos::prelude::*;

const FRUITS: [&str; 5] = ["Apple", "Banana", "Cherry", "Durian", "Elderberry"];

/// A combo box over fruits, filtering by "contains". "Cherry" is disabled. The value is shown in
/// `#test-cb-value`.
#[component]
pub fn PageAtomComboBox() -> impl IntoView {
    let fruits = use_list_collection(
        Signal::stored(FRUITS.to_vec()),
        |fruit| Key::from(*fruit),
        |fruit| (*fruit).to_owned(),
    );
    let value = RwSignal::new(String::new());

    view! {
        <div id="test-page-atom-combobox">
            <h1>"ComboBox"</h1>
            <button id="test-cb-before">"Before"</button>
            <ComboBox
                collection=fruits
                filter=use_contains_filter()
                disabled_keys=Signal::stored(HashSet::from([Key::from("Cherry")]))
                on_change=Callback::new(move |keys: Vec<Key>| {
                    let keys: Vec<String> = keys.iter().map(ToString::to_string).collect();
                    value.set(keys.join(","));
                })
            >
                <Label>"Fruit"</Label>
                <Input />
                <ComboBoxButton>"▼"</ComboBoxButton>
                <ComboBoxPopover>
                    <ListBox>
                        <ListBoxItems let:node>{node.text_value.to_string()}</ListBoxItems>
                    </ListBox>
                </ComboBoxPopover>
            </ComboBox>
            <button id="test-cb-after">"After"</button>
            <div>"Value: " <span id="test-cb-value">{value}</span></div>
        </div>
    }
}
