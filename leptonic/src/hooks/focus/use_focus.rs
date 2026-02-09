use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use leptos_use::use_document;
use web_sys::FocusEvent;

use crate::utils::{EventHandler, EventTargetExt};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/interactions/src/useFocus.ts

#[derive(Debug, Clone, Copy)]
pub struct UseFocusInput {
    /// Disables the handling focus events when true.
    pub disabled: Signal<bool>,

    pub on_focus: Option<Callback<FocusEvent>>,
    pub on_blur: Option<Callback<FocusEvent>>,
    pub on_focus_change: Option<Callback<bool>>,
}

#[derive(Debug, Clone)]
pub struct UseFocusReturn {
    /// Props for programmatic merging. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub props: UseFocusProps,
}

/// Props from `use_focus` that can be extracted and merged programmatically.
///
/// Use [`UseFocusProps::into_attrs()`] to convert to an attributes-tuple spreadable using Leptos's
/// spreading syntax (`<div {..props.into_attrs()}>`) (taking ownership).
///
/// Use [`UseFocusProps::to_attrs()`] to convert to an attributes-tuple spreadable using Leptos's
/// spreading syntax (`<div {..props.to_attrs()}>`) (without taking ownership, requiring internal
/// cloning).
#[derive(Debug, Clone)]
pub struct UseFocusProps {
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
}

impl UseFocusProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseFocusAttrs {
        (self.on_focus.to_on(ev::focus), self.on_blur.to_on(ev::blur))
    }

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
    let on_focus_handler = move |e: FocusEvent| {
        // Double check that document.activeElement actually matches e.target in case a previously chained
        // focus handler already moved focus somewhere else.
        if e.target() == e.current_target()
            && use_document().active_element() == e.target().and_then(|t| t.as_element())
            && !input.disabled.get_untracked()
        {
            if let Some(on_focus) = input.on_focus {
                on_focus.run(e);
            }

            if let Some(on_focus_change) = input.on_focus_change {
                on_focus_change.run(true);
            }
        }
    };

    let on_blur_handler = move |e: FocusEvent| {
        // In certain situations, we saw this blur handler being called after the disabled signal
        // was disposed. Mostly when interaction with this use_focus enabled element
        // lead to removal from said element from the DOM.
        let is_disabled = input.disabled.try_get_untracked().unwrap_or(true);

        if e.target() == e.current_target() && !is_disabled {
            if let Some(on_blur) = input.on_blur {
                on_blur.run(e);
            }

            if let Some(on_focus_change) = input.on_focus_change {
                on_focus_change.run(false);
            }
        }
    };

    UseFocusReturn {
        props: UseFocusProps {
            on_focus: EventHandler::new(on_focus_handler),
            on_blur: EventHandler::new(on_blur_handler),
        },
    }
}
