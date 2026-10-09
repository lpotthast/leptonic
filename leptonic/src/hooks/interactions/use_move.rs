// Upstream: react-aria/src/interactions/useMove.ts @ 99e6102368
// Upstream: react-aria/test/interactions/useMove.test.js @ 99e6102368
//! Move interactions: dragging with a pointer and the arrow keys.
use leptos::{ev, prelude::*};
use web_sys::{KeyboardEvent, PointerEvent};

use crate::{EventHandler, IntoAttrs, Modifiers, OnEvent, utils::pointer_type::PointerType};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Events carry `Modifiers` (react-aria: four boolean fields).
// - As upstream, move events have no `continuePropagation` (the pointer down and the arrow keys
//   are always stopped), so they don't implement `Propagation`.
//
// ## DIFFERENT BEHAVIOR
// - Pointer events of a type the browser can't tell (`PointerType::Unknown`: an empty or
//   vendor-specific `pointerType`) move as a mouse (react-aria: an empty `pointerType` is
//   `'mouse'`, others pass on).
//
// ## ADDITIONS
// - `is_disabled`: a disabled element starts no moves (react-aria's callers leave out the props
//   instead), so callers needn't switch the props themselves.
//
// ## OMITTED FEATURES
// - The mouse and touch fallbacks for environments without `PointerEvent` (react-aria uses them
//   in tests only): every supported browser has pointer events.
// - The IE key names `Left`/`Right`/`Up`/`Down` (no legacy).
//
// =============================================================================

/// A move started (on its first movement).
#[derive(Debug, Clone)]
pub struct MoveStartEvent {
    /// The pointer type that started the move.
    pub pointer_type: PointerType,
    /// Keyboard modifiers held during the event.
    pub modifiers: Modifiers,
}

/// A movement.
#[derive(Debug, Clone)]
pub struct MoveEvent {
    /// The pointer type of the move.
    pub pointer_type: PointerType,
    /// Keyboard modifiers held during the event.
    pub modifiers: Modifiers,
    /// The horizontal movement since the last move event, in pixels (keyboard: ±1).
    pub delta_x: f64,
    /// The vertical movement since the last move event, in pixels (keyboard: ±1).
    pub delta_y: f64,
}

/// A move that started ended.
#[derive(Debug, Clone)]
pub struct MoveEndEvent {
    /// The pointer type that ended the move.
    pub pointer_type: PointerType,
    /// Keyboard modifiers held during the event.
    pub modifiers: Modifiers,
}

/// Input of [`use_move`].
#[derive(Clone, Copy)]
pub struct UseMoveInput {
    /// Whether movement is disabled.
    pub is_disabled: Signal<bool>,
    /// Called when a move starts (on its first movement, not on pointer down).
    pub on_move_start: Option<Callback<MoveStartEvent>>,
    /// Called for each movement.
    pub on_move: Option<Callback<MoveEvent>>,
    /// Called when a move that started ends.
    pub on_move_end: Option<Callback<MoveEndEvent>>,
}

impl Default for UseMoveInput {
    fn default() -> Self {
        Self {
            is_disabled: Signal::stored(false),
            on_move_start: None,
            on_move: None,
            on_move_end: None,
        }
    }
}

/// Return value of [`use_move`].
pub struct UseMoveReturn {
    pub props: UseMoveProps,
}

/// Props for the movable element.
#[derive(Debug)]
pub struct UseMoveProps {
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
}

impl IntoAttrs for UseMoveProps {
    type Attrs = UseMoveAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.on_pointerdown.into_on(ev::pointerdown),
            self.on_keydown.into_on(ev::keydown),
        )
    }
}

pub type UseMoveAttrs = (OnEvent<ev::pointerdown>, OnEvent<ev::keydown>);

/// Handles move interactions: dragging with a mouse, pen or touch, and the arrow keys
/// (react-aria's `useMove`). A move starts on the first movement after a primary button press
/// (not on the press itself) and reports the movement since the last event; an arrow key reports a
/// one-pixel move in its direction (callers decide what a move along an axis means). Only one
/// pointer moves at a time.
pub fn use_move(input: UseMoveInput) -> UseMoveReturn {
    #[cfg(feature = "ssr")]
    {
        let _ = input;
        UseMoveReturn {
            props: UseMoveProps {
                on_pointerdown: EventHandler::empty(),
                on_keydown: EventHandler::empty(),
            },
        }
    }

    #[cfg(not(feature = "ssr"))]
    {
        use wasm_bindgen::JsCast;

        use crate::{
            EventModifiers,
            utils::{
                event_listeners::{Listener, listen_to},
                key::{KeyboardEventKey, KeyboardKey},
                text_selection::{disable_text_selection, restore_text_selection},
            },
        };

        /// The move in progress.
        struct MoveState {
            pointer_id: i32,
            last_position: (f64, f64),
            _listeners: Vec<Listener>,
        }

        let UseMoveInput {
            is_disabled,
            on_move_start,
            on_move,
            on_move_end,
        } = input;
        let state = StoredValue::new_local(None::<MoveState>);
        // Whether the current move (pointer or keyboard) reported a movement yet.
        let did_move = StoredValue::new(false);

        let start = move || {
            disable_text_selection(None);
            did_move.set_value(false);
        };
        let move_by = move |pointer_type: PointerType, modifiers: Modifiers, dx: f64, dy: f64| {
            if dx == 0.0 && dy == 0.0 {
                return;
            }
            if !did_move.get_value() {
                did_move.set_value(true);
                if let Some(on_move_start) = on_move_start {
                    on_move_start.run(MoveStartEvent {
                        pointer_type,
                        modifiers,
                    });
                }
            }
            if let Some(on_move) = on_move {
                on_move.run(MoveEvent {
                    pointer_type,
                    modifiers,
                    delta_x: dx,
                    delta_y: dy,
                });
            }
        };
        let end = move |pointer_type: PointerType, modifiers: Modifiers| {
            restore_text_selection(None);
            if did_move.get_value()
                && let Some(on_move_end) = on_move_end
            {
                on_move_end.run(MoveEndEvent {
                    pointer_type,
                    modifiers,
                });
            }
        };

        let on_pointer_move = move |e: PointerEvent| {
            let Some((last_x, last_y)) = state.with_value(|s| {
                s.as_ref()
                    .filter(|s| s.pointer_id == e.pointer_id())
                    .map(|s| s.last_position)
            }) else {
                return;
            };
            let (x, y) = (e.page_x(), e.page_y());
            state.update_value(|s| {
                if let Some(s) = s.as_mut() {
                    s.last_position = (x, y);
                }
            });
            // `movementX`/`movementY` are always 0 in Safari on macOS, and scaled by the device
            // pixel ratio in Chrome on Android (react-aria): the page coordinates' differences.
            move_by(move_pointer_type(&e), e.modifiers(), x - last_x, y - last_y);
        };
        let on_pointer_up = move |e: PointerEvent| {
            let is_ours =
                state.with_value(|s| s.as_ref().is_some_and(|s| s.pointer_id == e.pointer_id()));
            if !is_ours {
                return;
            }
            // Dropping the state removes the window listeners.
            state.set_value(None);
            end(move_pointer_type(&e), e.modifiers());
        };

        let handle_pointer_down = move |e: PointerEvent| {
            if e.button() != 0 || is_disabled.get_untracked() || state.with_value(Option::is_some) {
                return;
            }
            start();
            e.stop_propagation();
            e.prevent_default();
            // The window of the element's document (react-aria's `getOwnerWindow`).
            let Some(window) = e
                .target()
                .and_then(|target| target.dyn_into::<web_sys::Node>().ok())
                .and_then(|node| node.owner_document())
                .and_then(|document| document.default_view())
            else {
                return;
            };
            let listeners = vec![
                listen_to(&window, ev::pointermove, false, on_pointer_move),
                listen_to(&window, ev::pointerup, false, on_pointer_up),
                listen_to(&window, ev::pointercancel, false, on_pointer_up),
            ];
            state.set_value(Some(MoveState {
                pointer_id: e.pointer_id(),
                last_position: (e.page_x(), e.page_y()),
                _listeners: listeners,
            }));
        };

        let handle_keydown = move |e: KeyboardEvent| {
            if is_disabled.get_untracked() {
                return;
            }
            let (dx, dy) = match e.typed_key() {
                KeyboardKey::ArrowLeft => (-1.0, 0.0),
                KeyboardKey::ArrowRight => (1.0, 0.0),
                KeyboardKey::ArrowUp => (0.0, -1.0),
                KeyboardKey::ArrowDown => (0.0, 1.0),
                _ => return,
            };
            e.prevent_default();
            e.stop_propagation();
            let modifiers = e.modifiers();
            start();
            move_by(PointerType::Keyboard, modifiers, dx, dy);
            end(PointerType::Keyboard, modifiers);
        };

        // A move in progress when the element goes away: its listeners go with the state.
        on_cleanup(move || {
            if state.try_update_value(Option::take).flatten().is_some() {
                restore_text_selection(None);
            }
        });

        UseMoveReturn {
            props: UseMoveProps {
                on_pointerdown: EventHandler::new(handle_pointer_down),
                on_keydown: EventHandler::new(handle_keydown),
            },
        }
    }
}

/// The pointer type a pointer event moves as: an unknown one as a mouse (react-aria:
/// `e.pointerType || 'mouse'`).
#[cfg(not(feature = "ssr"))]
fn move_pointer_type(e: &PointerEvent) -> PointerType {
    match PointerType::of(e) {
        PointerType::Unknown => PointerType::Mouse,
        pointer_type => pointer_type,
    }
}
