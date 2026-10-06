// Upstream: react-aria/src/interactions/useMove.ts @ 99e6102368
#![cfg_attr(feature = "ssr", allow(dead_code, unused_imports))]

use leptos::{
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use web_sys::{KeyboardEvent, PointerEvent};

use crate::{
    hooks::IntoAttrs,
    utils::{
        EventAccessors, EventHandler, EventTargetExt,
        element_capture::{CapturedElement, ElementCaptureAttr},
        event_listeners::{Listener, listen_to},
        i18n::use_direction,
        locale::WritingDirection,
        modifiers::{EventModifiers, Modifiers},
        point::Point,
        pointer_type::PointerType,
        text_selection::{disable_text_selection, restore_text_selection},
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/react-aria/src/interactions/useMove.ts

// ## INTENTIONAL DEVIATIONS
//
// - `axis` constraint: Provided as a `Signal<MoveAxis>` on `UseMoveInput`
//   so callers can dynamically lock movement to a single axis.
//   React-aria does not have a built-in axis constraint on `useMove`.
//
// - `page_x` / `page_y` on `MoveStartEvent`: Exposes the initial pointer
//   position in page coordinates.
//   React-aria's `MoveStartEvent` does not include position data.
//
// - Constrained movement (`use_constrained_move(input, MoveConstraintOptions)`: the
//   `MoveConstraint` mode, container clicks, the initial position and
//   `on_position_change`): area-bounded movement within a container element,
//   with normalized and pixel (`Point`) positions. React-aria has no
//   equivalent; this absorbs the leptonic-specific `use_move_within` hook.
//
// - `NormalizedPosition` type: Typed wrapper for normalized [0.0, 1.0]
//   coordinates, preventing accidental mixing with pixel or page coordinates.
//   React-aria uses raw numbers.
//
// - `MoveConstraintOptions::on_position_change`: fires whenever the constrained
//   normalized position changes (pointer drag, container click, keyboard,
//   programmatic), for imperative sync without reactive Effects.
//
// ## OMISSIONS
//
// - Mouse/touch fallback event paths: We assume `PointerEvent` is always
//   available (see CLAUDE.md). React-aria has separate mouse and touch
//   code paths for older environments.

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

/// A position in normalized coordinates, each axis in [0.0, 1.0].
/// (0.0, 0.0) = top-left, (1.0, 1.0) = bottom-right.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct NormalizedPosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone)]
pub struct MoveStartEvent {
    /// The pointer type that initiated the move.
    pub pointer_type: PointerType,
    /// Keyboard modifiers held during the event.
    pub modifiers: Modifiers,
    /// The initial pointer position (page coordinates) when movement started.
    pub page_x: f64,
    pub page_y: f64,
}

#[derive(Debug, Clone)]
pub struct MoveEvent {
    /// The pointer type that initiated the move.
    pub pointer_type: PointerType,
    /// Keyboard modifiers held during the event.
    pub modifiers: Modifiers,
    pub delta_x: f64,
    pub delta_y: f64,
}

#[derive(Debug, Clone)]
pub struct MoveEndEvent {
    /// The pointer type that initiated the move.
    pub pointer_type: PointerType,
    /// Keyboard modifiers held during the event.
    pub modifiers: Modifiers,
}

/// How the movable element is constrained within its container.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveConstraint {
    /// Constrain the element's center within the container bounds.
    Center,
    /// Constrain the element's bounding box within the container bounds.
    Bounds,
}

#[derive(Copy, Clone)]
pub struct UseMoveInput {
    /// Whether movement is disabled.
    pub is_disabled: Signal<bool>,

    /// The axes the element moves along. Default: [`MoveAxis::Both`].
    pub axis: Signal<MoveAxis>,

    /// Callback fired when movement starts.
    pub on_move_start: Option<Callback<MoveStartEvent>>,

    /// Callback fired during movement.
    pub on_move: Option<Callback<MoveEvent>>,

    /// Callback fired when movement ends.
    pub on_move_end: Option<Callback<MoveEndEvent>>,
}

impl Default for UseMoveInput {
    /// Free movement in both axes, no callbacks.
    fn default() -> Self {
        Self {
            is_disabled: Signal::stored(false),
            axis: Signal::stored(MoveAxis::Both),
            on_move_start: None,
            on_move: None,
            on_move_end: None,
        }
    }
}

/// Area-bounded movement of the element within its container ([`use_constrained_move`]).
#[derive(Clone, Copy)]
pub struct MoveConstraintOptions {
    /// Whether the element's center or its bounding box stays within the container.
    pub mode: MoveConstraint,
    /// Whether pressing the container moves the element to that position.
    pub allow_container_click: bool,
    /// Where the element starts, normalized to the container.
    pub initial_position: NormalizedPosition,
    /// Called whenever the normalized position changes.
    pub on_position_change: Option<Callback<NormalizedPosition>>,
}

impl MoveConstraintOptions {
    /// Movement constrained by `mode`, starting at the top left, without container clicks.
    pub fn new(mode: MoveConstraint) -> Self {
        Self {
            mode,
            allow_container_click: false,
            initial_position: NormalizedPosition::default(),
            on_position_change: None,
        }
    }
}

pub struct UseMoveReturn {
    /// Props for the movable element. Call `.into_attrs()` for view spreading.
    pub props: UseMoveProps,

    /// Whether the element is currently being moved.
    pub is_moving: Signal<bool>,
}

/// Props from `use_move` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseMoveProps {
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub element_capture: ElementCaptureAttr,
}

impl IntoAttrs for UseMoveProps {
    type Attrs = UseMoveAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.on_pointerdown.into_on(ev::pointerdown),
            self.on_keydown.into_on(ev::keydown),
            self.element_capture,
        )
    }
}

pub type UseMoveAttrs = (
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    ElementCaptureAttr,
);

/// Return value of [`use_constrained_move`].
pub struct UseConstrainedMoveReturn {
    /// Props for the movable element. Call `.into_attrs()` for view spreading.
    pub props: UseMoveProps,

    /// Whether the element is currently being moved.
    pub is_moving: Signal<bool>,

    /// Props for the container element. Call `.into_attrs()` for view spreading.
    pub container_props: UseMoveContainerProps,

    /// Normalized position (each axis in 0.0 to 1.0).
    pub normalized_position: Signal<NormalizedPosition>,

    /// Pixel position relative to the container.
    pub pixel_position: Signal<Point>,

    /// Programmatically set normalized position.
    pub set_position: Callback<NormalizedPosition>,
}

/// Props for the container element in constrained movement.
#[derive(Debug)]
pub struct UseMoveContainerProps {
    pub element_capture: ElementCaptureAttr,
    pub on_pointerdown: EventHandler<PointerEvent>,
}

impl IntoAttrs for UseMoveContainerProps {
    type Attrs = UseMoveContainerAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.element_capture,
            self.on_pointerdown.into_on(ev::pointerdown),
        )
    }
}

pub type UseMoveContainerAttrs = (
    ElementCaptureAttr,
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
);

struct MoveState {
    pointer_id: i32,
    pointer_type: PointerType,
    moved: bool,
    initial_pos: (f64, f64),
    last_pos: (f64, f64),
    /// The element on which text selection was disabled.
    text_selection_element: Option<web_sys::Element>,
    /// The drag's global listeners (on the document): removed when the state is dropped.
    _global_listeners: Vec<Listener>,
}

/// The pixel step size for keyboard-initiated movement.
const KEYBOARD_STEP_PX: f64 = 1.0;

/// The parts of constrained movement besides the movable element's.
struct ConstraintParts {
    container_props: UseMoveContainerProps,
    normalized_position: Signal<NormalizedPosition>,
    pixel_position: Signal<Point>,
    set_position: Callback<NormalizedPosition>,
}

/// Moves an element by pointer drags and arrow keys, reporting the deltas (react-aria's
/// `useMove`).
pub fn use_move(input: UseMoveInput) -> UseMoveReturn {
    move_with(input, None).0
}

/// Moves an element within a container element ([`MoveConstraintOptions`]): the deltas as
/// [`use_move`], plus its position in the container (normalized and in pixels), container presses
/// and a programmatic setter.
pub fn use_constrained_move(
    input: UseMoveInput,
    options: MoveConstraintOptions,
) -> UseConstrainedMoveReturn {
    let (UseMoveReturn { props, is_moving }, constraint) = move_with(input, Some(options));
    let Some(ConstraintParts {
        container_props,
        normalized_position,
        pixel_position,
        set_position,
    }) = constraint
    else {
        unreachable!("constraint options give the constraint's parts");
    };
    UseConstrainedMoveReturn {
        props,
        is_moving,
        container_props,
        normalized_position,
        pixel_position,
        set_position,
    }
}

/// Movement, constrained to a container with `options` (whose parts it returns then).
#[allow(clippy::too_many_lines, clippy::similar_names)]
fn move_with(
    input: UseMoveInput,
    options: Option<MoveConstraintOptions>,
) -> (UseMoveReturn, Option<ConstraintParts>) {
    #[cfg(feature = "ssr")]
    {
        let (is_moving, _) = signal(false);
        let movable_element = CapturedElement::new();
        let _ = input;
        let constraint_return = options.map(|options| {
            let container_element = CapturedElement::new();
            let (normalized_position, _) = signal(options.initial_position);
            let (pixel_position, _) = signal(Point::default());
            ConstraintParts {
                container_props: UseMoveContainerProps {
                    element_capture: container_element.attr(),
                    on_pointerdown: EventHandler::new(|_: PointerEvent| {}),
                },
                normalized_position: normalized_position.into(),
                pixel_position: pixel_position.into(),
                set_position: Callback::new(|_: NormalizedPosition| {}),
            }
        });

        (
            UseMoveReturn {
                props: UseMoveProps {
                    on_pointerdown: EventHandler::new(|_: PointerEvent| {}),
                    on_keydown: EventHandler::new(|_: KeyboardEvent| {}),
                    element_capture: movable_element.attr(),
                },
                is_moving: is_moving.into(),
            },
            constraint_return,
        )
    }

    #[cfg(not(feature = "ssr"))]
    {
        let UseMoveInput {
            is_disabled: disabled,
            axis,
            on_move_start,
            on_move,
            on_move_end,
        } = input;
        let constraint = options.map(|options| options.mode);
        let allow_container_click = options.is_some_and(|options| options.allow_container_click);
        let initial_position = options.map(|options| options.initial_position);
        let on_position_change = options.and_then(|options| options.on_position_change);

        let constrain_center = matches!(constraint, Some(MoveConstraint::Center));
        // Right-to-left layouts mirror the horizontal axis of constrained positions.
        let direction = use_direction();
        let is_rtl = move || direction.get_untracked() == WritingDirection::Rtl;

        let (is_moving, set_is_moving) = signal(false);
        let movable_element = CapturedElement::new();

        // Note: There may be multiple pointers. Every pointer event contains a unique identifier of the pointer used for the interaction.
        // We start movement tracking by listening for pointerdown events.
        // Only movements from the pointer which initiated the tracking is propagated.

        let state: StoredValue<Option<MoveState>, LocalStorage> = StoredValue::new_local(None);

        // --- Constraint-specific state (only allocated when constraint is configured) ---
        let constraint_state = constraint.map(|_| {
            let initial_pos = initial_position.unwrap_or_default();
            let (normalized_position, set_normalized_position) = signal(initial_pos);
            let (pixel_position, set_pixel_position) = signal(Point::default());
            let container_element = CapturedElement::new();

            ConstraintState {
                constrain_center,
                normalized_position,
                set_normalized_position,
                pixel_position,
                set_pixel_position,
                container_element,
                drag_offset: StoredValue::new_local((0.0, 0.0)),
                last_pixel_pos: StoredValue::new_local((0.0, 0.0)),
            }
        });

        // Helper to fire on_move_start callback
        let fire_move_start = move |pt: PointerType, mods: Modifiers, page_x: f64, page_y: f64| {
            if let Some(cb) = on_move_start {
                cb.run(MoveStartEvent {
                    pointer_type: pt,
                    modifiers: mods,
                    page_x,
                    page_y,
                });
            }
        };

        // Helper to fire on_move callback (returns true if the callback was called)
        let fire_move = move |pt: PointerType, mods: Modifiers, delta_x: f64, delta_y: f64| {
            if let Some(cb) = on_move {
                cb.run(MoveEvent {
                    pointer_type: pt,
                    modifiers: mods,
                    delta_x,
                    delta_y,
                });
            }
        };

        // Helper to fire on_move_end callback
        let fire_move_end = move |pt: PointerType, mods: Modifiers| {
            if let Some(cb) = on_move_end {
                cb.run(MoveEndEvent {
                    pointer_type: pt,
                    modifiers: mods,
                });
            }
        };

        // --- Constraint position calculation ---
        let calculate_constrained_position = constraint_state.map(|cs| {
            let container_element = cs.container_element;
            let movable_el = movable_element;
            let pixel_position = cs.pixel_position;
            move |client_x: f64,
                  client_y: f64,
                  drag_offset: (f64, f64)|
                  -> Option<(NormalizedPosition, f64, f64)> {
                let container_rect = container_element
                    .get_untracked()
                    .map(|e| e.get_bounding_client_rect())?;
                let movable_rect = movable_el
                    .get_untracked()
                    .map(|e| e.get_bounding_client_rect())?;

                let container_left = container_rect.left();
                let container_top = container_rect.top();
                let container_width = container_rect.width();
                let container_height = container_rect.height();
                let movable_width = movable_rect.width();
                let movable_height = movable_rect.height();

                // Available movement range
                let (available_width, available_height) = if constrain_center {
                    (container_width, container_height)
                } else {
                    (
                        (container_width - movable_width).max(0.0),
                        (container_height - movable_height).max(0.0),
                    )
                };

                // Calculate pixel position relative to container, accounting for drag offset.
                // Without constrain_center this is the movable element's top-left position.
                // With constrain_center this is shifted to the element's center so that
                // the center (not the corner) stays within [0, container_size].
                let mut pixel_x = client_x - container_left - drag_offset.0;
                let mut pixel_y = client_y - container_top - drag_offset.1;
                if constrain_center {
                    pixel_x += movable_width / 2.0;
                    pixel_y += movable_height / 2.0;
                }

                // Apply axis constraints
                let current_axis = axis.get_untracked();
                match current_axis {
                    MoveAxis::Horizontal => {
                        pixel_y = pixel_position.get_untracked().y;
                    }
                    MoveAxis::Vertical => {
                        pixel_x = pixel_position.get_untracked().x;
                    }
                    MoveAxis::Both => {}
                }

                // Clamp to available range
                pixel_x = pixel_x.clamp(0.0, available_width);
                pixel_y = pixel_y.clamp(0.0, available_height);

                // Normalize to 0.0-1.0
                let norm_x = if available_width > 0.0 {
                    pixel_x / available_width
                } else {
                    0.0
                };
                let norm_y = if available_height > 0.0 {
                    pixel_y / available_height
                } else {
                    0.0
                };

                // Apply RTL for horizontal
                let norm_x = if is_rtl() { 1.0 - norm_x } else { norm_x };

                Some((
                    NormalizedPosition {
                        x: norm_x,
                        y: norm_y,
                    },
                    pixel_x,
                    pixel_y,
                ))
            }
        });

        // Handle pointer move during drag
        let handle_pointer_move = move |e: PointerEvent| {
            let pointer_id = e.pointer_id();
            let modifiers = e.modifiers();

            state.update_value(move |s| {
                if let Some(s) = s.as_mut() {
                    if s.pointer_id != pointer_id {
                        return;
                    }

                    let first_move = !s.moved;
                    let (old_x, old_y) = s.last_pos;
                    let (new_x, new_y) = (e.page_x(), e.page_y());

                    s.last_pos = (new_x, new_y);

                    // Apply axis filtering.
                    let current_axis = axis.get_untracked();
                    let delta_x = match current_axis {
                        MoveAxis::Vertical => 0.0,
                        _ => new_x - old_x,
                    };
                    let delta_y = match current_axis {
                        MoveAxis::Horizontal => 0.0,
                        _ => new_y - old_y,
                    };

                    // Zero-delta filtering: skip if no actual movement occurred. Only real movement
                    // starts the move (react-aria's `didMove`).
                    if delta_x == 0.0 && delta_y == 0.0 {
                        return;
                    }
                    s.moved = true;

                    let pt = s.pointer_type.clone();

                    if first_move {
                        let (initial_x, initial_y) = s.initial_pos;
                        fire_move_start(pt.clone(), modifiers, initial_x, initial_y);
                        set_is_moving.set(true);
                    }
                    fire_move(pt, modifiers, delta_x, delta_y);

                    // Update constraint signals if constrained
                    if let Some(ref calc) = calculate_constrained_position
                        && let Some(ref cs) = constraint_state
                    {
                        let client_x = e.client_x();
                        let client_y = e.client_y();
                        let drag_offset = cs.drag_offset.get_value();

                        if let Some((norm_pos, pixel_x, pixel_y)) =
                            calc(client_x, client_y, drag_offset)
                        {
                            cs.set_normalized_position.set(norm_pos);
                            cs.set_pixel_position.set(Point::new(pixel_x, pixel_y));
                            cs.last_pixel_pos.set_value((pixel_x, pixel_y));
                            if let Some(cb) = on_position_change {
                                cb.run(norm_pos);
                            }
                        }
                    }
                }
            });
        };

        // Handle pointer up to end the drag
        let handle_pointer_up = move |e: PointerEvent| {
            let pointer_id = e.pointer_id();
            let modifiers = e.modifiers();

            let should_clear = state.with_value(|s| {
                if let Some(s) = s.as_ref()
                    && s.pointer_id == pointer_id
                {
                    if s.moved {
                        fire_move_end(s.pointer_type.clone(), modifiers);
                    }
                    if let Some(ref el) = s.text_selection_element {
                        restore_text_selection(el);
                    }
                    set_is_moving.set(false);
                    return true;
                }
                false
            });

            if should_clear {
                state.set_value(None);
            }
        };

        // Handle pointer cancel to end the drag
        let handle_pointer_cancel = move |e: PointerEvent| {
            let pointer_id = e.pointer_id();
            let modifiers = e.modifiers();

            let should_clear = state.with_value(|s| {
                if let Some(s) = s.as_ref()
                    && s.pointer_id == pointer_id
                {
                    if s.moved {
                        fire_move_end(s.pointer_type.clone(), modifiers);
                    }
                    if let Some(ref el) = s.text_selection_element {
                        restore_text_selection(el);
                    }
                    set_is_moving.set(false);
                    return true;
                }
                false
            });

            if should_clear {
                state.set_value(None);
            }
        };

        // Start a pointer drag (shared between movable and constraint-container pointerdown)
        let start_pointer_drag = move |e: PointerEvent, is_container_click: bool| {
            if disabled.get_untracked() {
                return;
            }

            let pointer_id = e.pointer_id();

            if e.button() == 0 && state.with_value(Option::is_none) {
                e.stop_propagation();
                e.prevent_default();

                let pointer_type = PointerType::from(e.pointer_type());

                // Disable text selection on the target element
                let current_target = e.expect_current_target();
                let current_target_element = current_target.to_element();
                if let Some(ref el) = current_target_element {
                    disable_text_selection(el);
                }

                // Handle constraint-specific drag offset calculation
                if let Some(ref cs) = constraint_state {
                    let client_x = e.client_x();
                    let client_y = e.client_y();

                    let drag_offset = if is_container_click {
                        // When clicking container, center the movable element on the pointer
                        if let Some(rect) = movable_element
                            .get_untracked()
                            .map(|e| e.get_bounding_client_rect())
                        {
                            (rect.width() / 2.0, rect.height() / 2.0)
                        } else {
                            (0.0, 0.0)
                        }
                    } else {
                        // When dragging movable, offset is pointer position relative to movable element
                        if let Some(rect) = movable_element
                            .get_untracked()
                            .map(|e| e.get_bounding_client_rect())
                        {
                            (client_x - rect.left(), client_y - rect.top())
                        } else {
                            (0.0, 0.0)
                        }
                    };

                    cs.drag_offset.set_value(drag_offset);

                    // Update constrained position immediately
                    if let Some(ref calc) = calculate_constrained_position
                        && let Some((norm_pos, pixel_x, pixel_y)) =
                            calc(client_x, client_y, drag_offset)
                    {
                        cs.set_normalized_position.set(norm_pos);
                        cs.set_pixel_position.set(Point::new(pixel_x, pixel_y));
                        cs.last_pixel_pos.set_value((pixel_x, pixel_y));
                        if let Some(cb) = on_position_change {
                            cb.run(norm_pos);
                        }
                    }
                }

                // Get the document to attach global listeners

                // Attach global event listeners for the duration of the drag
                let global_listeners = current_target
                    .get_owner_document()
                    .map(|doc| {
                        vec![
                            listen_to(&doc, ev::pointermove, false, handle_pointer_move),
                            listen_to(&doc, ev::pointerup, false, handle_pointer_up),
                            listen_to(&doc, ev::pointercancel, false, handle_pointer_cancel),
                        ]
                    })
                    .unwrap_or_default();

                let pos = (e.page_x(), e.page_y());
                state.set_value(Some(MoveState {
                    pointer_id,
                    pointer_type,
                    moved: false,
                    initial_pos: pos,
                    last_pos: pos,
                    text_selection_element: current_target_element,
                    _global_listeners: global_listeners,
                }));
            }
        };

        let handle_pointer_down = move |e: PointerEvent| {
            start_pointer_drag(e, false);
        };

        // Keyboard movement handler: arrow keys fire full movestart→move→moveend sequence.
        let handle_keydown = move |e: KeyboardEvent| {
            if disabled.get_untracked() {
                return;
            }

            let key = e.key();
            let (mut dx, mut dy) = match key.as_str() {
                "ArrowLeft" => (-KEYBOARD_STEP_PX, 0.0),
                "ArrowRight" => (KEYBOARD_STEP_PX, 0.0),
                "ArrowUp" => (0.0, -KEYBOARD_STEP_PX),
                "ArrowDown" => (0.0, KEYBOARD_STEP_PX),
                _ => return,
            };

            e.prevent_default();
            e.stop_propagation();

            // Apply axis filtering
            let current_axis = axis.get_untracked();
            match current_axis {
                MoveAxis::Vertical => dx = 0.0,
                MoveAxis::Horizontal => dy = 0.0,
                MoveAxis::Both => {}
            }

            // Zero-delta check after axis filtering
            if dx == 0.0 && dy == 0.0 {
                return;
            }

            let modifiers = e.modifiers();
            let pt = PointerType::Keyboard;

            set_is_moving.set(true);
            fire_move_start(pt.clone(), modifiers, 0.0, 0.0);
            fire_move(pt.clone(), modifiers, dx, dy);

            // Update constraint signals if constrained
            if let Some(ref cs) = constraint_state {
                let (old_px, old_py) = cs.last_pixel_pos.get_value();
                // For keyboard: translate the delta into a new pixel position
                let new_px = old_px + dx;
                let new_py = old_py + dy;

                // Clamp using container/movable rects
                let clamped = (|| {
                    let container_rect = cs
                        .container_element
                        .get_untracked()
                        .map(|e| e.get_bounding_client_rect())?;
                    let movable_rect = movable_element
                        .get_untracked()
                        .map(|e| e.get_bounding_client_rect())?;

                    let (available_width, available_height) = if cs.constrain_center {
                        (container_rect.width(), container_rect.height())
                    } else {
                        (
                            (container_rect.width() - movable_rect.width()).max(0.0),
                            (container_rect.height() - movable_rect.height()).max(0.0),
                        )
                    };

                    let pixel_x = new_px.clamp(0.0, available_width);
                    let pixel_y = new_py.clamp(0.0, available_height);

                    let norm_x = if available_width > 0.0 {
                        pixel_x / available_width
                    } else {
                        0.0
                    };
                    let norm_y = if available_height > 0.0 {
                        pixel_y / available_height
                    } else {
                        0.0
                    };
                    let norm_x = if is_rtl() { 1.0 - norm_x } else { norm_x };

                    Some((
                        NormalizedPosition {
                            x: norm_x,
                            y: norm_y,
                        },
                        pixel_x,
                        pixel_y,
                    ))
                })();

                if let Some((norm_pos, pixel_x, pixel_y)) = clamped {
                    cs.set_normalized_position.set(norm_pos);
                    cs.set_pixel_position.set(Point::new(pixel_x, pixel_y));
                    cs.last_pixel_pos.set_value((pixel_x, pixel_y));
                    if let Some(cb) = on_position_change {
                        cb.run(norm_pos);
                    }
                }
            }

            fire_move_end(pt, modifiers);
            set_is_moving.set(false);
        };

        on_cleanup(move || {
            // Dropping the state removes its global listeners.
            state.try_update_value(|s| {
                if let Some(s) = s.take()
                    && let Some(ref el) = s.text_selection_element
                {
                    restore_text_selection(el);
                }
            });
        });

        // Build constraint return if configured
        let constraint_return = constraint_state.map(|cs| {
            let container_element = cs.container_element;

            // Container click handler
            let handle_container_pointer_down = move |e: PointerEvent| {
                if !allow_container_click {
                    return;
                }
                // Only handle if the click is directly on the container (not on the movable)
                let target = e.expect_target();
                let current_target = e.expect_current_target();
                if target == current_target {
                    start_pointer_drag(e, true);
                }
            };

            // Programmatic position setter
            let set_normalized = cs.set_normalized_position;
            let set_pixel = cs.set_pixel_position;
            let last_pixel = cs.last_pixel_pos;
            // The pixel position of a normalized one, once both elements are mounted.
            let pixel_for = move |norm_x: f64, norm_y: f64| {
                let (container_rect, movable_rect) = container_element
                    .get_untracked()
                    .map(|e| e.get_bounding_client_rect())
                    .zip(
                        movable_element
                            .get_untracked()
                            .map(|e| e.get_bounding_client_rect()),
                    )?;
                let (available_width, available_height) = if constrain_center {
                    (container_rect.width(), container_rect.height())
                } else {
                    (
                        (container_rect.width() - movable_rect.width()).max(0.0),
                        (container_rect.height() - movable_rect.height()).max(0.0),
                    )
                };
                let adjusted_norm_x = if is_rtl() { 1.0 - norm_x } else { norm_x };
                Some((adjusted_norm_x * available_width, norm_y * available_height))
            };

            // The pixel position starts at the initial position as soon as it can be measured.
            let normalized = cs.normalized_position;
            Effect::new(move |done: Option<bool>| {
                if done == Some(true) {
                    return true;
                }
                let _ = (container_element.get(), movable_element.get());
                let pos = normalized.get_untracked();
                let Some((pixel_x, pixel_y)) = pixel_for(pos.x, pos.y) else {
                    return false;
                };
                set_pixel.set(Point::new(pixel_x, pixel_y));
                last_pixel.set_value((pixel_x, pixel_y));
                true
            });

            let set_position_callback = Callback::new(move |pos: NormalizedPosition| {
                let norm_pos = NormalizedPosition {
                    x: pos.x.clamp(0.0, 1.0),
                    y: pos.y.clamp(0.0, 1.0),
                };
                set_normalized.set(norm_pos);
                if let Some((pixel_x, pixel_y)) = pixel_for(norm_pos.x, norm_pos.y) {
                    set_pixel.set(Point::new(pixel_x, pixel_y));
                    last_pixel.set_value((pixel_x, pixel_y));
                }
                if let Some(cb) = on_position_change {
                    cb.run(norm_pos);
                }
            });

            ConstraintParts {
                container_props: UseMoveContainerProps {
                    element_capture: container_element.attr(),
                    on_pointerdown: EventHandler::new(handle_container_pointer_down),
                },
                normalized_position: cs.normalized_position.into(),
                pixel_position: cs.pixel_position.into(),
                set_position: set_position_callback,
            }
        });

        (
            UseMoveReturn {
                props: UseMoveProps {
                    on_pointerdown: EventHandler::new(handle_pointer_down),
                    on_keydown: EventHandler::new(handle_keydown),
                    element_capture: movable_element.attr(),
                },
                is_moving: is_moving.into(),
            },
            constraint_return,
        )
    }
}

/// Internal state for constraint tracking during a `use_move` session.
#[derive(Copy, Clone)]
struct ConstraintState {
    constrain_center: bool,
    normalized_position: ReadSignal<NormalizedPosition>,
    set_normalized_position: WriteSignal<NormalizedPosition>,
    pixel_position: ReadSignal<Point>,
    set_pixel_position: WriteSignal<Point>,
    container_element: CapturedElement,
    drag_offset: StoredValue<(f64, f64), LocalStorage>,
    last_pixel_pos: StoredValue<(f64, f64), LocalStorage>,
}
