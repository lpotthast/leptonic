// Upstream: react-aria/src/dnd/useDraggableCollection.ts @ 99e6102368
use leptos::prelude::*;

use super::{use_draggable_collection_state::DraggableCollectionState, utils};
use crate::utils::CapturedElement;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// A collection whose items can be dragged: while its items are dragged, it is the collection
/// drags come from (drops into it are internal: reorders and moves).
pub fn use_draggable_collection(state: DraggableCollectionState, element: CapturedElement) {
    Effect::new(move || {
        if state.dragging_keys.with(|keys| !keys.is_empty())
            && let Some(element) = element.get()
        {
            utils::set_dragging_collection(Some((*element).clone()));
        }
    });
}
