// Upstream: react-aria/src/dnd/useVirtualDrop.ts @ 99e6102368
use leptos::prelude::*;

use super::{drag_manager::use_drag_session, messages, utils::use_drag_modality};
use crate::utils::use_description::use_description;
use crate::utils::intl_strings::{DndStrings, use_localized_strings};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Returns the description id; drop targets add it to their `aria-describedby` (react-aria
//   also adds an empty click handler, which only matters for React's synthetic events).
//
// =============================================================================

/// The description of a drop target during a keyboard or screen reader drag: how to drop.
pub fn use_virtual_drop() -> Signal<Option<String>> {
    let modality = use_drag_modality();
    let session = use_drag_session();
    let strings = use_localized_strings::<DndStrings>();
    use_description(Signal::derive(move || {
        session
            .with(Option::is_some)
            .then(|| messages::drop_description(&strings.read(), modality.get()))
    }))
}
