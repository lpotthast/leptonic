// Upstream: react-aria/src/interactions/useFocusWithin.ts @ 99e6102368
use leptos::{
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use web_sys::FocusEvent;

use crate::{hooks::IntoAttrs, utils::EventHandler};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Returns `is_focus_within: Signal<bool>` for views (react-aria keeps the state in a ref).
// - The callbacks get a `FocusWithinEvent` wrapping the native event (react-aria: React's
//   synthetic `FocusEvent`).
//
// ## DIFFERENT BEHAVIOR
// - `is_disabled` is reactive: becoming disabled while focus is within ends the focus-within
//   state (`on_focus_within_change(false)`) and removes the global listener. React-aria only
//   detaches its handlers.
// - Native `focusin`/`focusout` listeners (React's `onFocus`/`onBlur` are `focusin`/`focusout`
//   under the hood).
//
// ## LEPTOS-SPECIFIC ADAPTATIONS
// - The focus-out path reads signals with `try_get_untracked` and runs callbacks with `try_run`:
//   removing a focused element blurs it after its owner was disposed ("Blur After Disposal").
//
// =============================================================================

/// Event fired when focus enters or leaves an element tree. Like react-aria's focus events, it
/// doesn't stop propagation (other focus listeners above, e.g. an outer `use_focus_within`, must
/// see it), so it has no `Propagation`.
#[derive(Debug, Clone)]
pub struct FocusWithinEvent {
    /// The underlying focus event.
    pub event: FocusEvent,
}

/// Input parameters for the `use_focus_within` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseFocusWithinInput {
    /// Whether focus within events should be disabled.
    pub is_disabled: Signal<bool>,

    /// Handler called when focus enters the target element or any descendant.
    pub on_focus_within: Option<Callback<FocusWithinEvent>>,

    /// Handler called when focus leaves the target element and all descendants.
    pub on_blur_within: Option<Callback<FocusWithinEvent>>,

    /// Handler called when the focus within state changes.
    pub on_focus_within_change: Option<Callback<bool>>,
}

impl Default for UseFocusWithinInput {
    /// Enabled, no callbacks.
    fn default() -> Self {
        Self {
            is_disabled: Signal::stored(false),
            on_focus_within: None,
            on_blur_within: None,
            on_focus_within_change: None,
        }
    }
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
pub fn use_focus_within(input: UseFocusWithinInput) -> UseFocusWithinReturn {
    #[cfg(feature = "ssr")]
    {
        let _ = input;
        let (is_focus_within, _) = signal(false);
        UseFocusWithinReturn {
            props: UseFocusWithinProps {
                on_focusin: EventHandler::new(|_: FocusEvent| {}),
                on_focusout: EventHandler::new(|_: FocusEvent| {}),
            },
            is_focus_within: is_focus_within.into(),
        }
    }

    #[cfg(not(feature = "ssr"))]
    {
        use wasm_bindgen::JsCast;

        use crate::utils::{
            EventAccessors, EventTargetExt,
            event_listeners::{Listener, listen_to},
            set_event_target,
            shadow_dom::{get_active_element, get_event_target, node_contains},
            synthetic_blur::SyntheticBlurObserver,
        };

        let UseFocusWithinInput {
            is_disabled: disabled,
            on_focus_within,
            on_blur_within,
            on_focus_within_change,
        } = input;

        let (is_focus_within, set_is_focus_within) = signal(false);

        // The global focus listener while focus is within (removed when dropped).
        let global_listener: StoredValue<Option<Listener>, LocalStorage> =
            StoredValue::new_local(None);
        // Blur events for a form element disabled while focused (Firefox fires none).
        let blur_observer: StoredValue<Option<SyntheticBlurObserver>, LocalStorage> =
            StoredValue::new_local(None);
        let stop_tracking = move || {
            global_listener.try_update_value(Option::take);
            blur_observer.try_update_value(Option::take);
        };

        let trigger_blur_within = move |e: FocusEvent| {
            if !is_focus_within.try_get_untracked().unwrap_or(false) {
                return;
            }

            set_is_focus_within.set(false);
            stop_tracking();

            // Removing a focused element blurs it after its owner was disposed: the callbacks
            // may be gone then (`try_run`).
            if let Some(on_blur_within) = on_blur_within {
                on_blur_within.try_run(FocusWithinEvent { event: e });
            }

            if let Some(on_focus_within_change) = on_focus_within_change {
                on_focus_within_change.try_run(false);
            }
        };

        let contains =
            |container: &web_sys::EventTarget, target: Option<&web_sys::EventTarget>| match (
                container.dyn_ref::<web_sys::Node>(),
                target.and_then(|target| target.dyn_ref::<web_sys::Node>()),
            ) {
                (Some(container), Some(target)) => node_contains(container, target),
                _ => false,
            };

        // Handle focus entering the element tree
        let handle_focus_in = move |e: FocusEvent| {
            if disabled.try_get_untracked().unwrap_or(true) {
                return;
            }

            // Ignore events bubbling through portals.
            let current_target = e.expect_current_target();
            let target = get_event_target(&e).unwrap_or_else(|| e.expect_target());
            if !contains(&current_target, Some(&target)) {
                return;
            }

            // Double check that the active element is the target, in case a previously chained
            // focus handler already moved focus elsewhere.
            let Some(document) = target
                .dyn_ref::<web_sys::Node>()
                .and_then(web_sys::Node::owner_document)
            else {
                return;
            };
            if is_focus_within.try_get_untracked().unwrap_or(true)
                || get_active_element(&document) != target.to_element()
            {
                return;
            }

            if let Some(on_focus_within) = on_focus_within {
                on_focus_within.run(FocusWithinEvent { event: e.clone() });
            }
            if let Some(on_focus_within_change) = on_focus_within_change {
                on_focus_within_change.run(true);
            }
            set_is_focus_within.set(true);

            if let Some(element) = target.dyn_ref::<web_sys::Element>() {
                blur_observer.set_value(SyntheticBlurObserver::observe(element));
            }

            // Browsers fire no blur when the focused element is removed from the DOM: a focus
            // event outside the tracked element ends focus within then. A capture-phase `focus`
            // listener (as react-aria), which a handler stopping `focusin` can't hide.
            let listener = listen_to(&document, ev::focus, true, move |focus_e: FocusEvent| {
                if !is_focus_within.try_get_untracked().unwrap_or(false) {
                    return;
                }
                let focus_target = get_event_target(&focus_e);
                if contains(&current_target, focus_target.as_ref()) {
                    return;
                }
                // A blur event of the tracked element, with the newly focused element as its
                // related target (react-aria's `setEventTarget`).
                let init = web_sys::FocusEventInit::new();
                init.set_related_target(focus_target.as_ref());
                if let Ok(blur) = FocusEvent::new_with_focus_event_init_dict("blur", &init) {
                    set_event_target(&blur, &current_target, &current_target);
                    trigger_blur_within(blur);
                }
            });
            global_listener.set_value(Some(listener));
        };

        // Handle focus leaving - but only if it's actually leaving the element tree
        let handle_focus_out = move |e: FocusEvent| {
            if disabled.try_get_untracked().unwrap_or(true) {
                return;
            }

            // Ignore events bubbling through portals.
            let current_target = e.expect_current_target();
            let target = get_event_target(&e).unwrap_or_else(|| e.expect_target());
            if !contains(&current_target, Some(&target)) {
                return;
            }

            // Focus moving within the tree is no blur within.
            if contains(&current_target, e.related_target().as_ref()) {
                return;
            }

            // Focus is leaving the element tree
            trigger_blur_within(e);
        };

        // When disabled transitions to true, clean up any active focus-within state.
        Effect::new(move |_| {
            if disabled.get() && is_focus_within.get_untracked() {
                set_is_focus_within.set(false);
                stop_tracking();

                if let Some(on_focus_within_change) = on_focus_within_change {
                    on_focus_within_change.run(false);
                }
            }
        });

        on_cleanup(stop_tracking);

        UseFocusWithinReturn {
            props: UseFocusWithinProps {
                on_focusin: EventHandler::new(handle_focus_in),
                on_focusout: EventHandler::new(handle_focus_out),
            },
            is_focus_within: is_focus_within.into(),
        }
    }
}
