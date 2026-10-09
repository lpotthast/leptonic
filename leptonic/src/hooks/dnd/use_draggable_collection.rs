// Upstream: react-aria/src/dnd/useDraggableCollection.ts @ 99e6102368
// Upstream: react-aria/test/dnd/useDraggableCollection.test.js @ 99e6102368
use leptos::prelude::*;

use super::{use_draggable_collection_state::DraggableCollectionState, utils};
use crate::CapturedElement;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Takes the collection element as a `CapturedElement` in its input (react-aria: a ref).
//
// =============================================================================

/// Input of [`use_draggable_collection`].
#[derive(Debug, Clone, Copy)]
pub struct UseDraggableCollectionInput {
    pub state: DraggableCollectionState,
    /// The collection element (captured by the collection hook's props).
    pub element: CapturedElement,
}

/// A collection whose items can be dragged: while its items are dragged, it is the collection
/// drags come from (drops into it are internal: reorders and moves).
pub fn use_draggable_collection(input: UseDraggableCollectionInput) {
    let UseDraggableCollectionInput { state, element } = input;
    Effect::new(move || {
        if state.dragging_keys.with(|keys| !keys.is_empty())
            && let Some(element) = element.get()
        {
            utils::set_dragging_collection(Some((*element).clone()));
        }
    });
}
