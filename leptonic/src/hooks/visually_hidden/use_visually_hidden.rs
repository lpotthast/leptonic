// Upstream: react-aria/src/visually-hidden/VisuallyHidden.tsx @ 99e6102368
// Upstream: react-aria/test/visually-hidden/VisuallyHidden.test.tsx @ 99e6102368
use leptos::{ev, prelude::*};
use web_sys::FocusEvent;

use crate::{
    EventHandler, IntoAttrs, OnEvent,
    hooks::focus::use_focus_within::{UseFocusWithinInput, use_focus_within},
    utils::{
        part::{Part, Remount},
        signal_attr::{SignalAttr, signal_attr},
        visually_hidden::VISUALLY_HIDDEN_STYLE,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - No `style` input to merge: the props render the hiding `style` attribute (absent while a
//   focusable element shows). Reason: callers style the element itself; merging inline styles
//   into the hiding ones has no use the hook needs to support.
//
// =============================================================================

/// Input of [`use_visually_hidden`].
#[derive(Debug, Clone, Copy, Default)]
pub struct UseVisuallyHiddenInput {
    /// Whether the element shows while focus is within it (e.g. a "skip to content" link).
    pub is_focusable: Signal<bool>,
}

/// The return value of [`use_visually_hidden`].
#[derive(Debug)]
pub struct UseVisuallyHiddenReturn {
    /// The visually hidden element.
    pub target: Part<UseVisuallyHiddenProps>,
}

/// Props for the hidden element.
#[derive(Debug)]
pub struct UseVisuallyHiddenProps {
    pub style: Signal<Option<&'static str>>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
}

impl Remount for UseVisuallyHiddenProps {
    fn remount(&self) -> Self {
        Self {
            style: self.style,
            on_focusin: self.on_focusin.clone(),
            on_focusout: self.on_focusout.clone(),
        }
    }
}

pub type UseVisuallyHiddenAttrs = (
    SignalAttr<Option<&'static str>>,
    OnEvent<ev::focusin>,
    OnEvent<ev::focusout>,
);

impl IntoAttrs for UseVisuallyHiddenProps {
    type Attrs = UseVisuallyHiddenAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            signal_attr("style", self.style),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
        )
    }
}

/// Hides an element visually while keeping it available to assistive technology; a focusable
/// element shows while focus is within it.
pub fn use_visually_hidden(input: UseVisuallyHiddenInput) -> UseVisuallyHiddenReturn {
    crate::metrics::subject!();
    let UseVisuallyHiddenInput { is_focusable } = input;
    let focus_within = use_focus_within(UseFocusWithinInput {
        is_disabled: Signal::derive(move || !is_focusable.get()),
        on_focus_within: None,
        on_blur_within: None,
        on_focus_within_change: None,
    });
    let focus_within_props = focus_within.target.into_props();
    let is_focused = focus_within.is_focus_within;
    UseVisuallyHiddenReturn {
        target: Part::stored(UseVisuallyHiddenProps {
            style: Signal::derive(move || (!is_focused.get()).then_some(VISUALLY_HIDDEN_STYLE)),
            on_focusin: focus_within_props.on_focusin,
            on_focusout: focus_within_props.on_focusout,
        }),
    }
}
