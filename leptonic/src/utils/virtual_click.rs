/// Detects screen reader / assistive technology clicks.
///
/// Screen readers fire synthetic click events with specific characteristics
/// that differ from real user clicks. This function identifies those patterns.
pub fn is_virtual_click(e: &web_sys::MouseEvent) -> bool {
    let detail = e.detail();
    let offset_x = e.offset_x();
    let offset_y = e.offset_y();

    // detail === 0 indicates a non-physical click
    // However, real clicks on zero-size elements also have offsetX/offsetY === 0,
    // so we need to distinguish those cases.
    if detail == 0 && !(offset_x == 0 && offset_y == 0) {
        return true;
    }

    let client_x = e.client_x();
    let client_y = e.client_y();

    // `VoiceOver` on macOS/iOS fires clicks with all coordinates at 0.
    detail == 0 && offset_x == 0 && offset_y == 0 && client_x == 0 && client_y == 0
}

/// Detects virtual pointer events (e.g., `VoiceOver` on iOS).
///
/// `VoiceOver` on iOS can fire pointer events that look like real touch events
/// but have telltale characteristics (zero-size touch, zero pressure, etc.).
pub fn is_virtual_pointer_event(e: &web_sys::PointerEvent) -> bool {
    let width = e.width();
    let height = e.height();

    // Zero-size touch area indicates a virtual event.
    if width == 0 && height == 0 {
        return true;
    }

    // `VoiceOver` on iOS fires pointer events with these specific characteristics.
    // `PointerEvent` extends `MouseEvent` via Deref, so `.detail()` is available directly.
    width == 1
        && height == 1
        && e.pressure() == 0.0
        && e.detail() == 0
        && e.pointer_type() == "mouse"
}
