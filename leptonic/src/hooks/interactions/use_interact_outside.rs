#![cfg_attr(feature = "ssr", allow(dead_code, unused_imports))]

use leptos::prelude::*;
use leptos_use::{UseEventListenerOptions, use_event_listener_with_options};
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use web_sys::PointerEvent;

use crate::{
    hooks::IntoAttrs,
    utils::{CapturedElement, ElementCaptureAttr, EventAccessors, dom_ext::node_contains},
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/interactions/src/useInteractOutside.ts

// ## OMITTED FEATURES
//
// - Legacy mouse/touch fallback (`process.env.NODE_ENV === 'test'` branch) — WASM always has `PointerEvent`.
//
// ## DIFFERENT BEHAVIOR
//
// - `on_interact_outside` callback receives `MouseEvent` (from the click event) rather than
//   react-aria's loose `PointerEvent` typing. `on_interact_outside_start` receives a real
//   `PointerEvent` from the pointerdown listener.
//
// ## LEPTOS-SPECIFIC ADAPTATIONS
//
// - Uses `ElementCaptureAttr` instead of `RefObject`.

/// Input parameters for the `use_interact_outside` hook.
#[derive(Debug, Clone)]
pub struct UseInteractOutsideInput {
    /// Whether the interact outside events should be disabled.
    pub disabled: Signal<bool>,

    /// Handler called when an interaction starts outside the element.
    pub on_interact_outside_start: Option<Callback<PointerEvent>>,

    /// Handler called when an interaction completes outside the element.
    /// Receives a `MouseEvent` from the click listener (not `PointerEvent`).
    pub on_interact_outside: Option<Callback<web_sys::MouseEvent>>,
}

#[derive(Debug)]
pub struct UseInteractOutsideReturn {
    /// Props for programmatic merging. Call `.into_attrs()` for view spreading.
    pub props: UseInteractOutsideProps,
}

/// Props from `use_interact_outside` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseInteractOutsideProps {
    pub element_capture: ElementCaptureAttr,
}

impl IntoAttrs for UseInteractOutsideProps {
    type Attrs = UseInteractOutsideAttrs;

    fn into_attrs(self) -> Self::Attrs {
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
    #[cfg(feature = "ssr")]
    {
        let _ = input;
        let element = CapturedElement::new();
        return UseInteractOutsideReturn {
            props: UseInteractOutsideProps {
                element_capture: element.attr(),
            },
        };
    }

    #[cfg(not(feature = "ssr"))]
    {
        let UseInteractOutsideInput {
            disabled,
            on_interact_outside_start,
            on_interact_outside,
        } = input;

        let element = CapturedElement::new();

        let is_pointer_down: StoredValue<bool, LocalStorage> = StoredValue::new_local(false);

        // Set up pointer down listener to track interaction start.
        // Uses `element.get()` (reactive) so the Effect re-runs when the element
        // is captured — critical for client-side navigation where the element may
        // not exist yet when the Effect first runs.
        Effect::new(move |_| {
            // Reset pointer-down tracking when listeners are re-created.
            // Prevents stale state from a previous overlay lifecycle (e.g., if
            // the overlay closed via Escape while the pointer was down, the old
            // click handler is cleaned up before it can reset is_pointer_down).
            is_pointer_down.set_value(false);

            if disabled.get() {
                return;
            }

            // Get document from the captured element's owner document.
            // This correctly handles elements in iframes or shadow DOM.
            let document = element.get().as_ref().and_then(|el| el.owner_document());

            let Some(document) = document else {
                return;
            };

            let _cleanup_pointerdown = use_event_listener_with_options(
                document.clone(),
                leptos::ev::pointerdown,
                move |e: PointerEvent| {
                    if disabled.get_untracked() {
                        return;
                    }

                    if on_interact_outside.is_some()
                        && is_valid_event(&e, element.get_untracked().as_ref())
                    {
                        if let Some(on_interact_outside_start) = on_interact_outside_start {
                            on_interact_outside_start.run(e);
                        }
                        is_pointer_down.set_value(true);
                    }
                },
                UseEventListenerOptions::default().capture(true),
            );

            let _cleanup_click = use_event_listener_with_options(
                document,
                leptos::ev::click,
                move |e: web_sys::MouseEvent| {
                    if !disabled.get_untracked()
                        && is_pointer_down.get_value()
                        && is_valid_event(&e, element.get_untracked().as_ref())
                    {
                        if let Some(on_interact_outside) = on_interact_outside {
                            on_interact_outside.run(e);
                        }
                    }
                    is_pointer_down.set_value(false);
                },
                UseEventListenerOptions::default().capture(true),
            );
        });

        UseInteractOutsideReturn {
            props: UseInteractOutsideProps {
                element_capture: element.attr(),
            },
        }
    }
}

/// Check if a pointer/mouse event is valid (outside the element and meets other criteria).
///
/// Works for both `PointerEvent` (from pointerdown) and `MouseEvent` (from click) because
/// `PointerEvent` derefs to `MouseEvent` in the web-sys type hierarchy.
///
/// Uses shadow-DOM-aware `node_contains` for containment checks, matching react-aria's
/// `nodeContains` behavior.
fn is_valid_event(
    event: &web_sys::MouseEvent,
    element: Option<&SendWrapper<web_sys::Element>>,
) -> bool {
    // Only handle primary button (left click)
    if event.button() > 0 {
        return false;
    }

    // Check if target is still in the document (shadow-DOM-aware)
    let target = event.expect_target();
    if let Some(target_node) = target.dyn_ref::<web_sys::Node>() {
        let owner_document = target_node.owner_document();
        if let Some(doc) = owner_document {
            if let Some(doc_element) = doc.document_element() {
                if !node_contains(Some(doc_element.as_ref()), Some(target_node)).unwrap_or(false) {
                    return false;
                }
            }
        }
    }

    // Check if target is within a top layer element (e.g. toasts)
    if let Some(target_el) = target.dyn_ref::<web_sys::Element>() {
        if target_el
            .closest("[data-leptonic-top-layer]")
            .ok()
            .flatten()
            .is_some()
        {
            return false;
        }
    }

    // Check if we have an element to compare against
    let Some(el) = element else {
        return false;
    };
    // Dereference SendWrapper to get the actual element
    let el: &web_sys::Element = el;

    // Check if the event target is inside our element (shadow-DOM-aware).
    // node_contains traverses shadow DOM boundaries via slot assignments and shadow root hosts.
    if node_contains(Some(el.as_ref()), target.dyn_ref::<web_sys::Node>()).unwrap_or(false) {
        return false;
    }

    // Event is outside the element
    true
}
