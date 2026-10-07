use std::collections::HashSet;

use leptonic::{
    atoms::button::Button,
    hooks::{
        IntoAttrs, SelectionMode, TagGroupData, UseTagGroupInput, UseTagGroupReturn, UseTagInput,
        UseTagReturn,
        collections::{
            Key, Selection, SelectionOptions, UseListStateInput, use_list_collection,
            use_list_state,
        },
        use_button, use_tag, use_tag_group,
    },
    utils::CapturedElement,
};
use leptos::prelude::*;

const ALL_TAGS: [&str; 5] = ["Rust", "Leptos", "WebAssembly", "Accessibility", "CSS"];

#[component]
pub fn TagDemo() -> impl IntoView {
    // App state: the tags, and the selected ones.
    let tags = RwSignal::new(ALL_TAGS.to_vec());
    let selection = RwSignal::new(Selection::default());

    let collection =
        use_list_collection(tags.into(), |tag| Key::from(*tag), |tag| (*tag).to_owned());
    let state = use_list_state(UseListStateInput {
        collection,
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            selection: Some(selection.into()),
            ..SelectionOptions::default()
        },
    });
    let UseTagGroupReturn {
        grid_props,
        label_props,
        data,
        ..
    } = use_tag_group(UseTagGroupInput {
        has_label: true.into(),
        // Removing is up to you: drop the keys from your data.
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

    let status = move || {
        let tags = match tags.with(Vec::len) {
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
        format!("{tags}. Selected: {selected}.")
    };

    view! {
        <span {..label_props.into_attrs()} class="demo-tag-group-label">"Technologies"</span>
        <div {..grid_props.into_attrs()} class="demo-tag-group">
            <For each=move || tags.get() key=|tag| *tag let:tag>
                <Tag group=data.clone() key=Key::from(tag)/>
            </For>
        </div>
        <p class="demo-status">{status}</p>
        <div class="demo-controls">
            <Button on_press=move |_| tags.set(ALL_TAGS.to_vec()) classes="demo-btn">"Restore all"</Button>
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
        is_focus_visible,
        ..
    } = use_tag(UseTagInput { group, key });
    let (attrs, styles) = row_props.into_parts();
    // Only present when the group allows removing tags.
    let remove = remove_button.map(|input| {
        let (attrs, styles) = use_button(input).props.into_parts();
        view! {
            <button {..attrs} class="demo-tag-remove" style=styles>
                <span aria-hidden="true">"\u{00d7}"</span>
            </button>
        }
    });

    view! {
        <div {..attrs} data-focus-visible=move || is_focus_visible.get().then_some("") class="demo-tag" style=styles>
            <div {..grid_cell_props.into_attrs()} class="demo-tag-cell">{text} {remove}</div>
        </div>
    }
}
