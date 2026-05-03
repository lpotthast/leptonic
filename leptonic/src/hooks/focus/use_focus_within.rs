#![cfg_attr(feature = "ssr", allow(unused_imports))]

use leptos::{
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use leptos_use::use_event_listener;
use wasm_bindgen::JsCast;
use web_sys::FocusEvent;

use crate::{
    hooks::IntoAttrs,
    utils::{
        EventAccessors, EventHandler, EventTargetExt, dom_ext::node_contains, set_event_target,
        synthetic_blur,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/interactions/src/useFocusWithin.ts

// ## DIFFERENT BEHAVIOR
//
// - `is_focus_within` reactive signal
//   React-aria does not expose a signal for the current focus-within state.
//   Leptonic returns `is_focus_within: Signal<bool>` for convenient reactive
//   use in views (e.g., conditional styling).
//
// - Reactive `disabled` prop with auto-cleanup
//   React-aria checks `disabled` only in handler closures.
//   Leptonic additionally watches `disabled` reactively: when it transitions
//   to `true`, any active focus-within state and global listener are cleaned up.
//
// - Native `focusin`/`focusout` events
//   React-aria synthesises focus/blur callbacks from `onFocus`/`onBlur` React
//   events (which are actually `focusin`/`focusout` under the hood). Leptonic
//   attaches native `focusin`/`focusout` listeners directly.
//
// - Global `focusin` listener (bubbling) instead of capture-phase `focus`
//   React-aria attaches a capture-phase `focus` listener on the document.
//   Leptonic uses a bubbling `focusin` listener, which achieves the same
//   document-level detection since `focusin` bubbles natively.
//
// - Defensive `try_get_untracked` in event handlers
//   When an element is removed from the DOM (e.g., popover closed by scroll),
//   the browser fires synthetic focusout events during teardown. By that point
//   the Leptos reactive scope may already be disposed. We use
//   `try_get_untracked().unwrap_or(...)` to treat a disposed signal as
//   disabled/inactive rather than panicking. React-aria does not face this
//   issue because React's synthetic event system defers cleanup.

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
    #[cfg(feature = "ssr")]
    {
        let _ = input;
        let (is_focus_within, _) = signal(false);
        return UseFocusWithinReturn {
            props: UseFocusWithinProps {
                on_focusin: EventHandler::new(|_: FocusEvent| {}),
                on_focusout: EventHandler::new(|_: FocusEvent| {}),
            },
            is_focus_within: is_focus_within.into(),
        };
    }

    #[cfg(not(feature = "ssr"))]
    {
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

        // Store cleanup function for synthetic blur MutationObserver (Firefox workaround).
        let blur_observer_cleanup: StoredValue<Option<Box<dyn Fn()>>, LocalStorage> =
            StoredValue::new_local(None);

        let cleanup_blur_observer = move || {
            blur_observer_cleanup.update_value(|cleanup| {
                if let Some(cleanup_fn) = cleanup.take() {
                    cleanup_fn();
                }
            });
        };

        let trigger_blur_within = move |e: FocusEvent| {
            if !is_focus_within.try_get_untracked().unwrap_or(false) {
                return;
            }

            set_is_focus_within.set(false);
            cleanup_global_listener();
            cleanup_blur_observer();

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
                if disabled.try_get_untracked().unwrap_or(true) {
                    return;
                }

                // Ignore events bubbling through portals — use shadow-DOM-aware containment.
                let current_target = e.expect_current_target();
                let target = e.expect_target();

                if !node_contains(
                    current_target.dyn_ref::<web_sys::Node>(),
                    target.dyn_ref::<web_sys::Node>(),
                )
                .unwrap_or(false)
                {
                    return;
                }

                // Check if focus is actually on the target.
                // Use owner document from current_target to correctly handle iframes/shadow DOM.
                let document = current_target
                    .dyn_ref::<web_sys::Node>()
                    .and_then(web_sys::Node::owner_document);
                let active_element = document
                    .as_ref()
                    .and_then(crate::utils::shadow_dom::get_active_element);
                let target_element = target.to_element();

                if active_element != target_element {
                    return;
                }

                if !is_focus_within.try_get_untracked().unwrap_or(true) {
                    // Fire focus within event
                    if let Some(on_focus_within) = on_focus_within {
                        on_focus_within.run(FocusWithinEvent { event: e.clone() });
                    }

                    if let Some(on_focus_within_change) = on_focus_within_change {
                        on_focus_within_change.run(true);
                    }

                    set_is_focus_within.set(true);

                    // Set up focusin listener on the element's owner document to detect focus
                    // moving outside. This handles cases where elements are removed from
                    // DOM (which don't fire blur/focusout). We use `focusin` instead of
                    // `focus` because `focus` does NOT bubble — a document-level `focus`
                    // listener in the bubble phase would never fire.
                    let document = current_target
                        .dyn_ref::<web_sys::Node>()
                        .and_then(web_sys::Node::owner_document);
                    if let Some(document) = document {
                        let cleanup = use_event_listener(
                            document,
                            ev::focusin,
                            move |focus_e: FocusEvent| {
                                if !is_focus_within.try_get_untracked().unwrap_or(false) {
                                    return;
                                }

                                // Check if the new focus target is outside our element.
                                // Use shadow-DOM-aware containment check.
                                let focus_target = focus_e.expect_target();
                                let is_outside = !node_contains(
                                    current_target.dyn_ref::<web_sys::Node>(),
                                    focus_target.dyn_ref::<web_sys::Node>(),
                                )
                                .unwrap_or(false);

                                if is_outside {
                                    // Focus moved outside — synthesize a proper blur event
                                    // (matching react-aria). The relatedTarget is the element
                                    // that received focus outside our tree.
                                    let blur_init = web_sys::FocusEventInit::new();
                                    blur_init.set_related_target(focus_e.target().as_ref());
                                    if let Ok(synthetic_blur) =
                                        web_sys::FocusEvent::new_with_focus_event_init_dict(
                                            "blur", &blur_init,
                                        )
                                    {
                                        // Set target and currentTarget on the synthetic event
                                        // to match the tracked element (react-aria's setEventTarget).
                                        set_event_target(
                                            &synthetic_blur,
                                            &current_target,
                                            &current_target,
                                        );
                                        trigger_blur_within(synthetic_blur);
                                    }
                                }
                            },
                        );

                        global_listener_cleanup.set_value(Some(Box::new(cleanup)));
                    }

                    // Set up synthetic blur observer for form elements (Firefox workaround:
                    // Firefox does not fire blur when a form element becomes disabled while focused).
                    if let Some(el) = target.dyn_ref::<web_sys::Element>() {
                        let cleanup = synthetic_blur::setup_synthetic_blur_observer(el);
                        blur_observer_cleanup.set_value(Some(cleanup));
                    }
                }
            }
        };

        // Handle focus leaving - but only if it's actually leaving the element tree
        let handle_focus_out = move |e: FocusEvent| {
            if disabled.try_get_untracked().unwrap_or(true) {
                return;
            }

            // Ignore events bubbling through portals — use shadow-DOM-aware containment.
            let current_target = e.expect_current_target();
            let target = e.expect_target();

            if !node_contains(
                current_target.dyn_ref::<web_sys::Node>(),
                target.dyn_ref::<web_sys::Node>(),
            )
            .unwrap_or(false)
            {
                return;
            }

            // Check if focus is moving to another element within the same tree.
            // If relatedTarget (where focus is going) is within current_target, don't trigger blur.
            if let Some(related_target) = e.related_target() {
                if node_contains(
                    current_target.dyn_ref::<web_sys::Node>(),
                    related_target.dyn_ref::<web_sys::Node>(),
                )
                .unwrap_or(false)
                {
                    return;
                }
            }

            // Focus is leaving the element tree
            trigger_blur_within(e);
        };

        // When disabled transitions to true, clean up any active focus-within state.
        Effect::new(move |_| {
            if disabled.get() && is_focus_within.get_untracked() {
                set_is_focus_within.set(false);
                cleanup_global_listener();
                cleanup_blur_observer();

                if let Some(on_focus_within_change) = on_focus_within_change {
                    on_focus_within_change.run(false);
                }
            }
        });

        // Cleanup on unmount
        on_cleanup(move || {
            cleanup_global_listener();
            cleanup_blur_observer();
        });

        UseFocusWithinReturn {
            props: UseFocusWithinProps {
                on_focusin: EventHandler::new(handle_focus_in),
                on_focusout: EventHandler::new(handle_focus_out),
            },
            is_focus_within: is_focus_within.into(),
        }
    }
}
