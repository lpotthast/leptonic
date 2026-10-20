// Upstream: react-aria/src/interactions/useFocus.ts @ 99e6102368
// Upstream: react-aria/test/interactions/useFocus.test.js @ 99e6102368
use std::sync::Arc;

use leptos::{ev, prelude::*};
use web_sys::FocusEvent;

use crate::{
    EventHandler, IntoAttrs, OnEvent,
    utils::part::{Part, Remount},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `mark` (a `FocusMark`): a reaction to the focus state that runs before the callbacks, for
//   hooks tracking the state themselves on top of the caller's callbacks (`use_focus_ring`). Not
//   in react-aria, whose `useFocusRing` passes only its own `onFocusChange` (ours also forwards
//   the caller's callbacks: one `Arc` instead of wrapping each in a new `Callback`,
//   `performance.md` R8).
//
// ## DIFFERENT BEHAVIOR
// - The handlers are attached whenever there is a callback, and check `is_disabled` when they run
//   (react-aria attaches none while disabled): `is_disabled` is reactive. As react-aria, without
//   callbacks none are attached.
// - An element disabled while focused reports one blur: Chrome fires its own blur after the
//   synthetic one `SyntheticBlurObserver` dispatched, and that next blur is dropped (react-aria
//   would report both). Every other blur of the element is reported, as in react-aria: virtual
//   focus blurs an element synthetically while it keeps DOM focus, and its real blur follows.
//
// ## LEPTOS-SPECIFIC ADAPTATIONS
// - The blur handler reads `is_disabled` with `try_get_untracked` and runs callbacks with
//   `try_run`: removing a focused element blurs it after its owner was disposed ("Blur After
//   Disposal" in leptos-and-dom.md).
//
// =============================================================================

/// A reaction to an element's focus state, run before [`use_focus`]'s callbacks: `true` before
/// `on_focus`, `false` before `on_blur`, under the same conditions. For hooks that track the focus
/// state themselves and also forward their caller's callbacks (`use_focus_ring`): the state is
/// up to date when the callbacks run. One allocation (the `Arc`), unlike wrapping the caller's
/// callbacks in new `Callback`s.
#[derive(Clone)]
pub struct FocusMark(Arc<dyn Fn(bool) + Send + Sync>);

impl FocusMark {
    /// Runs `mark` with the new focus state.
    pub fn new(mark: impl Fn(bool) + Send + Sync + 'static) -> Self {
        Self(Arc::new(mark))
    }

    #[cfg_attr(feature = "ssr", allow(dead_code))]
    fn run(&self, is_focused: bool) {
        (self.0)(is_focused);
    }
}

impl std::fmt::Debug for FocusMark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("FocusMark")
    }
}

/// Input of [`use_focus`].
#[derive(Debug, Clone, Default)]
pub struct UseFocusInput {
    /// Disables the handling of focus events when true.
    pub is_disabled: Signal<bool>,
    /// Called when the element receives focus.
    pub on_focus: Option<Callback<FocusEvent>>,
    /// Called when the element loses focus.
    pub on_blur: Option<Callback<FocusEvent>>,
    /// Called when the element's focus state changes.
    pub on_focus_change: Option<Callback<bool>>,
    /// Runs before the callbacks with the new focus state (a hook tracking the state itself).
    /// Default: `None`.
    pub mark: Option<FocusMark>,
}

/// The return value of [`use_focus`].
#[derive(Debug)]
pub struct UseFocusReturn {
    /// The element whose focus is tracked.
    pub target: Part<UseFocusProps>,
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

impl Remount for UseFocusProps {
    fn remount(&self) -> Self {
        Self {
            on_focus: self.on_focus.clone(),
            on_blur: self.on_blur.clone(),
        }
    }
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
    crate::metrics::subject!();
    #[cfg(feature = "ssr")]
    {
        let _ = input;
        UseFocusReturn {
            target: Part::stored(UseFocusProps {
                on_focus: EventHandler::empty(),
                on_blur: EventHandler::empty(),
            }),
        }
    }

    #[cfg(not(feature = "ssr"))]
    {
        use wasm_bindgen::JsCast;

        use crate::utils::{
            dom_ext::{EventAccessors, EventTargetExt},
            shadow_dom,
            stored_field::StoredField,
            synthetic_blur::SyntheticBlurObserver,
        };

        let UseFocusInput {
            is_disabled: disabled,
            on_focus,
            on_blur,
            on_focus_change,
            mark,
        } = input;

        // Nothing to report to: no handlers (react-aria).
        if on_focus.is_none() && on_blur.is_none() && on_focus_change.is_none() && mark.is_none() {
            return UseFocusReturn {
                target: Part::stored(UseFocusProps {
                    on_focus: EventHandler::empty(),
                    on_blur: EventHandler::empty(),
                }),
            };
        }
        let blur_mark = mark.clone();

        // The focus state, in one `StoredValue` (`performance.md` R7). The blur observer: blur
        // events for a form element disabled while focused (Firefox fires none), set up on focus,
        // dropped on blur or unmount. Whether the next blur is the one Chrome fires after the
        // observer's (dropped): set by the observer's blur, cleared by that blur or a new focus.
        let state = StoredValue::new_local((None::<SyntheticBlurObserver>, false));
        let blur_observer = StoredField::new(state, |s| &s.0, |s| &mut s.0);
        let drop_next_blur = StoredField::new(state, |s| &s.1, |s| &mut s.1);

        let handle_focus = move |e: FocusEvent| {
            // The event is the element's own, and the active element is the focused one, in case
            // a previously chained focus handler already moved focus somewhere else (react-aria:
            // `getEventTarget(e) === e.currentTarget && getEventTarget(e) === activeElement`,
            // across shadow roots and iframes). Focus inside a shadow host's own shadow root is
            // not the host's.
            let target = shadow_dom::get_event_target(&e).unwrap_or_else(|| e.expect_target());
            let owner_doc = target
                .dyn_ref::<web_sys::Node>()
                .and_then(web_sys::Node::owner_document);
            let active = owner_doc.as_ref().and_then(shadow_dom::get_active_element);

            if target == e.expect_current_target()
                && active == target.to_element()
                && !disabled.get_untracked()
            {
                if let Some(el) = target.dyn_ref::<web_sys::Element>() {
                    blur_observer.set_value(SyntheticBlurObserver::observe(el));
                }
                drop_next_blur.set_value(false);

                if let Some(mark) = &mark {
                    mark.run(true);
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
            // Removing a focused element blurs it after its owner was disposed ("Blur After
            // Disposal"): the signal and the callbacks may be gone then (`try_*`).
            let is_disabled = disabled.try_get_untracked().unwrap_or(true);

            // No longer needed once blurred. Its own blur is followed by Chrome's, which is dropped.
            let observer_blurred = blur_observer
                .try_update_value(Option::take)
                .flatten()
                .is_some_and(|observer| observer.has_blurred());
            let is_followup =
                drop_next_blur.try_update_value(|d| std::mem::replace(d, observer_blurred));

            let target = shadow_dom::get_event_target(&e).unwrap_or_else(|| e.expect_target());
            if target == e.expect_current_target() && !is_disabled && is_followup == Some(false) {
                if let Some(mark) = &blur_mark {
                    mark.run(false);
                }
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
            target: Part::stored(UseFocusProps {
                on_focus: EventHandler::new(handle_focus),
                on_blur: EventHandler::new(handle_blur),
            }),
        }
    }
}
