// Upstream: react-aria/src/tree/useTreeItem.ts @ 99e6102368
// Upstream: react-aria-components/test/Tree.test.tsx @ 99e6102368
// Upstream: react-aria-components/test/Tree.browser.test.tsx @ 99e6102368
use leptos::prelude::*;

use crate::{
    hooks::{
        button::use_button::UseButtonInput,
        collections::Key,
        gridlist::{
            FocusMode, GridListData, UseGridListItemInput, UseGridListItemReturn, grid_list_row_id,
            use_grid_list_item,
        },
        interactions::{ContextMenuEvent, use_press::PressEvent},
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
    /// What gets focus when the item is focused: the row, or its first focusable child.
    pub focus_mode: FocusMode,
    /// Let ArrowUp/ArrowDown move between items while a child has focus, also with
    /// `KeyboardNavigationBehavior::Tab`.
    pub allows_arrow_navigation: bool,
    /// Called when a context menu is requested on the item (right click, Shift+F10, the context
    /// menu key).
    pub on_context_menu: Option<Callback<ContextMenuEvent>>,
}

/// The return value of [`use_tree_item`].
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
    crate::metrics::subject!();
    let UseTreeItemInput {
        tree,
        key,
        focus_mode,
        allows_arrow_navigation,
        on_context_menu,
    } = input;
    let expansion = tree.tree;
    let selection = tree.state.selection;
    let row_id = grid_list_row_id(&tree.id, &key);
    let button_id = crate::utils::id::use_id("tree-expand");
    let item = use_grid_list_item(UseGridListItemInput {
        list: tree,
        key: key.clone(),
        focus_mode,
        allows_arrow_navigation,
        on_context_menu,
    });
    let is_disabled = item.is_disabled;
    let has_child_items = item.has_child_rows;

    let key = StoredValue::new(key);
    // A memo: the expanded keys change with every item expanded or collapsed.
    let is_expanded: Signal<bool> = Memo::new(move |_| {
        expansion.is_some_and(|expansion| key.with_value(|k| expansion.is_expanded(k)))
    })
    .into();
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
        aria_labelledby: Signal::stored(crate::utils::id::join_ids([
            Some(button_id.clone()),
            Some(row_id.clone()),
        ])),
        exclude_from_tab_order: Signal::stored(true),
        prevent_focus_on_press: true.into(),
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
        element_type: crate::hooks::button::ButtonElementType::Button,
        button_type: crate::hooks::button::ButtonType::Button,
        is_disabled: Signal::stored(false),
        is_pending: Signal::stored(false),
        allow_focus_when_disabled: Signal::stored(false),
        auto_focus: false,
        href: Signal::stored(None),
        target: Signal::stored(crate::hooks::link::LinkTarget::Same),
        rel: Vec::new(),
        form: crate::hooks::button::ButtonFormAttributes::default(),
        aria_haspopup: Signal::stored(None),
        aria_expanded: Signal::stored(None),
        aria_controls: Signal::stored(None),
        aria_describedby: Signal::stored(None),
        aria_pressed: Signal::stored(None),
        aria_checked: Signal::stored(None),
        role: None,
        aria_current: Signal::stored(None),
        aria_disabled: Signal::stored(None),
        on_press_start: None,
        on_press_end: None,
        on_press_up: None,
        on_press_change: None,
        long_press: None,
        on_hover_start: None,
        on_hover_end: None,
        on_hover_change: None,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
        on_key_down: None,
        on_key_up: None,
        shortcuts: None,
        on_context_menu: None,
    };

    UseTreeItemReturn {
        item,
        expand_button,
        expand_button_attrs: prevent_focus_attr(),
        is_expanded,
        has_child_items,
    }
}
