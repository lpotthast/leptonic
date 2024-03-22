use std::rc::Rc;

use leptos::{ev::FocusEvent, Callable, Callback, MaybeSignal, SignalGet};
use leptos_use::use_document;

use crate::utils::{attributes::Attributes, event_handlers::EventHandlers, EventTargetExt};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/interactions/src/useFocus.ts

#[derive(Debug, Clone, Copy)]
pub struct UseFocusInput {
    /// Disables the handling focus events when true.
    pub disabled: MaybeSignal<bool>,

    pub on_focus: Option<Callback<FocusEvent>>,
    pub on_blur: Option<Callback<FocusEvent>>,
    pub on_focus_change: Option<Callback<bool>>,
}

#[derive(Debug)]
pub struct UseFocusReturn {
    pub props: UseFocusProps,
}

#[derive(Debug)]
pub struct UseFocusProps {
    /// These attributes must be spread onto the target element: `<foo use:attrs=props.attrs />`
    pub attrs: Attributes,
    /// These handlers must be spread onto the target element: `<foo use:handlers=props.handlers />`
    pub handlers: EventHandlers,
}

pub fn use_focus(input: UseFocusInput) -> UseFocusReturn {
    let on_focus = Rc::new(move |e: FocusEvent| {
        // Double check that document.activeElement actually matches e.target in case a previously chained
        // focus handler already moved focus somewhere else.
        if e.target() == e.current_target()
            && use_document().active_element() == e.target().and_then(|t| t.as_element())
            && !input.disabled.get()
        {
            if let Some(on_focus) = input.on_focus {
                Callable::call(&on_focus, e);
            }

            if let Some(on_focus_change) = input.on_focus_change {
                Callable::call(&on_focus_change, true);
            }
        }
    });

    let on_blur = Rc::new(move |e: FocusEvent| {
        if e.target() == e.current_target() && !input.disabled.get() {
            if let Some(on_blur) = input.on_blur {
                Callable::call(&on_blur, e);
            }

            if let Some(on_focus_change) = input.on_focus_change {
                Callable::call(&on_focus_change, true);
            }
        }
    });

    UseFocusReturn {
        props: UseFocusProps {
            attrs: Attributes::new(),
            handlers: EventHandlers::builder()
                .on_focus(on_focus)
                .on_blur(on_blur)
                .build(),
        },
    }
}
