use leptonic::{
    atoms::grid_list::{GridList, GridListItem},
    hooks::{Key, SelectionBehavior, SelectionMode, collections::Selection, use_list_collection},
};
use leptos::prelude::*;

const FILES: [(&str, &str); 3] = [("doc", "Document.pdf"), ("photo", "Photo.jpg"), ("sheet", "Budget.xlsx")];

#[component]
pub fn GridListConceptDemo() -> impl IntoView {
    // A key and a text value (for type-ahead) per file.
    let files = use_list_collection(Signal::stored(FILES.to_vec()), |(key, _)| Key::from(*key), |(_, name)| (*name).to_owned());
    let selection = RwSignal::new(Selection::default());
    let opened = RwSignal::new(None::<Key>);

    view! {
        <GridList
            collection=files
            selection_mode=SelectionMode::Single
            // A press selects a file, Enter or a double click opens it.
            selection_behavior=SelectionBehavior::Replace
            selection=selection
            set_selection=selection
            on_action=move |key: Key| opened.set(Some(key))
            aria_label="Files"
            classes="demo-grid-list"
        >
            {FILES
                .map(|(key, name)| view! {
                    <GridListItem key=key classes="demo-grid-list-item">
                        <span class="demo-grid-list-name">{name}</span>
                    </GridListItem>
                })
                .collect_view()}
        </GridList>
        <p class="demo-status">
            {move || match opened.get() {
                Some(key) => format!("Opened: {key}."),
                None => "Nothing opened yet (Enter or double click opens a file).".to_owned(),
            }}
        </p>
    }
}
