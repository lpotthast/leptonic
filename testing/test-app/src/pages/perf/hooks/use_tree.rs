//! A tree built from the tree hooks on plain elements, for the performance test of `use_tree`
//! (`leptonic/tests/perf_tests/hooks/use_tree.rs`): `use_tree` on a `div` (`[role=treegrid]`) with
//! a `use_tree_item` per visible item (a `div` with an expand button, `use_button` with the item's
//! `expand_button`, for items with children) of a tree of [`SMALL`] parents "Parent <p>" with a
//! child "Child <p>" each, all collapsed, without selection, between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. The state lives outside
//! the mounted part, so mounting measures `use_tree` and its items.
//!
//! Its controls ([`Control`]) mount and unmount the tree and set its size while unmounted
//! (`small`, `large`: [`LARGE`] parents).

use std::collections::HashSet;

use leptonic::{
    CapturedElement, flag,
    hooks::{
        button::use_button,
        collections::{CollectionMemo, CollectionOptions, Key, SelectionOptions, use_collection},
        gridlist::{FocusMode, GridListData, KeyboardNavigationBehavior},
        tree::{
            TreeState, UseTreeInput, UseTreeItemInput, UseTreeItemReturn, UseTreeStateInput,
            use_tree, use_tree_item, use_tree_state,
        },
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The parent items of the small tree (10 items).
const SMALL: usize = 5;

/// The parent items of the large tree (1,000 items).
const LARGE: usize = 500;

/// `parents` parent items ("Parent <p>", key `p<p>`), each with a child item ("Child <p>", key
/// `c<p>`).
fn tree_collection(parents: RwSignal<usize>) -> CollectionMemo {
    use_collection(move |b| {
        for p in 0..parents.get() {
            let _ = b
                .item(format!("p{p}"), format!("Parent {p}"))
                .children(|c| {
                    c.item(format!("c{p}"), format!("Child {p}"));
                });
        }
    })
}

/// An item: `use_tree_item` on a `div` (its pressed state in `data-pressed`), with its expand
/// button while it has children.
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
    let is_pressed = item.is_pressed;
    let (attrs, styles) = item.row.attrs();
    let button = move || {
        has_child_items.get().then(|| {
            let (attrs, styles) = use_button(expand_button.clone()).button.attrs();
            view! {
                <button {..attrs} {..expand_button_attrs.clone()} style=styles>
                    "›"
                </button>
            }
        })
    };
    view! {
        <div {..attrs} style=styles data-pressed=flag(is_pressed)>
            <div {..item.cell.attrs()}>{button} <span>{text}</span></div>
        </div>
    }
}

/// The tree: `use_tree` on a `div`, with an item per visible item.
#[component]
fn PerfTree(state: TreeState) -> impl IntoView {
    let tree = use_tree(UseTreeInput {
        aria_label: "Perf tree".into(),
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
        <div {..tree.grid_list.attrs()}>
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
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let parents = RwSignal::new(SMALL);
    let state = use_tree_state(UseTreeStateInput {
        collection: tree_collection(parents),
        selection: SelectionOptions::default(),
        default_expanded_keys: HashSet::new(),
        expanded_keys: None,
        on_expanded_change: None,
    });
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || parents.set(SMALL)),
        Control::action("large", move || parents.set(LARGE)),
    ];
    view! {
        <h1>"use_tree performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <PerfTree state />
        </Show>
        <button id="perf-after">"After"</button>
    }
}
