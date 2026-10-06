use std::collections::HashSet;

use leptonic::{
    atoms::grid_list::{GridList, GridListItem},
    hooks::{
        SelectionBehavior, SelectionMode,
        collections::{EscapeKeyBehavior, Key, Selection, use_list_collection},
    },
};
use leptos::prelude::*;

/// A file: key, name and icon.
type File = (&'static str, &'static str, &'static str);

const FILES: [File; 5] = [
    ("doc", "Document.pdf", "\u{1F4C4}"),
    ("photo", "Photo.jpg", "\u{1F5BC}"),
    ("sheet", "Spreadsheet.xlsx", "\u{1F4CA}"),
    ("slides", "Presentation.pptx", "\u{1F4CA}"),
    ("archive", "Archive.zip", "\u{1F4E6}"),
];

fn describe(selection: &Selection) -> String {
    match selection {
        Selection::All => "all".to_owned(),
        Selection::Keys(keys) if keys.is_empty() => "none".to_owned(),
        Selection::Keys(keys) => {
            let mut keys: Vec<String> = keys.iter().map(ToString::to_string).collect();
            keys.sort();
            keys.join(", ")
        }
    }
}

#[component]
pub fn GridFileListDemo() -> impl IntoView {
    // The rows: a key and a text value (for type-ahead) per file.
    let files = use_list_collection(
        Signal::stored(FILES.to_vec()),
        |(key, _, _)| Key::from(*key),
        |(_, name, _)| (*name).to_owned(),
    );
    let selected = RwSignal::new(String::from("none"));
    let last_action = RwSignal::new(None::<Key>);

    view! {
        <GridList
            collection=files
            disabled_keys=Signal::stored(HashSet::from([Key::from("archive")]))
            selection_mode=SelectionMode::Multiple
            selection_behavior=SelectionBehavior::Replace
            escape_key_behavior=EscapeKeyBehavior::ClearSelection
            on_selection_change=Callback::new(move |selection| selected.set(describe(&selection)))
            on_action=Callback::new(move |key| last_action.set(Some(key)))
            aria_label="Files"
            classes="demo-grid-list"
        >
            {FILES
                .map(|(key, name, icon)| view! {
                    <GridListItem key=key classes="demo-grid-list-item">
                        <span class="demo-grid-list-icon">{icon}</span>
                        <span>{name}</span>
                    </GridListItem>
                })
                .collect_view()}
        </GridList>

        <p class="demo-caption">
            "\u{201c}Archive.zip\u{201d} is disabled: keyboard navigation skips it and it can\u{2019}t be selected."
        </p>

        <div class="demo-state-display">
            <div><strong>"Selected: "</strong>{selected}</div>
            <div>
                <strong>"Last action: "</strong>
                {move || last_action.get().map_or_else(|| "none".to_owned(), |key| key.to_string())}
            </div>
        </div>
    }
}
