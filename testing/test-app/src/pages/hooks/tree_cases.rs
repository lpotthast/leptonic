use std::collections::HashSet;

use leptonic::{
    hooks::{
        GridListData, IntoAttrs, SelectionMode, UseTreeInput, UseTreeItemInput, UseTreeItemReturn,
        UseTreeStateInput,
        collections::{
            CollectionOptions, DisabledBehavior, Key, Selection, SelectionOptions, use_collection,
        },
        use_button, use_tree, use_tree_item, use_tree_state,
    },
    utils::{
        CapturedElement, ValueBinding,
        i18n::{I18nProvider, Locale},
    },
};
use leptos::prelude::*;

use crate::pages::atoms::listbox::describe_selection;

/// react-aria-components' `AriaTreeTests` tree: Photos, Projects (Projects-1 (Projects-1A),
/// Projects-2, Projects-3), School (Homework-1 (Homework-1A), Homework-2, Homework-3); School
/// is disabled. With `add_notes`, a "Notes" item without children follows, and `#test-tc-{id}-
/// add-child` gives it one.
///
/// Shows the selection in `#test-tc-{id}-selection` and the expanded items in
/// `#test-tc-{id}-expanded` (bound app state: `#test-tc-{id}-collapse-all` collapses all).
#[component]
fn TestTree(
    label: &'static str,
    id: &'static str,
    selection_mode: SelectionMode,
    disabled_behavior: DisabledBehavior,
) -> impl IntoView {
    let notes_child = RwSignal::new(false);
    let collection = use_collection(move |b| {
        b.item("photos", "Photos");
        b.item("projects", "Projects").children(|c| {
            c.item("projects-1", "Projects-1").children(|c| {
                c.item("projects-1A", "Projects-1A");
            });
            c.item("projects-2", "Projects-2");
            c.item("projects-3", "Projects-3");
        });
        b.item("school", "School").children(|c| {
            c.item("homework-1", "Homework-1").children(|c| {
                c.item("homework-1A", "Homework-1A");
            });
            c.item("homework-2", "Homework-2");
            c.item("homework-3", "Homework-3");
        });
        if notes_child.get() {
            b.item("notes", "Notes").children(|c| {
                c.item("draft", "Draft");
            });
        } else {
            b.item("notes", "Notes");
        }
    });
    let expanded = RwSignal::new(HashSet::<Key>::new());
    let selection = RwSignal::new(String::new());
    let state = use_tree_state(UseTreeStateInput {
        collection,
        selection: SelectionOptions {
            selection_mode: Signal::stored(selection_mode),
            disabled_keys: Signal::stored(HashSet::from([Key::from("school")])),
            disabled_behavior,
            on_selection_change: Some(Callback::new(move |s: Selection| {
                selection.set(describe_selection(&s));
            })),
            ..SelectionOptions::default()
        },
        default_expanded_keys: HashSet::new(),
        expanded_keys: Some(ValueBinding::from(expanded)),
        on_expanded_change: None,
    });
    let tree = use_tree(UseTreeInput {
        aria_label: label.into(),
        state,
        element: CapturedElement::new(),
        id: None,
        aria_labelledby: None,
        options: CollectionOptions::default(),
        on_action: None,
    });
    let data = tree.data;
    let visible = state.list.collection;
    let expanded_text = move || {
        let mut keys: Vec<String> = expanded.get().iter().map(ToString::to_string).collect();
        keys.sort();
        keys.join(",")
    };

    view! {
        <div {..tree.props.into_attrs()}>
            <For
                each=move || visible.with(|c| c.items().map(|n| n.key.clone()).collect::<Vec<_>>())
                key=Clone::clone
                let:key
            >
                <TreeItem tree=data.clone() key=key />
            </For>
        </div>
        <div>
            "Selection: " <span id=format!("test-tc-{id}-selection")>{selection}</span>
            " Expanded: " <span id=format!("test-tc-{id}-expanded")>{expanded_text}</span>
        </div>
        <button id=format!("test-tc-{id}-collapse-all") on:click=move |_| expanded.set(HashSet::new())>
            "Collapse all"
        </button>
        <button id=format!("test-tc-{id}-add-child") on:click=move |_| notes_child.set(true)>
            "Add a child to Notes"
        </button>
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
    } = use_tree_item(UseTreeItemInput { tree, key });
    let (attrs, styles) = item.row_props.into_parts();
    let button = move || {
        has_child_items.get().then(|| {
            let (attrs, styles) = use_button(expand_button.clone()).props.into_parts();
            view! { <button {..attrs} {..expand_button_attrs.clone()} style=styles>"›"</button> }
        })
    };
    view! {
        <div {..attrs} style=styles>
            <div {..item.grid_cell_props.into_attrs()}>{button} <span>{text}</span></div>
        </div>
    }
}

/// Trees from the tree hooks with a disabled item (react-aria-components' `AriaTreeTests`):
/// single selection where disabled items can still be focused and expanded
/// (`DisabledBehavior::Selection`), one where they can't be used at all, and a right-to-left
/// tree. Each follows a "Before" button (`#test-tc-before-{id}`).
#[component]
pub fn PageHookTreeCases() -> impl IntoView {
    let rtl: Locale = "ar-AE".parse().expect("a valid locale");
    view! {
        <div id="test-page-hook-tree-cases">
            <h1>"Tree cases"</h1>
            <button id="test-tc-before-selection">"Before"</button>
            <TestTree
                label="Selection tree"
                id="selection"
                selection_mode=SelectionMode::Single
                disabled_behavior=DisabledBehavior::Selection
            />
            <button id="test-tc-before-all">"Before"</button>
            <TestTree
                label="Disabled tree"
                id="all"
                selection_mode=SelectionMode::Single
                disabled_behavior=DisabledBehavior::All
            />
            <button id="test-tc-before-rtl">"Before"</button>
            <div dir="rtl">
                <I18nProvider locale=rtl>
                    <TestTree
                        label="RTL tree"
                        id="rtl"
                        selection_mode=SelectionMode::None
                        disabled_behavior=DisabledBehavior::All
                    />
                </I18nProvider>
            </div>
        </div>
    }
}
