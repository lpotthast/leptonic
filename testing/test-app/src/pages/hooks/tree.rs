use std::collections::HashSet;

use leptonic::{
    CapturedElement, IntoAttrs,
    hooks::{
        button::use_button,
        collections::{CollectionOptions, Key, SelectionOptions, use_collection},
        gridlist::{FocusMode, GridListData, KeyboardNavigationBehavior},
        tree::{
            UseTreeInput, UseTreeItemInput, UseTreeItemReturn, UseTreeStateInput, use_tree,
            use_tree_item, use_tree_state,
        },
    },
};
use leptos::prelude::*;

/// A file tree built from the tree hooks, all items collapsed:
/// Documents (Project (Report, Budget), CV), Photos (Cat), Notes.
#[component]
pub fn PageHookTree() -> impl IntoView {
    let collection = use_collection(|b| {
        b.item("documents", "Documents").children(|c| {
            c.item("project", "Project").children(|c| {
                c.item("report", "Report");
                c.item("budget", "Budget");
            });
            c.item("cv", "CV");
        });
        b.item("photos", "Photos").children(|c| {
            c.item("cat", "Cat");
        });
        b.item("notes", "Notes");
    });
    let state = use_tree_state(UseTreeStateInput {
        collection,
        selection: SelectionOptions::default(),
        default_expanded_keys: HashSet::new(),
        expanded_keys: None,
        on_expanded_change: None,
    });
    let tree = use_tree(UseTreeInput {
        aria_label: "Files".into(),
        state,
        element: CapturedElement::new(),
        id: None,
        aria_labelledby: Signal::stored(None),
        options: CollectionOptions::default(),
        on_action: None,
        keyboard_navigation_behavior: KeyboardNavigationBehavior::Arrow,
        should_select_on_press_up: false,
    });
    let data = tree.data;
    let visible = state.list.collection;

    view! {
        <div id="test-page-hook-tree">
            <h1>"Tree"</h1>
            <button id="test-tree-before">"Before"</button>
            <div {..tree.props.into_attrs()}>
                <For
                    each=move || {
                        visible.with(|c| c.items().map(|n| n.key.clone()).collect::<Vec<_>>())
                    }
                    key=Clone::clone
                    let:key
                >
                    <TreeItem tree=data.clone() key=key />
                </For>
            </div>
            <button id="test-tree-after">"After"</button>
        </div>
    }
}

#[component]
fn TreeItem(tree: GridListData, key: Key) -> impl IntoView {
    let text = tree
        .state
        .collection
        .with_untracked(|c| c.get(&key).map(|n| n.text_value.to_string()))
        .unwrap_or_default();
    let UseTreeItemReturn {
        item,
        expand_button,
        expand_button_attrs,
        has_child_items,
        ..
    } = use_tree_item(UseTreeItemInput {
        tree,
        key,
        focus_mode: FocusMode::Row,
        allows_arrow_navigation: false,
        on_context_menu: None,
    });
    let (attrs, styles) = item.row_props.into_parts();
    // Only items with children have one (an item can get children later).
    let button = move || {
        has_child_items.get().then(|| {
            let (attrs, styles) = use_button(expand_button.clone()).props.into_parts();
            view! {
                <button {..attrs} {..expand_button_attrs.clone()} style=styles>
                    "›"
                </button>
            }
        })
    };

    view! {
        <div {..attrs} style=styles>
            <div {..item.grid_cell_props.into_attrs()}>{button} <span>{text}</span></div>
        </div>
    }
}
