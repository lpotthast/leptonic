// Upstream: react-aria/src/tree/useTree.ts @ 99e6102368
use leptos::prelude::*;

use super::TreeState;
use crate::{
    hooks::{
        collections::{CollectionOptions, Key},
        gridlist::{UseGridListInput, UseGridListReturn, use_grid_list},
    },
    utils::{CapturedElement, aria::AriaRole},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
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

impl UseTreeInput {
    /// A tree for `state`, with all other settings at their defaults.
    pub fn new(state: TreeState, element: CapturedElement) -> Self {
        Self {
            state,
            element,
            id: None,
            aria_label: MaybeProp::default(),
            aria_labelledby: None,
            options: CollectionOptions::default(),
            on_action: None,
        }
    }
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
        aria_labelledby,
        options,
        on_action,
        tree: Some(state.expansion),
        ..UseGridListInput::new(state.list, element)
    });
    tree.props.role = Signal::stored(AriaRole::Treegrid);
    tree
}
