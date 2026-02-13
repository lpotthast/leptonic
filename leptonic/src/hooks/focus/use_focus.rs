use leptos::{
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use wasm_bindgen::JsCast;
use web_sys::FocusEvent;

use crate::{
    hooks::IntoAttrs,
    utils::{shadow_dom, synthetic_blur, EventAccessors, EventHandler, EventTargetExt},
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/interactions/src/useFocus.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## DIFFERENT BEHAVIOR
//
// - Defensive `try_get_untracked` in blur handler
//   In certain Leptos scenarios (e.g., the focused element is removed from the
//   DOM), the `disabled` signal may be disposed before the blur handler fires.
//   We use `try_get_untracked().unwrap_or(true)` to treat a disposed signal as
//   disabled rather than panicking. React-aria does not face this issue because
//   React's synthetic event system defers cleanup.
//
// - Always-attached event handlers
//   React-aria returns `undefined` props when no callbacks are provided,
//   relying on React's reconciliation to avoid attaching empty listeners.
//   Our `EventHandler` always attaches a listener but checks the disabled
//   state inside the handler. The overhead is negligible.
//
// =============================================================================

#[derive(Debug, Clone, Copy)]
pub struct UseFocusInput {
    /// Disables the handling focus events when true.
    pub disabled: Signal<bool>,

    pub on_focus: Option<Callback<FocusEvent>>,
    pub on_blur: Option<Callback<FocusEvent>>,
    pub on_focus_change: Option<Callback<bool>>,
}

#[derive(Debug)]
pub struct UseFocusReturn {
    /// Props for programmatic merging. Call `.into_attrs()` for view spreading.
    pub props: UseFocusProps,
}

/// Props from `use_focus` that can be extracted and merged programmatically.
///
/// Use [`UseFocusProps::into_attrs()`] to convert to an attributes-tuple spreadable using Leptos's
/// spreading syntax (`<div {..props.into_attrs()}>`) (taking ownership).
#[derive(Debug)]
pub struct UseFocusProps {
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
}

impl IntoAttrs for UseFocusProps {
    type Attrs = UseFocusAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
        )
    }
}

/// These attributes must be spread onto the target element: `<foo {..attrs} />`
pub type UseFocusAttrs = (
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
);

/// Track focus of an element.
pub fn use_focus(input: UseFocusInput) -> UseFocusReturn {
    let UseFocusInput {
        disabled,
        on_focus,
        on_blur,
        on_focus_change,
    } = input;

    // Cleanup handle for the synthetic blur MutationObserver (Firefox workaround).
    // Set up on focus, disconnected on blur or unmount.
    let blur_observer_cleanup: StoredValue<Option<Box<dyn Fn()>>, LocalStorage> =
        StoredValue::new_local(None);

    let handle_focus = move |e: FocusEvent| {
        // Double check that document.activeElement actually matches e.target in case a previously chained
        // focus handler already moved focus somewhere else.
        // Use owner document from target to correctly handle iframes/shadow DOM.
        // Use shadow-DOM-aware get_active_element to pierce shadow roots.
        let target = shadow_dom::get_event_target(&e).unwrap_or_else(|| e.expect_target());
        let owner_doc = target
            .dyn_ref::<web_sys::Node>()
            .and_then(web_sys::Node::owner_document);
        let active = owner_doc.as_ref().and_then(shadow_dom::get_active_element);

        if target == e.expect_current_target()
            && active == target.to_element()
            && !disabled.get_untracked()
        {
            // Set up synthetic blur observer for form elements (Firefox workaround:
            // Firefox does not fire blur when a form element becomes disabled while focused).
            if let Some(el) = e
                .target()
                .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
            {
                let cleanup = synthetic_blur::setup_synthetic_blur_observer(&el);
                blur_observer_cleanup.set_value(Some(cleanup));
            }

            if let Some(on_focus) = on_focus {
                on_focus.run(e);
            }

            if let Some(on_focus_change) = on_focus_change {
                on_focus_change.run(true);
            }
        }
    };

    let handle_blur = move |e: FocusEvent| {
        // In certain situations, we saw this blur handler being called after the disabled signal
        // was disposed. Mostly when interaction with this use_focus enabled element
        // lead to removal from said element from the DOM.
        let is_disabled = disabled.try_get_untracked().unwrap_or(true);

        // Disconnect synthetic blur observer (no longer needed once blur fires).
        blur_observer_cleanup.update_value(|cleanup| {
            if let Some(cleanup_fn) = cleanup.take() {
                cleanup_fn();
            }
        });

        if e.expect_target() == e.expect_current_target() && !is_disabled {
            if let Some(on_blur) = on_blur {
                on_blur.run(e);
            }

            if let Some(on_focus_change) = on_focus_change {
                on_focus_change.run(false);
            }
        }
    };

    // Cleanup observer on unmount.
    on_cleanup(move || {
        blur_observer_cleanup.update_value(|cleanup| {
            if let Some(cleanup_fn) = cleanup.take() {
                cleanup_fn();
            }
        });
    });

    UseFocusReturn {
        props: UseFocusProps {
            on_focus: EventHandler::new(handle_focus),
            on_blur: EventHandler::new(handle_blur),
        },
    }
}
