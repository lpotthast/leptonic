use std::collections::HashSet;

use leptonic::{
    components::prelude::*,
    hooks::{
        IntoAttrs, SelectionMode, TagGroupData, UseTagGroupInput, UseTagGroupReturn, UseTagInput,
        UseTagReturn,
        collections::{
            Key, SelectionOptions, UseListStateInput, use_list_collection, use_list_state,
        },
        use_button, use_tag, use_tag_group,
    },
    utils::{CapturedElement, classes::Classes},
};
use leptos::prelude::*;

const ALL_TAGS: [&str; 5] = ["Rust", "Leptos", "WebAssembly", "Accessibility", "CSS"];

#[component]
pub fn TagDemo() -> impl IntoView {
    let tags = RwSignal::new(ALL_TAGS.to_vec());

    let collection =
        use_list_collection(tags.into(), |tag| Key::from(*tag), |tag| (*tag).to_owned());
    let state = use_list_state(UseListStateInput {
        collection,
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            ..SelectionOptions::default()
        },
    });
    let UseTagGroupReturn {
        grid_props,
        label_props,
        data,
        ..
    } = use_tag_group(UseTagGroupInput {
        has_label: true,
        // Removing is up to you: drop the keys from your data.
        on_remove: Some(Callback::new(move |keys: HashSet<Key>| {
            tags.update(|tags| tags.retain(|tag| !keys.contains(&Key::from(*tag))));
        })),
        ..UseTagGroupInput::new(state, CapturedElement::new())
    });

    view! {
        <span {..label_props.into_attrs()} class="demo-tag-group-label">"Technologies"</span>
        <div {..grid_props.into_attrs()} class="demo-tag-group">
            <For each=move || tags.get() key=|tag| *tag let:tag>
                <Tag group=data.clone() key=Key::from(tag)/>
            </For>
        </div>
        <p class="demo-caption">
            "Click tags to select them. Delete or Backspace removes the focused tag, or all selected tags."
        </p>
        <Button on_press=move |_| tags.set(ALL_TAGS.to_vec())>"Restore all"</Button>
    }
}

#[component]
fn Tag(group: TagGroupData, key: Key) -> impl IntoView {
    let text = key.to_string();
    let UseTagReturn {
        row_props,
        grid_cell_props,
        remove_button,
        is_focus_visible,
        ..
    } = use_tag(UseTagInput { group, key });
    let (attrs, styles) = row_props.into_parts();
    // Only present when the group allows removing tags.
    let remove = remove_button.map(|input| {
        let (attrs, styles) = use_button(input).props.into_parts();
        view! { <button {..attrs} class="demo-tag-remove" style=styles>"\u{00d7}"</button> }
    });

    view! {
        <div {..attrs} class=Classes::from("demo-tag").add_reactive("focus-visible", is_focus_visible) style=styles>
            <div {..grid_cell_props.into_attrs()} class="demo-tag-cell">{text} {remove}</div>
        </div>
    }
}
