use std::collections::HashSet;

use leptonic::{
    atoms::{
        field::Label,
        tag_group::{TagGroup, TagItems, TagList, TagRemoveButton},
    },
    hooks::collections::{Key, UseListCollectionInput, use_list_collection},
};
use leptos::prelude::*;

#[component]
pub fn TagGroupConceptDemo() -> impl IntoView {
    // App state: the active filters.
    let filters = RwSignal::new(vec!["In stock", "Under $50", "Free shipping"]);
    let collection = use_list_collection(UseListCollectionInput {
        items: filters.into(),
        key: |filter| Key::from(*filter),
        text_value: |filter| (*filter).to_owned(),
    });

    view! {
        <TagGroup
            collection=collection
            // Removing a tag (its button, Delete or Backspace) asks you to drop it from your data.
            on_remove={move |keys: HashSet<Key>| {
                filters.update(|filters| filters.retain(|filter| !keys.contains(&Key::from(*filter))));
            }}
            classes="demo-tag-atom-group"
        >
            <Label classes="demo-tag-group-label">"Filters"</Label>
            <TagList empty_state=|| "No filters." classes="demo-tag-atom-list">
                <TagItems classes="demo-tag-atom" let:node>
                    {node.text_value.to_string()}
                    <TagRemoveButton classes="demo-tag-atom-remove">"\u{00d7}"</TagRemoveButton>
                </TagItems>
            </TagList>
        </TagGroup>
        <p class="demo-status">
            {move || match filters.with(Vec::len) {
                1 => "1 filter applied.".to_owned(),
                count => format!("{count} filters applied."),
            }}
        </p>
    }
}
