//! Focusability and tabbability detection utilities.
//!
//! Shared infrastructure for determining whether DOM elements are focusable or tabbable,
//! used by `use_focus_manager`, `use_has_tabbable_child`, and other focus hooks.
//!
//! Based on react-aria's `isFocusable.ts` and `isElementVisible.ts`.

use wasm_bindgen::{JsCast, JsValue};

/// Selector for focusable elements.
///
/// Aligned with react-aria's `FOCUSABLE_ELEMENT_SELECTOR`. Each segment includes
/// `:not([hidden])` to exclude elements with the HTML `hidden` attribute.
pub const FOCUSABLE_SELECTOR: &str = concat!(
    "input:not([disabled]):not([type=hidden]):not([hidden]),",
    "select:not([disabled]):not([hidden]),",
    "textarea:not([disabled]):not([hidden]),",
    "button:not([disabled]):not([hidden]),",
    "a[href]:not([hidden]),",
    "area[href]:not([hidden]),",
    "summary:not([hidden]),",
    "iframe:not([hidden]),",
    "object:not([hidden]),",
    "embed:not([hidden]),",
    "audio[controls]:not([hidden]),",
    "video[controls]:not([hidden]),",
    "permission:not([hidden]),",
    "[contenteditable]:not([contenteditable^=\"false\"]):not([hidden]),",
    "[tabindex]:not([disabled]):not([hidden])"
);

/// Selector for tabbable elements (focusable elements that are reachable via Tab).
///
/// Extends `FOCUSABLE_SELECTOR` by additionally excluding elements with `tabindex="-1"`
/// and adding `:not([disabled])` to the `[tabindex]` segment.
pub const TABBABLE_SELECTOR: &str = concat!(
    "input:not([disabled]):not([type=hidden]):not([hidden]):not([tabindex=\"-1\"]),",
    "select:not([disabled]):not([hidden]):not([tabindex=\"-1\"]),",
    "textarea:not([disabled]):not([hidden]):not([tabindex=\"-1\"]),",
    "button:not([disabled]):not([hidden]):not([tabindex=\"-1\"]),",
    "a[href]:not([hidden]):not([tabindex=\"-1\"]),",
    "area[href]:not([hidden]):not([tabindex=\"-1\"]),",
    "summary:not([hidden]):not([tabindex=\"-1\"]),",
    "iframe:not([hidden]):not([tabindex=\"-1\"]),",
    "object:not([hidden]):not([tabindex=\"-1\"]),",
    "embed:not([hidden]):not([tabindex=\"-1\"]),",
    "audio[controls]:not([hidden]):not([tabindex=\"-1\"]),",
    "video[controls]:not([hidden]):not([tabindex=\"-1\"]),",
    "permission:not([hidden]):not([tabindex=\"-1\"]),",
    "[contenteditable]:not([contenteditable^=\"false\"]):not([hidden]):not([tabindex=\"-1\"]),",
    "[tabindex]:not([tabindex=\"-1\"]):not([disabled]):not([hidden])"
);

/// Check if an element or any ancestor has the `inert` property set.
///
/// The `inert` attribute makes an entire subtree non-interactive:
/// elements within an inert subtree are not focusable, not clickable,
/// and not reachable via assistive technology.
///
/// Uses the `HtmlElement.inert` IDL property instead of `hasAttribute("inert")`
/// for correctness (the IDL property reflects the effective state).
///
/// Based on react-aria's `isInert()` from `isFocusable.ts`.
pub fn is_inert(element: &web_sys::Element) -> bool {
    let mut current: Option<web_sys::Element> = Some(element.clone());

    while let Some(el) = current {
        if let Some(html_el) = el.dyn_ref::<web_sys::HtmlElement>() {
            if html_el.inert() {
                return true;
            }
        }
        current = el.parent_element();
    }

    false
}

/// Check if an element is visible (not hidden via CSS or attributes).
///
/// Handles both `HTMLElement` and `SVGElement` types. Elements that are neither
/// (e.g., `MathMLElement`) are considered not visible (matching react-aria).
///
/// Prefers `checkVisibility()` when available (modern browsers), falling back
/// to a recursive ancestor walk checking `display` and `visibility` CSS properties
/// plus the `hidden` HTML attribute.
///
/// Inline style properties are checked first as an optimization — if the element's
/// own `style.display` is `"none"` or `style.visibility` is `"hidden"`, we can
/// return `false` immediately without the more expensive `checkVisibility()` call
/// or computed style walk.
///
/// Based on react-aria's `isElementVisible()` from `isElementVisible.ts`.
pub fn is_element_visible(element: &web_sys::Element) -> bool {
    // Get inline style from HTMLElement or SVGElement. Elements that are neither
    // (matching react-aria's `instanceof HTMLElement || instanceof SVGElement` check)
    // are considered not visible.
    let style = if let Some(html_el) = element.dyn_ref::<web_sys::HtmlElement>() {
        html_el.style()
    } else if let Some(svg_el) = element.dyn_ref::<web_sys::SvgElement>() {
        svg_el.style()
    } else {
        return false;
    };

    // Fast-path: check inline style properties before anything else.
    let inline_display = style.get_property_value("display").unwrap_or_default();
    if inline_display == "none" {
        return false;
    }
    let inline_visibility = style.get_property_value("visibility").unwrap_or_default();
    if inline_visibility == "hidden" || inline_visibility == "collapse" {
        return false;
    }

    // Try `checkVisibility()` (available in modern browsers).
    // Cache whether the method exists to avoid repeated Reflect::get lookups.
    if has_check_visibility() {
        if let Ok(check_visibility) = js_sys::Reflect::get(element, &"checkVisibility".into()) {
            if check_visibility.is_function() {
                let options = js_sys::Object::new();
                let _ = js_sys::Reflect::set(&options, &"visibilityProperty".into(), &true.into());
                if let Ok(result) = js_sys::Reflect::apply(
                    check_visibility.unchecked_ref(),
                    element,
                    &js_sys::Array::of1(&options),
                ) {
                    return result.as_bool().unwrap_or(true);
                }
            }
        }
    }

    // Fallback: recursive ancestor walk.
    is_element_visible_fallback(element, None)
}

/// Cache whether `checkVisibility` is supported. Checked once per page load.
fn has_check_visibility() -> bool {
    use std::sync::atomic::{AtomicU8, Ordering};

    // 0 = unknown, 1 = supported, 2 = unsupported
    static SUPPORT: AtomicU8 = AtomicU8::new(0);

    match SUPPORT.load(Ordering::Relaxed) {
        1 => true,
        2 => false,
        _ => {
            let supported = js_sys::Reflect::get(&js_sys::global(), &"HTMLElement".into())
                .ok()
                .and_then(|ctor| js_sys::Reflect::get(&ctor, &"prototype".into()).ok())
                .and_then(|proto| js_sys::Reflect::get(&proto, &"checkVisibility".into()).ok())
                .is_some_and(|v| v.is_function());

            SUPPORT.store(if supported { 1 } else { 2 }, Ordering::Relaxed);
            supported
        }
    }
}

/// Fallback visibility check that walks the ancestor chain.
///
/// `child_element` tracks the previous element in the walk so that
/// `<details>`/`<summary>` visibility can be evaluated correctly:
/// content inside a closed `<details>` is invisible unless it is a `<summary>`.
fn is_element_visible_fallback(
    element: &web_sys::Element,
    child_element: Option<&web_sys::Element>,
) -> bool {
    let Some(window) = element.owner_document().and_then(|d| d.default_view()) else {
        return true;
    };

    // Check attribute-based visibility (hidden attribute, <details>/<summary>).
    if !is_attribute_visible(element, child_element) {
        return false;
    }

    // Check computed style. `getComputedStyle` works on any Element (HTML or SVG).
    if let Ok(Some(style)) = window.get_computed_style(element) {
        if let Ok(display) = style.get_property_value("display") {
            if display == "none" {
                return false;
            }
        }
        if let Ok(visibility) = style.get_property_value("visibility") {
            if visibility == "hidden" || visibility == "collapse" {
                return false;
            }
        }
    }

    // Recurse into parent.
    if let Some(parent) = element.parent_element() {
        return is_element_visible_fallback(&parent, Some(element));
    }

    true
}

/// Check attribute-based visibility for a single element.
///
/// Returns `false` when:
/// - The element has a `hidden` attribute.
/// - The element is a closed `<details>` and `child_element` is not a `<summary>`.
fn is_attribute_visible(
    element: &web_sys::Element,
    child_element: Option<&web_sys::Element>,
) -> bool {
    if element.has_attribute("hidden") {
        return false;
    }

    // Content inside a closed <details> is invisible unless it is a <summary>.
    if element.tag_name() == "DETAILS"
        && !element.has_attribute("open")
        && child_element.is_some_and(|child| child.tag_name() != "SUMMARY")
    {
        return false;
    }

    true
}

/// Check if an element is focusable.
///
/// An element is focusable if it matches the focusable selector,
/// is visible, and is not inside an `inert` subtree.
pub fn is_focusable(element: &web_sys::Element) -> bool {
    if let Ok(matches) = element.matches(FOCUSABLE_SELECTOR) {
        if !matches {
            return false;
        }
    } else {
        return false;
    }

    if is_inert(element) {
        return false;
    }

    if !is_element_visible(element) {
        return false;
    }

    true
}

/// Check if an element is tabbable (reachable via Tab key).
///
/// An element is tabbable if it matches the tabbable selector,
/// is visible, is not inside an `inert` subtree, and has a non-negative tabindex.
pub fn is_tabbable(element: &web_sys::Element) -> bool {
    if let Ok(matches) = element.matches(TABBABLE_SELECTOR) {
        if !matches {
            return false;
        }
    } else {
        return false;
    }

    if is_inert(element) {
        return false;
    }

    if !is_element_visible(element) {
        return false;
    }

    // Double-check tabindex (the selector already filters -1, but be defensive).
    if let Some(html_el) = element.dyn_ref::<web_sys::HtmlElement>() {
        if html_el.tab_index() < 0 {
            return false;
        }
    }

    true
}

/// Get the radio group name for an element, if it is an `input[type=radio]` with a name.
pub fn get_radio_group_name(element: &web_sys::Element) -> Option<String> {
    let input = element.dyn_ref::<web_sys::HtmlInputElement>()?;
    if input.type_() != "radio" {
        return None;
    }
    let name = input.name();
    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}

/// Get all radio buttons in the same group as `element`.
///
/// Based on react-aria's `getRadiosInGroup` from `FocusScope.tsx`.
///
/// - If the radio has a form, uses `form.elements.namedItem(name)` which correctly
///   finds radio buttons associated via the `form` attribute (not just DOM descendants).
/// - If the radio has no form, queries the document for same-name radios
///   that also have no form.
fn get_radios_in_group(element: &web_sys::HtmlInputElement) -> Vec<web_sys::HtmlInputElement> {
    let name = element.name();
    if name.is_empty() {
        return Vec::new();
    }

    if let Some(form) = element.form() {
        // Use form.elements.namedItem() to correctly find radios associated via
        // the `form` attribute (which aren't DOM descendants of the form).
        collect_form_radios(&form, &name)
    } else {
        let Some(doc) = element.owner_document() else {
            return Vec::new();
        };
        let escaped_name = css_escape(&name);
        let selector = format!("input[type=\"radio\"][name=\"{escaped_name}\"]");
        // Only include radios that do NOT belong to a form.
        collect_radio_nodes(&doc, &selector, |input| input.form().is_none())
    }
}

/// Collect radio buttons from a form's elements collection using `namedItem`.
///
/// `namedItem` returns either a single `Element` or a `RadioNodeList`, so we
/// handle both cases.
fn collect_form_radios(
    form: &web_sys::HtmlFormElement,
    name: &str,
) -> Vec<web_sys::HtmlInputElement> {
    let elements = form.elements();
    let Some(item) = elements.named_item(name) else {
        return Vec::new();
    };

    // namedItem can return a single Element or a RadioNodeList.
    if let Some(radio_list) = item.dyn_ref::<web_sys::RadioNodeList>() {
        let mut radios = Vec::new();
        for i in 0..radio_list.length() {
            if let Some(node) = radio_list.get(i) {
                if let Some(input) = node.dyn_ref::<web_sys::HtmlInputElement>() {
                    if input.type_() == "radio" {
                        radios.push(input.clone());
                    }
                }
            }
        }
        radios
    } else if let Some(input) = item.dyn_ref::<web_sys::HtmlInputElement>() {
        if input.type_() == "radio" {
            vec![input.clone()]
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    }
}

/// Query `root` with `selector` and collect matching `HtmlInputElement`s that pass `filter`.
fn collect_radio_nodes(
    root: &web_sys::Node,
    selector: &str,
    filter: impl Fn(&web_sys::HtmlInputElement) -> bool,
) -> Vec<web_sys::HtmlInputElement> {
    // `querySelectorAll` is on `Element` and `Document`; try both via `ParentNode`.
    let nodes = if let Some(el) = root.dyn_ref::<web_sys::Element>() {
        el.query_selector_all(selector).ok()
    } else if let Some(doc) = root.dyn_ref::<web_sys::Document>() {
        doc.query_selector_all(selector).ok()
    } else {
        None
    };

    let Some(nodes) = nodes else {
        return Vec::new();
    };

    let mut radios = Vec::new();
    for i in 0..nodes.length() {
        if let Some(node) = nodes.get(i) {
            if let Some(input) = node.dyn_ref::<web_sys::HtmlInputElement>() {
                if filter(input) {
                    radios.push(input.clone());
                }
            }
        }
    }
    radios
}

/// Escape a string for use in a CSS selector, using the native `CSS.escape()` API.
fn css_escape(value: &str) -> String {
    let global_css = js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str("CSS"));
    if let Ok(css_obj) = global_css {
        let escape_fn = js_sys::Reflect::get(&css_obj, &JsValue::from_str("escape"));
        if let Ok(func) = escape_fn {
            if let Some(func) = func.dyn_ref::<js_sys::Function>() {
                if let Ok(result) = func.call1(&JsValue::NULL, &JsValue::from_str(value)) {
                    if let Some(s) = result.as_string() {
                        return s;
                    }
                }
            }
        }
    }
    // Fallback: simple escaping of quotes and backslashes.
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

/// Check if a radio button should be considered tabbable.
///
/// Based on react-aria's `isTabbableRadio` from `FocusScope.tsx`.
///
/// A radio is tabbable if:
/// - It is checked, OR
/// - No radio in its group is checked (the first one encountered by the `TreeWalker`
///   will be accepted, and all subsequent ones in the same group will be skipped
///   via the `from_radio_group` check in the walker).
pub fn is_tabbable_radio(element: &web_sys::HtmlInputElement) -> bool {
    if element.checked() {
        return true;
    }
    let radios = get_radios_in_group(element);
    !radios.is_empty() && !radios.iter().any(web_sys::HtmlInputElement::checked)
}

/// Returns true if the element is a text input (input[text-like], textarea, contenteditable).
/// Used by focus-visible to suppress keyboard modality on regular typing in text fields.
pub(crate) fn is_text_input(element: &web_sys::Element) -> bool {
    if let Some(input) = element.dyn_ref::<web_sys::HtmlInputElement>() {
        return !NON_TEXT_INPUT_TYPES.contains(&input.type_().as_str());
    }
    if element.dyn_ref::<web_sys::HtmlTextAreaElement>().is_some() {
        return true;
    }
    if let Some(el) = element.dyn_ref::<web_sys::HtmlElement>() {
        if el.is_content_editable() {
            return true;
        }
    }
    false
}

const NON_TEXT_INPUT_TYPES: &[&str] = &[
    "checkbox", "radio", "range", "color", "file", "image", "button", "submit", "reset",
];

/// Returns true if either the given element or the document's active element is a text input.
/// Matches react-aria's `isKeyboardFocusEvent` behavior.
///
/// Uses shadow-DOM-aware `get_active_element` to pierce shadow roots.
pub(crate) fn is_text_input_or_active_text_input(
    element: &web_sys::Element,
    document: &web_sys::Document,
) -> bool {
    if is_text_input(element) {
        return true;
    }
    if let Some(active) = super::shadow_dom::get_active_element(document) {
        return is_text_input(&active);
    }
    false
}
