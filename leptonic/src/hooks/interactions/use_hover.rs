use std::rc::Rc;

use leptos_reactive::{
    create_effect, create_signal, on_cleanup, store_value, Callable, Callback, MaybeSignal, Signal,
    SignalDispose, SignalGet, SignalGetUntracked, SignalSet,
};
use web_sys::PointerEvent;

use crate::utils::{attributes::Attributes, event_handlers::EventHandlers, pointer_type::PointerType, EventExt};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/interactions/src/useHover.ts

#[derive(Debug, Clone)]
pub struct HoverStartEvent {
    pub pointer_type: PointerType,
    pub current_target: Option<web_sys::EventTarget>,
}

#[derive(Debug, Clone)]
pub struct HoverEndEvent {
    pub pointer_type: PointerType,
    pub current_target: Option<web_sys::EventTarget>,
}

#[derive(Debug, Clone, Copy)]
pub struct UseHoverInput {
    /// Whether hover callbacks should be disabled.
    /// When true, both `on_hover_start` and `on_hover_end` are no longer called.
    /// When the element is currently hovered when this switches to `true`,
    /// a programmatic `on_hover_end` is triggered and `is_hovered` transitions to `false`.
    pub disabled: MaybeSignal<bool>,

    /// Called whenever a pointer starts hovering the element.
    pub on_hover_start: Option<Callback<HoverStartEvent>>,

    /// Called whenever a pointer stops hovering the element
    /// or when the element is hovered and `disabled` transitions to `true`.
    pub on_hover_end: Option<Callback<HoverEndEvent>>,
}

#[derive(Debug)]
pub struct UseHoverProps {
    /// These attributes must be spread onto the target element: `<foo use:attrs=props.attrs />`
    pub attrs: Attributes,
    /// These handlers must be spread onto the target element: `<foo use:handlers=props.handlers />`
    pub handlers: EventHandlers,
}

#[derive(Debug)]
pub struct UseHoverReturn {
    /// Properties which must be spread on an element.
    pub props: UseHoverProps,

    /// Whether the element is currently hovered.
    pub is_hovered: Signal<bool>,
}

#[derive(Debug, Clone)]
struct HoverState {
    pointer_type: PointerType,
}

pub fn use_hover(input: UseHoverInput) -> UseHoverReturn {
    let state = store_value(Option::<HoverState>::None);
    let (is_hovered, set_is_hovered) = create_signal(false);

    let trigger_hover_start =
        move |pointer_type: PointerType, current_target: Option<web_sys::EventTarget>| {
            if is_hovered.get_untracked() {
                return;
            }

            if pointer_type != PointerType::Mouse && pointer_type != PointerType::Pen {
                return;
            }

            if let Some(on_hover_start) = input.on_hover_start {
                Callable::call(
                    &on_hover_start,
                    HoverStartEvent {
                        pointer_type: pointer_type.clone(),
                        current_target,
                    }
                );
            }

            set_is_hovered.set(true);
            state.set_value(Some(HoverState { pointer_type }));
        };

    let trigger_hover_end = move |current_target: Option<web_sys::EventTarget>| {
        if !is_hovered.get_untracked() {
            return;
        }

        let s = state.get_value().expect("present");
        if let Some(on_hover_end) = input.on_hover_end {
            Callable::call(
                &on_hover_end,
                HoverEndEvent {
                    pointer_type: s.pointer_type,
                    current_target,
                }
            );
        }

        set_is_hovered.set(false);
        state.set_value(None);
    };

    let on_pointer_enter = Rc::new(move |e: PointerEvent| {
        if input.disabled.get_untracked() {
            return;
        }

        trigger_hover_start(PointerType::from(e.pointer_type()), e.current_target());
    });

    let on_pointer_leave = Rc::new(move |e: PointerEvent| {
        if input.disabled.get_untracked()
            || state.with_value(|s| s.is_none())
            || !e.current_target_contains_target()
        {
            return;
        }

        trigger_hover_end(e.current_target());
    });

    let cancel_hover_when_disabled = create_effect(move |_| {
        if input.disabled.get() {
            trigger_hover_end(None);
        }
    });

    on_cleanup(move || {
        cancel_hover_when_disabled.dispose();
        is_hovered.dispose();
        set_is_hovered.dispose();
    });

    UseHoverReturn {
        props: UseHoverProps {
            attrs: Attributes::new(),
            handlers: EventHandlers::builder()
                .on_pointer_enter(on_pointer_enter)
                .on_pointer_leave(on_pointer_leave)
                .build()
        },
        is_hovered: is_hovered.into(),
    }
}
