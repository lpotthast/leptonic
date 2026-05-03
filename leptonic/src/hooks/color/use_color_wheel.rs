use leptos::{
    attr,
    attr::{
        Attr,
        custom::{CustomAttr, custom_attribute},
    },
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use leptos_use::use_event_listener;
use web_sys::{FocusEvent, KeyboardEvent, PointerEvent};

use crate::{
    hooks::{
        IntoAttrs, MoveEndEvent, MoveEvent, MoveStartEvent, UseFocusRingInput, UseFocusRingReturn,
        UseMoveInput, interactions::use_move::UseMoveAttrs, use_focus_ring, use_move,
    },
    utils::{
        EventHandler,
        color::ColorValue,
        element_capture::{CapturedElement, ElementCaptureAttr},
        focus::focus_element,
        pointer_type::PointerType,
    },
};

use super::use_color_wheel_state::UseColorWheelStateReturn;

// This is based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/color/src/useColorWheel.ts

// ## INTENTIONAL DEVIATIONS
//
// - Simplified pointer handling: Uses direct pointer events on the track
//   instead of react-aria's complex multi-layer pointer/mouse/touch fallback.
//   We assume PointerEvent is always available (see CLAUDE.md).
//
// - Uses `use_move` on the thumb for pointer drag and keyboard arrow support.
//   Track-initiated drags use manual global pointer listeners (matching the
//   pattern in `use_move` internals).
//
// - Coordinate convention: 0° at 12 o'clock (top), increasing clockwise.
//   React-aria uses 0° at 3 o'clock with `conic-gradient(from 90deg, ...)`.
//   Both are internally consistent; ours is arguably more intuitive for users.

/// Input parameters for `use_color_wheel`.
#[derive(Debug, Clone)]
pub struct UseColorWheelInput<C: ColorValue> {
    /// The color wheel state (from `use_color_wheel_state`).
    pub state: UseColorWheelStateReturn<C>,

    /// Outer radius of the wheel in pixels.
    pub outer_radius: f64,

    /// Inner radius of the wheel in pixels (creates annulus/donut shape).
    pub inner_radius: f64,

    /// Whether the wheel is disabled.
    pub disabled: Signal<bool>,

    /// An accessibility label for the wheel.
    pub aria_label: Option<&'static str>,

    /// HTML `name` attribute for the hidden range input (form submission).
    pub name: Option<&'static str>,

    /// HTML `form` attribute for form association.
    pub form: Option<&'static str>,
}

/// Return value of `use_color_wheel`.
pub struct UseColorWheelReturn {
    /// Props for the wheel track/container.
    pub track_props: UseColorWheelTrackProps,

    /// Props for the thumb on the wheel.
    pub thumb_props: UseColorWheelThumbProps,

    /// Props for the visually hidden range input. Render inside the thumb element
    /// with visually-hidden styling (e.g. `opacity: 0.0001; width: 100%; height: 100%;
    /// pointer-events: none; position: absolute;`). This input is the focus target
    /// and carries ARIA slider semantics.
    pub input_props: UseColorWheelInputProps,

    /// CSS background for the track (conic gradient).
    pub background: Signal<String>,

    /// CSS clip-path for the annulus shape.
    pub clip_path: String,

    /// Thumb X position relative to the track element (in pixels).
    pub thumb_x: Signal<f64>,

    /// Thumb Y position relative to the track element (in pixels).
    pub thumb_y: Signal<f64>,

    /// Track width and height (`outer_radius * 2`).
    pub track_size: f64,
}

/// Props for the wheel track element.
///
/// The track element should have `touch-action: none` CSS to prevent
/// browser touch scrolling during drag interaction.
#[derive(Debug)]
pub struct UseColorWheelTrackProps {
    /// Handler for pointer-down events on the track.
    pub on_pointerdown: EventHandler<PointerEvent>,
    /// Element capture attribute for obtaining a reference to the track element.
    pub element_capture: ElementCaptureAttr,
}

impl IntoAttrs for UseColorWheelTrackProps {
    type Attrs = UseColorWheelTrackAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.on_pointerdown.into_on(ev::pointerdown),
            self.element_capture,
        )
    }
}

/// Attribute tuple produced by [`UseColorWheelTrackProps::into_attrs`].
pub type UseColorWheelTrackAttrs = (
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
    ElementCaptureAttr,
);

/// Props for the wheel thumb element.
///
/// The thumb is a visual-only container for pointer drag interaction and
/// focus-ring display. The actual focus target is the hidden range input
/// rendered inside the thumb (see [`UseColorWheelInputProps`]).
///
/// The thumb element should have `touch-action: none` CSS to prevent
/// browser touch scrolling during drag interaction.
#[derive(Debug)]
pub struct UseColorWheelThumbProps {
    /// Pointer-down handler that focuses the hidden input.
    pub on_pointerdown: EventHandler<PointerEvent>,
    /// Keyboard event handler for PageUp/Down/Home/End (catches events bubbling from input).
    pub on_keydown: EventHandler<KeyboardEvent>,
    /// Focus event handler (from `use_focus_ring`).
    pub on_focus: EventHandler<FocusEvent>,
    /// Blur event handler (from `use_focus_ring`).
    pub on_blur: EventHandler<FocusEvent>,
    /// Focus-in event handler (from `use_focus_ring`).
    pub on_focusin: EventHandler<FocusEvent>,
    /// Focus-out event handler (from `use_focus_ring`).
    pub on_focusout: EventHandler<FocusEvent>,
    /// Data attribute for keyboard focus visibility.
    pub data_focus_visible: Signal<Option<&'static str>>,
    /// Move interaction attributes for pointer drag and arrow key support.
    pub move_attrs: UseMoveAttrs,
}

impl IntoAttrs for UseColorWheelThumbProps {
    type Attrs = UseColorWheelThumbAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.on_pointerdown.into_on(ev::pointerdown),
            self.on_keydown.into_on(ev::keydown),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
            custom_attribute("data-focus-visible", self.data_focus_visible),
            self.move_attrs,
        )
    }
}

/// Attribute tuple produced by [`UseColorWheelThumbProps::into_attrs`].
pub type UseColorWheelThumbAttrs = (
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
    CustomAttr<&'static str, Signal<Option<&'static str>>>,
    UseMoveAttrs,
);

/// Props for the visually hidden range input inside the color wheel thumb.
///
/// This input is the focus target and carries ARIA slider semantics.
/// Render it inside the thumb element with visually-hidden styling
/// (e.g. `opacity: 0.0001; width: 100%; height: 100%; pointer-events: none;
/// position: absolute;`).
#[derive(Debug)]
pub struct UseColorWheelInputProps {
    /// Input type (always `"range"`).
    pub r#type: &'static str,
    /// Tab index. 0 when enabled, -1 when disabled.
    pub tabindex: Signal<i32>,
    /// Channel minimum value.
    pub min: f64,
    /// Channel maximum value.
    pub max: f64,
    /// Channel step.
    pub step: f64,
    /// Current channel value.
    pub value: Signal<f64>,
    /// Whether the input is disabled.
    pub disabled: Signal<bool>,
    /// HTML `name` attribute for form submission.
    pub name: Option<&'static str>,
    /// HTML `form` attribute for form association.
    pub form: Option<&'static str>,
    /// Human-readable channel value description (e.g., "120°, Green").
    pub aria_valuetext: Signal<String>,
    /// Accessibility label.
    pub aria_label: Option<&'static str>,
    /// Element capture for the hook to obtain a reference for focusing.
    pub element_capture: ElementCaptureAttr,
}

impl IntoAttrs for UseColorWheelInputProps {
    type Attrs = UseColorWheelInputAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Type, self.r#type),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::Min, self.min),
            Attr(attr::Max, self.max),
            Attr(attr::Step, self.step),
            Attr(attr::Value, self.value),
            Attr(attr::Disabled, self.disabled),
            Attr(attr::Name, self.name),
            Attr(attr::Form, self.form),
            Attr(attr::AriaValuetext, self.aria_valuetext),
            Attr(attr::AriaLabel, self.aria_label),
            self.element_capture,
        )
    }
}

/// Attribute tuple produced by [`UseColorWheelInputProps::into_attrs`].
pub type UseColorWheelInputAttrs = (
    Attr<attr::Type, &'static str>,
    Attr<attr::Tabindex, Signal<i32>>,
    Attr<attr::Min, f64>,
    Attr<attr::Max, f64>,
    Attr<attr::Step, f64>,
    Attr<attr::Value, Signal<f64>>,
    Attr<attr::Disabled, Signal<bool>>,
    Attr<attr::Name, Option<&'static str>>,
    Attr<attr::Form, Option<&'static str>>,
    Attr<attr::AriaValuetext, Signal<String>>,
    Attr<attr::AriaLabel, Option<&'static str>>,
    ElementCaptureAttr,
);

/// Creates behavior and ARIA props for a circular channel wheel.
///
/// The wheel displays a conic gradient of all channel values. Users interact by
/// clicking/dragging on the annulus between the inner and outer radius,
/// or by dragging the thumb, or via keyboard navigation.
#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn use_color_wheel<C: ColorValue>(input: UseColorWheelInput<C>) -> UseColorWheelReturn {
    let UseColorWheelInput {
        state,
        outer_radius,
        inner_radius,
        disabled,
        aria_label,
        name,
        form,
    } = input;

    let channel = state.channel;
    let range = C::get_channel_range(channel);
    let track_size = outer_radius * 2.0;
    let thumb_radius = f64::midpoint(inner_radius, outer_radius);
    let track_element = CapturedElement::new();
    let input_element = CapturedElement::new();

    // Thumb position derived from channel value.
    let thumb_x = Signal::derive(move || {
        let (x, _) = state.get_thumb_position.run(thumb_radius);
        outer_radius + x
    });
    let thumb_y = Signal::derive(move || {
        let (_, y) = state.get_thumb_position.run(thumb_radius);
        outer_radius + y
    });

    // Conic gradient background using channel-specific stops.
    // Follows the pattern from use_color_slider.rs:122-156 adapted for conic gradients.
    let background = Signal::derive(move || {
        let color = state.display_color.get();
        let stops: Vec<String> = if let Some(fixed_stops) = range.gradient_stops {
            fixed_stops
                .iter()
                .map(|&val| {
                    let c = color.with_channel_value(channel, val);
                    let frac = (val - range.min_value) / (range.max_value - range.min_value);
                    format!("{} {}deg", c.to_css_string(), frac * 360.0)
                })
                .collect()
        } else {
            (0..=12)
                .map(|i| {
                    let frac = f64::from(i) / 12.0;
                    let val = range.min_value + frac * (range.max_value - range.min_value);
                    let c = color.with_channel_value(channel, val);
                    format!("{} {}deg", c.to_css_string(), frac * 360.0)
                })
                .collect()
        };
        format!("conic-gradient(from 0deg, {})", stops.join(", "))
    });

    // SVG clip-path for the annulus (donut) shape.
    let clip_path = format!(
        "path(evenodd, 'M {o} 0 A {o} {o} 0 1 1 {o} {d} A {o} {o} 0 1 1 {o} 0 Z \
         M {o} {oi} A {i} {i} 0 1 0 {o} {di} A {i} {i} 0 1 0 {o} {oi} Z')",
        o = outer_radius,
        d = track_size,
        i = inner_radius,
        oi = outer_radius - inner_radius,
        di = outer_radius + inner_radius,
    );

    let thumb_move_return = setup_thumb_move(&state, disabled, thumb_radius, input_element);

    let handle_track_pointerdown = setup_track_interaction(
        &state,
        disabled,
        track_element,
        input_element,
        inner_radius,
        outer_radius,
    );

    let handle_keydown = setup_keyboard_handler(&state, disabled);

    // ARIA attributes for the hidden range input.
    let aria_valuetext = Signal::derive(move || {
        let color = state.value.get();
        let mut text = color.format_channel_value(channel);
        if let Some(hue_name) = color.get_hue_name_for_channel(channel) {
            text.push_str(", ");
            text.push_str(hue_name);
        }
        text
    });

    // Reactive tabindex: -1 when disabled, 0 when enabled.
    let tabindex = Signal::derive(move || if disabled.get() { -1 } else { 0 });

    // Focus the hidden input on thumb pointer-down. use_move calls
    // prevent_default() on pointerdown which suppresses the browser's
    // default focus behavior.
    let handle_thumb_pointerdown = EventHandler::new(move |_: PointerEvent| {
        if let Some(el) = input_element.get_untracked() {
            focus_element(&el, true);
        }
    });

    // Focus ring for keyboard focus visibility on the thumb.
    // Uses within=true so focus on the hidden input child triggers the ring
    // on the parent thumb element.
    let UseFocusRingReturn {
        props: focus_ring_props,
        is_focus_visible: _,
        is_focused: _,
    } = use_focus_ring(UseFocusRingInput {
        disabled,
        within: true,
        auto_focus: false,
        is_text_input: false,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });

    UseColorWheelReturn {
        track_props: UseColorWheelTrackProps {
            on_pointerdown: handle_track_pointerdown,
            element_capture: track_element.attr(),
        },
        thumb_props: UseColorWheelThumbProps {
            on_pointerdown: handle_thumb_pointerdown,
            on_keydown: handle_keydown,
            on_focus: focus_ring_props.on_focus,
            on_blur: focus_ring_props.on_blur,
            on_focusin: focus_ring_props.on_focusin,
            on_focusout: focus_ring_props.on_focusout,
            data_focus_visible: focus_ring_props.data_focus_visible,
            move_attrs: thumb_move_return.props.into_attrs(),
        },
        input_props: UseColorWheelInputProps {
            r#type: "range",
            tabindex,
            min: range.min_value,
            max: range.max_value,
            step: state.step,
            value: state.hue,
            disabled,
            name,
            form,
            aria_valuetext,
            aria_label,
            element_capture: input_element.attr(),
        },
        background,
        clip_path,
        thumb_x,
        thumb_y,
        track_size,
    }
}

/// Sets up `use_move` on the thumb for pointer drag and keyboard arrow support.
fn setup_thumb_move<C: ColorValue>(
    state: &UseColorWheelStateReturn<C>,
    disabled: Signal<bool>,
    thumb_radius: f64,
    input_el: CapturedElement,
) -> crate::hooks::interactions::use_move::UseMoveReturn {
    // Accumulated position during drag, in Cartesian coordinates relative to
    // the wheel center. Initialized from the current thumb position on drag
    // start, then updated with deltas on each move event.
    let current_position: StoredValue<Option<(f64, f64)>> = StoredValue::new(None);

    let set_hue_from_point = state.set_hue_from_point;
    let set_dragging_start = state.set_dragging;
    let set_dragging_end = state.set_dragging;
    let increment = state.increment;
    let decrement = state.decrement;
    let get_thumb_pos = state.get_thumb_position;
    let step = state.step;
    let page_step = state.page_step;

    use_move(UseMoveInput {
        disabled,
        axis: Signal::derive(|| None), // unrestricted — circular movement
        is_rtl: false,
        on_move_start: Some(Callback::new(move |_: MoveStartEvent| {
            current_position.set_value(None);
            set_dragging_start.run(true);
        })),
        on_move: Some(Callback::new(move |e: MoveEvent| {
            if e.pointer_type == PointerType::Keyboard {
                // Keyboard: translate deltas to channel increment/decrement.
                // Shift modifier uses page step (matching react-aria).
                let step_size = if e.modifiers.shift_key {
                    page_step
                } else {
                    step
                };
                if e.delta_x > 0.0 || e.delta_y < 0.0 {
                    increment.run(Some(step_size));
                } else if e.delta_x < 0.0 || e.delta_y > 0.0 {
                    decrement.run(Some(step_size));
                }
            } else {
                // Pointer: accumulate deltas and convert to channel value via angle.
                let pos = current_position
                    .get_value()
                    .unwrap_or_else(|| get_thumb_pos.run(thumb_radius));
                let new_x = pos.0 + e.delta_x;
                let new_y = pos.1 + e.delta_y;
                current_position.set_value(Some((new_x, new_y)));
                let dist = (new_x * new_x + new_y * new_y).sqrt();
                set_hue_from_point.run((new_x, new_y, dist));
            }
        })),
        on_move_end: Some(Callback::new(move |_: MoveEndEvent| {
            current_position.set_value(None);
            set_dragging_end.run(false);
            // Re-focus the hidden input after drag ends (matches react-aria's onMoveEnd).
            if let Some(el) = input_el.get_untracked() {
                focus_element(&el, true);
            }
        })),
        on_position_change: None,
        constraint: None,
        allow_container_click: false,
        initial_position: None,
    })
}

/// Sets up the track pointerdown handler with global pointer listeners for
/// continuous drag tracking after clicking within the annulus.
fn setup_track_interaction<C: ColorValue>(
    state: &UseColorWheelStateReturn<C>,
    disabled: Signal<bool>,
    track_el: CapturedElement,
    input_el: CapturedElement,
    inner_radius: f64,
    outer_radius: f64,
) -> EventHandler<PointerEvent> {
    let set_hue_from_point = state.set_hue_from_point;
    let set_dragging = state.set_dragging;
    let set_dragging_up = state.set_dragging;
    let set_hue_from_point_move = state.set_hue_from_point;

    // Store cleanup functions for global listeners registered during track drag.
    // Uses `update_value` for access since `Box<dyn Fn()>` is not Clone.
    let track_drag_cleanup: StoredValue<
        Option<(Box<dyn Fn() + Send + Sync>, Box<dyn Fn() + Send + Sync>)>,
    > = StoredValue::new(None);

    EventHandler::new(move |e: PointerEvent| {
        if disabled.get_untracked() || e.button() != 0 {
            return;
        }

        // Filter out modifier keys for mouse interactions.
        if e.pointer_type() == "mouse" && (e.alt_key() || e.ctrl_key() || e.meta_key()) {
            return;
        }

        let Some(el) = track_el.get_untracked() else {
            return;
        };
        let rect = el.get_bounding_client_rect();
        let center_x = rect.left() + rect.width() / 2.0;
        let center_y = rect.top() + rect.height() / 2.0;
        let x = e.client_x() - center_x;
        let y = e.client_y() - center_y;
        let dist = (x * x + y * y).sqrt();

        // Only handle clicks within the annulus.
        if dist < inner_radius || dist > outer_radius {
            return;
        }

        e.prevent_default();
        set_hue_from_point.run((x, y, dist));
        set_dragging.run(true);

        // Focus the hidden input so keyboard navigation works after a track click.
        if let Some(input) = input_el.get_untracked() {
            focus_element(&input, true);
        }

        let pointer_id = e.pointer_id();
        let track_el_move = track_el;

        let cleanup_move = use_event_listener(
            leptos_use::use_document(),
            ev::pointermove,
            move |e: PointerEvent| {
                if e.pointer_id() != pointer_id {
                    return;
                }
                let Some(el) = track_el_move.get_untracked() else {
                    return;
                };
                let rect = el.get_bounding_client_rect();
                let cx = rect.left() + rect.width() / 2.0;
                let cy = rect.top() + rect.height() / 2.0;
                let mx = e.client_x() - cx;
                let my = e.client_y() - cy;
                let d = (mx * mx + my * my).sqrt();
                set_hue_from_point_move.run((mx, my, d));
            },
        );

        let cleanup_up = use_event_listener(
            leptos_use::use_document(),
            ev::pointerup,
            move |e: PointerEvent| {
                if e.pointer_id() != pointer_id {
                    return;
                }
                set_dragging_up.run(false);
                // Clean up both global listeners.
                track_drag_cleanup.update_value(|cleanup| {
                    if let Some((cm, cu)) = cleanup.take() {
                        cm();
                        cu();
                    }
                });
            },
        );

        track_drag_cleanup.set_value(Some((Box::new(cleanup_move), Box::new(cleanup_up))));
    })
}

/// Sets up the keyboard handler for PageUp/Down/Home/End.
/// Arrow keys are handled by `use_move`'s keyboard support.
fn setup_keyboard_handler<C: ColorValue>(
    state: &UseColorWheelStateReturn<C>,
    disabled: Signal<bool>,
) -> EventHandler<KeyboardEvent> {
    let increment = state.increment;
    let decrement = state.decrement;
    let set_hue = state.set_hue;
    let set_dragging = state.set_dragging;
    let page_step = state.page_step;
    let channel = state.channel;
    let range = C::get_channel_range(channel);

    EventHandler::new(move |e: KeyboardEvent| {
        if disabled.get_untracked() {
            return;
        }
        let key = e.key();
        match key.as_str() {
            "PageUp" => {
                e.prevent_default();
                set_dragging.run(true);
                increment.run(Some(page_step));
                set_dragging.run(false);
            }
            "PageDown" => {
                e.prevent_default();
                set_dragging.run(true);
                decrement.run(Some(page_step));
                set_dragging.run(false);
            }
            "Home" => {
                e.prevent_default();
                set_dragging.run(true);
                set_hue.run(range.min_value);
                set_dragging.run(false);
            }
            "End" => {
                e.prevent_default();
                set_dragging.run(true);
                set_hue.run(range.max_value - range.step);
                set_dragging.run(false);
            }
            _ => {}
        }
    })
}
