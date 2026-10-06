// Upstream: react-aria/src/focus/virtualFocus.ts @ 99e6102368
//! Virtual focus: DOM focus stays on one element (e.g. a combo box input) while another
//! element (e.g. a listbox option) is "focused" for assistive technology through
//! `aria-activedescendant`. Synthetic focus/blur events let the virtually focused elements react
//! as if they had real focus.

use super::shadow_dom::get_active_element;

/// Moves virtual focus to `to` (or away from the currently virtually focused element), sending
/// blur/focusout to the previous element and focus/focusin to the new one.
pub fn move_virtual_focus(to: Option<&web_sys::Element>) {
    let document = to
        .and_then(|el| el.owner_document())
        .or_else(|| leptos_use::use_document().as_ref().cloned());
    let Some(document) = document else {
        return;
    };
    let from = get_virtually_focused_element(&document);
    if from.as_ref() == to {
        return;
    }
    if let Some(from) = &from {
        dispatch_virtual_blur(from, to);
    }
    if let Some(to) = to {
        dispatch_virtual_focus(to, from.as_ref());
    }
}

/// Sends `blur` and `focusout` (bubbling) to `from`.
pub fn dispatch_virtual_blur(from: &web_sys::Element, to: Option<&web_sys::Element>) {
    let _ = from.dispatch_event(&focus_event("blur", false, to));
    let _ = from.dispatch_event(&focus_event("focusout", true, to));
}

/// Sends `focus` and `focusin` (bubbling) to `to`.
pub fn dispatch_virtual_focus(to: &web_sys::Element, from: Option<&web_sys::Element>) {
    let _ = to.dispatch_event(&focus_event("focus", false, from));
    let _ = to.dispatch_event(&focus_event("focusin", true, from));
}

/// The element that is focused, virtually or for real: the active element, or the element its
/// `aria-activedescendant` references.
pub fn get_virtually_focused_element(document: &web_sys::Document) -> Option<web_sys::Element> {
    let active = get_active_element(document)?;
    match active.get_attribute("aria-activedescendant") {
        Some(id) if !id.is_empty() => document.get_element_by_id(&id).or(Some(active)),
        _ => Some(active),
    }
}

fn focus_event(
    kind: &str,
    bubbles: bool,
    related_target: Option<&web_sys::Element>,
) -> web_sys::FocusEvent {
    let init = web_sys::FocusEventInit::new();
    init.set_bubbles(bubbles);
    init.set_related_target(related_target.map(AsRef::as_ref));
    web_sys::FocusEvent::new_with_focus_event_init_dict(kind, &init)
        .expect("FocusEvent creation should not fail")
}
