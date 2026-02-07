use std::cell::{Cell, RefCell};

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use super::run_after_transition::run_after_transition;

/// State machine for iOS text selection management.
/// On iOS, we modify `document.documentElement.style.webkitUserSelect` at the document level
/// because iOS doesn't respect `user-select: none` on individual elements during touch interactions.
#[derive(Clone, Copy, PartialEq, Eq)]
enum IosSelectionState {
    Default,
    Disabled,
    Restoring,
}

thread_local! {
    static IOS_STATE: Cell<IosSelectionState> = const { Cell::new(IosSelectionState::Default) };
}

// Saved original `webkit-user-select` value for the iOS path.
thread_local! {
    static SAVED_USER_SELECT: RefCell<String> = const { RefCell::new(String::new()) };
}

const DATA_SAVED_USER_SELECT: &str = "data-leptonic-saved-user-select";

fn is_ios() -> bool {
    let Some(window) = web_sys::window() else {
        return false;
    };
    let navigator = window.navigator();
    navigator
        .platform()
        .ok()
        .is_some_and(|p| p.contains("iPhone") || p.contains("iPad") || p.contains("iPod"))
        || navigator
            .user_agent()
            .ok()
            .is_some_and(|ua| ua.contains("AppleWebKit") && ua.contains("Mobile"))
}

/// Returns the `CssStyleDeclaration` for an element, supporting both `HtmlElement` and `SvgElement`.
fn get_element_style(element: &web_sys::Element) -> Option<web_sys::CssStyleDeclaration> {
    if let Some(html_el) = element.dyn_ref::<web_sys::HtmlElement>() {
        return Some(html_el.style());
    }
    if let Some(svg_el) = element.dyn_ref::<web_sys::SvgElement>() {
        return Some(svg_el.style());
    }
    None
}

fn get_document_element_style() -> Option<web_sys::CssStyleDeclaration> {
    let el = web_sys::window()?
        .document()?
        .document_element()?
        .dyn_into::<web_sys::HtmlElement>()
        .ok()?;
    Some(el.style())
}

pub(crate) fn disable_text_selection(element: &web_sys::Element) {
    if is_ios() {
        disable_text_selection_ios();
    } else {
        disable_text_selection_standard(element);
    }
}

pub(crate) fn restore_text_selection(element: &web_sys::Element) {
    if is_ios() {
        restore_text_selection_ios();
    } else {
        restore_text_selection_standard(element);
    }
}

// --- iOS implementation ---

fn disable_text_selection_ios() {
    let current = IOS_STATE.with(Cell::get);
    if current == IosSelectionState::Restoring {
        // Was in the process of restoring, cancel that and keep disabled.
        IOS_STATE.with(|s| s.set(IosSelectionState::Disabled));
        return;
    }
    if current == IosSelectionState::Disabled {
        return; // Already disabled
    }

    // Transitioning from Default: save the original value before overwriting.
    if let Some(style) = get_document_element_style() {
        let original = style
            .get_property_value("webkit-user-select")
            .unwrap_or_default();
        SAVED_USER_SELECT.with(|saved| {
            *saved.borrow_mut() = original;
        });
        let _ = style.set_property("webkit-user-select", "none");
    }

    IOS_STATE.with(|s| s.set(IosSelectionState::Disabled));
}

fn restore_text_selection_ios() {
    if IOS_STATE.with(Cell::get) != IosSelectionState::Disabled {
        return;
    }

    IOS_STATE.with(|s| s.set(IosSelectionState::Restoring));

    // Use a 300ms timeout to allow touch events to complete before restoring.
    // This prevents text selection from triggering during the brief period
    // after a touch interaction ends.
    let callback = Closure::once(Box::new(move || {
        // Wait for any CSS transitions to complete so we don't recompute style
        // for the whole page in the middle of the animation and cause jank.
        run_after_transition(move || {
            if IOS_STATE.with(Cell::get) == IosSelectionState::Restoring {
                if let Some(style) = get_document_element_style() {
                    // Guard: only restore if the current value is still "none".
                    // Another piece of code may have changed it in the meantime.
                    if style.get_property_value("webkit-user-select").as_deref() == Ok("none") {
                        let saved = SAVED_USER_SELECT
                            .with(|saved| std::mem::take(&mut *saved.borrow_mut()));
                        if saved.is_empty() {
                            let _ = style.remove_property("webkit-user-select");
                        } else {
                            let _ = style.set_property("webkit-user-select", &saved);
                        }
                    }
                }
                SAVED_USER_SELECT.with(|saved| saved.borrow_mut().clear());
                IOS_STATE.with(|s| s.set(IosSelectionState::Default));
            }
        });
    }) as Box<dyn FnOnce()>);

    if let Some(window) = web_sys::window() {
        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            300,
        );
    }
    callback.forget();
}

// --- Standard (non-iOS) implementation ---

fn disable_text_selection_standard(element: &web_sys::Element) {
    let Some(style) = get_element_style(element) else {
        return;
    };

    // Determine which property name the browser supports.
    let property = user_select_property(&style);

    // Save the original value as a data attribute on the element.
    let original = style.get_property_value(property).unwrap_or_default();
    let _ = element.set_attribute(DATA_SAVED_USER_SELECT, &original);

    let _ = style.set_property(property, "none");
}

fn restore_text_selection_standard(element: &web_sys::Element) {
    let Some(style) = get_element_style(element) else {
        return;
    };

    let property = user_select_property(&style);

    // Guard: only restore if the current value is still "none".
    if style.get_property_value(property).as_deref() != Ok("none") {
        // Another piece of code changed the value; don't clobber it.
        let _ = element.remove_attribute(DATA_SAVED_USER_SELECT);
        return;
    }

    // Read the saved original value.
    if let Some(saved) = element.get_attribute(DATA_SAVED_USER_SELECT) {
        if saved.is_empty() {
            let _ = style.remove_property(property);
        } else {
            let _ = style.set_property(property, &saved);
        }
    } else {
        let _ = style.remove_property(property);
    }

    let _ = element.remove_attribute(DATA_SAVED_USER_SELECT);

    // Clean up empty style attributes to avoid leaving behind `style=""`.
    if element.get_attribute("style").is_some_and(|s| s.is_empty()) {
        element.remove_attribute("style").ok();
    }
}

/// Determine the correct CSS property name for the current browser.
/// Modern browsers support `user-select`; older `WebKit` browsers need `-webkit-user-select`.
fn user_select_property(style: &web_sys::CssStyleDeclaration) -> &'static str {
    // `CSSStyleDeclaration.getPropertyValue` returns "" for unknown properties,
    // so we check via JS reflection whether the camelCase property exists on the object.
    let style_val: &wasm_bindgen::JsValue = style.as_ref();
    let has_user_select =
        js_sys::Reflect::has(style_val, &wasm_bindgen::JsValue::from_str("userSelect"))
            .unwrap_or(false);
    if has_user_select {
        "user-select"
    } else {
        "-webkit-user-select"
    }
}
