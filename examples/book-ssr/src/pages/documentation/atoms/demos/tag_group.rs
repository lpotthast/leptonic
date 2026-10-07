use std::collections::HashSet;

use leptonic::{
    atoms::{
        button::Button,
        field::Label,
        tag_group::{TagGroup, TagItems, TagList, TagRemoveButton},
    },
    hooks::{
        SelectionMode,
        collections::{Key, Selection, use_list_collection},
    },
};
use leptos::prelude::*;

const ALL_TAGS: [&str; 5] = ["Rust", "Leptos", "WebAssembly", "Accessibility", "CSS"];

#[component]
pub fn TagGroupAtomDemo() -> impl IntoView {
    // App state: the tags, and the selected ones.
    let tags = RwSignal::new(ALL_TAGS.to_vec());
    let selection = RwSignal::new(Selection::default());
    let collection = use_list_collection(tags.into(), |tag| Key::from(*tag), |tag| (*tag).to_owned());

    let status = move || {
        let count = match tags.with(Vec::len) {
            1 => "1 tag".to_owned(),
            count => format!("{count} tags"),
        };
        let selected = selection.with(|selection| match selection {
            Selection::All => "all".to_owned(),
            Selection::Keys(keys) if keys.is_empty() => "none".to_owned(),
            Selection::Keys(keys) => {
                let mut keys: Vec<String> = keys.iter().map(ToString::to_string).collect();
                keys.sort();
                keys.join(", ")
            }
        });
        format!("{count}. Selected: {selected}.")
    };

    view! {
        <TagGroup
            collection=collection
            selection_mode=SelectionMode::Multiple
            selection=selection
            set_selection=selection
            // Removing is up to you: drop the keys from your data.
            on_remove={move |keys: HashSet<Key>| {
                tags.update(|tags| tags.retain(|tag| !keys.contains(&Key::from(*tag))));
            }}
            classes="demo-tag-atom-group"
        >
            <Label classes="demo-tag-group-label">"Technologies"</Label>
            <TagList empty_state=|| "No tags left." classes="demo-tag-atom-list">
                <TagItems classes="demo-tag-atom" let:node>
                    {node.text_value.to_string()}
                    // Named "Remove" and the tag's name; the glyph is decorative.
                    <TagRemoveButton classes="demo-tag-atom-remove">"\u{00d7}"</TagRemoveButton>
                </TagItems>
            </TagList>
        </TagGroup>
        <p class="demo-status">{status}</p>
        <div class="demo-controls">
            <Button on_press=move |_| tags.set(ALL_TAGS.to_vec()) classes="demo-btn">"Restore all"</Button>
        </div>
    }
}
