// Upstream: react-aria/src/tree/useTreeItem.ts @ 99e6102368
use leptos::prelude::*;

use crate::hooks::{
    button::use_button::UseButtonInput,
    collections::Key,
    gridlist::{
        GridListData, UseGridListItemInput, UseGridListItemReturn, grid_list_row_id,
        use_grid_list_item,
    },
    interactions::use_press::PressEvent,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The expand button is configured, not rendered: `expand_button` is the `UseButtonInput` for
//   `use_button`. Its label ("Expand"/"Collapse") is English only.
//
// =============================================================================

/// Input of [`use_tree_item`].
#[derive(Debug, Clone)]
pub struct UseTreeItemInput {
    /// The tree (`use_tree`'s `data`).
    pub tree: GridListData,
    /// The item's key in the tree's collection.
    pub key: Key,
}

/// Return value of [`use_tree_item`].
pub struct UseTreeItemReturn {
    pub item: UseGridListItemReturn,
    /// The expand/collapse button's configuration, for `use_button` (render it for items with
    /// children).
    pub expand_button: UseButtonInput,
    /// The expand button's `aria-label` ("Expand" or "Collapse"); set it on the button.
    pub expand_button_label: Signal<&'static str>,
    pub is_expanded: Signal<bool>,
    pub has_child_items: bool,
}

/// An item of a tree: a grid list row with `aria-expanded`, `aria-level`, `aria-posinset` and
/// `aria-setsize`, plus an expand button.
pub fn use_tree_item(input: UseTreeItemInput) -> UseTreeItemReturn {
    let UseTreeItemInput { tree, key } = input;
    let expansion = tree.tree;
    let selection = tree.state.selection;
    let row_id = grid_list_row_id(&tree.id, &key);
    let button_id = crate::utils::id::use_id("tree-expand");
    let has_child_items = untrack(|| {
        tree.state
            .collection
            .with(|c| c.get(&key).is_some_and(|n| n.has_child_nodes))
    });
    let item = use_grid_list_item(UseGridListItemInput::new(tree, key.clone()));
    let is_disabled = item.is_disabled;

    let key = StoredValue::new(key);
    let is_expanded = Signal::derive(move || {
        expansion.is_some_and(|expansion| key.with_value(|k| expansion.is_expanded(k)))
    });
    let expand_button = UseButtonInput {
        // Labelled by its own label ("Expand"/"Collapse") and the item.
        id: Some(button_id.clone().into()),
        aria_labelledby: Some(format!("{button_id} {row_id}").into()),
        exclude_from_tab_order: Signal::stored(true),
        prevent_focus_on_press: true,
        on_press: Some(Callback::new(move |_: PressEvent| {
            if is_disabled.get_untracked() {
                return;
            }
            if let Some(expansion) = expansion {
                expansion.toggle_key(key.get_value());
            }
            selection.set_focused(true);
            selection.set_focused_key(Some(key.get_value()), None);
        })),
        ..UseButtonInput::default()
    };

    UseTreeItemReturn {
        item,
        expand_button,
        expand_button_label: Signal::derive(move || {
            if is_expanded.get() {
                "Collapse"
            } else {
                "Expand"
            }
        }),
        is_expanded,
        has_child_items,
    }
}
