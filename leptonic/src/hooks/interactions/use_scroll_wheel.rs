use educe::Educe;
use leptos::on_cleanup;
use leptos_reactive::{Callable, Callback, MaybeSignal, SignalGetUntracked};
use leptos_use::{core::ElementMaybeSignal, use_event_listener_with_options, UseEventListenerOptions};
use typed_builder::TypedBuilder;

use crate::utils::{attributes::Attributes, event_handlers::EventHandlers, EventExt};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/interactions/src/useScrollWheel.ts

#[derive(Debug, Clone, Copy)]
pub struct ScrollWheelEvent {
    pub delta_x: f64,
    pub delta_y: f64,
    pub delta_z: f64,
    pub delta_mode: DeltaMode,
}

#[derive(Debug, Clone, Copy)]
pub enum DeltaMode {
    /// The delta values are specified in pixels.
    Pixel,
    /// The delta values are specified in lines.
    Line,
    /// The delta values are specified in pages.
    Page,
    /// An unknown delta mode, not specified in `https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaMode`.
    Unknown(u32),
}

impl From<u32> for DeltaMode {
    fn from(value: u32) -> Self {
        match value {
            0 => DeltaMode::Pixel,
            1 => DeltaMode::Line,
            2 => DeltaMode::Page,
            other => DeltaMode::Unknown(other),
        }
    }
}

#[derive(Clone, Copy, Educe, TypedBuilder)]
#[educe(Debug)]
pub struct UseScrollWheelInput {
    /// Wether scroll wheel events should be disabled.
    #[builder(setter(into))]
    pub(crate) disabled: MaybeSignal<bool>,

    #[builder(setter(into))]
    pub(crate) on_scroll: Callback<ScrollWheelEvent>,
}

#[derive(Debug)]
pub struct UseScrollWheelProps {
    /// These attributes must be spread onto the target element: `<foo {..props.attrs} />`
    pub attrs: Attributes,
    /// These handlers must be spread onto the target element: `<foo {..props.handlers} />`
    pub handlers: EventHandlers,
}

#[derive(Debug)]
pub struct UseScrollWheelReturn {
    pub props: UseScrollWheelProps,
}

pub fn use_scroll_wheel<El, T>(el: El, input: UseScrollWheelInput) -> UseScrollWheelReturn
where
    El: Into<ElementMaybeSignal<T, web_sys::EventTarget>> + 'static,
    T: Into<web_sys::EventTarget> + Clone + 'static,
{
    // We add the listener explicitly, marking it as non-passive, so that we can use `prevent_default`!
    // Wheel event listeners are passive by default.
    let stop = use_event_listener_with_options(
        el,
        leptos::ev::wheel,
        move |e| {
            // Hook is disabled, return immediately, don't interfere with events.
            if input.disabled.get_untracked() {
                return;
            }

            // If the ctrlKey is pressed, this is a zoom event, which we should not interfere with.
            if e.is_ctrl_key_pressed() {
                return;
            }

            e.stop_propagation();
            e.prevent_default();

            input.on_scroll.call(ScrollWheelEvent {
                delta_x: e.delta_x(),
                delta_y: e.delta_y(),
                delta_z: e.delta_z(),
                delta_mode: e.delta_mode().into(),
            })
        },
        UseEventListenerOptions::default().passive(false),
    );

    on_cleanup(move || {
        stop();
    });

    UseScrollWheelReturn {
        props: UseScrollWheelProps {
            attrs: Attributes::new(),
            handlers: EventHandlers::builder().build(),
        },
    }
}
