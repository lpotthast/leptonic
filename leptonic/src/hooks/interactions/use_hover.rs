// Upstream: react-aria/src/interactions/useHover.ts @ 99e6102368
use leptos::{
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use send_wrapper::SendWrapper;
use web_sys::PointerEvent;

use crate::{
    hooks::IntoAttrs,
    utils::{EventHandler, pointer_type::PointerType},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `HoverStartEvent`/`HoverEndEvent` instead of one `HoverEvent` with a `type`; the hovered
//   element is `current_target` (react-aria: `target`, set to the event's current target).
// - As upstream, hover events don't stop propagation and have no `continuePropagation`, so they
//   don't implement `Propagation`.
//
// ## OMITTED FEATURES
// - The `mouseenter`/`mouseleave`/`touchstart` fallbacks for environments without
//   `PointerEvent`: every supported browser has pointer events.
//
// =============================================================================

/// iOS fires `pointerenter` twice: once with `pointerType="touch"` and again with
/// `pointerType="mouse"` (https://bugs.webkit.org/show_bug.cgi?id=214609). After a touch
/// `pointerup`, emulated mouse hovers are ignored for 500 ms. One document listener, shared by
/// all hover hooks (react-aria's `setupGlobalTouchEvents`).
#[cfg(not(feature = "ssr"))]
mod global_touch {
    use std::{
        cell::{Cell, RefCell},
        time::Duration,
    };

    use leptos::{ev, prelude::set_timeout};

    use crate::utils::{
        event_listeners::{Listener, listen_to},
        pointer_type::PointerType,
    };

    thread_local! {
        static IGNORE_EMULATED_MOUSE_EVENTS: Cell<bool> = const { Cell::new(false) };
        static HOVER_COUNT: Cell<usize> = const { Cell::new(0) };
        static POINTERUP_LISTENER: RefCell<Option<Listener>> = const { RefCell::new(None) };
    }

    pub(super) fn ignore_emulated_mouse_events() -> bool {
        IGNORE_EMULATED_MOUSE_EVENTS.get()
    }

    /// Registers a hover hook; the first one adds the document listener.
    pub(super) fn setup() {
        if HOVER_COUNT.get() == 0
            && let Some(document) = leptos_use::use_document().as_ref()
        {
            let listener = listen_to(
                document,
                ev::pointerup,
                false,
                |e: web_sys::PointerEvent| {
                    if PointerType::from(e.pointer_type()) == PointerType::Touch {
                        IGNORE_EMULATED_MOUSE_EVENTS.set(true);
                        set_timeout(
                            || IGNORE_EMULATED_MOUSE_EVENTS.set(false),
                            Duration::from_millis(500),
                        );
                    }
                },
            );
            POINTERUP_LISTENER.set(Some(listener));
        }
        HOVER_COUNT.set(HOVER_COUNT.get() + 1);
    }

    /// Unregisters a hover hook; the last one removes the document listener.
    pub(super) fn teardown() {
        HOVER_COUNT.set(HOVER_COUNT.get().saturating_sub(1));
        if HOVER_COUNT.get() == 0 {
            POINTERUP_LISTENER.set(None);
        }
    }
}

/// A pointer started hovering the element.
#[derive(Debug, Clone)]
pub struct HoverStartEvent {
    /// The pointer's type (`Mouse` or `Pen`).
    pub pointer_type: PointerType,
    /// The hovered element.
    pub current_target: SendWrapper<web_sys::EventTarget>,
}

/// A pointer stopped hovering the element.
#[derive(Debug, Clone)]
pub struct HoverEndEvent {
    /// The pointer's type (`Mouse` or `Pen`).
    pub pointer_type: PointerType,
    /// The element that was hovered.
    pub current_target: SendWrapper<web_sys::EventTarget>,
}

/// Input of [`use_hover`].
#[derive(Debug, Clone, Copy)]
pub struct UseHoverInput {
    /// Whether hover callbacks should be disabled.
    /// When true, both `on_hover_start` and `on_hover_end` are no longer called.
    /// When the element is currently hovered when this switches to `true`,
    /// a programmatic `on_hover_end` is triggered and `is_hovered` transitions to `false`.
    pub is_disabled: Signal<bool>,

    /// Called whenever a pointer starts hovering the element.
    pub on_hover_start: Option<Callback<HoverStartEvent>>,

    /// Called whenever a pointer stops hovering the element
    /// or when the element is hovered and `is_disabled` transitions to `true`.
    pub on_hover_end: Option<Callback<HoverEndEvent>>,

    /// Called whenever the hover state changes.
    pub on_hover_change: Option<Callback<bool>>,
}

impl Default for UseHoverInput {
    /// Enabled, no callbacks.
    fn default() -> Self {
        Self {
            is_disabled: Signal::stored(false),
            on_hover_start: None,
            on_hover_end: None,
            on_hover_change: None,
        }
    }
}

/// Return value of [`use_hover`].
#[derive(Debug)]
pub struct UseHoverReturn {
    /// Props for programmatic merging. Call `.into_attrs()` for view spreading.
    pub props: UseHoverProps,

    /// Whether the element is currently hovered.
    pub is_hovered: Signal<bool>,
}

/// Props from `use_hover` that can be extracted and merged programmatically.
///
/// Use [`UseHoverProps::into_attrs()`] to convert to an attributes-tuple spreadable using Leptos's
/// spreading syntax (`<div {..props.into_attrs()}>`) (taking ownership).
#[derive(Debug)]
pub struct UseHoverProps {
    pub on_pointerenter: EventHandler<PointerEvent>,
    pub on_pointerleave: EventHandler<PointerEvent>,
}

impl IntoAttrs for UseHoverProps {
    type Attrs = UseHoverAttrs;

    fn into_attrs(self) -> Self::Attrs {
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

#[cfg(not(feature = "ssr"))]
struct HoverState {
    pointer_type: PointerType,
    target: web_sys::EventTarget,
    /// The global `pointerover` listener that detects the removal of the hovered element
    /// (removed when the state is dropped).
    _pointerover: Option<crate::utils::event_listeners::Listener>,
}

/// Handles pointer hover interactions for an element (react-aria's `useHover`): hover starts when
/// a mouse or pen pointer enters the element (touch never hovers, and the mouse events iOS emulates
/// after a touch are ignored) and ends when it leaves, the element is removed or `is_disabled`
/// becomes true.
#[allow(clippy::too_many_lines)]
pub fn use_hover(input: UseHoverInput) -> UseHoverReturn {
    #[cfg(feature = "ssr")]
    {
        let _ = input;
        let (is_hovered, _) = signal(false);
        UseHoverReturn {
            props: UseHoverProps {
                on_pointerenter: EventHandler::new(|_: PointerEvent| {}),
                on_pointerleave: EventHandler::new(|_: PointerEvent| {}),
            },
            is_hovered: is_hovered.into(),
        }
    }

    #[cfg(not(feature = "ssr"))]
    {
        use crate::utils::{
            ContainsTarget, EventAccessors, EventTargetExt, event_listeners::listen_to,
            node_contains,
        };

        let UseHoverInput {
            is_disabled: disabled,
            on_hover_start,
            on_hover_end,
            on_hover_change,
        } = input;

        global_touch::setup();
        on_cleanup(global_touch::teardown);

        let state: StoredValue<Option<HoverState>, LocalStorage> = StoredValue::new_local(None);
        let (is_hovered, set_is_hovered) = signal(false);

        let trigger_hover_end = move || {
            if !is_hovered.get_untracked() {
                return;
            }

            // Dropping the state removes its global listener.
            let Some(HoverState {
                pointer_type,
                target,
                ..
            }) = state.try_update_value(Option::take).flatten()
            else {
                return;
            };

            if let Some(on_hover_end) = on_hover_end {
                on_hover_end.run(HoverEndEvent {
                    pointer_type,
                    current_target: SendWrapper::new(target),
                });
            }

            if let Some(on_hover_change) = on_hover_change {
                on_hover_change.run(false);
            }

            set_is_hovered.set(false);
        };

        let trigger_hover_start =
            move |pointer_type: PointerType,
                  current_target: web_sys::EventTarget,
                  target: web_sys::EventTarget| {
                if is_hovered.get_untracked() {
                    return;
                }

                if pointer_type == PointerType::Touch {
                    return;
                }

                // Ensure that the event target is contained within current_target.
                // This guards against events that bubble from outside the element.
                if node_contains(current_target.as_node().as_ref(), target.as_node().as_ref())
                    == Some(false)
                {
                    return;
                }

                if let Some(on_hover_start) = on_hover_start {
                    on_hover_start.run(HoverStartEvent {
                        pointer_type: pointer_type.clone(),
                        current_target: SendWrapper::new(current_target.clone()),
                    });
                }

                if let Some(on_hover_change) = on_hover_change {
                    on_hover_change.run(true);
                }

                set_is_hovered.set(true);

                // When an element that is hovered over is removed from the DOM, no pointerleave event
                // is fired by the browser. However, a pointerover event will be fired on the new target
                // the mouse is over. We detect this case by checking if the new pointerover target is
                // still contained within our hovered element — if not, the element was removed and we
                // trigger a hover end.
                let pointerover = current_target.get_owner_document().map(|document| {
                    let ct_for_closure = current_target.clone();
                    listen_to(&document, ev::pointerover, true, move |e: PointerEvent| {
                        if is_hovered.get_untracked() {
                            let event_target = e.expect_target();
                            if node_contains(
                                ct_for_closure.as_node().as_ref(),
                                event_target.as_node().as_ref(),
                            ) == Some(false)
                            {
                                trigger_hover_end();
                            }
                        }
                    })
                });

                state.set_value(Some(HoverState {
                    pointer_type,
                    target: current_target,
                    _pointerover: pointerover,
                }));
            };

        let handle_pointer_enter = move |e: PointerEvent| {
            if disabled.get_untracked() {
                return;
            }

            let pointer_type = PointerType::from(e.pointer_type());
            if global_touch::ignore_emulated_mouse_events() && pointer_type == PointerType::Mouse {
                return;
            }

            trigger_hover_start(pointer_type, e.expect_current_target(), e.expect_target());
        };

        let handle_pointer_leave = move |e: PointerEvent| {
            if disabled.get_untracked()
                || state.with_value(Option::is_none)
                || !e.current_target_contains_target()
            {
                return;
            }

            trigger_hover_end();
        };

        let _cancel_hover_when_disabled = Effect::new(move |_| {
            if disabled.get() {
                trigger_hover_end();
            }
        });

        on_cleanup(move || {
            // Dropping the state removes its global listener.
            state.try_update_value(Option::take);
        });

        UseHoverReturn {
            props: UseHoverProps {
                on_pointerenter: EventHandler::new(handle_pointer_enter),
                on_pointerleave: EventHandler::new(handle_pointer_leave),
            },
            is_hovered: is_hovered.into(),
        }
    }
}
