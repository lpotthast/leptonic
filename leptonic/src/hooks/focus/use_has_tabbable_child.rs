use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::utils::element_capture::{CapturedElement, ElementCaptureAttr};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/focus/src/useHasTabbableChild.ts
//
// ## React-aria deviation
//
// **React-aria pattern**: `useHasTabbableChild(scopeRef)` - caller passes a ref as parameter.
//
// **Leptonic pattern**: `use_has_tabbable_child(input)` returns props with `ElementCaptureAttr` that
// automatically captures the element when spread.
//
// This is a deliberate deviation for better ergonomics - users don't need to manually create
// and wire up NodeRefs. The element is captured automatically when attributes are spread.

/// Input parameters for the `use_has_tabbable_child` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseHasTabbableChildInput {
    /// Whether the check should be disabled.
    pub disabled: Signal<bool>,
}

impl Default for UseHasTabbableChildInput {
    fn default() -> Self {
        Self {
            disabled: Signal::derive(|| false),
        }
    }
}

/// The return value of the `use_has_tabbable_child` hook.
#[derive(Debug, Clone)]
pub struct UseHasTabbableChildReturn {
    /// Whether the element has at least one tabbable child.
    pub has_tabbable_child: Signal<bool>,

    /// Props for programmatic merging. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub props: UseHasTabbableChildProps,
}

/// Props from `use_has_tabbable_child` that can be extracted and merged programmatically.
#[derive(Debug, Clone)]
pub struct UseHasTabbableChildProps {
    pub element_capture: ElementCaptureAttr,
}

impl UseHasTabbableChildProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseHasTabbableChildAttrs {
        (self.element_capture.clone(),)
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseHasTabbableChildAttrs {
        (self.element_capture,)
    }
}

/// These attributes must be spread onto the target element: `<foo {..attrs} />`
pub type UseHasTabbableChildAttrs = (ElementCaptureAttr,);

/// Selector for potentially tabbable elements.
const TABBABLE_SELECTOR: &str = "input:not([disabled]):not([type=hidden]), select:not([disabled]), textarea:not([disabled]), button:not([disabled]), a[href], area[href], summary, iframe, object, embed, audio[controls], video[controls], [contenteditable]:not([contenteditable=false]), [tabindex]:not([tabindex=\"-1\"])";

/// Checks whether an element has any tabbable (focusable via Tab key) children.
///
/// This is useful for determining whether to make a container itself focusable
/// when it might contain focusable children.
///
/// The hook automatically captures the DOM element through [`ElementCaptureAttr`],
/// so you don't need to create or pass a `NodeRef`. Just spread the props
/// onto your element and the detection works automatically.
///
/// # Example
///
/// ```ignore
/// let UseHasTabbableChildReturn { has_tabbable_child, props } =
///     use_has_tabbable_child(UseHasTabbableChildInput::default());
///
/// view! {
///     <div
///         {..props.into_attrs()}
///         tabindex=move || if has_tabbable_child.get() { -1 } else { 0 }
///     >
///         <button>"Child button"</button>
///     </div>
/// }
/// ```
pub fn use_has_tabbable_child(input: UseHasTabbableChildInput) -> UseHasTabbableChildReturn {
    let UseHasTabbableChildInput { disabled } = input;

    let (has_tabbable_child, set_has_tabbable_child) = signal(false);

    let element = CapturedElement::new();

    // Check for tabbable children when element is captured or disabled changes.
    // Uses `element.get()` (reactive) so the Effect re-runs when the element
    // is captured — critical for client-side navigation.
    Effect::new(move |_| {
        if disabled.get() {
            set_has_tabbable_child.set(false);
            return;
        }

        let Some(el) = element.get() else {
            set_has_tabbable_child.set(false);
            return;
        };

        let has_tabbable = has_tabbable_element(&el);
        set_has_tabbable_child.set(has_tabbable);
    });

    UseHasTabbableChildReturn {
        has_tabbable_child: has_tabbable_child.into(),
        props: UseHasTabbableChildProps {
            element_capture: element.attr(),
        },
    }
}

/// Check if an element has any tabbable descendant elements.
fn has_tabbable_element(element: &web_sys::Element) -> bool {
    // Query for potentially tabbable elements
    let Ok(nodes) = element.query_selector_all(TABBABLE_SELECTOR) else {
        return false;
    };

    // Check each element to see if it's actually tabbable
    for i in 0..nodes.length() {
        let Some(node) = nodes.get(i) else {
            continue;
        };

        let Some(el) = node.dyn_ref::<web_sys::HtmlElement>() else {
            continue;
        };

        if is_tabbable(el) {
            return true;
        }
    }

    false
}

/// Check if a specific element is tabbable.
fn is_tabbable(element: &web_sys::HtmlElement) -> bool {
    // Check if element is visible
    if !is_element_visible(element) {
        return false;
    }

    // Check tabindex
    let tab_index = element.tab_index();
    if tab_index < 0 {
        return false;
    }

    // Check if it's a disabled form element
    if let Some(input) = element.dyn_ref::<web_sys::HtmlInputElement>() {
        if input.disabled() {
            return false;
        }
        // Hidden inputs are not tabbable
        if input.type_() == "hidden" {
            return false;
        }
    }

    if let Some(button) = element.dyn_ref::<web_sys::HtmlButtonElement>() {
        if button.disabled() {
            return false;
        }
    }

    if let Some(select) = element.dyn_ref::<web_sys::HtmlSelectElement>() {
        if select.disabled() {
            return false;
        }
    }

    if let Some(textarea) = element.dyn_ref::<web_sys::HtmlTextAreaElement>() {
        if textarea.disabled() {
            return false;
        }
    }

    true
}

/// Check if an element is visible (not hidden via CSS).
fn is_element_visible(element: &web_sys::HtmlElement) -> bool {
    // Check if element or any ancestor is hidden
    // Use owner document's default view to correctly handle iframes/shadow DOM
    let Some(window) = element.owner_document().and_then(|d| d.default_view()) else {
        return true;
    };

    let Ok(Some(style)) = window.get_computed_style(element) else {
        return true;
    };

    // Check display
    if let Ok(display) = style.get_property_value("display") {
        if display == "none" {
            return false;
        }
    }

    // Check visibility
    if let Ok(visibility) = style.get_property_value("visibility") {
        if visibility == "hidden" || visibility == "collapse" {
            return false;
        }
    }

    // Check if element has zero size (often used for hiding)
    let rect = element.get_bounding_client_rect();
    if rect.width() == 0.0 && rect.height() == 0.0 {
        // Could be collapsed, but might still be tabbable if it has size from children
        // Check the offset dimensions as well
        if element.offset_width() == 0 && element.offset_height() == 0 {
            return false;
        }
    }

    true
}
