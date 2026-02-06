use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use send_wrapper::SendWrapper;
use web_sys::PointerEvent;

use crate::utils::{pointer_type::PointerType, EventExt, EventHandler};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/interactions/src/useHover.ts

#[derive(Debug, Clone)]
pub struct HoverStartEvent {
    pub pointer_type: PointerType,
    pub current_target: Option<SendWrapper<web_sys::EventTarget>>,
}

#[derive(Debug, Clone)]
pub struct HoverEndEvent {
    pub pointer_type: PointerType,
    pub current_target: Option<SendWrapper<web_sys::EventTarget>>,
}

#[derive(Debug, Clone, Copy)]
pub struct UseHoverInput {
    /// Whether hover callbacks should be disabled.
    /// When true, both `on_hover_start` and `on_hover_end` are no longer called.
    /// When the element is currently hovered when this switches to `true`,
    /// a programmatic `on_hover_end` is triggered and `is_hovered` transitions to `false`.
    pub disabled: Signal<bool>,

    /// Called whenever a pointer starts hovering the element.
    pub on_hover_start: Option<Callback<HoverStartEvent>>,

    /// Called whenever a pointer stops hovering the element
    /// or when the element is hovered and `disabled` transitions to `true`.
    pub on_hover_end: Option<Callback<HoverEndEvent>>,
}

#[derive(Debug, Clone)]
pub struct UseHoverReturn {
    /// Props for programmatic merging. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub props: UseHoverProps,

    /// Whether the element is currently hovered.
    pub is_hovered: Signal<bool>,
}

/// Props from `use_hover` that can be extracted and merged programmatically.
///
/// Use [`UseHoverProps::into_attrs()`] to convert to an attributes-tuple spreadable using Leptos's
/// spreading syntax (`<div {..props.into_attrs()}>`) (taking ownership).
///
/// Use [`UseHoverProps::to_attrs()`] to convert to an attributes-tuple spreadable using Leptos's
/// spreading syntax (`<div {..props.to_attrs()}>`) (without taking ownership, requiring internal
/// cloning).
#[derive(Debug, Clone)]
pub struct UseHoverProps {
    pub on_pointerenter: EventHandler<PointerEvent>,
    pub on_pointerleave: EventHandler<PointerEvent>,
}

impl UseHoverProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseHoverAttrs {
        (
            self.on_pointerenter.to_on(ev::pointerenter),
            self.on_pointerleave.to_on(ev::pointerleave),
        )
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseHoverAttrs {
        (
            self.on_pointerenter.into_on(ev::pointerenter),
            self.on_pointerleave.into_on(ev::pointerleave),
        )
    }
}

/// These attributes must be spread onto the target element using the spread syntax `<div {..attrs}/>`.
pub type UseHoverAttrs = (
    On<ev::pointerenter, SharedEventCallback<PointerEvent>>,
    On<ev::pointerleave, SharedEventCallback<PointerEvent>>,
);

#[derive(Debug, Clone)]
struct HoverState {
    pointer_type: PointerType,
}

/// # Panics
///
/// Panics if the hover state is expected to be present but is not.
pub fn use_hover(input: UseHoverInput) -> UseHoverReturn {
    let state = StoredValue::new(Option::<HoverState>::None);
    let (is_hovered, set_is_hovered) = signal(false);

    let trigger_hover_start =
        move |pointer_type: PointerType, current_target: Option<web_sys::EventTarget>| {
            if is_hovered.get_untracked() {
                return;
            }

            if pointer_type != PointerType::Mouse && pointer_type != PointerType::Pen {
                return;
            }

            if let Some(on_hover_start) = input.on_hover_start {
                on_hover_start.run(HoverStartEvent {
                    pointer_type: pointer_type.clone(),
                    current_target: current_target.map(SendWrapper::new),
                });
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
            on_hover_end.run(HoverEndEvent {
                pointer_type: s.pointer_type,
                current_target: current_target.map(SendWrapper::new),
            });
        }

        set_is_hovered.set(false);
        state.set_value(None);
    };

    let on_pointer_enter = move |e: PointerEvent| {
        if input.disabled.get_untracked() {
            return;
        }

        trigger_hover_start(PointerType::from(e.pointer_type()), e.current_target());
    };

    let on_pointer_leave = move |e: PointerEvent| {
        if input.disabled.get_untracked()
            || state.with_value(Option::is_none)
            || !e.current_target_contains_target()
        {
            return;
        }

        trigger_hover_end(e.current_target());
    };

    let cancel_hover_when_disabled = Effect::new(move |_| {
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
            on_pointerenter: EventHandler::new(on_pointer_enter),
            on_pointerleave: EventHandler::new(on_pointer_leave),
        },
        is_hovered: is_hovered.into(),
    }
}
