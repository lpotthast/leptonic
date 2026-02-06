use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use leptos_use::use_event_listener;
use web_sys::PointerEvent;

use crate::utils::{EventHandler, EventTargetExt};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/interactions/src/useMove.ts

/// Axis constraint for movement.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum MoveAxis {
    /// Only allow horizontal movement.
    Horizontal,
    /// Only allow vertical movement.
    Vertical,
    /// Allow movement in both directions.
    #[default]
    Both,
}

#[derive(Debug, Clone, Copy)]
pub struct MoveStartEvent {
    /// The initial pointer position (page coordinates) when movement started.
    pub page_x: f64,
    pub page_y: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct MoveEvent {
    pub delta_x: f64,
    pub delta_y: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct MoveEndEvent {}

#[derive(Debug, Clone, Copy)]
pub struct UseMoveInput {
    /// Optional axis constraint for movement. When `None`, movement is unrestricted.
    pub axis: Signal<Option<MoveAxis>>,
    pub on_move_start: Callback<MoveStartEvent>,
    pub on_move: Callback<MoveEvent>,
    pub on_move_end: Callback<MoveEndEvent>,
}

#[derive(Debug, Clone)]
pub struct UseMoveReturn {
    /// Props for programmatic merging. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub props: UseMoveProps,
}

/// Props from `use_move` that can be extracted and merged programmatically.
#[derive(Debug, Clone)]
pub struct UseMoveProps {
    pub on_pointerdown: EventHandler<PointerEvent>,
}

impl UseMoveProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseMoveAttrs {
        (self.on_pointerdown.to_on(ev::pointerdown),)
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseMoveAttrs {
        (self.on_pointerdown.into_on(ev::pointerdown),)
    }
}

pub type UseMoveAttrs = (On<ev::pointerdown, SharedEventCallback<PointerEvent>>,);

#[allow(clippy::struct_field_names)]
struct MoveEventHandlers {
    global_on_pointer_move_cleanup: Box<dyn Fn() + Send + Sync + 'static>,
    global_on_pointer_up_cleanup: Box<dyn Fn() + Send + Sync + 'static>,
    global_on_pointer_cancel_cleanup: Box<dyn Fn() + Send + Sync + 'static>,
}

impl MoveEventHandlers {
    fn cleanup(&self) {
        (self.global_on_pointer_move_cleanup)();
        (self.global_on_pointer_up_cleanup)();
        (self.global_on_pointer_cancel_cleanup)();
    }
}

struct MoveState {
    pointer_id: i32,
    moved: bool,
    initial_pos: (f64, f64),
    last_pos: (f64, f64),
    event_handlers: MoveEventHandlers,
}

/// # Panics
///
/// Panics if the current target of the pointer event is not available.
#[allow(clippy::too_many_lines)]
pub fn use_move(input: UseMoveInput) -> UseMoveReturn {
    // Note: There may be multiple pointers. Every pointer event contains a unique identifier of the pointer used for the interaction.
    // We start movement tracking by listening for on_pointer_down events.
    // Only movements from the pointer which initiated the tracking is propagated.

    let state: StoredValue<Option<MoveState>, LocalStorage> = StoredValue::new_local(None);
    let axis = input.axis;

    // Handle pointer move during drag
    let on_pointer_move = move |e: PointerEvent| {
        let pointer_id = e.pointer_id();

        state.update_value(move |s| {
            if let Some(s) = s.as_mut() {
                if s.pointer_id != pointer_id {
                    return;
                }

                let first_move = !s.moved;
                let (old_x, old_y) = s.last_pos;
                let (new_x, new_y) = (f64::from(e.page_x()), f64::from(e.page_y()));

                s.moved = true;
                s.last_pos = (new_x, new_y);

                // Apply axis filtering.
                let axis = axis.get_untracked();
                let delta_x = match axis {
                    Some(MoveAxis::Vertical) => 0.0,
                    _ => new_x - old_x,
                };
                let delta_y = match axis {
                    Some(MoveAxis::Horizontal) => 0.0,
                    _ => new_y - old_y,
                };

                if first_move {
                    let (initial_x, initial_y) = s.initial_pos;
                    input.on_move_start.run(MoveStartEvent {
                        page_x: initial_x,
                        page_y: initial_y,
                    });
                }
                input.on_move.run(MoveEvent { delta_x, delta_y });
            }
        });
    };

    // Handle pointer up to end the drag
    let on_pointer_up = move |e: PointerEvent| {
        let pointer_id = e.pointer_id();

        let should_clear = state.with_value(|s| {
            if let Some(s) = s.as_ref() {
                if s.pointer_id == pointer_id {
                    if s.moved {
                        input.on_move_end.run(MoveEndEvent {});
                    }
                    s.event_handlers.cleanup();
                    return true;
                }
            }
            false
        });

        if should_clear {
            state.set_value(None);
        }
    };

    // Handle pointer cancel to end the drag
    let on_pointer_cancel = move |e: PointerEvent| {
        let pointer_id = e.pointer_id();

        let should_clear = state.with_value(|s| {
            if let Some(s) = s.as_ref() {
                if s.pointer_id == pointer_id {
                    if s.moved {
                        input.on_move_end.run(MoveEndEvent {});
                    }
                    s.event_handlers.cleanup();
                    return true;
                }
            }
            false
        });

        if should_clear {
            state.set_value(None);
        }
    };

    let on_pointer_down = move |e: PointerEvent| {
        let pointer_id = e.pointer_id();

        if e.button() == 0 && state.with_value(Option::is_none) {
            e.stop_propagation();
            e.prevent_default();

            // Get the document to attach global listeners
            let doc = e.current_target().unwrap().get_owner_document();

            // Attach global event listeners for the duration of the drag
            // Clone handlers before passing to use_event_listener to avoid consuming them
            let event_handlers = MoveEventHandlers {
                global_on_pointer_move_cleanup: Box::new(use_event_listener(
                    doc.clone(),
                    ev::pointermove,
                    on_pointer_move,
                )),
                global_on_pointer_up_cleanup: Box::new(use_event_listener(
                    doc.clone(),
                    ev::pointerup,
                    on_pointer_up,
                )),
                global_on_pointer_cancel_cleanup: Box::new(use_event_listener(
                    doc,
                    ev::pointercancel,
                    on_pointer_cancel,
                )),
            };

            let pos = (f64::from(e.page_x()), f64::from(e.page_y()));
            state.set_value(Some(MoveState {
                pointer_id,
                moved: false,
                initial_pos: pos,
                last_pos: pos,
                event_handlers,
            }));
        }
    };

    on_cleanup(move || {
        state.with_value(|s| {
            if let Some(s) = s.as_ref() {
                s.event_handlers.cleanup();
            }
        });
    });

    UseMoveReturn {
        props: UseMoveProps {
            on_pointerdown: EventHandler::new(on_pointer_down),
        },
    }
}
