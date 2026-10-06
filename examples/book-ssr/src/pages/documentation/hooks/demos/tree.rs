use std::collections::HashSet;

use leptonic::{
    hooks::{
        GridListData, IntoAttrs, SelectionMode, UseTreeInput, UseTreeItemInput, UseTreeItemReturn,
        UseTreeStateInput,
        collections::{Key, Selection, SelectionOptions, use_collection},
        use_button, use_tree, use_tree_item, use_tree_state,
    },
    utils::CapturedElement,
};
use leptos::prelude::*;

#[component]
pub fn TreeDemo() -> impl IntoView {
    let selected = RwSignal::new(String::from("none"));

    // Tree items are nested with `.children(..)`.
    let collection = use_collection(|b| {
        b.item("documents", "Documents").children(|c| {
            c.item("resume", "resume.pdf");
            c.item("cover-letter", "cover-letter.docx");
        });
        b.item("photos", "Photos").children(|c| {
            c.item("vacation", "Vacation").children(|c| {
                c.item("beach", "beach.jpg");
                c.item("sunset", "sunset.jpg");
            });
            c.item("profile", "profile.jpg");
        });
        b.item("notes", "notes.txt");
    });
    let state = use_tree_state(UseTreeStateInput {
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Single),
            on_selection_change: Some(Callback::new(move |selection: Selection| {
                selected.set(match selection {
                    Selection::Keys(keys) => keys
                        .iter()
                        .next()
                        .map_or_else(|| "none".to_owned(), ToString::to_string),
                    Selection::All => "all".to_owned(),
                });
            })),
            ..SelectionOptions::default()
        },
        default_expanded_keys: HashSet::from([Key::from("documents")]),
        ..UseTreeStateInput::new(collection)
    });
    let tree = use_tree(UseTreeInput {
        aria_label: "Files".into(),
        ..UseTreeInput::new(state, CapturedElement::new())
    });
    let data = tree.data;
    // The visible items, in order: children of collapsed items are left out.
    let visible = state.list.collection;

    view! {
        <div {..tree.props.into_attrs()} class="demo-tree">
            <For
                each=move || visible.with(|c| c.items().map(|node| node.key.clone()).collect::<Vec<_>>())
                key=Clone::clone
                let:key
            >
                <TreeItem tree=data.clone() key/>
            </For>
        </div>
        <p class="demo-status">"Selected: "{selected}</p>
    }
}

#[component]
fn TreeItem(tree: GridListData, key: Key) -> impl IntoView {
    let text = tree
        .state
        .collection
        .with_untracked(|c| c.get(&key).map(|node| node.text_value.to_string()))
        .unwrap_or_default();
    let UseTreeItemReturn {
        item,
        expand_button,
        expand_button_label,
        is_expanded,
        has_child_items,
    } = use_tree_item(UseTreeItemInput { tree, key });
    let (attrs, styles) = item.row_props.into_parts();
    // Parents get a button expanding or collapsing them; leaves an empty spacer of the same width.
    let toggle = if has_child_items {
        let (attrs, styles) = use_button(expand_button).props.into_parts();
        view! {
            <button {..attrs} aria-label=expand_button_label class="demo-tree-item-toggle" style=styles>
                <span aria-hidden="true">{move || if is_expanded.get() { "\u{25be}" } else { "\u{25b8}" }}</span>
            </button>
        }
        .into_any()
    } else {
        view! { <span class="demo-tree-item-toggle"></span> }.into_any()
    };

    view! {
        <div {..attrs} data-focus-visible=move || item.is_focus_visible.get().then_some("") class="demo-tree-item" style=styles>
            <div {..item.grid_cell_props.into_attrs()} class="demo-tree-item-content">{toggle}{text}</div>
        </div>
    }
}
