// Upstream: react-aria/src/tree/useTree.ts @ 99e6102368
// Upstream: react-aria-components/test/Tree.test.tsx @ 99e6102368
// Upstream: react-aria-components/test/AriaTree.test-util.tsx @ 99e6102368
use leptos::prelude::*;

use super::TreeState;
use crate::{
    CapturedElement,
    hooks::{
        collections::{CollectionOptions, Key, ListLayout},
        gridlist::{
            KeyboardNavigationBehavior, UseGridListInput, UseGridListReturn, use_grid_list,
        },
    },
    utils::aria::AriaRole,
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
// - A custom keyboard delegate: trees use the grid list's (type-ahead, Home/End, paging).
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
    /// Ids of elements labelling the tree.
    pub aria_labelledby: Signal<Option<String>>,
    pub options: CollectionOptions,
    /// Called with the key of an activated item. Without it (and without selection), pressing a
    /// parent item toggles it.
    pub on_action: Option<Callback<Key>>,
    /// How the keyboard moves between the items' interactive children: arrow keys, or Tab.
    pub keyboard_navigation_behavior: KeyboardNavigationBehavior,
    /// Select items when a press ends instead of when it starts (e.g. for draggable items).
    pub should_select_on_press_up: bool,
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
        keyboard_navigation_behavior,
        should_select_on_press_up,
    } = input;
    let mut tree = use_grid_list(UseGridListInput {
        id,
        aria_label,
        aria_labelledby,
        options,
        on_action,
        tree: Some(state.expansion),
        state: state.list,
        element,
        layout: ListLayout::Stack,
        keyboard_delegate: None,
        keyboard_navigation_behavior,
        should_select_on_press_up,
    });
    tree.props.role = Signal::stored(AriaRole::Treegrid);
    tree
}
