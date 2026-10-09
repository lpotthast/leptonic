use std::collections::HashSet;

use leptonic::{
    I18nProvider, Locale,
    atoms::{
        field::{Description, Label},
        tag_group::{TagGroup, TagItems, TagList, TagRemoveButton},
    },
    hooks::collections::{
        CollectionMemo, Key, SelectionBehavior, SelectionMode, UseListCollectionInput,
        use_list_collection,
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
/// - `#test-tg-plain`: neither selectable nor with actions (One, Two).
/// - `#test-tg-actions`: actions without selection (Alpha, Beta), logged in `#test-tg-actions-log`.
/// - `#test-tg-replace`: single selection replacing on press, with actions (Red, Green), logged in
///   `#test-tg-replace-log`.
/// - `#test-tg-ordered`: removable tags `#test-tg-insert` inserts at the start ("Item 1", ...).
/// - `#test-tg-rtl`: an Arabic (right-to-left) group (One, Two, Three).
#[component]
pub fn PageAtomTagGroup() -> impl IntoView {
    let animals = use_list_collection(UseListCollectionInput {
        items: Signal::stored(vec!["cat", "dog", "kangaroo"]),
        key: |id| Key::from(*id),
        text_value: |id| capitalize(id),
    });
    let removed = RwSignal::new(String::new());
    let remove_count = RwSignal::new(0);
    let on_remove = move |keys: HashSet<Key>| {
        let mut keys: Vec<String> = keys.iter().map(ToString::to_string).collect();
        keys.sort();
        removed.set(keys.join(","));
        remove_count.update(|count| *count += 1);
    };

    let empty = use_list_collection(UseListCollectionInput {
        items: Signal::stored(Vec::<&str>::new()),
        key: |id| Key::from(*id),
        text_value: |id| (*id).to_owned(),
    });

    let fruits = RwSignal::new(vec!["Grape", "Plum", "Watermelon"]);
    let fruit_collection = use_list_collection(UseListCollectionInput {
        items: fruits.into(),
        key: |fruit| Key::from(*fruit),
        text_value: |fruit| (*fruit).to_owned(),
    });
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
                        <span data-tag-label>{node.text_value.to_string()}</span> <TagRemoveButton>"x"</TagRemoveButton>
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
        <MoreGroups />
        <div id="test-tg-fruits">
            <TagGroup
                collection=fruit_collection
                aria_label="Fruits"
                selection_mode=SelectionMode::Multiple
                disabled_keys=disabled_fruits
                on_remove=remove_fruits
            >
                <TagList>
                    <TagItems let:node>{node.text_value.to_string()} <TagRemoveButton /></TagItems>
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

fn tags(items: Signal<Vec<String>>) -> CollectionMemo {
    use_list_collection(UseListCollectionInput {
        items,
        key: |tag: &String| Key::from(tag.as_str()),
        text_value: |tag: &String| tag.clone(),
    })
}

fn texts(items: &[&str]) -> Signal<Vec<String>> {
    Signal::stored(items.iter().map(|item| (*item).to_owned()).collect())
}

/// The groups after the main one (see [`PageAtomTagGroup`]).
#[component]
fn MoreGroups() -> impl IntoView {
    let actions = RwSignal::new(Vec::<String>::new());
    let replace_actions = RwSignal::new(Vec::<String>::new());
    let ordered = RwSignal::new(Vec::<String>::new());
    let next = RwSignal::new(1);
    let insert = move |_| {
        ordered.update(|items| items.insert(0, format!("Item {}", next.get_untracked())));
        next.update(|next| *next += 1);
    };
    let remove_ordered = move |keys: HashSet<Key>| {
        ordered.update(|items| items.retain(|item| !keys.contains(&Key::from(item.as_str()))));
    };
    let rtl: Locale = "ar-AE".parse().expect("a valid locale");
    view! {
        <div id="test-tg-plain">
            <TagGroup collection=tags(texts(&["One", "Two"])) aria_label="Plain">
                <TagList>
                    <TagItems let:node>{node.text_value.to_string()}</TagItems>
                </TagList>
            </TagGroup>
        </div>
        <div id="test-tg-actions">
            <TagGroup
                collection=tags(texts(&["Alpha", "Beta"]))
                aria_label="Actions"
                on_action=move |key: Key| actions.update(|a| a.push(key.to_string()))
            >
                <TagList>
                    <TagItems let:node>{node.text_value.to_string()}</TagItems>
                </TagList>
            </TagGroup>
        </div>
        <p>"Actions: " <span id="test-tg-actions-log">{move || actions.get().join(",")}</span></p>
        <div id="test-tg-replace">
            <TagGroup
                collection=tags(texts(&["Red", "Green"]))
                aria_label="Colors"
                selection_mode=SelectionMode::Single
                selection_behavior=SelectionBehavior::Replace
                on_action=move |key: Key| replace_actions.update(|a| a.push(key.to_string()))
            >
                <TagList>
                    <TagItems let:node>{node.text_value.to_string()}</TagItems>
                </TagList>
            </TagGroup>
        </div>
        <p>
            "Actions: "
            <span id="test-tg-replace-log">{move || replace_actions.get().join(",")}</span>
        </p>
        <button id="test-tg-insert" on:click=insert>
            "Insert item"
        </button>
        <div id="test-tg-ordered">
            <TagGroup
                collection=tags(ordered.into())
                aria_label="Categories"
                on_remove=remove_ordered
            >
                <TagList>
                    <TagItems let:node>
                        <span data-tag-label>{node.text_value.to_string()}</span> <TagRemoveButton>"x"</TagRemoveButton>
                    </TagItems>
                </TagList>
            </TagGroup>
        </div>
        <div id="test-tg-rtl">
            <I18nProvider locale=rtl>
                <TagGroup collection=tags(texts(&["One", "Two", "Three"])) aria_label="RTL">
                    <TagList>
                        <TagItems let:node>{node.text_value.to_string()}</TagItems>
                    </TagList>
                </TagGroup>
            </I18nProvider>
        </div>
    }
}
