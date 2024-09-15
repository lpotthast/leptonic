use std::{cell::Cell, rc::Rc, time::Duration};

use educe::Educe;
use leptos::{
    leptos_dom::helpers::TimeoutHandle, set_timeout_with_handle, Callable, Oco, SignalGetUntracked,
};
use leptos_reactive::{create_signal, Callback, MaybeSignal, Signal, SignalSet};
use typed_builder::TypedBuilder;

use crate::{
    hooks::{use_hover, HoverEndEvent, HoverStartEvent, UseHoverInput, UseHoverReturn},
    utils::{attributes::Attributes, event_handlers::EventHandlers, pointer_type::PointerType},
};

#[derive(Educe, Clone)]
#[educe(Debug)]
pub struct LongHoverStartEvent {
    /// The pointer type that triggered the hover event.
    pub pointer_type: PointerType,

    pub current_target: Option<web_sys::EventTarget>,
}

#[derive(Educe, Clone)]
#[educe(Debug)]
pub struct LongHoverEndEvent {
    /// The pointer type that triggered the hover event.
    pub pointer_type: PointerType,

    pub current_target: Option<web_sys::EventTarget>,
}

impl From<HoverStartEvent> for LongHoverStartEvent {
    fn from(e: HoverStartEvent) -> Self {
        Self {
            pointer_type: e.pointer_type,
            current_target: e.current_target,
        }
    }
}

impl From<HoverEndEvent> for LongHoverEndEvent {
    fn from(e: HoverEndEvent) -> Self {
        Self {
            pointer_type: e.pointer_type,
            current_target: e.current_target,
        }
    }
}

#[derive(Debug, TypedBuilder)]
pub struct UseLongHoverInput {
    /// Whether long hover events should be disabled.
    #[builder(setter(into))]
    pub(crate) disabled: MaybeSignal<bool>,

    /// Handler that is called when the threshold time is met while
    /// the target is hovered (long hover interaction starts).
    #[builder(setter(into))]
    pub(crate) on_long_hover_start: Callback<LongHoverStartEvent>,

    /// Handler that is called when a long hover interaction ends,
    /// either over the target because it got disabled
    /// or when the pointer leaves the target.
    #[builder(setter(into))]
    pub(crate) on_long_hover_end: Callback<LongHoverEndEvent>,

    /// The amount of time to wait before triggering a long hover.
    /// Default is 500ms.
    pub(crate) threshold: Duration,

    /// A description for assistive technology users indicating that a long hover
    /// action is available, e.g. "Long hover to open menu".
    pub(crate) accessibility_description: Oco<'static, str>,
}

#[derive(Debug)]
pub struct UseLongHoverReturn {
    /// Properties which must be spread on an element.
    pub props: UseLongHoverProps,

    /// Whether the element is currently (long-)hovered.
    pub is_hovered: Signal<bool>,
}

#[derive(Debug)]
pub struct UseLongHoverProps {
    /// These attributes must be spread onto the target element: `<foo {..props.attrs} />`
    pub attrs: Attributes,
    /// These handlers must be spread onto the target element: `<foo {..props.handlers} />`
    pub handlers: EventHandlers,
}

/// Handles long hover interactions.
/// Supports a customizable time threshold, accessibility description,
/// and normalizes behavior across browsers and devices.
pub fn use_long_hover(input: UseLongHoverInput) -> UseLongHoverReturn {
    let timeout_handle: Rc<Cell<Option<TimeoutHandle>>> = Rc::new(Cell::new(None));
    let timeout_handle2: Rc<Cell<Option<TimeoutHandle>>> = timeout_handle.clone();

    let (is_hovered, set_is_hovered) = create_signal(false);

    let UseHoverReturn {
        props: hover_props,
        is_hovered: _,
        hover_responder: _,
    } = use_hover(UseHoverInput {
        disabled: input.disabled,
        on_hover_start: Some(Callback::new(move |e: HoverStartEvent| {
            // Let's set the timeout notifying us about a successful long hover.
            let prev = timeout_handle.replace(Some(
                set_timeout_with_handle(
                    move || {
                        input
                            .on_long_hover_start
                            .call(LongHoverStartEvent::from(e.clone()));
                        set_is_hovered.set(true);
                    },
                    input.threshold,
                )
                .expect("set_timeout_with_handle to not fail"),
            ));
            if let Some(prev) = prev {
                prev.clear();
            }
        })),
        on_hover_end: Some(Callback::new(move |e: HoverEndEvent| {
            if let Some(timeout_handle) = timeout_handle2.replace(None) {
                timeout_handle.clear();
            }
            if is_hovered.get_untracked() {
                input
                    .on_long_hover_end
                    .call(LongHoverEndEvent::from(e.clone()));
            }
            set_is_hovered.set(false);
        })),
    });

    UseLongHoverReturn {
        props: UseLongHoverProps {
            attrs: Attributes::new()
                .merge(hover_props.attrs)
                // TODO: react-aria uses dynamically created dom nodes, populated with the description string and referenced with aria-describedby. We could also do that instead...
                .insert("aria-description", input.accessibility_description),
            handlers: hover_props.handlers,
        },
        is_hovered: is_hovered.into(),
    }
}
