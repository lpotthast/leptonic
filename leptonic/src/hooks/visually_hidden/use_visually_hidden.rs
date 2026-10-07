// Upstream: react-aria/src/visually-hidden/VisuallyHidden.tsx @ 99e6102368
use leptos::{
    attr::custom::{CustomAttr, custom_attribute},
    ev::{self, On, SharedEventCallback},
    prelude::*,
};
use web_sys::FocusEvent;

use crate::{
    hooks::{
        IntoAttrs,
        focus::use_focus_within::{UseFocusWithinInput, use_focus_within},
    },
    utils::{EventHandler, visually_hidden::VISUALLY_HIDDEN_STYLE},
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

/// Return value of [`use_visually_hidden`].
#[derive(Debug)]
pub struct UseVisuallyHiddenReturn {
    pub props: UseVisuallyHiddenProps,
}

/// Props for the hidden element.
#[derive(Debug)]
pub struct UseVisuallyHiddenProps {
    pub style: Signal<Option<&'static str>>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
}

pub type UseVisuallyHiddenAttrs = (
    CustomAttr<&'static str, Signal<Option<&'static str>>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
);

impl IntoAttrs for UseVisuallyHiddenProps {
    type Attrs = UseVisuallyHiddenAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            custom_attribute("style", self.style),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
        )
    }
}

/// Hides an element visually while keeping it available to assistive technology; a focusable
/// element shows while focus is within it.
pub fn use_visually_hidden(input: UseVisuallyHiddenInput) -> UseVisuallyHiddenReturn {
    let UseVisuallyHiddenInput { is_focusable } = input;
    let focus_within = use_focus_within(UseFocusWithinInput {
        is_disabled: Signal::derive(move || !is_focusable.get()),
        ..UseFocusWithinInput::default()
    });
    let is_focused = focus_within.is_focus_within;
    UseVisuallyHiddenReturn {
        props: UseVisuallyHiddenProps {
            style: Signal::derive(move || (!is_focused.get()).then_some(VISUALLY_HIDDEN_STYLE)),
            on_focusin: focus_within.props.on_focusin,
            on_focusout: focus_within.props.on_focusout,
        },
    }
}
