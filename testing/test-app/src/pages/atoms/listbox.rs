use std::collections::HashSet;

use leptonic::{
    atoms::listbox::{ListBox, ListBoxItem},
    hooks::{Selection, SelectionMode},
};
use leptos::prelude::*;

const FRUITS: [&str; 5] = ["Apple", "Banana", "Cherry", "Durian", "Elderberry"];

/// A multi-select listbox. "Cherry" is disabled.
#[component]
pub fn PageAtomListBox() -> impl IntoView {
    let items = Signal::stored(FRUITS.map(String::from).to_vec());
    let selection = RwSignal::new(String::new());

    view! {
        <div id="test-page-atom-listbox">
            <h1>"ListBox"</h1>
            <button id="test-lb-before">"Before"</button>
            <ListBox<String>
                items=items
                selection_mode=SelectionMode::Multiple
                disabled_keys=Signal::stored(HashSet::from(["Cherry".to_owned()]))
                aria_label="Fruits"
                get_text_value=Callback::new(|key: String| key)
                on_selection_change=Callback::new(move |s: Selection<String>| {
                    selection.set(describe(&s));
                })
            >
                {FRUITS
                    .map(|fruit| {
                        view! {
                            <ListBoxItem<String> key=fruit.to_owned()>
                                {fruit}
                            </ListBoxItem<String>>
                        }
                    })
                    .collect_view()}
            </ListBox<String>>
            <button id="test-lb-after">"After"</button>
            <div>"Selection: " <span id="test-lb-selection">{selection}</span></div>
        </div>
    }
}

pub fn describe(selection: &Selection<String>) -> String {
    match selection {
        Selection::All => "all".to_owned(),
        Selection::Keys(keys) => {
            let mut keys: Vec<_> = keys.iter().cloned().collect();
            keys.sort();
            keys.join(",")
        }
    }
}
