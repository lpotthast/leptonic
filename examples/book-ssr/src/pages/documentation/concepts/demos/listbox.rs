use leptonic::{
    atoms::listbox::{ListBox, ListBoxItems},
    hooks::{Key, SelectionMode, collections::Selection, use_list_collection},
};
use leptos::prelude::*;

const FRUITS: [&str; 5] = ["Apple", "Banana", "Cherry", "Grape", "Orange"];

#[component]
pub fn ListboxConceptDemo() -> impl IntoView {
    // The options: a key and a text (for type-ahead) each.
    let fruits = use_list_collection(Signal::stored(FRUITS.to_vec()), |fruit| Key::from(*fruit), |fruit| (*fruit).to_owned());
    // App state: the selected fruits.
    let selection = RwSignal::new(Selection::keys([Key::from("Cherry")]));

    let status = move || {
        let mut names: Vec<String> = selection.with(|selection| match selection {
            Selection::All => FRUITS.iter().map(ToString::to_string).collect(),
            Selection::Keys(keys) => keys.iter().map(ToString::to_string).collect(),
        });
        names.sort();
        if names.is_empty() { "Nothing selected.".to_owned() } else { format!("Selected: {}.", names.join(", ")) }
    };

    view! {
        <ListBox
            collection=fruits
            selection_mode=SelectionMode::Multiple
            selection=selection
            set_selection=selection
            aria_label="Fruits"
            classes="demo-fruit-listbox"
        >
            <ListBoxItems classes="demo-fruit-option" let:node>{node.text_value.to_string()}</ListBoxItems>
        </ListBox>
        <p class="demo-status">{status}</p>
    }
}
