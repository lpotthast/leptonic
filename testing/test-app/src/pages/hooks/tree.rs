use leptonic::hooks::collections::CollectionOptions;
use leptonic::hooks::collections::SelectionOptions;
use leptonic::{
    hooks::{
        GridListData, IntoAttrs, UseTreeInput, UseTreeItemInput, UseTreeItemReturn,
        UseTreeStateInput,
        collections::{Key, use_collection},
        use_button, use_tree, use_tree_item, use_tree_state,
    },
    utils::CapturedElement,
};
use leptos::prelude::*;
use std::collections::HashSet;

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
        on_expanded_change: None,
    });
    let tree = use_tree(UseTreeInput {
        aria_label: "Files".into(),
        state,
        element: CapturedElement::new(),
        id: None,
        aria_labelledby: None,
        options: CollectionOptions::default(),
        on_action: None,
    });
    let data = tree.data;
    let visible = state.list.collection;

    view! {
        <div id="test-page-hook-tree">
            <h1>"Tree"</h1>
            <button id="test-tree-before">"Before"</button>
            <div {..tree.props.into_attrs()}>
                <For
                    each=move || visible.with(|c| c.items().map(|n| n.key.clone()).collect::<Vec<_>>())
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
        expand_button_label,
        has_child_items,
        ..
    } = use_tree_item(UseTreeItemInput { tree, key });
    let (attrs, styles) = item.row_props.into_parts();
    let button = has_child_items.then(|| {
        let (attrs, styles) = use_button(expand_button).props.into_parts();
        view! { <button {..attrs} aria-label=expand_button_label style=styles>"›"</button> }
    });

    view! {
        <div {..attrs} style=styles>
            <div {..item.grid_cell_props.into_attrs()}>{button} {text}</div>
        </div>
    }
}
