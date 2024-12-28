use leptos::ev;
use leptos::ev::{on, On};
use leptos::prelude::*;
use leptos_use::use_document;
use web_sys::FocusEvent;

use crate::utils::EventTargetExt;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/interactions/src/useFocus.ts

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
    pub attrs: UseFocusAttrs,
}

/// These attributes must be spread onto the target element: `<foo {..attrs} />`
pub type UseFocusAttrs = (
    On<ev::focus, Box<dyn Fn(FocusEvent) + Send + Sync + 'static>>,
    On<ev::blur, Box<dyn Fn(FocusEvent) + Send + Sync + 'static>>,
);

pub fn use_focus(input: UseFocusInput) -> UseFocusReturn {
    let on_focus = Box::new(move |e: FocusEvent| {
        // Double check that document.activeElement actually matches e.target in case a previously chained
        // focus handler already moved focus somewhere else.
        if e.target() == e.current_target()
            && use_document().active_element() == e.target().and_then(|t| t.as_element())
            && !input.disabled.get()
        {
            if let Some(on_focus) = input.on_focus {
                on_focus.run(e);
            }

            if let Some(on_focus_change) = input.on_focus_change {
                on_focus_change.run(true);
            }
        }
    });

    let on_blur = Box::new(move |e: FocusEvent| {
        if e.target() == e.current_target() && !input.disabled.get() {
            if let Some(on_blur) = input.on_blur {
                on_blur.run(e);
            }

            if let Some(on_focus_change) = input.on_focus_change {
                on_focus_change.run(true);
            }
        }
    });

    UseFocusReturn {
        attrs: (
            on(ev::focus, on_focus),
            on(ev::blur, on_blur),
        ),
    }
}
