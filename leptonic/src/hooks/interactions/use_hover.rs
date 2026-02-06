use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use leptos_use::{use_event_listener, use_event_listener_with_options, UseEventListenerOptions};
use send_wrapper::SendWrapper;
use std::sync::atomic::{AtomicBool, Ordering};
use web_sys::PointerEvent;

use crate::utils::{
    current_target_contains_target, pointer_type::PointerType, EventExt, EventHandler,
    EventTargetExt,
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/interactions/src/useHover.ts

// iOS fires onPointerEnter twice: once with pointerType="touch" and again with
// pointerType="mouse". We want to ignore these emulated events so they do not trigger hover
// behavior. See https://bugs.webkit.org/show_bug.cgi?id=214609.
//
// When a touch pointerenter is detected, a temporary global pointerup listener is registered.
// When the touch pointerup fires (finger lifts), this flag is set to true. The emulated mouse
// pointerenter that follows sees the flag and bails out. After 50ms, a timeout clears the flag
// and removes the temporary listener. The 50ms window is measured from pointerup to keep it
// tight. Long enough to catch the emulated event, short enough to not block real mouse hovers
// that may happen shortly after a touch.
static IGNORE_EMULATED_MOUSE_EVENTS: AtomicBool = AtomicBool::new(false);

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

    /// Called whenever the hover state changes.
    pub on_hover_change: Option<Callback<bool>>,
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

struct HoverState {
    pointer_type: PointerType,
    target: Option<web_sys::EventTarget>,
    /// Cleanup function for the global `pointerover` listener that detects element removal.
    global_pointerover_cleanup: Option<Box<dyn Fn()>>,
}

impl HoverState {
    fn cleanup_global_listeners(&self) {
        if let Some(cleanup) = &self.global_pointerover_cleanup {
            cleanup();
        }
    }
}

/// # Panics
///
/// Panics if the hover state is expected to be present but is not.
#[allow(clippy::too_many_lines)]
pub fn use_hover(input: UseHoverInput) -> UseHoverReturn {
    let state: StoredValue<Option<HoverState>, LocalStorage> = StoredValue::new_local(None);
    let (is_hovered, set_is_hovered) = signal(false);

    let trigger_hover_end = move || {
        if !is_hovered.get_untracked() {
            return;
        }

        let (pointer_type, target) = state.with_value(|s| {
            let s = s.as_ref().expect("present");
            (s.pointer_type.clone(), s.target.clone())
        });

        // Clean up global listeners before clearing state.
        state.with_value(|s| {
            if let Some(s) = s.as_ref() {
                s.cleanup_global_listeners();
            }
        });

        if let Some(on_hover_end) = input.on_hover_end {
            on_hover_end.run(HoverEndEvent {
                pointer_type,
                current_target: target.map(SendWrapper::new),
            });
        }

        if let Some(on_hover_change) = input.on_hover_change {
            on_hover_change.run(false);
        }

        set_is_hovered.set(false);
        state.set_value(None);
    };

    let trigger_hover_start =
        move |pointer_type: PointerType,
              current_target: Option<web_sys::EventTarget>,
              target: Option<web_sys::EventTarget>| {
            if is_hovered.get_untracked() {
                return;
            }

            if pointer_type != PointerType::Mouse && pointer_type != PointerType::Pen {
                return;
            }

            // Ensure that the event target is contained within current_target.
            // This guards against events that bubble from outside the element.
            if current_target_contains_target(current_target.as_ref(), target.as_ref())
                == Some(false)
            {
                return;
            }

            if let Some(on_hover_start) = input.on_hover_start {
                on_hover_start.run(HoverStartEvent {
                    pointer_type: pointer_type.clone(),
                    current_target: current_target.clone().map(SendWrapper::new),
                });
            }

            if let Some(on_hover_change) = input.on_hover_change {
                on_hover_change.run(true);
            }

            set_is_hovered.set(true);

            // When an element that is hovered over is removed from the DOM, no pointerleave event
            // is fired by the browser. However, a pointerover event will be fired on the new target
            // the mouse is over. We detect this case by checking if the new pointerover target is
            // still contained within our hovered element — if not, the element was removed and we
            // trigger a hover end.
            let global_pointerover_cleanup = current_target.as_ref().and_then(|ct| {
                let ct_for_closure = ct.clone();
                let cleanup = use_event_listener_with_options(
                    ct.as_node()?.owner_document()?,
                    ev::pointerover,
                    move |e: PointerEvent| {
                        if is_hovered.get_untracked() {
                            if let Some(event_target) = e.target() {
                                if current_target_contains_target(
                                    Some(&ct_for_closure),
                                    Some(&event_target),
                                ) == Some(false)
                                {
                                    trigger_hover_end();
                                }
                            }
                        }
                    },
                    UseEventListenerOptions::default().capture(true),
                );

                Some(Box::new(cleanup) as Box<dyn Fn()>)
            });

            state.set_value(Some(HoverState {
                pointer_type,
                target: current_target,
                global_pointerover_cleanup,
            }));
        };

    let on_pointer_enter = move |e: PointerEvent| {
        if input.disabled.get_untracked() {
            return;
        }

        if IGNORE_EMULATED_MOUSE_EVENTS.load(Ordering::Acquire) && e.pointer_type() == "mouse" {
            return;
        }

        // When a touch pointerenter is detected, register a temporary global pointerup listener.
        // On iOS, after a touch, a phantom pointerenter with pointerType="mouse" is fired.
        // The pointerup listener sets the ignore flag so the emulated mouse event is skipped.
        if e.pointer_type() == "touch" {
            let cleanup = use_event_listener(document(), ev::pointerup, move |pu: PointerEvent| {
                if pu.pointer_type() == "touch" {
                    IGNORE_EMULATED_MOUSE_EVENTS.store(true, Ordering::Release);
                }
            });
            let cleanup = StoredValue::<Option<Box<dyn Fn()>>, LocalStorage>::new_local(Some(
                Box::new(cleanup),
            ));
            set_timeout(
                move || {
                    IGNORE_EMULATED_MOUSE_EVENTS.store(false, Ordering::Release);
                    cleanup.with_value(|c| {
                        if let Some(cleanup_fn) = c.as_ref() {
                            cleanup_fn();
                        }
                    });
                    cleanup.set_value(None);
                },
                std::time::Duration::from_millis(50),
            );
        }

        trigger_hover_start(
            PointerType::from(e.pointer_type()),
            e.current_target(),
            e.target(),
        );
    };

    let on_pointer_leave = move |e: PointerEvent| {
        if input.disabled.get_untracked()
            || state.with_value(Option::is_none)
            || !e.current_target_contains_target()
        {
            return;
        }

        trigger_hover_end();
    };

    let _cancel_hover_when_disabled = Effect::new(move |_| {
        if input.disabled.get() {
            trigger_hover_end();
        }
    });

    on_cleanup(move || {
        // Clean up any active global listeners.
        state.with_value(|s| {
            if let Some(s) = s.as_ref() {
                s.cleanup_global_listeners();
            }
        });
    });

    UseHoverReturn {
        props: UseHoverProps {
            on_pointerenter: EventHandler::new(on_pointer_enter),
            on_pointerleave: EventHandler::new(on_pointer_leave),
        },
        is_hovered: is_hovered.into(),
    }
}
