// Upstream: react-aria/src/interactions/useInteractOutside.ts @ 99e6102368
// Upstream: react-aria/test/interactions/useInteractOutside.test.js @ 99e6102368
use leptos::prelude::*;

use crate::{CapturedElement, ElementCaptureAttr, IntoAttrs};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Both callbacks get an `InteractOutsideEvent` (the `pointerdown` that starts the interaction
//   or the `click` that completes it; react-aria's handlers get the untyped DOM event), whose
//   `target` is the element interacted with (inside open shadow roots too).
// - The element is captured by the returned props, or given as `element` (react-aria: `ref`).
// - Top-layer elements are marked `data-leptonic-top-layer` (react-aria:
//   `data-react-aria-top-layer`).
//
// ## OMITTED FEATURES
// - The mouse/touch fallback for environments without `PointerEvent` (react-aria uses it in tests
//   only): every supported browser has pointer events.
//
// =============================================================================

/// An interaction outside the element: the `pointerdown` that starts it
/// ([`on_interact_outside_start`](UseInteractOutsideInput::on_interact_outside_start)) or the
/// `click` that completes it ([`on_interact_outside`](UseInteractOutsideInput::on_interact_outside)).
#[derive(Debug, Clone)]
pub struct InteractOutsideEvent {
    event: web_sys::MouseEvent,
}

impl InteractOutsideEvent {
    /// The element interacted with: the event's target, inside an open shadow root the element
    /// there (react-aria's `getEventTarget`).
    pub fn target(&self) -> web_sys::EventTarget {
        use crate::utils::dom_ext::EventAccessors;

        crate::utils::shadow_dom::get_event_target(&self.event)
            .unwrap_or_else(|| self.event.expect_target())
    }

    /// Keeps the event from reaching the element interacted with (its default action still
    /// happens).
    pub fn stop_propagation(&self) {
        self.event.stop_propagation();
    }

    /// Prevents the event's default action.
    pub fn prevent_default(&self) {
        self.event.prevent_default();
    }

    /// The DOM event (a `PointerEvent` for the start, a `MouseEvent` for the click).
    pub fn event(&self) -> &web_sys::MouseEvent {
        &self.event
    }
}

/// Input parameters for the `use_interact_outside` hook.
#[derive(Debug, Clone, Default)]
pub struct UseInteractOutsideInput {
    /// Whether the interact outside events should be disabled.
    pub is_disabled: Signal<bool>,

    /// Handler called when an interaction starts outside the element (with the `pointerdown`).
    pub on_interact_outside_start: Option<Callback<InteractOutsideEvent>>,

    /// Handler called when an interaction completes outside the element (with the `click`).
    pub on_interact_outside: Option<Callback<InteractOutsideEvent>>,
    /// The element interactions are outside of (e.g. a popover group). Default: the element the
    /// returned props are spread on (the props then capture nothing).
    pub element: Option<CapturedElement>,
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
///     is_disabled: Signal::derive(|| false),
///     on_interact_outside_start: None,
///     on_interact_outside: Some(Callback::new(|_| {
///         // Close the popover/dialog
///     })),
///     element: None,
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
        UseInteractOutsideReturn {
            props: UseInteractOutsideProps {
                element_capture: element.attr(),
            },
        }
    }

    #[cfg(not(feature = "ssr"))]
    {
        use leptos_use::{UseEventListenerOptions, use_event_listener_with_options};
        use web_sys::PointerEvent;

        let UseInteractOutsideInput {
            is_disabled: disabled,
            on_interact_outside_start,
            on_interact_outside,
            element: external,
        } = input;

        let element = external.unwrap_or_else(CapturedElement::new);
        // An external element is captured elsewhere; the props must not capture another one.
        let capture = if external.is_some() {
            CapturedElement::new()
        } else {
            element
        };

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
                            on_interact_outside_start.run(InteractOutsideEvent { event: e.into() });
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
                        && let Some(on_interact_outside) = on_interact_outside
                    {
                        on_interact_outside.run(InteractOutsideEvent { event: e });
                    }
                    is_pointer_down.set_value(false);
                },
                UseEventListenerOptions::default().capture(true),
            );
        });

        UseInteractOutsideReturn {
            props: UseInteractOutsideProps {
                element_capture: capture.attr(),
            },
        }
    }
}

/// Whether a pointer/mouse event counts as an interaction outside `element` (react-aria's
/// `isValidEvent`): a primary button, a target still in the document and not in a top layer, and
/// `element` not on the event's composed path (which also sees into open shadow roots).
#[cfg(not(feature = "ssr"))]
fn is_valid_event(
    event: &web_sys::MouseEvent,
    element: Option<&send_wrapper::SendWrapper<web_sys::Element>>,
) -> bool {
    use wasm_bindgen::JsCast;

    use crate::utils::shadow_dom::{get_event_target, node_contains};

    if event.button() > 0 {
        return false;
    }
    if let Some(target) =
        get_event_target(event).and_then(|t| t.dyn_into::<web_sys::Element>().ok())
    {
        // A target no longer in the document.
        let Some(document_element) = target
            .owner_document()
            .and_then(|document| document.document_element())
        else {
            return false;
        };
        if !node_contains(&document_element, &target) {
            return false;
        }
        // A target in a top layer element (e.g. toasts).
        if target
            .closest("[data-leptonic-top-layer]")
            .ok()
            .flatten()
            .is_some()
        {
            return false;
        }
    }
    let Some(element) = element else {
        return false;
    };
    let element: &web_sys::Element = element;
    !event.composed_path().includes(element, 0)
}
