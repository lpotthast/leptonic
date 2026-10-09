// Upstream: react-aria/src/interactions/useFocus.ts @ 99e6102368
// Upstream: react-aria/test/interactions/useFocus.test.js @ 99e6102368
use leptos::{ev, prelude::*};
use web_sys::FocusEvent;

use crate::{EventHandler, IntoAttrs, OnEvent};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## DIFFERENT BEHAVIOR
// - The handlers are always attached and check `is_disabled` when they run (react-aria attaches
//   none when there are no callbacks): `is_disabled` is reactive.
// - A blur is reported only after a reported focus, once: Chrome fires its own blur for an
//   element disabled while focused after the synthetic one (react-aria would report both).
// - The disabled-while-focused observer is set up only with a blur callback (`on_blur` or
//   `on_focus_change`), react-aria: whenever the focus handler is attached.
//
// ## LEPTOS-SPECIFIC ADAPTATIONS
// - The blur handler reads `is_disabled` with `try_get_untracked` and runs callbacks with
//   `try_run`: removing a focused element blurs it after its owner was disposed ("Blur After
//   Disposal" in leptos-and-dom.md).
//
// =============================================================================

/// Input of [`use_focus`].
#[derive(Debug, Clone, Copy, Default)]
pub struct UseFocusInput {
    /// Disables the handling focus events when true.
    pub is_disabled: Signal<bool>,
    /// Called when the element receives focus.
    pub on_focus: Option<Callback<FocusEvent>>,
    /// Called when the element loses focus.
    pub on_blur: Option<Callback<FocusEvent>>,
    /// Called when the element's focus state changes.
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
pub type UseFocusAttrs = (OnEvent<ev::focus>, OnEvent<ev::blur>);

/// Track focus of an element.
pub fn use_focus(input: UseFocusInput) -> UseFocusReturn {
    #[cfg(feature = "ssr")]
    {
        let _ = input;
        UseFocusReturn {
            props: UseFocusProps {
                on_focus: EventHandler::new(|_: FocusEvent| {}),
                on_blur: EventHandler::new(|_: FocusEvent| {}),
            },
        }
    }

    #[cfg(not(feature = "ssr"))]
    {
        use wasm_bindgen::JsCast;

        use crate::utils::{
            dom_ext::{EventAccessors, EventTargetExt},
            shadow_dom,
            synthetic_blur::SyntheticBlurObserver,
        };

        let UseFocusInput {
            is_disabled: disabled,
            on_focus,
            on_blur,
            on_focus_change,
        } = input;

        // Blur events for a form element disabled while focused (Firefox fires none). Set up on
        // focus, dropped on blur or unmount.
        let blur_observer: StoredValue<Option<SyntheticBlurObserver>, LocalStorage> =
            StoredValue::new_local(None);
        // Whether the element has focus (as reported): a blur ends it once. Chrome blurs an element
        // disabled while focused only after the synthetic blur was dispatched.
        let has_focus = StoredValue::new(false);

        // Only blurs that are reported need the observer (react-aria: the synthetic blur event
        // calls `onBlur`).
        let reports_blur = on_blur.is_some() || on_focus_change.is_some();

        let handle_focus = move |e: FocusEvent| {
            // The event is the element's own (as for blur: its target, retargeted to a shadow
            // host, is the element), and the active element is the focused one, in case a
            // previously chained focus handler already moved focus somewhere else (react-aria:
            // `getActiveElement() === getEventTarget(e)`, across shadow roots and iframes).
            let target = e.expect_target();
            let focused = shadow_dom::get_event_target(&e).unwrap_or_else(|| target.clone());
            let owner_doc = focused
                .dyn_ref::<web_sys::Node>()
                .and_then(web_sys::Node::owner_document);
            let active = owner_doc.as_ref().and_then(shadow_dom::get_active_element);

            if target == e.expect_current_target()
                && active == focused.to_element()
                && !disabled.get_untracked()
            {
                if reports_blur && let Some(el) = target.dyn_ref::<web_sys::Element>() {
                    blur_observer.set_value(SyntheticBlurObserver::observe(el));
                }
                has_focus.set_value(true);

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

            // No longer needed once blurred.
            blur_observer.try_update_value(Option::take);

            // Removing a focused element blurs it after its owner was disposed: the callbacks
            // may be gone then (`try_run`).
            if e.expect_target() == e.expect_current_target()
                && !is_disabled
                && has_focus.try_update_value(|f| std::mem::replace(f, false)) == Some(true)
            {
                if let Some(on_blur) = on_blur {
                    on_blur.try_run(e);
                }

                if let Some(on_focus_change) = on_focus_change {
                    on_focus_change.try_run(false);
                }
            }
        };

        on_cleanup(move || {
            blur_observer.try_update_value(Option::take);
        });

        UseFocusReturn {
            props: UseFocusProps {
                on_focus: EventHandler::new(handle_focus),
                on_blur: EventHandler::new(handle_blur),
            },
        }
    }
}
