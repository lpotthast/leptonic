// Upstream: react-aria/src/interactions/textSelection.ts @ 99e6102368
//! Disabling text selection while an element is pressed or moved (react-aria's
//! `disableTextSelection`/`restoreTextSelection`).
//!
//! WebKit on iOS starts selecting text on a long press, and only `user-select: none` on the whole
//! page prevents it: there, the document's `-webkit-user-select` is set while pressed (global
//! state, since several elements may be pressed in turn). Elsewhere, `user-select: none` goes on
//! the pressed element only (setting it on the page is slow, adobe/react-spectrum#1609).

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No deviations from react-aria beyond the project-wide API conventions.
//
// =============================================================================

use std::{
    cell::{Cell, RefCell},
    time::Duration,
};

use leptos::prelude::set_timeout;
use wasm_bindgen::{JsCast, JsValue};

use super::{
    platform::{browser::is_webkit, device::is_ios},
    run_after_transition::run_after_transition,
};

/// The iOS page-level state. `Default` always goes to `Disabled`, which goes to `Restoring` (the
/// delay after pointer up), which goes back to `Disabled` or to `Default`.
#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Default,
    Disabled,
    Restoring,
}

thread_local! {
    static STATE: Cell<State> = const { Cell::new(State::Default) };
    /// The document's `-webkit-user-select` before it was disabled (iOS).
    static SAVED_USER_SELECT: RefCell<String> = const { RefCell::new(String::new()) };
    /// Each element's `user-select` before it was disabled (elsewhere). Weak: an element removed
    /// while pressed doesn't stay alive.
    static MODIFIED_ELEMENTS: js_sys::WeakMap = js_sys::WeakMap::new();
}

/// Disables text selection for an interaction on `target`: on iOS for the whole page, elsewhere
/// for `target` (nothing without one).
pub(crate) fn disable_text_selection(target: Option<&web_sys::Element>) {
    if is_ios() && is_webkit() {
        if STATE.get() == State::Default
            && let Some(style) = document_element_style(target)
        {
            SAVED_USER_SELECT.set(
                style
                    .get_property_value("-webkit-user-select")
                    .unwrap_or_default(),
            );
            let _ = style.set_property("-webkit-user-select", "none");
        }
        STATE.set(State::Disabled);
    } else if let Some((target, style)) = target.and_then(|t| element_style(t).map(|s| (t, s))) {
        let property = user_select_property(&style);
        let original = style.get_property_value(property).unwrap_or_default();
        MODIFIED_ELEMENTS.with(|map| map.set(target.as_ref(), &JsValue::from_str(&original)));
        let _ = style.set_property(property, "none");
    }
}

/// Undoes [`disable_text_selection`]: on iOS after a delay (selection may still start right after
/// the pointer up) and after running transitions; elsewhere right away, and only for an element
/// it disabled, unless something else changed its `user-select` meanwhile.
pub(crate) fn restore_text_selection(target: Option<&web_sys::Element>) {
    if is_ios() && is_webkit() {
        // Already default, or a restore is queued.
        if STATE.get() != State::Disabled {
            return;
        }
        STATE.set(State::Restoring);
        let target = target.cloned().map(send_wrapper::SendWrapper::new);
        set_timeout(
            move || {
                // Not in the middle of an animation: restyling the whole page would make it jank.
                run_after_transition(move |_| {
                    // Disabled again meanwhile.
                    if STATE.get() != State::Restoring {
                        return;
                    }
                    if let Some(style) = document_element_style(target.as_deref())
                        && style.get_property_value("-webkit-user-select").as_deref() == Ok("none")
                    {
                        let saved = SAVED_USER_SELECT.take();
                        let _ = style.set_property("-webkit-user-select", &saved);
                    }
                    SAVED_USER_SELECT.take();
                    STATE.set(State::Default);
                });
            },
            Duration::from_millis(300),
        );
    } else if let Some(target) = target
        && let Some(style) = element_style(target)
    {
        let Some(original) = MODIFIED_ELEMENTS.with(|map| {
            map.has(target.as_ref())
                .then(|| map.get(target.as_ref()).as_string())
                .flatten()
        }) else {
            return;
        };
        let property = user_select_property(&style);
        // Something else changed it meanwhile: keep that.
        if style.get_property_value(property).as_deref() == Ok("none") {
            let _ = style.set_property(property, &original);
        }
        if target.get_attribute("style").as_deref() == Some("") {
            let _ = target.remove_attribute("style");
        }
        MODIFIED_ELEMENTS.with(|map| map.delete(target.as_ref()));
    }
}

/// The inline style of an HTML or SVG element (others have none).
fn element_style(element: &web_sys::Element) -> Option<web_sys::CssStyleDeclaration> {
    if let Some(element) = element.dyn_ref::<web_sys::HtmlElement>() {
        Some(element.style())
    } else {
        element
            .dyn_ref::<web_sys::SvgElement>()
            .map(web_sys::SvgElement::style)
    }
}

/// The inline style of the `<html>` element of `target`'s document (else the global one).
fn document_element_style(
    target: Option<&web_sys::Element>,
) -> Option<web_sys::CssStyleDeclaration> {
    let document = target
        .and_then(|target| target.owner_document())
        .or_else(|| leptos_use::use_document().as_ref().cloned())?;
    element_style(&document.document_element()?)
}

/// `user-select`, or `-webkit-user-select` where only that exists (Safari before 17).
fn user_select_property(style: &web_sys::CssStyleDeclaration) -> &'static str {
    if js_sys::Reflect::has(style, &JsValue::from_str("userSelect")).unwrap_or(false) {
        "user-select"
    } else {
        "-webkit-user-select"
    }
}
