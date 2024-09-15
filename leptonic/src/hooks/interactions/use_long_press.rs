use std::{cell::Cell, rc::Rc, time::Duration};

use educe::Educe;
use leptos::{
    leptos_dom::helpers::TimeoutHandle, set_timeout, set_timeout_with_handle, Callable, Oco,
};
use leptos_reactive::{Callback, MaybeSignal};
use leptos_use::{use_event_listener_with_options, use_window, UseEventListenerOptions};
use typed_builder::TypedBuilder;

use crate::{
    hooks::{use_press, PressEvent, UsePressInput, UsePressReturn},
    utils::{
        attributes::Attributes, event_handlers::EventHandlers, pointer_type::PointerType, Modifiers,
    },
};

#[derive(Educe, Clone)]
#[educe(Debug)]
pub struct LongPressEvent {
    /// The pointer type that triggered the press event.
    pub pointer_type: PointerType,

    /// The target element of the press event.
    pub target: Option<web_sys::EventTarget>,

    /// Sates which modifier keys were held during the press event.
    pub modifiers: Modifiers,

    /// By default, press events stop propagation to parent elements.
    /// In cases where a handler decides not to handle a specific event,
    /// it can call `continuePropagation()` to allow a parent to handle it.
    #[educe(Debug(ignore))]
    pub continue_propagation: Rc<dyn Fn()>,
}

impl From<PressEvent> for LongPressEvent {
    fn from(e: PressEvent) -> Self {
        Self {
            pointer_type: e.pointer_type,
            target: e.target,
            modifiers: e.modifiers,
            continue_propagation: e.continue_propagation,
        }
    }
}

#[derive(Debug, TypedBuilder)]
pub struct UseLongPressInput {
    /// Whether long press events should be disabled.
    #[builder(setter(into))]
    pub(crate) disabled: MaybeSignal<bool>,

    /// Set this to true if you want controlled press behavior
    /// with the guarantee of no browser-specific behavior happening on user interactions.
    #[builder(default = false)]
    pub(crate) force_prevent_default: bool,

    /// Handler that is called when a long press interaction starts.
    #[builder(default, setter(into, strip_option))]
    pub(crate) on_long_press_start: Option<Callback<LongPressEvent>>,

    /// Handler that is called when a long press interaction ends, either
    /// over the target or when the pointer leaves the target.
    #[builder(default, setter(into, strip_option))]
    pub(crate) on_long_press_end: Option<Callback<LongPressEvent>>,

    /// Handler that is called when the threshold time is met while
    /// the press is over the target.
    #[builder(setter(into))]
    pub(crate) on_long_press: Callback<LongPressEvent>,

    /// The amount of time to wait before triggering a long press.
    /// Default is 500ms.
    pub(crate) threshold: Duration,

    /// A description for assistive technology users indicating that a long press
    /// action is available, e.g. "Long press to open menu".
    pub(crate) accessibility_description: Oco<'static, str>,
}

#[derive(Debug)]
pub struct UseLongPressReturn {
    pub props: UseLongPressProps,
}

#[derive(Debug)]
pub struct UseLongPressProps {
    /// These attributes must be spread onto the target element: `<foo {..props.attrs} />`
    pub attrs: Attributes,
    /// These handlers must be spread onto the target element: `<foo {..props.handlers} />`
    pub handlers: EventHandlers,
}

/// Handles long press interactions across mouse and touch devices.
/// Supports a customizable time threshold, accessibility description,
/// and normalizes behavior across browsers and devices.
pub fn use_long_press(input: UseLongPressInput) -> UseLongPressReturn {
    let timeout_handle: Rc<Cell<Option<TimeoutHandle>>> = Rc::new(Cell::new(None));
    let timeout_handle2: Rc<Cell<Option<TimeoutHandle>>> = timeout_handle.clone();

    let UsePressReturn {
        props: press_props,
        is_pressed: _,
        press_responder: _,
    } = use_press(UsePressInput {
        disabled: input.disabled,
        force_prevent_default: input.force_prevent_default,
        on_press: None,
        on_press_up: None,
        on_press_start: Some(Callback::new(move |e: PressEvent| {
            (e.continue_propagation)();
            match e.pointer_type {
                PointerType::Mouse | PointerType::Touch => {
                    if let Some(on_long_press_start) = input.on_long_press_start {
                        on_long_press_start.call(LongPressEvent::from(e.clone()));
                    }

                    // Let's set the timeout notifying us about a successful long press.
                    let e2: PressEvent = e.clone();
                    let prev = timeout_handle.replace(Some(
                        set_timeout_with_handle(
                            move || {
                                // Prevent other use_press handlers from also handling this event.
                                if let Some(target) = &e2.target {
                                    let pointer_cancel_event =
                                        web_sys::PointerEvent::new("pointercancel").expect("valid"); // TODO: {bubbles: true} ?
                                    target
                                        .dispatch_event(&pointer_cancel_event)
                                        .expect("dispatchable");
                                }
                                input.on_long_press.call(LongPressEvent::from(e2));
                            },
                            input.threshold,
                        )
                        .expect("set_timeout_with_handle to not fail"),
                    ));
                    if let Some(prev) = prev {
                        prev.clear();
                    }

                    // Prevent context menu, which may be opened on long press on touch devices!
                    if let PointerType::Touch = e.pointer_type {
                        let remove_contextmenu_listener = use_event_listener_with_options(
                            e.target,
                            leptos::ev::contextmenu,
                            move |e| {
                                e.prevent_default();
                            },
                            UseEventListenerOptions::default().once(true),
                        );
                        // There is no need to remove this listener explicitly, as it is `once` and removed automatically.
                        let _ = use_event_listener_with_options(
                            use_window(),
                            leptos::ev::pointerup,
                            move |_e| {
                                // If no contextmenu event is fired quickly after pointerup, remove the handler
                                // so future context menu events outside a long press are not prevented.
                                let remove_contextmenu_listener =
                                    remove_contextmenu_listener.clone();
                                set_timeout(
                                    move || {
                                        remove_contextmenu_listener();
                                    },
                                    Duration::from_millis(30),
                                );
                            },
                            UseEventListenerOptions::default().once(true),
                        );
                    }
                }
                _ => {}
            }
        })),
        on_press_end: Some(Callback::new(move |e: PressEvent| {
            if let Some(timeout_handle) = timeout_handle2.replace(None) {
                timeout_handle.clear();
            }

            match e.pointer_type {
                PointerType::Mouse | PointerType::Touch => {
                    if let Some(on_long_press_end) = input.on_long_press_end {
                        on_long_press_end.call(LongPressEvent::from(e.clone()));
                    }
                }
                _ => {}
            }
        })),
    });

    UseLongPressReturn {
        props: UseLongPressProps {
            attrs: Attributes::new()
                .merge(press_props.attrs)
                // TODO: react-aria uses dynamically created dom nodes, populated with the description string and referenced with aria-describedby. We could also do that instead...
                .insert("aria-description", input.accessibility_description),
            handlers: EventHandlers::builder().build().merge(press_props.handlers),
        },
    }
}
