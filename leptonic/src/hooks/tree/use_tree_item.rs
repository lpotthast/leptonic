// Upstream: react-aria/src/tree/useTreeItem.ts @ 99e6102368
use leptos::prelude::*;

use crate::{
    hooks::{
        FocusMode,
        button::use_button::UseButtonInput,
        collections::Key,
        gridlist::{
            GridListData, UseGridListItemInput, UseGridListItemReturn, grid_list_row_id,
            use_grid_list_item,
        },
        interactions::use_press::PressEvent,
    },
    utils::{
        focusability::{PreventFocusAttr, prevent_focus_attr},
        intl_strings::{TreeStrings, use_localized_strings},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The expand button is configured, not rendered: `expand_button` is the `UseButtonInput` for
//   `use_button` (with its label). Its `data-leptonic-prevent-focus` attribute comes separately
//   (`expand_button_attrs`), as `UseButtonInput` takes no extra attributes.
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
    /// children). Labelled "Expand" or "Collapse", plus the item.
    pub expand_button: UseButtonInput,
    /// Spread onto the expand button besides `use_button`'s attributes: it keeps focus walks
    /// (ArrowRight on an expanded row) off the button (`data-leptonic-prevent-focus`).
    pub expand_button_attrs: PreventFocusAttr,
    pub is_expanded: Signal<bool>,
    /// Whether the item has children (follows the collection).
    pub has_child_items: Signal<bool>,
}

/// An item of a tree: a grid list row with `aria-expanded`, `aria-level`, `aria-posinset` and
/// `aria-setsize`, plus an expand button.
pub fn use_tree_item(input: UseTreeItemInput) -> UseTreeItemReturn {
    let UseTreeItemInput { tree, key } = input;
    let expansion = tree.tree;
    let selection = tree.state.selection;
    let row_id = grid_list_row_id(&tree.id, &key);
    let button_id = crate::utils::id::use_id("tree-expand");
    let has_child_items = {
        let collection = tree.state.collection;
        let key = key.clone();
        Memo::new(move |_| collection.with(|c| c.get(&key).is_some_and(|n| n.has_child_nodes)))
    };
    let item = use_grid_list_item(UseGridListItemInput {
        list: tree,
        key: key.clone(),
        focus_mode: FocusMode::Row,
        allows_arrow_navigation: false,
        on_context_menu: None,
    });
    let is_disabled = item.is_disabled;

    let key = StoredValue::new(key);
    let is_expanded = Signal::derive(move || {
        expansion.is_some_and(|expansion| key.with_value(|k| expansion.is_expanded(k)))
    });
    let strings = use_localized_strings::<TreeStrings>();
    let expand_button = UseButtonInput {
        // Labelled by its own label ("Expand"/"Collapse") and the item.
        id: Some(button_id.clone()),
        aria_label: MaybeProp::derive(move || {
            let strings = strings.read();
            Some(if is_expanded.get() {
                strings.collapse()
            } else {
                strings.expand()
            })
        }),
        aria_labelledby: Signal::stored(Some(format!("{button_id} {row_id}"))),
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
        expand_button_attrs: prevent_focus_attr(),
        is_expanded,
        has_child_items: has_child_items.into(),
    }
}
