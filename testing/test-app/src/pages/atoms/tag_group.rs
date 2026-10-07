use std::collections::HashSet;

use leptonic::{
    atoms::{
        field::{Description, Label},
        tag_group::{TagGroup, TagItems, TagList, TagRemoveButton},
    },
    hooks::{
        SelectionMode,
        collections::{Key, use_list_collection},
    },
};
use leptos::prelude::*;

/// Tag group atoms, as react-aria-components' `TagGroup` tests render them:
/// - `#test-tg-main`: label "Test", removable tags Cat, Dog, Kangaroo (multiple selection), a
///   description; `on_remove` doesn't remove: the last removed keys show in `#test-tg-removed`
///   (sorted, comma-separated), the number of removals in `#test-tg-remove-count`.
/// - `#test-tg-after-group`: a `Label` after the main group, outside any field.
/// - `#test-tg-empty`: no tags, the empty state "No results".
/// - `#test-tg-fruits`: Grape and Plum disabled, Watermelon; removing removes.
#[component]
pub fn PageAtomTagGroup() -> impl IntoView {
    let animals = use_list_collection(
        Signal::stored(vec!["cat", "dog", "kangaroo"]),
        |id| Key::from(*id),
        |id| capitalize(id),
    );
    let removed = RwSignal::new(String::new());
    let remove_count = RwSignal::new(0);
    let on_remove = move |keys: HashSet<Key>| {
        let mut keys: Vec<String> = keys.iter().map(ToString::to_string).collect();
        keys.sort();
        removed.set(keys.join(","));
        remove_count.update(|count| *count += 1);
    };

    let empty = use_list_collection(
        Signal::stored(Vec::<&str>::new()),
        |id| Key::from(*id),
        |id| (*id).to_owned(),
    );

    let fruits = RwSignal::new(vec!["Grape", "Plum", "Watermelon"]);
    let fruit_collection = use_list_collection(
        fruits.into(),
        |fruit| Key::from(*fruit),
        |fruit| (*fruit).to_owned(),
    );
    let disabled_fruits = HashSet::from([Key::from("Grape"), Key::from("Plum")]);
    let remove_fruits = move |keys: HashSet<Key>| {
        fruits.update(|fruits| fruits.retain(|fruit| !keys.contains(&Key::from(*fruit))));
    };

    view! {
        <h1>"TagGroup"</h1>
        <button id="test-tg-before">"Before"</button>
        <div id="test-tg-main">
            <TagGroup collection=animals selection_mode=SelectionMode::Multiple on_remove=on_remove>
                <Label>"Test"</Label>
                <TagList>
                    <TagItems let:node>
                        {node.text_value.to_string()}
                        <TagRemoveButton>"x"</TagRemoveButton>
                    </TagItems>
                </TagList>
                <Description>"Description"</Description>
            </TagGroup>
        </div>
        // A label after a tag group isn't the group's (its contexts stay inside it).
        <div id="test-tg-after-group">
            <Label>"Not the group's"</Label>
        </div>
        <p>"Removed: " <span id="test-tg-removed">{move || removed.get()}</span></p>
        <p>"Removals: " <span id="test-tg-remove-count">{move || remove_count.get()}</span></p>
        <div id="test-tg-empty">
            <TagGroup collection=empty aria_label="Empty">
                <TagList empty_state=|| "No results">
                    <TagItems let:node>{node.text_value.to_string()}</TagItems>
                </TagList>
            </TagGroup>
        </div>
        <div id="test-tg-fruits">
            <TagGroup
                collection=fruit_collection
                aria_label="Fruits"
                selection_mode=SelectionMode::Multiple
                disabled_keys=disabled_fruits
                on_remove=remove_fruits
            >
                <TagList>
                    <TagItems let:node>
                        {node.text_value.to_string()}
                        <TagRemoveButton />
                    </TagItems>
                </TagList>
            </TagGroup>
        </div>
    }
}

fn capitalize(id: &str) -> String {
    let mut chars = id.chars();
    chars
        .next()
        .map(|first| first.to_uppercase().chain(chars).collect())
        .unwrap_or_default()
}
