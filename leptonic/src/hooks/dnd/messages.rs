// Upstream: react-aria/intl/dnd/en-US.json @ 99e6102368
//! The texts drag and drop shows and announces (English; localization comes with the localized
//! strings infrastructure, see PLAN.md).

use super::utils::DragModality;

fn selected_items(count: usize) -> String {
    if count == 1 {
        "1 selected item".to_owned()
    } else {
        format!("{count} selected items")
    }
}

pub(crate) fn drag_item(item_text: &str) -> String {
    format!("Drag {item_text}")
}

pub(crate) fn drag_selected_items(count: usize) -> String {
    format!("Drag {}", selected_items(count))
}

/// How to start dragging (`useDrag`'s description before a drag).
pub(crate) fn drag_description(modality: DragModality) -> &'static str {
    match modality {
        DragModality::Keyboard => "Press Enter to start dragging.",
        DragModality::Touch => "Double tap to start dragging.",
        DragModality::Virtual => "Click to start dragging.",
    }
}

/// How to end a drag (`useDrag`'s description during a drag).
pub(crate) fn end_drag(modality: DragModality) -> &'static str {
    match modality {
        DragModality::Keyboard => "Dragging. Press Enter to cancel drag.",
        DragModality::Touch => "Dragging. Double tap to cancel drag.",
        DragModality::Virtual => "Dragging. Click to cancel drag.",
    }
}

/// How to drag a collection item (`useDraggableItem`).
pub(crate) fn drag_item_description(
    modality: DragModality,
    selected_count: Option<usize>,
    alt: bool,
) -> String {
    let enter = if alt { "Alt + Enter" } else { "Enter" };
    match (modality, selected_count) {
        (DragModality::Keyboard, Some(count)) => {
            format!("Press {enter} to drag {}.", selected_items(count))
        }
        (DragModality::Keyboard, None) => format!("Press {enter} to start dragging."),
        (DragModality::Touch, Some(count)) => {
            format!("Long press to drag {}.", selected_items(count))
        }
        (DragModality::Touch, None) => "Long press to start dragging.".to_owned(),
        (DragModality::Virtual, _) => "Click to start dragging.".to_owned(),
    }
}

pub(crate) fn drag_started(modality: DragModality) -> &'static str {
    match modality {
        DragModality::Keyboard => {
            "Started dragging. Press Tab to navigate to a drop target, then press Enter to drop, or press Escape to cancel."
        }
        DragModality::Touch => {
            "Started dragging. Navigate to a drop target, then double tap to drop."
        }
        DragModality::Virtual => {
            "Started dragging. Navigate to a drop target, then click or press Enter to drop."
        }
    }
}

/// How to drop on a drop target (`useVirtualDrop`).
pub(crate) fn drop_description(modality: DragModality) -> &'static str {
    match modality {
        DragModality::Keyboard => "Press Enter to drop. Press Escape to cancel drag.",
        DragModality::Touch => "Double tap to drop.",
        DragModality::Virtual => "Click to drop.",
    }
}

pub(crate) const DROP_CANCELED: &str = "Drop canceled.";
pub(crate) const DROP_COMPLETE: &str = "Drop complete.";
pub(crate) const DROP_INDICATOR: &str = "drop indicator";
pub(crate) const DROP_ON_ROOT: &str = "Drop on";

pub(crate) fn drop_on_item(item_text: &str) -> String {
    format!("Drop on {item_text}")
}

pub(crate) fn insert_before(item_text: &str) -> String {
    format!("Insert before {item_text}")
}

pub(crate) fn insert_between(before: &str, after: &str) -> String {
    format!("Insert between {before} and {after}")
}

pub(crate) fn insert_after(item_text: &str) -> String {
    format!("Insert after {item_text}")
}
