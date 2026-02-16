use leptos::{
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use leptos_use::use_event_listener;
use web_sys::{KeyboardEvent, PointerEvent};

use crate::{
    hooks::IntoAttrs,
    utils::{
        element_capture::{CapturedElement, ElementCaptureAttr},
        modifiers::{EventModifiers, Modifiers},
        pointer_type::PointerType,
        text_selection::{disable_text_selection, restore_text_selection},
        EventAccessors, EventHandler, EventTargetExt,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/interactions/src/useMove.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## INTENTIONAL DEVIATIONS
//
// - `axis` constraint: Provided as a `Signal<Option<MoveAxis>>` on `UseMoveInput`
//   so callers can dynamically lock movement to a single axis.
//   React-aria does not have a built-in axis constraint on `useMove`.
//
// - `page_x` / `page_y` on `MoveStartEvent`: Exposes the initial pointer
//   position in page coordinates.
//   React-aria's `MoveStartEvent` does not include position data.
//
// - `MoveConstraint` system: An optional constraint configuration that enables
//   area-bounded movement within a container element, with normalized/pixel
//   position tracking and container-click support.
//   React-aria has no equivalent; this absorbs the leptonic-specific
//   `use_move_within` hook.
//
// ## OMISSIONS
//
// - Mouse/touch fallback event paths: We assume `PointerEvent` is always
//   available (see CLAUDE.md). React-aria has separate mouse and touch
//   code paths for older environments.
//
// =============================================================================

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

/// Configuration for constraining movement within a container element.
///
/// When provided in `UseMoveInput::constraint`, the hook tracks normalized
/// and pixel positions relative to a container, and returns reactive signals
/// for both.
#[derive(Debug, Clone)]
pub struct MoveConstraint {
    /// Whether to use RTL layout (reverses horizontal axis).
    pub is_rtl: bool,

    /// If true, constrain the element center; if false, constrain element bounds.
    pub constrain_center: bool,

    /// If true, clicking the container moves the element to that position.
    pub allow_container_click: bool,

    /// Initial normalized position (x, y), each in 0.0–1.0.
    pub initial_position: Option<(f64, f64)>,
}

pub struct UseMoveInput {
    /// Whether movement is disabled.
    pub disabled: Signal<bool>,

    /// Optional axis constraint for movement. When `None`, movement is unrestricted.
    pub axis: Signal<Option<MoveAxis>>,

    /// Callback fired when movement starts.
    pub on_move_start: Option<Callback<MoveStartEvent>>,

    /// Callback fired during movement.
    pub on_move: Option<Callback<MoveEvent>>,

    /// Callback fired when movement ends.
    pub on_move_end: Option<Callback<MoveEndEvent>>,

    /// Optional constraint configuration for area-bounded movement.
    pub constraint: Option<MoveConstraint>,
}

pub struct UseMoveReturn {
    /// Props for the movable element. Call `.into_attrs()` for view spreading.
    pub props: UseMoveProps,

    /// Whether the element is currently being moved.
    pub is_moving: Signal<bool>,

    /// Constraint-related return values (present when `constraint` was configured).
    pub constraint: Option<UseMoveConstraintReturn>,
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

/// Return values specific to constrained movement.
pub struct UseMoveConstraintReturn {
    /// Props for the container element. Call `.into_attrs()` for view spreading.
    pub container_props: UseMoveContainerProps,

    /// Normalized position (0.0 to 1.0) for each axis.
    pub normalized_position: Signal<(f64, f64)>,

    /// Pixel position relative to the container.
    pub pixel_position: Signal<(f64, f64)>,

    /// Programmatically set position (normalized 0.0–1.0).
    pub set_position: Callback<(f64, f64)>,
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
    pointer_type: PointerType,
    moved: bool,
    initial_pos: (f64, f64),
    last_pos: (f64, f64),
    /// The element on which text selection was disabled.
    text_selection_element: Option<web_sys::Element>,
    event_handlers: MoveEventHandlers,
}

/// The pixel step size for keyboard-initiated movement.
const KEYBOARD_STEP_PX: f64 = 1.0;

/// # Panics
///
/// Panics if the current target of the pointer event is not available.
#[allow(clippy::too_many_lines, clippy::similar_names)]
pub fn use_move(input: UseMoveInput) -> UseMoveReturn {
    let UseMoveInput {
        disabled,
        axis,
        on_move_start,
        on_move,
        on_move_end,
        constraint,
    } = input;

    let (is_moving, set_is_moving) = signal(false);
    let movable_element = CapturedElement::new();

    // Note: There may be multiple pointers. Every pointer event contains a unique identifier of the pointer used for the interaction.
    // We start movement tracking by listening for pointerdown events.
    // Only movements from the pointer which initiated the tracking is propagated.

    let state: StoredValue<Option<MoveState>, LocalStorage> = StoredValue::new_local(None);

    // --- Constraint-specific state (only allocated when constraint is configured) ---
    let constraint_state = constraint.map(|c| {
        let initial_pos = c.initial_position.unwrap_or((0.0, 0.0));
        let config = ConstraintConfig::from(&c);
        let (normalized_position, set_normalized_position) = signal(initial_pos);
        let (pixel_position, set_pixel_position) = signal((0.0, 0.0));
        let container_element = CapturedElement::new();

        ConstraintState {
            config,
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
        let config = cs.config;
        let container_element = cs.container_element;
        let movable_el = movable_element;
        let pixel_position = cs.pixel_position;
        move |client_x: f64, client_y: f64, drag_offset: (f64, f64)| -> Option<(f64, f64, f64, f64)> {
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
            let (available_width, available_height) = if config.constrain_center {
                (container_width, container_height)
            } else {
                (
                    (container_width - movable_width).max(0.0),
                    (container_height - movable_height).max(0.0),
                )
            };

            // Calculate pixel position relative to container, accounting for drag offset
            let mut pixel_x = client_x - container_left - drag_offset.0;
            let mut pixel_y = client_y - container_top - drag_offset.1;

            // Apply axis constraints
            let current_axis = axis.get_untracked();
            match current_axis {
                Some(MoveAxis::Horizontal) => {
                    pixel_y = pixel_position.get_untracked().1;
                }
                Some(MoveAxis::Vertical) => {
                    pixel_x = pixel_position.get_untracked().0;
                }
                _ => {}
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
            let norm_x = if config.is_rtl { 1.0 - norm_x } else { norm_x };

            Some((norm_x, norm_y, pixel_x, pixel_y))
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
                let (new_x, new_y) = (f64::from(e.page_x()), f64::from(e.page_y()));

                s.moved = true;
                s.last_pos = (new_x, new_y);

                // Apply axis filtering.
                let current_axis = axis.get_untracked();
                let delta_x = match current_axis {
                    Some(MoveAxis::Vertical) => 0.0,
                    _ => new_x - old_x,
                };
                let delta_y = match current_axis {
                    Some(MoveAxis::Horizontal) => 0.0,
                    _ => new_y - old_y,
                };

                // Zero-delta filtering: skip if no actual movement occurred.
                if delta_x == 0.0 && delta_y == 0.0 {
                    return;
                }

                let pt = s.pointer_type.clone();

                if first_move {
                    let (initial_x, initial_y) = s.initial_pos;
                    fire_move_start(pt.clone(), modifiers, initial_x, initial_y);
                    set_is_moving.set(true);
                }
                fire_move(pt, modifiers, delta_x, delta_y);

                // Update constraint signals if constrained
                if let Some(ref calc) = calculate_constrained_position {
                    if let Some(ref cs) = constraint_state {
                        let client_x = f64::from(e.client_x());
                        let client_y = f64::from(e.client_y());
                        let drag_offset = cs.drag_offset.get_value();

                        if let Some((norm_x, norm_y, pixel_x, pixel_y)) =
                            calc(client_x, client_y, drag_offset)
                        {
                            cs.set_normalized_position.set((norm_x, norm_y));
                            cs.set_pixel_position.set((pixel_x, pixel_y));
                            cs.last_pixel_pos.set_value((pixel_x, pixel_y));
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
            if let Some(s) = s.as_ref() {
                if s.pointer_id == pointer_id {
                    if s.moved {
                        fire_move_end(s.pointer_type.clone(), modifiers);
                    }
                    if let Some(ref el) = s.text_selection_element {
                        restore_text_selection(el);
                    }
                    s.event_handlers.cleanup();
                    set_is_moving.set(false);
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
    let handle_pointer_cancel = move |e: PointerEvent| {
        let pointer_id = e.pointer_id();
        let modifiers = e.modifiers();

        let should_clear = state.with_value(|s| {
            if let Some(s) = s.as_ref() {
                if s.pointer_id == pointer_id {
                    if s.moved {
                        fire_move_end(s.pointer_type.clone(), modifiers);
                    }
                    if let Some(ref el) = s.text_selection_element {
                        restore_text_selection(el);
                    }
                    s.event_handlers.cleanup();
                    set_is_moving.set(false);
                    return true;
                }
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
                let client_x = f64::from(e.client_x());
                let client_y = f64::from(e.client_y());

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
                if let Some(ref calc) = calculate_constrained_position {
                    if let Some((norm_x, norm_y, pixel_x, pixel_y)) =
                        calc(client_x, client_y, drag_offset)
                    {
                        cs.set_normalized_position.set((norm_x, norm_y));
                        cs.set_pixel_position.set((pixel_x, pixel_y));
                        cs.last_pixel_pos.set_value((pixel_x, pixel_y));
                    }
                }
            }

            // Get the document to attach global listeners
            let doc = current_target.get_owner_document();

            // Attach global event listeners for the duration of the drag
            let event_handlers = MoveEventHandlers {
                global_on_pointer_move_cleanup: Box::new(use_event_listener(
                    doc.clone(),
                    ev::pointermove,
                    handle_pointer_move,
                )),
                global_on_pointer_up_cleanup: Box::new(use_event_listener(
                    doc.clone(),
                    ev::pointerup,
                    handle_pointer_up,
                )),
                global_on_pointer_cancel_cleanup: Box::new(use_event_listener(
                    doc,
                    ev::pointercancel,
                    handle_pointer_cancel,
                )),
            };

            let pos = (f64::from(e.page_x()), f64::from(e.page_y()));
            state.set_value(Some(MoveState {
                pointer_id,
                pointer_type,
                moved: false,
                initial_pos: pos,
                last_pos: pos,
                text_selection_element: current_target_element,
                event_handlers,
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
            Some(MoveAxis::Vertical) => dx = 0.0,
            Some(MoveAxis::Horizontal) => dy = 0.0,
            _ => {}
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
                let container_rect = cs.container_element
                    .get_untracked()
                    .map(|e| e.get_bounding_client_rect())?;
                let movable_rect = movable_element
                    .get_untracked()
                    .map(|e| e.get_bounding_client_rect())?;

                let (available_width, available_height) = if cs.config.constrain_center {
                    (container_rect.width(), container_rect.height())
                } else {
                    (
                        (container_rect.width() - movable_rect.width()).max(0.0),
                        (container_rect.height() - movable_rect.height()).max(0.0),
                    )
                };

                let pixel_x = new_px.clamp(0.0, available_width);
                let pixel_y = new_py.clamp(0.0, available_height);

                let norm_x = if available_width > 0.0 { pixel_x / available_width } else { 0.0 };
                let norm_y = if available_height > 0.0 { pixel_y / available_height } else { 0.0 };
                let norm_x = if cs.config.is_rtl { 1.0 - norm_x } else { norm_x };

                Some((norm_x, norm_y, pixel_x, pixel_y))
            })();

            if let Some((norm_x, norm_y, pixel_x, pixel_y)) = clamped {
                cs.set_normalized_position.set((norm_x, norm_y));
                cs.set_pixel_position.set((pixel_x, pixel_y));
                cs.last_pixel_pos.set_value((pixel_x, pixel_y));
            }
        }

        fire_move_end(pt, modifiers);
        set_is_moving.set(false);
    };

    on_cleanup(move || {
        state.with_value(|s| {
            if let Some(s) = s.as_ref() {
                if let Some(ref el) = s.text_selection_element {
                    restore_text_selection(el);
                }
                s.event_handlers.cleanup();
            }
        });
    });

    // Build constraint return if configured
    let constraint_return = constraint_state.map(|cs| {
        let container_element = cs.container_element;

        // Container click handler
        let handle_container_pointer_down = move |e: PointerEvent| {
            if !cs.config.allow_container_click {
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
        let config = cs.config;
        let set_position_callback = Callback::new(move |(norm_x, norm_y): (f64, f64)| {
            let norm_x = norm_x.clamp(0.0, 1.0);
            let norm_y = norm_y.clamp(0.0, 1.0);

            if let Some((container_rect, movable_rect)) = container_element
                .get_untracked()
                .map(|e| e.get_bounding_client_rect())
                .zip(
                    movable_element
                        .get_untracked()
                        .map(|e| e.get_bounding_client_rect()),
                )
            {
                let container_width = container_rect.width();
                let container_height = container_rect.height();
                let movable_width = movable_rect.width();
                let movable_height = movable_rect.height();

                let (available_width, available_height) = if config.constrain_center {
                    (container_width, container_height)
                } else {
                    (
                        (container_width - movable_width).max(0.0),
                        (container_height - movable_height).max(0.0),
                    )
                };

                let adjusted_norm_x = if config.is_rtl { 1.0 - norm_x } else { norm_x };
                let pixel_x = adjusted_norm_x * available_width;
                let pixel_y = norm_y * available_height;

                set_normalized.set((norm_x, norm_y));
                set_pixel.set((pixel_x, pixel_y));
                last_pixel.set_value((pixel_x, pixel_y));
            } else {
                set_normalized.set((norm_x, norm_y));
            }
        });

        UseMoveConstraintReturn {
            container_props: UseMoveContainerProps {
                element_capture: container_element.attr(),
                on_pointerdown: EventHandler::new(handle_container_pointer_down),
            },
            normalized_position: cs.normalized_position.into(),
            pixel_position: cs.pixel_position.into(),
            set_position: set_position_callback,
        }
    });

    UseMoveReturn {
        props: UseMoveProps {
            on_pointerdown: EventHandler::new(handle_pointer_down),
            on_keydown: EventHandler::new(handle_keydown),
            element_capture: movable_element.attr(),
        },
        is_moving: is_moving.into(),
        constraint: constraint_return,
    }
}

/// Internal state for constraint tracking during a `use_move` session.
#[derive(Copy, Clone)]
struct ConstraintState {
    config: ConstraintConfig,
    normalized_position: ReadSignal<(f64, f64)>,
    set_normalized_position: WriteSignal<(f64, f64)>,
    pixel_position: ReadSignal<(f64, f64)>,
    set_pixel_position: WriteSignal<(f64, f64)>,
    container_element: CapturedElement,
    drag_offset: StoredValue<(f64, f64), LocalStorage>,
    last_pixel_pos: StoredValue<(f64, f64), LocalStorage>,
}

/// Copy-friendly subset of `MoveConstraint` used internally.
#[derive(Debug, Clone, Copy)]
struct ConstraintConfig {
    is_rtl: bool,
    constrain_center: bool,
    allow_container_click: bool,
}

impl From<&MoveConstraint> for ConstraintConfig {
    fn from(c: &MoveConstraint) -> Self {
        Self {
            is_rtl: c.is_rtl,
            constrain_center: c.constrain_center,
            allow_container_click: c.allow_container_click,
        }
    }
}
