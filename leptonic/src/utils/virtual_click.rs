use wasm_bindgen::JsCast;

/// Detects screen reader / assistive technology clicks.
///
/// Screen readers fire synthetic click events with specific characteristics
/// that differ from real user clicks. This function identifies those patterns.
///
/// Based on react-aria's `isVirtualClick` from `isVirtualEvent.ts`.
///
/// Accepts `&MouseEvent` because click handlers receive `MouseEvent`. Internally
/// attempts to downcast to `PointerEvent` to access `pointer_type()`, since modern
/// browsers emit click events as `PointerEvent` instances.
///
/// # Detection rules
///
/// - JAWS/NVDA with Firefox: `pointer_type` is empty and the event is trusted.
/// - Android `TalkBack`: event type is `"click"` with `buttons == 1` and a non-empty pointer type.
/// - Default (`VoiceOver`, other screen readers): `detail == 0` and no pointer type.
pub fn is_virtual_click(e: &web_sys::MouseEvent) -> bool {
    let pointer_type = e
        .dyn_ref::<web_sys::PointerEvent>()
        .map(web_sys::PointerEvent::pointer_type);

    // JAWS/NVDA with Firefox fire trusted pointer events with empty pointer_type.
    if pointer_type.as_deref() == Some("") && e.is_trusted() {
        return true;
    }

    // Android TalkBack fires click events with a non-empty pointer_type and buttons == 1.
    if crate::utils::platform::device::is_android()
        && pointer_type.as_ref().is_some_and(|pt| !pt.is_empty())
    {
        return e.type_() == "click" && e.buttons() == 1;
    }

    // Default: screen reader synthetic click has detail == 0 and no/empty pointer_type.
    // When the event is not a PointerEvent (pointer_type is None), or pointer_type is empty.
    e.detail() == 0 && !pointer_type.as_ref().is_some_and(|pt| !pt.is_empty())
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
