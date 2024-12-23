use leptos::StoredValue;
use leptos_reactive::{
    create_effect, create_signal, on_cleanup, store_value, Callable, Callback, MaybeSignal, Signal,
    SignalDispose, SignalGet, SignalGetUntracked, SignalSet,
};
use typed_builder::TypedBuilder;
use web_sys::PointerEvent;

use crate::utils::{
    attributes::Attributes, event_handlers::EventHandlers, pointer_type::PointerType, EventExt,
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/interactions/src/useHover.ts

#[derive(Debug, Clone)]
pub struct HoverStartEvent {
    /// The pointer type that triggered the hover event.
    /// This can only be either `PointerType::Mouse` or `PointerType::Pen`.
    pub pointer_type: PointerType,

    /// The target element.
    pub current_target: Option<web_sys::EventTarget>,
}

#[derive(Debug, Clone)]
pub struct HoverEndEvent {
    /// The pointer type that triggered the hover event.
    /// This can only be either `PointerType::Mouse` or `PointerType::Pen`.
    pub pointer_type: PointerType,

    /// The target element.
    pub current_target: Option<web_sys::EventTarget>,
}

#[derive(Debug, Clone, Copy, TypedBuilder)]
pub struct UseHoverInput {
    /// Whether hover callbacks should be disabled.
    /// When true, both `on_hover_start` and `on_hover_end` are no longer called.
    /// When the element is currently hovered when this switches to `true`,
    /// a programmatic `on_hover_end` is triggered and `is_hovered` transitions to `false`.
    #[builder(setter(into))]
    pub(crate) disabled: MaybeSignal<bool>,

    /// Called whenever a pointer starts hovering the element.
    /// This is optional, as `UseHoverReturn` also gives access to the `is_hovered` `Signal`.
    /// When using that signal alone, you may not need to react to this event.
    #[builder(default, setter(into, strip_option))]
    pub(crate) on_hover_start: Option<Callback<HoverStartEvent>>,

    /// Called whenever a pointer stops hovering the element
    /// or when the element is hovered and `disabled` transitions to `true`.
    /// This is optional, as `UseHoverReturn` also gives access to the `is_hovered` `Signal`.
    /// When using that signal alone, you may not need to react to this event.
    #[builder(default, setter(into, strip_option))]
    pub(crate) on_hover_end: Option<Callback<HoverEndEvent>>,
}

#[derive(Debug)]
pub struct UseHoverProps {
    /// These attributes must be spread onto the target element: `<foo {..props.attrs} />`
    pub attrs: Attributes,

    /// These handlers must be spread onto the target element: `<foo {..props.handlers} />`
    pub handlers: EventHandlers,
}

#[derive(Debug)]
pub struct UseHoverReturn {
    /// Properties which must be spread on an element.
    pub props: UseHoverProps,

    /// Whether the element is currently hovered.
    pub is_hovered: Signal<bool>,

    pub hover_responder: HoverResponder,
}

#[derive(Debug, Clone, Copy)]
pub struct HoverResponder {
    on_hover_start_handlers: StoredValue<Vec<Callback<HoverStartEvent>>>,
    on_hover_end_handlers: StoredValue<Vec<Callback<HoverEndEvent>>>,
}

impl HoverResponder {
    pub(crate) fn new() -> Self {
        HoverResponder {
            on_hover_start_handlers: store_value(Vec::new()),
            on_hover_end_handlers: store_value(Vec::new()),
        }
    }

    pub(crate) fn invoke_on_hover_start(&self, e: HoverStartEvent) {
        self.on_hover_start_handlers.with_value(move |handlers| {
            for h in handlers {
                h.call(e.clone());
            }
        });
    }

    pub(crate) fn invoke_on_hover_end(&self, e: HoverEndEvent) {
        self.on_hover_end_handlers.with_value(move |handlers| {
            for h in handlers {
                h.call(e.clone());
            }
        });
    }

    /// Adds an event handler to the end of the handler chain.
    pub fn add_on_hover_start(&self, handler: Callback<HoverStartEvent>) {
        self.on_hover_start_handlers.update_value(move |handlers| {
            handlers.push(handler);
        });
    }

    /// Adds an event handler to the end of the handler chain.
    pub fn add_on_hover_end(&self, handler: Callback<HoverEndEvent>) {
        self.on_hover_end_handlers.update_value(move |handlers| {
            handlers.push(handler);
        });
    }
}

#[derive(Debug, Clone)]
struct HoverState {
    pointer_type: PointerType,
}

pub fn use_hover(input: UseHoverInput) -> UseHoverReturn {
    let state = store_value(Option::<HoverState>::None);
    let (is_hovered, set_is_hovered) = create_signal(false);

    let hover_responder = HoverResponder::new();

    let trigger_hover_start =
        move |pointer_type: PointerType, current_target: Option<web_sys::EventTarget>| {
            if is_hovered.get_untracked() {
                return;
            }

            if pointer_type != PointerType::Mouse && pointer_type != PointerType::Pen {
                return;
            }

            let hover_start_event = HoverStartEvent {
                pointer_type: pointer_type.clone(),
                current_target,
            };

            hover_responder.invoke_on_hover_start(hover_start_event.clone());
            if let Some(on_hover_start) = input.on_hover_start {
                Callable::call(&on_hover_start, hover_start_event);
            }

            set_is_hovered.set(true);
            state.set_value(Some(HoverState { pointer_type }));
        };

    let trigger_hover_end = move |current_target: Option<web_sys::EventTarget>| {
        if !is_hovered.get_untracked() {
            return;
        }

        let hover_state = state.get_value().expect("present");
        let hover_end_event = HoverEndEvent {
            pointer_type: hover_state.pointer_type,
            current_target,
        };

        hover_responder.invoke_on_hover_end(hover_end_event.clone());
        if let Some(on_hover_end) = input.on_hover_end {
            Callable::call(&on_hover_end, hover_end_event);
        }

        set_is_hovered.set(false);
        state.set_value(None);
    };

    let on_pointer_enter = Box::new(move |e: PointerEvent| {
        if input.disabled.get_untracked() {
            return;
        }

        trigger_hover_start(PointerType::from(e.pointer_type()), e.current_target());
    });

    let on_pointer_leave = Box::new(move |e: PointerEvent| {
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
                .build(),
        },
        is_hovered: is_hovered.into(),
        hover_responder,
    }
}
