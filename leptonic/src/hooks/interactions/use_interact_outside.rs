use crate::utils::{element_capture, ElementCaptureAttr};
use leptos::prelude::*;
use leptos_use::{use_event_listener_with_options, UseEventListenerOptions};
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use web_sys::PointerEvent;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/interactions/src/useInteractOutside.ts
//
// ## DEVIATIONS FROM REACT-ARIA
//
// ### Omitted
// - Legacy mouse/touch fallback (`process.env.NODE_ENV === 'test'` branch) — WASM always has `PointerEvent`.
//
// ### Different
// - Click handler passes `MouseEvent` as `PointerEvent` via `unchecked_into` (matches react-aria's
//   JS loose typing — PointerEvent-specific fields return defaults).
// - Document containment uses native `Node.contains()` instead of react-aria's shadow-DOM-aware
//   `nodeContains()`. The `composedPath()` check handles the primary shadow DOM case.
//
// ### Leptos-specific
// - Uses `ElementCaptureAttr` instead of `RefObject`.

/// Input parameters for the `use_interact_outside` hook.
#[derive(Clone)]
pub struct UseInteractOutsideInput {
    /// Whether the interact outside events should be disabled.
    pub disabled: Signal<bool>,

    /// Handler called when an interaction starts outside the element.
    pub on_interact_outside_start: Option<Callback<PointerEvent>>,

    /// Handler called when an interaction completes outside the element.
    pub on_interact_outside: Option<Callback<PointerEvent>>,
}

#[derive(Clone)]
pub struct UseInteractOutsideReturn {
    /// Props for programmatic merging. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub props: UseInteractOutsideProps,
}

/// Props from `use_interact_outside` that can be extracted and merged programmatically.
#[derive(Debug, Clone)]
pub struct UseInteractOutsideProps {
    pub element_capture: ElementCaptureAttr,
}

impl UseInteractOutsideProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseInteractOutsideAttrs {
        (self.element_capture.clone(),)
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseInteractOutsideAttrs {
        (self.element_capture,)
    }
}

pub type UseInteractOutsideAttrs = (ElementCaptureAttr,);

/// Detects when the user interacts (clicks/touches) outside of a specified element.
///
/// This hook is commonly used by dialogs, popovers, and dropdown menus to close
/// when the user clicks outside of them.
///
/// The hook uses a two-phase approach:
/// 1. On `pointerdown`, it checks if the interaction started outside and calls `on_interact_outside_start`
/// 2. On `click`, if the interaction started outside and completed outside, it calls `on_interact_outside`
///
/// This two-phase approach allows drag interactions that start inside but end outside
/// to not trigger the outside interaction handler.
///
/// Both listeners use the capture phase so that outside-click detection works even when
/// child elements call `stopPropagation()`.
///
/// # Example
///
/// ```ignore
/// let interact_outside = use_interact_outside(UseInteractOutsideInput {
///     disabled: Signal::derive(|| false),
///     on_interact_outside_start: None,
///     on_interact_outside: Some(Callback::new(|_| {
///         // Close the popover/dialog
///     })),
/// });
///
/// view! {
///     <div {..interact_outside.props.into_attrs()}>
///         "Click outside to close"
///     </div>
/// }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_interact_outside(input: UseInteractOutsideInput) -> UseInteractOutsideReturn {
    // Storage for the captured element - will be populated by ElementCaptureAttr
    let element_storage: StoredValue<Option<SendWrapper<web_sys::Element>>> =
        StoredValue::new(None);

    let is_pointer_down: StoredValue<bool, LocalStorage> = StoredValue::new_local(false);

    let on_interact_outside = input.on_interact_outside;
    let on_interact_outside_start = input.on_interact_outside_start;
    let disabled = input.disabled;

    // Set up pointer down listener to track interaction start
    Effect::new(move |_| {
        if disabled.get() {
            return;
        }

        // Get document from the captured element's owner document
        // This correctly handles elements in iframes or shadow DOM
        let document = element_storage
            .read_value()
            .as_ref()
            .and_then(|el| el.owner_document());

        let Some(document) = document else {
            return;
        };

        let on_interact_outside_start = on_interact_outside_start;

        let _cleanup_pointerdown = use_event_listener_with_options(
            document.clone(),
            leptos::ev::pointerdown,
            move |e: PointerEvent| {
                if disabled.get_untracked() {
                    return;
                }

                if on_interact_outside.is_some()
                    && is_valid_event(&e, element_storage.read_value().as_ref())
                {
                    if let Some(on_interact_outside_start) = on_interact_outside_start {
                        on_interact_outside_start.run(e);
                    }
                    is_pointer_down.set_value(true);
                }
            },
            UseEventListenerOptions::default().capture(true),
        );

        let on_interact_outside = on_interact_outside;

        let _cleanup_click = use_event_listener_with_options(
            document,
            leptos::ev::click,
            move |e: web_sys::MouseEvent| {
                if disabled.get_untracked() {
                    return;
                }

                if is_pointer_down.get_value()
                    && is_valid_event(&e, element_storage.read_value().as_ref())
                {
                    if let Some(on_interact_outside) = on_interact_outside {
                        on_interact_outside.run(e.unchecked_into::<PointerEvent>());
                    }
                }
                is_pointer_down.set_value(false);
            },
            UseEventListenerOptions::default().capture(true),
        );
    });

    UseInteractOutsideReturn {
        props: UseInteractOutsideProps {
            element_capture: element_capture(move |el| {
                element_storage.set_value(Some(SendWrapper::new(el)));
            }),
        },
    }
}

/// Check if a pointer/mouse event is valid (outside the element and meets other criteria).
///
/// Works for both `PointerEvent` (from pointerdown) and `MouseEvent` (from click) because
/// `PointerEvent` derefs to `MouseEvent` in the web-sys type hierarchy.
fn is_valid_event(
    event: &web_sys::MouseEvent,
    element: Option<&SendWrapper<web_sys::Element>>,
) -> bool {
    // Only handle primary button (left click)
    if event.button() > 0 {
        return false;
    }

    // Check if target is still in the document
    if let Some(target) = event.target() {
        if let Some(target_node) = target.dyn_ref::<web_sys::Node>() {
            let owner_document = target_node.owner_document();
            if let Some(doc) = owner_document {
                if let Some(doc_element) = doc.document_element() {
                    if !doc_element.contains(Some(target_node)) {
                        return false;
                    }
                }
            }
        }

        // Check if target is within a top layer element (e.g. toasts)
        if let Some(target_el) = target.dyn_ref::<web_sys::Element>() {
            if target_el
                .closest("[data-react-aria-top-layer]")
                .ok()
                .flatten()
                .is_some()
            {
                return false;
            }
        }
    }

    // Check if we have an element to compare against
    let Some(el) = element else {
        return false;
    };
    // Dereference SendWrapper to get the actual element
    let el: &web_sys::Element = el;

    // Check if the event target is outside our element using composedPath
    // This handles shadow DOM correctly
    let composed_path = event.composed_path();
    for i in 0..composed_path.length() {
        let path_el = composed_path.get(i);
        if !path_el.is_undefined() && !path_el.is_null() {
            if let Some(path_node) = path_el.dyn_ref::<web_sys::Element>() {
                if path_node == el {
                    // Event target is inside the element
                    return false;
                }
            }
        }
    }

    // Event is outside the element
    true
}
