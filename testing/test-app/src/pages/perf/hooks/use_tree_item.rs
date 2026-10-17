//! Hook tree items (`use_tree_item` on plain elements) in a hook tree, for the performance test of
//! `use_tree_item` (`leptonic/tests/perf_tests/hooks/use_tree_item.rs`): `use_tree` on a `div`
//! (`[role=treegrid]`) with a `use_tree_item` per visible item (a `div` with an expand button,
//! `use_button` with the item's `expand_button`, for items with children) of a tree of [`SMALL`]
//! parents "Parent <p>" with a child "Child <p>" each, all collapsed, without selection (a press on
//! a parent expands or collapses it), after a plain button (`#perf-before`) to tab from, mounted at
//! first.
//!
//! Its controls ([`Control`]) mount and unmount the tree (`mount`, `mount-selectable`: with
//! multiple selection; `unmount`), set its size while unmounted (`small`, `large`: [`LARGE`]
//! parents), add and remove a parent (mounting and unmounting its item), expand and collapse every
//! parent, prepare the live announcer (`prepare-announcer`: its first announcement waits for its
//! live regions) and read how many items are selected (`selected`).

use std::collections::HashSet;

use leptonic::{
    Assertiveness, CapturedElement, announce, flag,
    hooks::{
        button::use_button,
        collections::{
            CollectionMemo, CollectionOptions, Key, SelectionMode, SelectionOptions, use_collection,
        },
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

/// Every parent of a tree of `parents` parents.
fn all_parents(parents: usize) -> HashSet<Key> {
    (0..parents).map(|p| Key::from(format!("p{p}"))).collect()
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
    let selection_mode = RwSignal::new(SelectionMode::None);
    let expanded = RwSignal::new(HashSet::<Key>::new());
    let state = use_tree_state(UseTreeStateInput {
        collection: tree_collection(parents),
        selection: SelectionOptions {
            selection_mode: selection_mode.into(),
            ..SelectionOptions::default()
        },
        default_expanded_keys: HashSet::new(),
        expanded_keys: Some(expanded.into()),
        on_expanded_change: None,
    });
    let controls = vec![
        Control::action("mount", move || {
            selection_mode.set(SelectionMode::None);
            mounted.set(true);
        }),
        Control::action("mount-selectable", move || {
            selection_mode.set(SelectionMode::Multiple);
            mounted.set(true);
        }),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || parents.set(SMALL)),
        Control::action("large", move || parents.set(LARGE)),
        Control::action("add-parent", move || parents.update(|count| *count += 1)),
        Control::action("remove-parent", move || parents.update(|count| *count -= 1)),
        Control::action("expand-all", move || {
            expanded.set(all_parents(parents.get_untracked()));
        }),
        Control::action("collapse-all", move || expanded.set(HashSet::new())),
        Control::action("prepare-announcer", || announce("", Assertiveness::Polite)),
        Control::read("selected", move || {
            let selected = untrack(|| state.list.selection.selected_keys().len());
            f64::from(u32::try_from(selected).unwrap_or(u32::MAX))
        }),
    ];
    view! {
        <h1>"use_tree_item performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <PerfTree state />
        </Show>
    }
}
