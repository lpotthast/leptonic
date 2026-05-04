use std::borrow::Cow;

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
use uuid::Uuid;
use web_sys::{FocusEvent, KeyboardEvent, PointerEvent};

use crate::{
    hooks::{
        IntoAttrs, UseFocusRingInput, UseFocusRingReturn, UseMoveProps, UseMoveReturn,
        ValidationState,
        interactions::{
            use_hover::{UseHoverInput, UseHoverReturn, use_hover},
            use_move::{MoveAxis, MoveEndEvent, MoveEvent, MoveStartEvent, UseMoveInput, use_move},
        },
        slider::{SliderOrientation, use_slider_state::UseSliderStateReturn},
        use_focus_ring,
    },
    utils::{
        ElementCaptureAttr, EventHandler,
        aria::{AriaDisabled, AriaInvalid, AriaOrientation, AriaRequired},
        element_capture::CapturedElement,
        focus::focus_element,
        math::percentage_in_range,
    },
};

//
// No intentional deviations from the react-aria implementation.
//

/// Input parameters for the `use_slider_thumb` hook.
#[derive(Debug, Clone)]
pub struct UseSliderThumbInput {
    /// The slider state (from `use_slider_state`).
    pub state: UseSliderStateReturn,

    /// Reactive handle to the track element (from `use_slider`'s return).
    pub track: CapturedElement,

    /// The index of this thumb in the slider.
    pub index: usize,

    /// The name attribute for form submission.
    pub name: Option<&'static str>,

    /// An accessibility label for this thumb.
    pub aria_label: Option<Cow<'static, str>>,

    /// ID of an element that labels this thumb.
    pub aria_labelledby: Option<&'static str>,

    /// Whether this thumb is disabled.
    pub disabled: Signal<bool>,

    /// The validation state.
    pub validation_state: ValidationState,

    /// Whether to use RTL layout (reverses arrow key direction).
    pub is_rtl: bool,

    /// Number of decimal places for display.
    pub decimal_places: Option<usize>,

    /// Whether the slider value is required for form submission.
    pub is_required: bool,

    /// ID of an element that describes this thumb.
    pub aria_describedby: Option<&'static str>,

    /// ID of an element that provides additional details about this thumb.
    pub aria_details: Option<&'static str>,

    /// ID of an element that contains an error message for this thumb.
    pub aria_errormessage: Option<&'static str>,

    /// Optional override for the ARIA valuetext. When `Some`, this is used instead
    /// of the auto-generated display value formatted from the numeric thumb value.
    pub aria_valuetext: Option<Signal<String>>,
}

/// The return value of the `use_slider_thumb` hook.
#[derive(Debug)]
pub struct UseSliderThumbReturn {
    /// Props for the thumb element.
    pub thumb_props: UseSliderThumbProps,

    /// Props for the visually hidden range input. Render inside the thumb element
    /// with visually-hidden styling (e.g. `opacity: 0.0001; width: 100%; height: 100%;
    /// pointer-events: none; position: absolute; top: 0; left: 0;`).
    /// This input is the focus target and carries ARIA slider semantics.
    pub input_props: UseSliderThumbInputProps,

    /// Whether this thumb is being dragged.
    pub is_dragging: Signal<bool>,

    /// Whether this thumb is hovered.
    pub is_hovered: Signal<bool>,

    /// Whether this thumb is focused.
    pub is_focused: Signal<bool>,

    /// Whether the focus ring should be visible.
    pub is_focus_visible: Signal<bool>,

    /// The current value of this thumb.
    pub value: Signal<f64>,

    /// The percentage of this thumb's value (0-100).
    pub percentage: Signal<f64>,

    /// The formatted display value.
    pub display_value: Signal<String>,

    /// The thumb ID (for ARIA associations).
    pub thumb_id: String,
}

/// Props for the slider thumb element.
///
/// The thumb is a visual container for pointer drag interaction and
/// focus-ring display. The actual focus target is the range input
/// rendered inside the thumb (see [`UseSliderThumbInputProps`]).
#[derive(Debug)]
pub struct UseSliderThumbProps {
    /// Pointer-down handler (focuses the hidden input + starts drag).
    pub on_pointerdown: EventHandler<PointerEvent>,
    /// Keyboard handler for arrow keys, PageUp/Down, Home/End (catches events bubbling from input).
    pub on_keydown: EventHandler<KeyboardEvent>,
    /// Focus event handler (from `use_focus_ring`).
    pub on_focus: EventHandler<FocusEvent>,
    /// Blur event handler (from `use_focus_ring`).
    pub on_blur: EventHandler<FocusEvent>,
    /// Focus-in event handler (from `use_focus_ring`).
    pub on_focusin: EventHandler<FocusEvent>,
    /// Focus-out event handler (from `use_focus_ring`).
    pub on_focusout: EventHandler<FocusEvent>,
    /// Pointer-enter handler (from `use_hover`).
    pub on_pointerenter: EventHandler<PointerEvent>,
    /// Pointer-leave handler (from `use_hover`).
    pub on_pointerleave: EventHandler<PointerEvent>,
    /// Data attribute for keyboard focus visibility.
    pub data_focus_visible: Signal<Option<&'static str>>,
}

impl IntoAttrs for UseSliderThumbProps {
    type Attrs = UseSliderThumbAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.on_pointerdown.into_on(ev::pointerdown),
            self.on_keydown.into_on(ev::keydown),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
            self.on_pointerenter.into_on(ev::pointerenter),
            self.on_pointerleave.into_on(ev::pointerleave),
            custom_attribute("data-focus-visible", self.data_focus_visible),
        )
    }
}

/// Attributes for the slider thumb element.
pub type UseSliderThumbAttrs = (
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
    On<ev::pointerenter, SharedEventCallback<PointerEvent>>,
    On<ev::pointerleave, SharedEventCallback<PointerEvent>>,
    CustomAttr<&'static str, Signal<Option<&'static str>>>,
);

/// Props for the visually hidden range input inside the slider thumb.
///
/// This input is the focus target and carries ARIA slider semantics.
/// Render it inside the thumb element with visually-hidden styling
/// (e.g. `opacity: 0.0001; width: 100%; height: 100%; pointer-events: none;
/// position: absolute; top: 0; left: 0;`).
#[derive(Debug)]
pub struct UseSliderThumbInputProps {
    /// Input type (always `"range"`).
    pub ty: &'static str,
    /// Unique ID for this thumb's input.
    pub id: String,
    /// Tab index (always `"0"` for enabled sliders).
    pub tabindex: &'static str,
    /// Accessibility label.
    pub aria_label: Option<Cow<'static, str>>,
    /// ID of an element that labels this thumb.
    pub aria_labelledby: Option<&'static str>,
    /// Current thumb value.
    pub aria_valuenow: Signal<f64>,
    /// Minimum value for this thumb (constrained by neighbors).
    pub aria_valuemin: Signal<f64>,
    /// Maximum value for this thumb (constrained by neighbors).
    pub aria_valuemax: Signal<f64>,
    /// Human-readable value description.
    pub aria_valuetext: Signal<String>,
    /// Slider orientation.
    pub aria_orientation: Signal<AriaOrientation>,
    /// Whether the slider value is invalid.
    pub aria_invalid: Option<AriaInvalid>,
    /// Whether the slider is disabled.
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    /// Whether the slider value is required.
    pub aria_required: Option<AriaRequired>,
    /// ID of an element that describes this thumb.
    pub aria_describedby: Option<&'static str>,
    /// ID of an element that provides additional details about this thumb.
    pub aria_details: Option<&'static str>,
    /// ID of an element that contains an error message for this thumb.
    pub aria_errormessage: Option<&'static str>,
    /// Form submission name.
    pub name: Option<&'static str>,
    /// Current thumb value for form submission.
    pub value: Signal<f64>,
    /// Native `min` attribute for browser form validation (mirrors the slider's minimum).
    pub min: f64,
    /// Native `max` attribute for browser form validation (mirrors the slider's maximum).
    pub max: f64,
    /// Native `step` attribute. `"any"` for continuous sliders, otherwise the numeric step.
    pub step: String,
    /// Whether the input is disabled.
    pub disabled: Signal<bool>,
    /// Element capture for the hook to obtain a reference for focusing.
    pub element_capture: ElementCaptureAttr,
}

impl IntoAttrs for UseSliderThumbInputProps {
    type Attrs = UseSliderThumbInputAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Type, self.ty),
            Attr(attr::Id, self.id),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaValuenow, self.aria_valuenow),
            Attr(attr::AriaValuemin, self.aria_valuemin),
            Attr(attr::AriaValuemax, self.aria_valuemax),
            Attr(attr::AriaValuetext, self.aria_valuetext),
            Attr(attr::AriaOrientation, self.aria_orientation),
            Attr(attr::AriaInvalid, self.aria_invalid),
            Attr(attr::AriaDisabled, self.aria_disabled),
            Attr(attr::AriaRequired, self.aria_required),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            Attr(attr::AriaDetails, self.aria_details),
            Attr(attr::AriaErrormessage, self.aria_errormessage),
            Attr(attr::Name, self.name),
            Attr(attr::Value, self.value),
            Attr(attr::Min, self.min),
            Attr(attr::Max, self.max),
            Attr(attr::Step, self.step),
            Attr(attr::Disabled, self.disabled),
            self.element_capture,
        )
    }
}

/// Attributes for the slider input element.
pub type UseSliderThumbInputAttrs = (
    Attr<attr::Type, &'static str>,
    Attr<attr::Id, String>,
    Attr<attr::Tabindex, &'static str>,
    Attr<attr::AriaLabel, Option<Cow<'static, str>>>,
    Attr<attr::AriaLabelledby, Option<&'static str>>,
    Attr<attr::AriaValuenow, Signal<f64>>,
    Attr<attr::AriaValuemin, Signal<f64>>,
    Attr<attr::AriaValuemax, Signal<f64>>,
    Attr<attr::AriaValuetext, Signal<String>>,
    Attr<attr::AriaOrientation, Signal<AriaOrientation>>,
    Attr<attr::AriaInvalid, Option<AriaInvalid>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    Attr<attr::AriaRequired, Option<AriaRequired>>,
    Attr<attr::AriaDescribedby, Option<&'static str>>,
    Attr<attr::AriaDetails, Option<&'static str>>,
    Attr<attr::AriaErrormessage, Option<&'static str>>,
    Attr<attr::Name, Option<&'static str>>,
    Attr<attr::Value, Signal<f64>>,
    Attr<attr::Min, f64>,
    Attr<attr::Max, f64>,
    Attr<attr::Step, String>,
    Attr<attr::Disabled, Signal<bool>>,
    ElementCaptureAttr,
);

/// Provides the behavior and accessibility implementation for a slider thumb.
///
/// # Example
///
/// ```ignore
/// let state = use_slider_state(UseSliderStateInput {
///     values: SliderValues::Uncontrolled(vec![50.0]),
///     ..Default::default()
/// });
///
/// let UseSliderReturn { track_props, track_ref, .. } = use_slider(UseSliderInput {
///     state,
///     ..Default::default()
/// });
///
/// let UseSliderThumbReturn { thumb_props, input_props, percentage, .. } =
///     use_slider_thumb(UseSliderThumbInput {
///         state,
///         track_ref,  // from use_slider return
///         index: 0,
///         ..Default::default()
///     });
/// ```
#[allow(clippy::too_many_lines)]
pub fn use_slider_thumb(input: UseSliderThumbInput) -> UseSliderThumbReturn {
    let UseSliderThumbInput {
        state,
        track,
        index,
        name,
        aria_label,
        aria_labelledby,
        disabled,
        validation_state,
        is_rtl,
        decimal_places,
        is_required,
        aria_describedby,
        aria_details,
        aria_errormessage,
        aria_valuetext,
    } = input;

    let base_id = Uuid::new_v4();
    let thumb_id = format!("slider-thumb-{base_id}");

    let orientation = state.orientation;
    let is_disabled = disabled;
    let input_element = CapturedElement::new();

    // Dragging state for this thumb
    let (is_dragging, set_is_dragging) = signal(false);
    let (is_focused, set_is_focused) = signal(false);

    // Value signal for this thumb - uses TRACKED access to values for reactivity
    let value = Signal::derive(move || {
        state
            .values
            .get() // TRACKED access - subscribes to changes
            .get(index)
            .copied()
            .unwrap_or(state.min_value)
    });

    // Percentage signal for this thumb (0-100 for CSS) - uses TRACKED access
    let percentage = Signal::derive(move || {
        let val = state
            .values
            .get() // TRACKED access
            .get(index)
            .copied()
            .unwrap_or(state.min_value);
        percentage_in_range(state.min_value, state.max_value, val) * 100.0
    });

    // Minimum value for this thumb, constrained by neighbors.
    let thumb_min = Signal::derive(move || {
        let global_min = state.min_value;
        if index == 0 {
            global_min
        } else {
            state
                .values
                .get()
                .get(index - 1)
                .copied()
                .unwrap_or(global_min)
        }
    });

    // Maximum value for this thumb, constrained by neighbors.
    let thumb_max = Signal::derive(move || {
        let global_max = state.max_value;
        let vals = state.values.get();
        if index >= vals.len().saturating_sub(1) {
            global_max
        } else {
            vals.get(index + 1).copied().unwrap_or(global_max)
        }
    });

    // Format display value
    let display_value = if let Some(override_text) = aria_valuetext {
        override_text
    } else {
        Signal::derive(move || {
            let value = value.get();
            if let Some(places) = decimal_places {
                format!("{value:.places$}")
            } else {
                value.to_string()
            }
        })
    };

    // Pixel accumulation: track the thumb's position in pixels (not percent) to avoid
    // precision loss when the percent snaps to steps. See module docs for details.
    let current_position_px: StoredValue<Option<f64>> = StoredValue::new(None);

    // Use use_move hook for thumb dragging. This handles all the pointer event
    // management (pointerdown, pointermove, pointerup, pointercancel) automatically.
    let UseMoveReturn {
        props: move_props, ..
    } = use_move(UseMoveInput {
        disabled: is_disabled,
        axis: Signal::derive(move || match orientation.get() {
            SliderOrientation::Horizontal => Some(MoveAxis::Horizontal),
            SliderOrientation::Vertical => Some(MoveAxis::Vertical),
        }),
        is_rtl: false,
        on_move_start: Some(Callback::new(move |_: MoveStartEvent| {
            // Initialize pixel position from current thumb percent
            if let Some(rect) = track.get_bounding_client_rect_untracked() {
                let size = match orientation.get() {
                    SliderOrientation::Horizontal => rect.width(),
                    SliderOrientation::Vertical => rect.height(),
                };
                let initial_px = state.get_thumb_percent.run(index) * size;
                current_position_px.set_value(Some(initial_px));
            }

            set_is_dragging.set(true);
            state.set_thumb_dragging.run((index, true));
            state.set_focused_thumb.run(Some(index));
        })),
        on_move: Some(Callback::new(move |e: MoveEvent| {
            if let Some(rect) = track.get_bounding_client_rect_untracked() {
                let orientation = orientation.get_untracked();
                let size = match orientation {
                    SliderOrientation::Horizontal => rect.width(),
                    SliderOrientation::Vertical => rect.height(),
                };

                // Get current position in pixels (should be initialized in on_move_start)
                let pos = current_position_px
                    .get_value()
                    .unwrap_or_else(|| state.get_thumb_percent.run(index) * size);

                // use_move provides raw deltas: delta_x for horizontal, delta_y for vertical
                // For vertical sliders, up should increase value (positive delta_y means cursor moved down)
                let delta = match orientation {
                    SliderOrientation::Horizontal => e.delta_x,
                    SliderOrientation::Vertical => -e.delta_y,
                };
                let delta = if is_rtl && orientation == SliderOrientation::Horizontal {
                    -delta
                } else {
                    delta
                };

                // Accumulate in pixels, then convert to percent
                let new_pos = pos + delta;
                current_position_px.set_value(Some(new_pos));

                let new_percent = (new_pos / size).clamp(0.0, 1.0);
                state.set_thumb_percent.run((index, new_percent));
            }
        })),
        on_move_end: Some(Callback::new(move |_: MoveEndEvent| {
            set_is_dragging.set(false);
            state.set_thumb_dragging.run((index, false));
            current_position_px.set_value(None);
        })),
        on_position_change: None,
        constraint: None,
        allow_container_click: false,
        initial_position: None,
    });

    // Destructure move_props to catch future type-changes / extensions early.
    // Sliders have their own keyboard handling, so on_keydown from use_move is ignored.
    // element_capture is harmless when unused.
    let UseMoveProps {
        on_pointerdown,
        on_keydown: _,
        element_capture: _,
    } = move_props;

    // Focus the hidden input on pointerdown. use_move calls prevent_default()
    // on pointerdown which suppresses the browser's default focus behavior.
    let handle_pointerdown = EventHandler::new(move |_: PointerEvent| {
        if let Some(el) = input_element.get_untracked() {
            focus_element(&el, true);
        }
    })
    .chain(on_pointerdown);

    // Handle keydown on thumb (catches events bubbling from the input)
    let handle_keydown = move |e: KeyboardEvent| {
        if is_disabled.get_untracked() {
            return;
        }

        // Default keyboard increment: step if present, else 1% of range.
        let range = state.max_value - state.min_value;
        let step_amount = state.step.unwrap_or(range / 100.0);
        let page_size = state.page_size;
        let min_val = state.min_value;
        let max_val = state.max_value;
        let shift = e.shift_key();
        let key = e.key();

        // Helper closures
        let increment = |amount: f64| {
            e.prevent_default();
            state.increment_thumb.run((index, Some(amount)));
        };
        let decrement = |amount: f64| {
            e.prevent_default();
            state.decrement_thumb.run((index, Some(amount)));
        };
        let set_to_min = || {
            e.prevent_default();
            state.set_thumb_value.run((index, min_val));
        };
        let set_to_max = || {
            e.prevent_default();
            state.set_thumb_value.run((index, max_val));
        };

        match (key.as_str(), shift, is_rtl) {
            // Right arrow / Up arrow (increment by step)
            ("ArrowRight", false, false) | ("ArrowLeft", false, true) | ("ArrowUp", false, _) => {
                increment(step_amount);
            }
            // Right arrow (shifted) / Up arrow (shifted) / Page up (increment by page)
            ("ArrowRight", true, false)
            | ("ArrowLeft", true, true)
            | ("ArrowUp", true, _)
            | ("PageUp", _, _) => increment(page_size),

            // Left arrow / Down arrow (decrement by step)
            ("ArrowLeft", false, false) | ("ArrowRight", false, true) | ("ArrowDown", false, _) => {
                decrement(step_amount);
            }
            // Left arrow (shifted) / Down arrow (shifted) / Page down (decrement by page)
            ("ArrowLeft", true, false)
            | ("ArrowRight", true, true)
            | ("ArrowDown", true, _)
            | ("PageDown", _, _) => decrement(page_size),

            // Home/End
            ("Home", _, _) => set_to_min(),
            ("End", _, _) => set_to_max(),

            _ => {}
        }
    };

    // Use hover hook for robust hover tracking (handles iOS, touch/pen, disabled state)
    let UseHoverReturn {
        props: hover_props,
        is_hovered,
    } = use_hover(UseHoverInput {
        disabled: is_disabled,
        on_hover_start: None,
        on_hover_end: None,
        on_hover_change: None,
    });

    // Use focus ring for keyboard focus visibility.
    // Uses within=true so focus on the hidden input child triggers the ring
    // on the parent thumb element.
    let UseFocusRingReturn {
        props: focus_ring_props,
        is_focus_visible,
        is_focused: _,
    } = use_focus_ring(UseFocusRingInput {
        disabled: is_disabled,
        within: true,
        auto_focus: false,
        is_text_input: false,
        on_focus: Some(Callback::new(move |_: FocusEvent| {
            set_is_focused.set(true);
            state.set_focused_thumb.run(Some(index));
        })),
        on_blur: Some(Callback::new(move |_: FocusEvent| {
            set_is_focused.set(false);
            if state.focused_thumb.get_untracked() == Some(index) {
                state.set_focused_thumb.run(None);
            }
        })),
        on_focus_change: None,
    });

    // Compute aria-invalid
    let aria_invalid = (validation_state == ValidationState::Invalid).then_some(AriaInvalid::True);

    // Compute aria-disabled
    let aria_disabled = Signal::derive(move || is_disabled.get().then_some(AriaDisabled::True));

    // Compute aria-required
    let aria_required = is_required.then_some(AriaRequired::True);

    // Orientation string
    let aria_orientation = Signal::derive(move || AriaOrientation::from(orientation.get()));

    UseSliderThumbReturn {
        thumb_props: UseSliderThumbProps {
            on_pointerdown: handle_pointerdown,
            on_keydown: EventHandler::new(handle_keydown),
            on_focus: focus_ring_props.on_focus,
            on_blur: focus_ring_props.on_blur,
            on_focusin: focus_ring_props.on_focusin,
            on_focusout: focus_ring_props.on_focusout,
            on_pointerenter: hover_props.on_pointerenter,
            on_pointerleave: hover_props.on_pointerleave,
            data_focus_visible: focus_ring_props.data_focus_visible,
        },
        input_props: UseSliderThumbInputProps {
            ty: "range",
            id: thumb_id.clone(),
            tabindex: "0",
            aria_label,
            aria_labelledby,
            aria_valuenow: value,
            aria_valuemin: thumb_min,
            aria_valuemax: thumb_max,
            aria_valuetext: display_value,
            aria_orientation,
            aria_invalid,
            aria_disabled,
            aria_required,
            aria_describedby,
            aria_details,
            aria_errormessage,
            name,
            value,
            min: state.min_value,
            max: state.max_value,
            step: state
                .step
                .map_or_else(|| "any".to_owned(), |s| s.to_string()),
            disabled: is_disabled,
            element_capture: input_element.attr(),
        },
        is_dragging: is_dragging.into(),
        is_hovered,
        is_focused: is_focused.into(),
        is_focus_visible,
        value,
        percentage,
        display_value,
        thumb_id,
    }
}
