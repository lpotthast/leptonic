use std::collections::HashSet;

use leptonic::{
    atoms::listbox::{ListBox, ListBoxItem},
    hooks::{
        SelectionMode,
        collections::{Key, Selection, use_list_collection},
    },
};
use leptos::prelude::*;

const FRUITS: [&str; 5] = ["Apple", "Banana", "Cherry", "Durian", "Elderberry"];

/// A multi-select listbox. "Cherry" is disabled.
#[component]
pub fn PageAtomListBox() -> impl IntoView {
    let fruits = use_list_collection(
        Signal::stored(FRUITS.to_vec()),
        |fruit| Key::from(*fruit),
        |fruit| (*fruit).to_owned(),
    );
    let selection = RwSignal::new(String::new());

    view! {
        <div id="test-page-atom-listbox">
            <h1>"ListBox"</h1>
            <button id="test-lb-before">"Before"</button>
            <ListBox
                collection=fruits
                selection_mode=SelectionMode::Multiple
                disabled_keys=Signal::stored(HashSet::from([Key::from("Cherry")]))
                aria_label="Fruits"
                on_selection_change=Callback::new(move |s: Selection| {
                    selection.set(describe_selection(&s));
                })
            >
                {FRUITS
                    .map(|fruit| view! { <ListBoxItem key=fruit>{fruit}</ListBoxItem> })
                    .collect_view()}
            </ListBox>
            <button id="test-lb-after">"After"</button>
            <div>"Selection: " <span id="test-lb-selection">{selection}</span></div>
        </div>
    }
}

/// The selected keys, sorted and comma-separated ("all" for a select-all).
pub fn describe_selection(selection: &Selection) -> String {
    match selection {
        Selection::All => "all".to_owned(),
        Selection::Keys(keys) => {
            let mut keys: Vec<String> = keys.iter().map(ToString::to_string).collect();
            keys.sort();
            keys.join(",")
        }
    }
}
