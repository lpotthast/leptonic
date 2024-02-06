use educe::Educe;
use leptos::{ev::FocusEvent, Callable, Callback, MaybeSignal, SignalGet};
use leptos_use::use_document;

use crate::utils::EventTargetExt;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/interactions/src/useFocus.ts

#[derive(Debug, Clone, Copy)]
pub struct UseFocusOptions {
    /// Disables the handling focus events when true.
    pub disabled: MaybeSignal<bool>,

    pub on_focus: Option<Callback<FocusEvent>>,
    pub on_blur: Option<Callback<FocusEvent>>,
    pub on_focus_change: Option<Callback<bool>>,
}

#[derive(Educe)]
#[educe(Debug)]
pub struct UseFocusReturn {
    #[educe(Debug(ignore))]
    pub on_focus: Box<dyn Fn(FocusEvent)>,
    #[educe(Debug(ignore))]
    pub on_blur: Box<dyn Fn(FocusEvent)>,
}

pub fn use_focus(options: UseFocusOptions) -> UseFocusReturn {
    let on_focus = Box::new(move |e: FocusEvent| {
        // Double check that document.activeElement actually matches e.target in case a previously chained
        // focus handler already moved focus somewhere else.
        if e.target() == e.current_target()
            && use_document().active_element() == e.target().and_then(|t| t.as_element())
            && !options.disabled.get()
        {
            if let Some(on_focus) = options.on_focus {
                on_focus.call(e);
            }

            if let Some(on_focus_change) = options.on_focus_change {
                on_focus_change.call(true);
            }
        }
    });

    let on_blur = Box::new(move |e: FocusEvent| {
        if e.target() == e.current_target() && !options.disabled.get() {
            if let Some(on_blur) = options.on_blur {
                on_blur.call(e);
            }

            if let Some(on_focus_change) = options.on_focus_change {
                on_focus_change.call(true);
            }
        }
    });

    UseFocusReturn { on_focus, on_blur }
}
