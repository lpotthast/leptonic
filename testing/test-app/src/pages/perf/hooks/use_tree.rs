//! A tree built from the tree hooks on plain elements, for the performance tests of `use_tree`
//! and `use_tree_item` (`leptonic/tests/perf_tests/hooks/use_tree.rs`, `.../use_tree_item.rs`,
//! whose fixture this is too): `use_tree` on a `div` (`[role=treegrid]`) with a `use_tree_item`
//! per visible item (a `div` with an expand button, `use_button` with the item's
//! `expand_button`, for items with children) of the `use_tree_state` fixture's tree
//! ([`super::use_tree_state`]: [`SMALL`] parents "Parent <p>" with a child "Child <p>" each, all
//! collapsed), without selection (a press on a parent expands or collapses it), between two plain
//! buttons (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. The state lives
//! outside the mounted part, so mounting measures `use_tree` and its items.
//!
//! Its controls ([`Control`]) mount and unmount the tree (`mount`, `mount-selectable`: with
//! multiple selection; `unmount`), set its size while unmounted (`small`, `large`: [`LARGE`]
//! parents), add and remove a parent, expand and collapse every parent, prepare the live
//! announcer (`prepare-announcer`: its first announcement waits for its live regions) and read
//! how many items are selected (`selected`).

use std::collections::HashSet;

use leptonic::{
    Assertiveness, CapturedElement, IntoAttrs, announce, flag,
    hooks::{
        button::use_button,
        collections::{CollectionOptions, Key, SelectionMode, SelectionOptions},
        gridlist::{FocusMode, GridListData, KeyboardNavigationBehavior},
        tree::{
            TreeState, UseTreeInput, UseTreeItemInput, UseTreeItemReturn, UseTreeStateInput,
            use_tree, use_tree_item, use_tree_state,
        },
    },
};
use leptos::prelude::*;

use super::use_tree_state::{LARGE, SMALL, all_parents, tree_collection};
use crate::pages::perf::{Control, PerfControls};

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
    let (attrs, styles) = item.row_props.into_parts();
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
        <div {..attrs} style=styles data-pressed=flag(is_pressed)>
            <div {..item.grid_cell_props.into_attrs()}>{button} <span>{text}</span></div>
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
        <h1>"use_tree performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <PerfTree state />
        </Show>
        <button id="perf-after">"After"</button>
    }
}
