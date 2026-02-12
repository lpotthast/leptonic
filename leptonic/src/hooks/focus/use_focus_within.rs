use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use leptos_use::use_event_listener;
use wasm_bindgen::JsCast;
use web_sys::FocusEvent;

use crate::utils::{EventAccessors, EventHandler, EventTargetExt};
use crate::hooks::IntoAttrs;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/interactions/src/useFocusWithin.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Event fired when focus enters or leaves an element tree.
#[derive(Debug, Clone)]
pub struct FocusWithinEvent {
    /// The underlying focus event.
    pub event: FocusEvent,
}

/// Input parameters for the `use_focus_within` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseFocusWithinInput {
    /// Whether focus within events should be disabled.
    pub disabled: Signal<bool>,

    /// Handler called when focus enters the target element or any descendant.
    pub on_focus_within: Option<Callback<FocusWithinEvent>>,

    /// Handler called when focus leaves the target element and all descendants.
    pub on_blur_within: Option<Callback<FocusWithinEvent>>,

    /// Handler called when the focus within state changes.
    pub on_focus_within_change: Option<Callback<bool>>,
}

/// The return value of the `use_focus_within` hook.
#[derive(Debug)]
pub struct UseFocusWithinReturn {
    /// Props for programmatic merging. Call `.into_attrs()` for view spreading.
    pub props: UseFocusWithinProps,

    /// Whether focus is currently within the element.
    pub is_focus_within: Signal<bool>,
}

/// Props from `use_focus_within` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseFocusWithinProps {
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
}

impl IntoAttrs for UseFocusWithinProps {
    type Attrs = UseFocusWithinAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
        )
    }
}

/// These attributes must be spread onto the target element.
pub type UseFocusWithinAttrs = (
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
);

/// Handles focus events for a target element and all its descendants.
///
/// Unlike `use_focus`, which only fires when the element itself receives focus,
/// `use_focus_within` fires when focus enters or leaves the element tree.
/// This is useful for components like dropdown menus or form groups that need
/// to track when focus is anywhere within them.
///
/// # Example
///
/// ```ignore
/// let focus_within = use_focus_within(UseFocusWithinInput {
///     disabled: Signal::derive(|| false),
///     on_focus_within: Some(Callback::new(|_| {
///         // Focus entered the element tree
///     })),
///     on_blur_within: Some(Callback::new(|_| {
///         // Focus left the element tree
///     })),
///     on_focus_within_change: Some(Callback::new(|is_focused| {
///         // Focus state changed
///     })),
/// });
///
/// view! {
///     <div {..focus_within.attrs}>
///         <input type="text" />
///         <button>"Submit"</button>
///     </div>
/// }
/// ```
#[allow(clippy::too_many_lines)]
pub fn use_focus_within(input: UseFocusWithinInput) -> UseFocusWithinReturn {
    let UseFocusWithinInput {
        disabled,
        on_focus_within,
        on_blur_within,
        on_focus_within_change,
    } = input;

    let (is_focus_within, set_is_focus_within) = signal(false);

    // Store cleanup function for global focus listener
    let global_listener_cleanup: StoredValue<Option<Box<dyn Fn()>>, LocalStorage> =
        StoredValue::new_local(None);

    let cleanup_global_listener = move || {
        global_listener_cleanup.update_value(|cleanup| {
            if let Some(cleanup_fn) = cleanup.take() {
                cleanup_fn();
            }
        });
    };

    let trigger_blur_within = move |e: FocusEvent| {
        if !is_focus_within.get_untracked() {
            return;
        }

        set_is_focus_within.set(false);
        cleanup_global_listener();

        if let Some(on_blur_within) = on_blur_within {
            on_blur_within.run(FocusWithinEvent { event: e });
        }

        if let Some(on_focus_within_change) = on_focus_within_change {
            on_focus_within_change.run(false);
        }
    };

    // Handle focus entering the element tree
    let handle_focus_in = {
        move |e: FocusEvent| {
            if disabled.get_untracked() {
                return;
            }

            // Ignore events bubbling through portals - check if target is contained in current_target
            let current_target = e.expect_current_target();
            let target = e.expect_target();

            if let (Some(current_el), Some(target_node)) = (
                current_target.dyn_ref::<web_sys::Node>(),
                target.dyn_ref::<web_sys::Node>(),
            ) {
                if !current_el.contains(Some(target_node)) {
                    return;
                }
            }

            // Check if focus is actually on the target
            // Use owner document from current_target to correctly handle iframes/shadow DOM
            let document = current_target
                .dyn_ref::<web_sys::Node>()
                .and_then(web_sys::Node::owner_document);
            let active_element = document
                .as_ref()
                .and_then(web_sys::Document::active_element);
            let target_element = target.to_element();

            if active_element != target_element {
                return;
            }

            if !is_focus_within.get_untracked() {
                // Fire focus within event
                if let Some(on_focus_within) = on_focus_within {
                    on_focus_within.run(FocusWithinEvent { event: e.clone() });
                }

                if let Some(on_focus_within_change) = on_focus_within_change {
                    on_focus_within_change.run(true);
                }

                set_is_focus_within.set(true);

                // Set up focus listener on the element's owner document to detect focus moving outside
                // This handles cases where elements are removed from DOM (which don't fire blur)
                // Using owner document correctly handles iframes and shadow DOM
                let document = current_target
                    .dyn_ref::<web_sys::Node>()
                    .and_then(web_sys::Node::owner_document);
                if let Some(document) = document {
                    let cleanup =
                        use_event_listener(document, ev::focus, move |focus_e: FocusEvent| {
                            if !is_focus_within.get_untracked() {
                                return;
                            }

                            // Check if the new focus target is outside our element
                            let focus_target = focus_e.expect_target();
                            if let Some(current_node) = current_target.dyn_ref::<web_sys::Node>() {
                                if let Some(target_node) = focus_target.dyn_ref::<web_sys::Node>() {
                                    if !current_node.contains(Some(target_node)) {
                                        // Focus moved outside - trigger blur
                                        trigger_blur_within(focus_e);
                                    }
                                }
                            }
                        });

                    global_listener_cleanup.set_value(Some(Box::new(cleanup)));
                }
            }
        }
    };

    // Handle focus leaving - but only if it's actually leaving the element tree
    let handle_focus_out = move |e: FocusEvent| {
        if disabled.get_untracked() {
            return;
        }

        // Ignore events bubbling through portals
        let current_target = e.expect_current_target();
        let target = e.expect_target();

        if let (Some(current_el), Some(target_node)) = (
            current_target.dyn_ref::<web_sys::Node>(),
            target.dyn_ref::<web_sys::Node>(),
        ) {
            if !current_el.contains(Some(target_node)) {
                return;
            }
        }

        // Check if focus is moving to another element within the same tree
        // If relatedTarget (where focus is going) is within current_target, don't trigger blur
        if let Some(related_target) = e.related_target() {
            if let Some(current_node) = current_target.dyn_ref::<web_sys::Node>() {
                if let Some(related_node) = related_target.dyn_ref::<web_sys::Node>() {
                    if current_node.contains(Some(related_node)) {
                        // Focus is moving within the tree, don't trigger blur
                        return;
                    }
                }
            }
        }

        // Focus is leaving the element tree
        trigger_blur_within(e);
    };

    // Cleanup on unmount
    on_cleanup(move || {
        cleanup_global_listener();
    });

    UseFocusWithinReturn {
        props: UseFocusWithinProps {
            on_focusin: EventHandler::new(handle_focus_in),
            on_focusout: EventHandler::new(handle_focus_out),
        },
        is_focus_within: is_focus_within.into(),
    }
}
