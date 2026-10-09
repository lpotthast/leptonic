use std::collections::HashSet;

use leptonic::{
    CapturedElement, IntoAttrs,
    hooks::{
        button::use_button,
        collections::{
            Key, ListState, SelectionOptions, UseListCollectionInput, UseListStateInput,
            use_list_collection, use_list_state,
        },
        tag::{
            TagGroupData, UseTagGroupInput, UseTagGroupReturn, UseTagInput, UseTagReturn, use_tag,
            use_tag_group,
        },
    },
};
use leptos::prelude::*;

/// A tag group built from the hooks: removable tags (Delete/Backspace or their remove button).
/// The remaining tags are listed in `#test-tg-tags`.
#[component]
pub fn PageHookTagGroup() -> impl IntoView {
    let tags = RwSignal::new(vec!["News", "Travel", "Gaming", "Shopping"]);
    let collection = use_list_collection(UseListCollectionInput {
        items: tags.into(),
        key: |tag| Key::from(*tag),
        text_value: |tag| (*tag).to_owned(),
    });
    let state: ListState = use_list_state(UseListStateInput {
        collection,
        selection: SelectionOptions::default(),
    });
    let UseTagGroupReturn {
        grid_props,
        label_props,
        data,
        ..
    } = use_tag_group(UseTagGroupInput {
        has_label: true.into(),
        on_remove: Some(Callback::new(move |keys: HashSet<Key>| {
            tags.update(|tags| tags.retain(|tag| !keys.contains(&Key::from(*tag))));
        })),
        state,
        element: CapturedElement::new(),
        id: None,
        aria_label: MaybeProp::default(),
        aria_labelledby: None,
        aria_describedby: None,
        keyboard_delegate: None,
        on_action: None,
    });

    view! {
        <div id="test-page-hook-tag-group">
            <h1>"Tag group"</h1>
            <button id="test-tg-before">"Before"</button>
            <span {..label_props.into_attrs()}>"Categories"</span>
            <div {..grid_props.into_attrs()}>
                <For each=move || tags.get() key=|tag| *tag let:tag>
                    <Tag group=data.clone() key=Key::from(tag) />
                </For>
            </div>
            <button id="test-tg-after">"After"</button>
            <div>"Tags: " <span id="test-tg-tags">{move || tags.get().join(",")}</span></div>
        </div>
    }
}

#[component]
fn Tag(group: TagGroupData, key: Key) -> impl IntoView {
    let text = key.to_string();
    let UseTagReturn {
        row_props,
        grid_cell_props,
        remove_button,
        ..
    } = use_tag(UseTagInput { group, key });
    let (attrs, styles) = row_props.into_parts();
    let remove = remove_button.map(|input| {
        let (attrs, styles) = use_button(input).props.into_parts();
        view! { <button {..attrs} style=styles>"×"</button> }
    });

    view! {
        <div {..attrs} style=styles>
            <div {..grid_cell_props.into_attrs()}>{text} {remove}</div>
        </div>
    }
}
