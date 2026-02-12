use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use leptos_use::use_document;
use web_sys::FocusEvent;

use crate::utils::{EventAccessors, EventHandler, EventTargetExt};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/interactions/src/useFocus.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
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

impl UseFocusProps {
    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseFocusAttrs {
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

    let handle_focus = move |e: FocusEvent| {
        // Double check that document.activeElement actually matches e.target in case a previously chained
        // focus handler already moved focus somewhere else.
        if e.expect_target() == e.expect_current_target()
            && use_document().active_element() == e.expect_target().to_element()
            && !disabled.get_untracked()
        {
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

        if e.expect_target() == e.expect_current_target() && !is_disabled {
            if let Some(on_blur) = on_blur {
                on_blur.run(e);
            }

            if let Some(on_focus_change) = on_focus_change {
                on_focus_change.run(false);
            }
        }
    };

    UseFocusReturn {
        props: UseFocusProps {
            on_focus: EventHandler::new(handle_focus),
            on_blur: EventHandler::new(handle_blur),
        },
    }
}
