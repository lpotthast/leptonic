// Upstream: react-aria/src/tree/useTree.ts @ 99e6102368
use leptos::prelude::*;

use super::TreeState;
use crate::{
    hooks::{
        KeyboardNavigationBehavior,
        collections::{CollectionOptions, Key, ListLayout},
        gridlist::{UseGridListInput, UseGridListReturn, use_grid_list},
    },
    utils::{CapturedElement, aria::AriaRole},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Returns `use_grid_list`'s props and data (react-aria's `useTree` is `useGridList` with
//   `role="treegrid"`); the expansion state comes from the `TreeState` given.
//
// ## OMITTED FEATURES
// - The grid list settings besides `options` and `on_action` (keyboard navigation behavior,
//   selecting on press up, a custom keyboard delegate): trees use the grid list defaults.
//
// =============================================================================

/// Input of [`use_tree`].
#[derive(Clone)]
pub struct UseTreeInput {
    pub state: TreeState,
    /// The tree element; the hook's props capture it.
    pub element: CapturedElement,
    /// The element id. Generated when `None`.
    pub id: Option<String>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
    pub options: CollectionOptions,
    /// Called with the key of an activated item. Without it (and without selection), pressing a
    /// parent item toggles it.
    pub on_action: Option<Callback<Key>>,
}

/// A tree: a grid list (`role="treegrid"`) whose items can have children, expanded and
/// collapsed with ArrowRight/ArrowLeft or an expand button. Render one `use_tree_item` per
/// visible item (`state.list.collection`), in order.
pub fn use_tree(input: UseTreeInput) -> UseGridListReturn {
    let UseTreeInput {
        state,
        element,
        id,
        aria_label,
        aria_labelledby,
        options,
        on_action,
    } = input;
    let mut tree = use_grid_list(UseGridListInput {
        id,
        aria_label,
        aria_labelledby: Signal::stored(aria_labelledby),
        options,
        on_action,
        tree: Some(state.expansion),
        state: state.list,
        element,
        layout: ListLayout::Stack,
        keyboard_delegate: None,
        keyboard_navigation_behavior: KeyboardNavigationBehavior::default(),
        should_select_on_press_up: false,
    });
    tree.props.role = Signal::stored(AriaRole::Treegrid);
    tree
}
