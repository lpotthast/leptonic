use std::collections::HashSet;

use leptonic::{
    atoms::grid_list::{GridList, GridListItem},
    hooks::collections::{
        Key, Selection, SelectionMode, UseListCollectionInput, use_list_collection,
    },
};
use leptos::prelude::*;

use super::listbox::describe_selection;

const FOLDERS: [&str; 5] = ["Inbox", "Drafts", "Spam", "Sent", "Trash"];

/// A multi-select grid list. "Spam" is disabled.
#[component]
pub fn PageAtomGridList() -> impl IntoView {
    let folders = use_list_collection(UseListCollectionInput {
        items: Signal::stored(FOLDERS.to_vec()),
        key: |folder| Key::from(*folder),
        text_value: |folder| (*folder).to_owned(),
    });
    let selection = RwSignal::new(String::new());

    view! {
        <div id="test-page-atom-grid-list">
            <h1>"GridList"</h1>
            <button id="test-gl-before">"Before"</button>
            <GridList
                collection=folders
                disabled_keys=Signal::stored(HashSet::from([Key::from("Spam")]))
                selection_mode=SelectionMode::Multiple
                aria_label="Folders"
                on_selection_change=Callback::new(move |s: Selection| {
                    selection.set(describe_selection(&s));
                })
            >
                {FOLDERS
                    .map(|folder| view! { <GridListItem key=folder>{folder}</GridListItem> })
                    .collect_view()}
            </GridList>
            <button id="test-gl-after">"After"</button>
            <div>"Selection: " <span id="test-gl-selection">{selection}</span></div>
        </div>
    }
}
