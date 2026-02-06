//! Hook for constraining element movement within a container.
//!
//! This hook provides the ability to drag an element within the bounds of a container,
//! with support for axis constraints, RTL layouts, and different constraint modes.

use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use leptos_use::use_event_listener;
use send_wrapper::SendWrapper;
use web_sys::PointerEvent;

use super::use_move::MoveAxis;
use crate::utils::element_capture::{element_capture, ElementCaptureAttr};
use crate::utils::{EventHandler, EventTargetExt};

/// Event fired when movement starts.
#[derive(Debug, Clone, Copy)]
pub struct MoveWithinStartEvent {
    /// Normalized X position (0.0 to 1.0).
    pub normalized_x: f64,
    /// Normalized Y position (0.0 to 1.0).
    pub normalized_y: f64,
    /// Pixel X position relative to container.
    pub pixel_x: f64,
    /// Pixel Y position relative to container.
    pub pixel_y: f64,
}

/// Event fired during movement.
#[derive(Debug, Clone, Copy)]
pub struct MoveWithinEvent {
    /// Normalized X position (0.0 to 1.0).
    pub normalized_x: f64,
    /// Normalized Y position (0.0 to 1.0).
    pub normalized_y: f64,
    /// Pixel X position relative to container.
    pub pixel_x: f64,
    /// Pixel Y position relative to container.
    pub pixel_y: f64,
    /// Pixel delta in X since last event.
    pub delta_x: f64,
    /// Pixel delta in Y since last event.
    pub delta_y: f64,
}

/// Event fired when movement ends.
#[derive(Debug, Clone, Copy)]
pub struct MoveWithinEndEvent {
    /// Final normalized X position (0.0 to 1.0).
    pub normalized_x: f64,
    /// Final normalized Y position (0.0 to 1.0).
    pub normalized_y: f64,
    /// Final pixel X position relative to container.
    pub pixel_x: f64,
    /// Final pixel Y position relative to container.
    pub pixel_y: f64,
}

/// Input configuration for `use_move_within`.
#[derive(Clone)]
pub struct UseMoveWithinInput {
    /// Movement axis constraint.
    pub axis: MoveAxis,

    /// Whether movement is disabled.
    pub disabled: Signal<bool>,

    /// Initial normalized position (x, y). Defaults to (0.0, 0.0).
    pub initial_position: Option<(f64, f64)>,

    /// RTL layout support (reverses horizontal axis).
    pub is_rtl: bool,

    /// If true, constrain element center; if false, constrain element bounds.
    pub constrain_center: bool,

    /// If true, clicking container moves element to that position.
    pub allow_container_click: bool,

    /// Callback fired when movement starts.
    pub on_move_start: Option<Callback<MoveWithinStartEvent>>,

    /// Callback fired during movement.
    pub on_move: Option<Callback<MoveWithinEvent>>,

    /// Callback fired when movement ends.
    pub on_move_end: Option<Callback<MoveWithinEndEvent>>,
}

impl Default for UseMoveWithinInput {
    fn default() -> Self {
        Self {
            axis: MoveAxis::default(),
            disabled: false.into(),
            initial_position: None,
            is_rtl: false,
            constrain_center: false,
            allow_container_click: false,
            on_move_start: None,
            on_move: None,
            on_move_end: None,
        }
    }
}

/// Return value from `use_move_within`.
pub struct UseMoveWithinReturn {
    /// Props for the container element. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub container_props: UseMoveWithinContainerProps,

    /// Props for the movable element. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub movable_props: UseMoveWithinMovableProps,

    /// Normalized position (0.0 to 1.0).
    pub normalized_position: Signal<(f64, f64)>,

    /// Pixel position relative to container.
    pub pixel_position: Signal<(f64, f64)>,

    /// Whether currently moving.
    pub is_moving: Signal<bool>,

    /// Programmatically set position (normalized).
    pub set_position: Callback<(f64, f64)>,
}

/// Props for the container element in `use_move_within`.
#[derive(Debug, Clone)]
pub struct UseMoveWithinContainerProps {
    pub element_capture: ElementCaptureAttr,
    pub on_pointerdown: EventHandler<PointerEvent>,
}

impl UseMoveWithinContainerProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseMoveWithinContainerAttrs {
        (
            self.element_capture.clone(),
            self.on_pointerdown.to_on(ev::pointerdown),
        )
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseMoveWithinContainerAttrs {
        (
            self.element_capture,
            self.on_pointerdown.into_on(ev::pointerdown),
        )
    }
}

/// Props for the movable element in `use_move_within`.
#[derive(Debug, Clone)]
pub struct UseMoveWithinMovableProps {
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub element_capture: ElementCaptureAttr,
}

impl UseMoveWithinMovableProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseMoveWithinMovableAttrs {
        (
            self.on_pointerdown.to_on(ev::pointerdown),
            self.element_capture.clone(),
        )
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseMoveWithinMovableAttrs {
        (
            self.on_pointerdown.into_on(ev::pointerdown),
            self.element_capture,
        )
    }
}

/// Attributes for the container element.
pub type UseMoveWithinContainerAttrs = (
    ElementCaptureAttr,
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
);

/// Attributes for the movable element.
pub type UseMoveWithinMovableAttrs = (
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
    ElementCaptureAttr,
);

/// Event handlers stored during drag operation.
#[allow(clippy::struct_field_names)]
struct MoveWithinEventHandlers {
    global_on_pointer_move_cleanup: Box<dyn Fn() + Send + Sync + 'static>,
    global_on_pointer_up_cleanup: Box<dyn Fn() + Send + Sync + 'static>,
    global_on_pointer_cancel_cleanup: Box<dyn Fn() + Send + Sync + 'static>,
}

impl MoveWithinEventHandlers {
    fn cleanup(&self) {
        (self.global_on_pointer_move_cleanup)();
        (self.global_on_pointer_up_cleanup)();
        (self.global_on_pointer_cancel_cleanup)();
    }
}

/// Internal state during drag operation.
struct MoveWithinState {
    /// Pointer ID being tracked.
    pointer_id: i32,
    /// Last page position of pointer.
    last_client_pos: (f64, f64),
    /// Last computed pixel position.
    last_pixel_pos: (f64, f64),
    /// Offset from pointer to element origin when drag started.
    drag_offset: (f64, f64),
    /// Event handlers for cleanup.
    event_handlers: MoveWithinEventHandlers,
}

/// Create a hook for constrained element movement within a container.
///
/// Returns attributes to spread on both the container and the movable element,
/// along with reactive signals for the current position.
///
/// # Example
///
/// ```ignore
/// let UseMoveWithinReturn {
///     container_attrs,
///     movable_attrs,
///     pixel_position,
///     ..
/// } = use_move_within(UseMoveWithinInput {
///     axis: MoveAxis::Both,
///     disabled: false.into(),
///     allow_container_click: false,
///     ..Default::default()
/// });
///
/// view! {
///     <div {..container_attrs} style="width: 300px; height: 200px; position: relative;">
///         <div
///             {..movable_attrs}
///             style=move || format!(
///                 "position: absolute; left: {}px; top: {}px;",
///                 pixel_position.get().0,
///                 pixel_position.get().1
///             )
///         >
///             "Drag me"
///         </div>
///     </div>
/// }
/// ```
///
/// # Panics
///
/// Panics if the current target of the pointer event is not available.
#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn use_move_within(input: UseMoveWithinInput) -> UseMoveWithinReturn {
    let initial_pos = input.initial_position.unwrap_or((0.0, 0.0));

    // Reactive state
    let (normalized_position, set_normalized_position) = signal(initial_pos);
    let (pixel_position, set_pixel_position) = signal((0.0, 0.0));
    let (is_moving, set_is_moving) = signal(false);

    // Element storage via ElementCaptureAttr
    let container_element: StoredValue<Option<SendWrapper<web_sys::Element>>, LocalStorage> =
        StoredValue::new_local(None);
    let movable_element: StoredValue<Option<SendWrapper<web_sys::Element>>, LocalStorage> =
        StoredValue::new_local(None);

    // Drag state
    let state: StoredValue<Option<MoveWithinState>, LocalStorage> = StoredValue::new_local(None);

    // Extract input values for closures
    let axis = input.axis;
    let is_rtl = input.is_rtl;
    let constrain_center = input.constrain_center;
    let disabled = input.disabled;
    let on_move_start = input.on_move_start;
    let on_move = input.on_move;
    let on_move_end = input.on_move_end;

    // Helper to calculate position from pointer event
    let calculate_position =
        move |page_x: f64, page_y: f64, drag_offset: (f64, f64)| -> Option<(f64, f64, f64, f64)> {
            let container_rect = container_element
                .with_value(|el| el.as_ref().map(|e| e.get_bounding_client_rect()))?;
            let movable_rect = movable_element
                .with_value(|el| el.as_ref().map(|e| e.get_bounding_client_rect()))?;

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

            // Calculate pixel position relative to container, accounting for drag offset
            let mut pixel_x = page_x - container_left - drag_offset.0;
            let mut pixel_y = page_y - container_top - drag_offset.1;

            // Apply axis constraints
            match axis {
                MoveAxis::Horizontal => {
                    pixel_y = pixel_position.get_untracked().1;
                }
                MoveAxis::Vertical => {
                    pixel_x = pixel_position.get_untracked().0;
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
            let norm_x = if is_rtl { 1.0 - norm_x } else { norm_x };

            Some((norm_x, norm_y, pixel_x, pixel_y))
        };

    // Handler for pointer move during drag
    let on_pointer_move = move |e: PointerEvent| {
        let pointer_id = e.pointer_id();

        state.update_value(move |s| {
            if let Some(s) = s.as_mut() {
                if s.pointer_id != pointer_id {
                    return;
                }

                // Use client coordinates to match getBoundingClientRect()
                let client_x = f64::from(e.client_x());
                let client_y = f64::from(e.client_y());

                if let Some((norm_x, norm_y, pixel_x, pixel_y)) =
                    calculate_position(client_x, client_y, s.drag_offset)
                {
                    let delta_x = pixel_x - s.last_pixel_pos.0;
                    let delta_y = pixel_y - s.last_pixel_pos.1;

                    s.last_client_pos = (client_x, client_y);
                    s.last_pixel_pos = (pixel_x, pixel_y);

                    set_normalized_position.set((norm_x, norm_y));
                    set_pixel_position.set((pixel_x, pixel_y));

                    if let Some(callback) = on_move {
                        callback.run(MoveWithinEvent {
                            normalized_x: norm_x,
                            normalized_y: norm_y,
                            pixel_x,
                            pixel_y,
                            delta_x,
                            delta_y,
                        });
                    }
                }
            }
        });
    };

    // Handler for pointer up to end drag
    let on_pointer_up = move |e: PointerEvent| {
        let pointer_id = e.pointer_id();

        let should_clear = state.with_value(|s| {
            if let Some(s) = s.as_ref() {
                if s.pointer_id == pointer_id {
                    let (norm_x, norm_y) = normalized_position.get_untracked();
                    let (pixel_x, pixel_y) = pixel_position.get_untracked();

                    if let Some(callback) = on_move_end {
                        callback.run(MoveWithinEndEvent {
                            normalized_x: norm_x,
                            normalized_y: norm_y,
                            pixel_x,
                            pixel_y,
                        });
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

    // Handler for pointer cancel
    let on_pointer_cancel = move |e: PointerEvent| {
        let pointer_id = e.pointer_id();

        let should_clear = state.with_value(|s| {
            if let Some(s) = s.as_ref() {
                if s.pointer_id == pointer_id {
                    let (norm_x, norm_y) = normalized_position.get_untracked();
                    let (pixel_x, pixel_y) = pixel_position.get_untracked();

                    if let Some(callback) = on_move_end {
                        callback.run(MoveWithinEndEvent {
                            normalized_x: norm_x,
                            normalized_y: norm_y,
                            pixel_x,
                            pixel_y,
                        });
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

    // Start drag from a specific element
    let start_drag = move |e: PointerEvent, is_container_click: bool| {
        if disabled.get_untracked() {
            return;
        }

        let pointer_id = e.pointer_id();

        if e.button() == 0 && state.with_value(Option::is_none) {
            e.stop_propagation();
            e.prevent_default();

            // Use client coordinates (viewport-relative) to match getBoundingClientRect()
            let client_x = f64::from(e.client_x());
            let client_y = f64::from(e.client_y());

            // Calculate drag offset (difference between pointer and element origin)
            let drag_offset = if is_container_click {
                // When clicking container, center the movable element on the pointer
                let movable_rect = movable_element
                    .with_value(|el| el.as_ref().map(|e| e.get_bounding_client_rect()));
                if let Some(rect) = movable_rect {
                    (rect.width() / 2.0, rect.height() / 2.0)
                } else {
                    (0.0, 0.0)
                }
            } else {
                // When dragging movable, offset is pointer position relative to movable element
                let movable_rect = movable_element
                    .with_value(|el| el.as_ref().map(|e| e.get_bounding_client_rect()));
                if let Some(rect) = movable_rect {
                    (client_x - rect.left(), client_y - rect.top())
                } else {
                    (0.0, 0.0)
                }
            };

            // Calculate initial position
            let initial_pixel_pos = if let Some((_, _, pixel_x, pixel_y)) =
                calculate_position(client_x, client_y, drag_offset)
            {
                (pixel_x, pixel_y)
            } else {
                pixel_position.get_untracked()
            };

            // Fire move start callback with initial position
            if let Some((norm_x, norm_y, pixel_x, pixel_y)) =
                calculate_position(client_x, client_y, drag_offset)
            {
                set_normalized_position.set((norm_x, norm_y));
                set_pixel_position.set((pixel_x, pixel_y));

                if let Some(callback) = on_move_start {
                    callback.run(MoveWithinStartEvent {
                        normalized_x: norm_x,
                        normalized_y: norm_y,
                        pixel_x,
                        pixel_y,
                    });
                }
            }

            set_is_moving.set(true);

            // Get the document to attach global listeners
            let doc = e.current_target().unwrap().get_owner_document();

            // Attach global event listeners
            let event_handlers = MoveWithinEventHandlers {
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

            state.set_value(Some(MoveWithinState {
                pointer_id,
                last_client_pos: (client_x, client_y),
                last_pixel_pos: initial_pixel_pos,
                drag_offset,
                event_handlers,
            }));
        }
    };

    // Handler for movable element pointer down
    let on_movable_pointer_down = move |e: PointerEvent| {
        start_drag(e, false);
    };

    // Handler for container pointer down (when allow_container_click is true)
    let allow_container_click = input.allow_container_click;
    let on_container_pointer_down = move |e: PointerEvent| {
        if !allow_container_click {
            return;
        }
        // Only handle if the click is directly on the container (not on the movable)
        let target = e.target();
        let current_target = e.current_target();

        if target == current_target {
            start_drag(e, true);
        }
    };

    // Programmatic position setter
    let set_position_callback = Callback::new(move |(norm_x, norm_y): (f64, f64)| {
        let norm_x = norm_x.clamp(0.0, 1.0);
        let norm_y = norm_y.clamp(0.0, 1.0);

        // Calculate pixel position from normalized
        if let Some((container_rect, movable_rect)) = container_element
            .with_value(|c| c.as_ref().map(|e| e.get_bounding_client_rect()))
            .zip(movable_element.with_value(|m| m.as_ref().map(|e| e.get_bounding_client_rect())))
        {
            let container_width = container_rect.width();
            let container_height = container_rect.height();
            let movable_width = movable_rect.width();
            let movable_height = movable_rect.height();

            let (available_width, available_height) = if constrain_center {
                (container_width, container_height)
            } else {
                (
                    (container_width - movable_width).max(0.0),
                    (container_height - movable_height).max(0.0),
                )
            };

            // Apply RTL for horizontal
            let adjusted_norm_x = if is_rtl { 1.0 - norm_x } else { norm_x };

            let pixel_x = adjusted_norm_x * available_width;
            let pixel_y = norm_y * available_height;

            set_normalized_position.set((norm_x, norm_y));
            set_pixel_position.set((pixel_x, pixel_y));
        } else {
            // Elements not yet captured, just set normalized
            set_normalized_position.set((norm_x, norm_y));
        }
    });

    // Cleanup on unmount
    on_cleanup(move || {
        state.with_value(|s| {
            if let Some(s) = s.as_ref() {
                s.event_handlers.cleanup();
            }
        });
    });

    // Build container props
    let container_props = UseMoveWithinContainerProps {
        element_capture: element_capture(move |el| {
            container_element.set_value(Some(SendWrapper::new(el)));
        }),
        on_pointerdown: EventHandler::new(on_container_pointer_down),
    };

    // Build movable props
    let movable_props = UseMoveWithinMovableProps {
        on_pointerdown: EventHandler::new(on_movable_pointer_down),
        element_capture: element_capture(move |el| {
            movable_element.set_value(Some(SendWrapper::new(el)));
        }),
    };

    UseMoveWithinReturn {
        container_props,
        movable_props,
        normalized_position: normalized_position.into(),
        pixel_position: pixel_position.into(),
        is_moving: is_moving.into(),
        set_position: set_position_callback,
    }
}
