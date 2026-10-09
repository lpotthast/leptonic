// Upstream: react-aria/src/dnd/DragManager.ts @ 99e6102368
// Upstream: react-aria/src/dnd/useDrag.ts @ 99e6102368
// Upstream: react-aria/src/dnd/useDraggableItem.ts @ 99e6102368
// Upstream: react-aria/src/dnd/useVirtualDrop.ts @ 99e6102368
//! The texts drag and drop shows and announces, by drag modality (react-aria picks the message
//! keys the same way).

use super::utils::DragModality;
use crate::utils::intl_strings::DndStrings;

/// How to start dragging (`useDrag`'s description before a drag).
pub(crate) fn drag_description(strings: &DndStrings, modality: DragModality) -> String {
    match modality {
        DragModality::Keyboard => strings.drag_description_keyboard(),
        DragModality::Touch => strings.drag_description_touch(),
        DragModality::Virtual => strings.drag_description_virtual(),
    }
}

/// How to end a drag (`useDrag`'s description during a drag).
pub(crate) fn end_drag(strings: &DndStrings, modality: DragModality) -> String {
    match modality {
        DragModality::Keyboard => strings.end_drag_keyboard(),
        DragModality::Touch => strings.end_drag_touch(),
        DragModality::Virtual => strings.end_drag_virtual(),
    }
}

/// How to drag a collection item (`useDraggableItem`): `selected_count` while it is one of
/// several selected items, `alt` where Enter performs the item's action.
pub(crate) fn drag_item_description(
    strings: &DndStrings,
    modality: DragModality,
    selected_count: Option<usize>,
    alt: bool,
) -> String {
    match (modality, selected_count) {
        (DragModality::Keyboard, Some(count)) if alt => strings.drag_selected_keyboard_alt(count),
        (DragModality::Keyboard, Some(count)) => strings.drag_selected_keyboard(count),
        (DragModality::Keyboard, None) if alt => strings.drag_description_keyboard_alt(),
        (DragModality::Keyboard, None) => strings.drag_description_keyboard(),
        (DragModality::Touch, Some(count)) => strings.drag_selected_long_press(count),
        (DragModality::Touch, None) => strings.drag_description_long_press(),
        (DragModality::Virtual, _) => strings.drag_description_virtual(),
    }
}

pub(crate) fn drag_started(strings: &DndStrings, modality: DragModality) -> String {
    match modality {
        DragModality::Keyboard => strings.drag_started_keyboard(),
        DragModality::Touch => strings.drag_started_touch(),
        DragModality::Virtual => strings.drag_started_virtual(),
    }
}

/// How to drop on a drop target (`useVirtualDrop`).
pub(crate) fn drop_description(strings: &DndStrings, modality: DragModality) -> String {
    match modality {
        DragModality::Keyboard => strings.drop_description_keyboard(),
        DragModality::Touch => strings.drop_description_touch(),
        DragModality::Virtual => strings.drop_description_virtual(),
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::utils::{i18n::Locale, intl_strings::LocalizedStrings};

    /// Touch drags (a coarse primary pointer) have descriptions and announcements of their own
    /// (react-aria's "should use touch specific aria descriptions when available").
    #[test]
    fn touch_drags_have_their_own_texts() {
        let locale: Locale = "en-US".parse().expect("a valid locale");
        let strings = DndStrings::for_locale(locale);
        let touch = DragModality::Touch;
        assert_that!(drag_description(&strings, touch))
            .is_equal_to("Double tap to start dragging.");
        assert_that!(end_drag(&strings, touch)).is_equal_to("Dragging. Double tap to cancel drag.");
        assert_that!(drag_started(&strings, touch))
            .is_equal_to("Started dragging. Navigate to a drop target, then double tap to drop.");
        assert_that!(drop_description(&strings, touch)).is_equal_to("Double tap to drop.");
    }
}
