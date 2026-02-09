use crate::hooks::form::use_slider_state::UseSliderStateReturn;
use crate::hooks::interactions::use_hover::{use_hover, UseHoverInput, UseHoverReturn};
use crate::hooks::interactions::use_move::{
    use_move, MoveAxis, MoveEndEvent, MoveEvent, MoveStartEvent, UseMoveInput,
};
use crate::hooks::{
    use_focus_ring, SliderOrientation, UseFocusRingInput, UseFocusRingReturn, UseMoveProps,
    UseMoveReturn, ValidationState,
};
use crate::utils::focus::focus_element;
use crate::utils::math::percentage_in_range;
use crate::utils::CapturedElement;
use crate::utils::EventHandler;
use leptos::attr;
use leptos::attr::custom::CustomAttr;
use leptos::attr::Attr;
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use std::borrow::Cow;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{FocusEvent, KeyboardEvent, PointerEvent};

/// Input parameters for the `use_slider_thumb` hook.
#[derive(Clone)]
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
}

/// The return value of the `use_slider_thumb` hook.
#[derive(Clone)]
pub struct UseSliderThumbReturn {
    /// Props for the thumb element.
    pub thumb_props: UseSliderThumbProps,

    /// Props for a hidden input for form submission.
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

/// Intermediate props struct for the slider thumb element.
/// Call `into_attrs()` or `to_attrs()` to convert to spreadable attributes.
#[derive(Debug, Clone)]
pub struct UseSliderThumbProps {
    id: String,
    role: &'static str,
    tabindex: &'static str,
    aria_label: Option<Cow<'static, str>>,
    aria_labelledby: Option<&'static str>,
    aria_valuenow: Signal<f64>,
    aria_valuemin: Signal<f64>,
    aria_valuemax: Signal<f64>,
    aria_valuetext: Signal<String>,
    aria_orientation: Signal<&'static str>,
    aria_invalid: Option<&'static str>,
    aria_disabled: Signal<Option<&'static str>>,
    aria_required: Option<&'static str>,
    aria_describedby: Option<&'static str>,
    aria_details: Option<&'static str>,
    aria_errormessage: Option<&'static str>,
    on_keydown: EventHandler<KeyboardEvent>,
    on_pointerdown: EventHandler<PointerEvent>,
    on_focus: EventHandler<FocusEvent>,
    on_blur: EventHandler<FocusEvent>,
    on_pointerenter: EventHandler<PointerEvent>,
    on_pointerleave: EventHandler<PointerEvent>,
    data_focus_visible: CustomAttr<&'static str, Signal<Option<&'static str>>>,
}

impl UseSliderThumbProps {
    /// Converts these props into spreadable attributes, consuming self.
    pub fn into_attrs(self) -> UseSliderThumbAttrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
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
            self.on_keydown.into_on(ev::keydown),
            self.on_pointerdown.into_on(ev::pointerdown),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_pointerenter.into_on(ev::pointerenter),
            self.on_pointerleave.into_on(ev::pointerleave),
            self.data_focus_visible,
        )
    }

    /// Converts these props into spreadable attributes by cloning.
    pub fn to_attrs(&self) -> UseSliderThumbAttrs {
        self.clone().into_attrs()
    }
}

/// Attributes for the slider thumb element.
pub type UseSliderThumbAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, &'static str>,
    Attr<attr::Tabindex, &'static str>,
    Attr<attr::AriaLabel, Option<Cow<'static, str>>>,
    Attr<attr::AriaLabelledby, Option<&'static str>>,
    Attr<attr::AriaValuenow, Signal<f64>>,
    Attr<attr::AriaValuemin, Signal<f64>>,
    Attr<attr::AriaValuemax, Signal<f64>>,
    Attr<attr::AriaValuetext, Signal<String>>,
    Attr<attr::AriaOrientation, Signal<&'static str>>,
    Attr<attr::AriaInvalid, Option<&'static str>>,
    Attr<attr::AriaDisabled, Signal<Option<&'static str>>>,
    Attr<attr::AriaRequired, Option<&'static str>>,
    Attr<attr::AriaDescribedby, Option<&'static str>>,
    Attr<attr::AriaDetails, Option<&'static str>>,
    Attr<attr::AriaErrormessage, Option<&'static str>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::pointerenter, SharedEventCallback<PointerEvent>>,
    On<ev::pointerleave, SharedEventCallback<PointerEvent>>,
    CustomAttr<&'static str, Signal<Option<&'static str>>>,
);

#[derive(Debug, Clone)]
pub struct UseSliderThumbInputProps {
    ty: &'static str,
    name: Option<&'static str>,
    value: Signal<f64>,
    disabled: Signal<bool>,
    aria_hidden: &'static str,
}

impl UseSliderThumbInputProps {
    pub fn into_attrs(self) -> UseSliderThumbInputAttrs {
        (
            Attr(attr::Type, self.ty),
            Attr(attr::Name, self.name),
            Attr(attr::Value, self.value),
            Attr(attr::Disabled, self.disabled),
            Attr(attr::AriaHidden, self.aria_hidden),
        )
    }
}

/// Attributes for the hidden input element.
pub type UseSliderThumbInputAttrs = (
    Attr<attr::Type, &'static str>,
    Attr<attr::Name, Option<&'static str>>,
    Attr<attr::Value, Signal<f64>>,
    Attr<attr::Disabled, Signal<bool>>,
    Attr<attr::AriaHidden, &'static str>,
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
    let base_id = Uuid::new_v4();
    let thumb_id = format!("slider-thumb-{base_id}");

    let state = input.state;
    let orientation = state.orientation;
    let track = input.track;
    let index = input.index;
    let is_disabled = input.disabled;
    let is_rtl = input.is_rtl;
    let decimal_places = input.decimal_places;

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
    let display_value = Signal::derive(move || {
        let value = value.get();
        if let Some(places) = decimal_places {
            format!("{value:.places$}")
        } else {
            value.to_string()
        }
    });

    // Pixel accumulation: track the thumb's position in pixels (not percent) to avoid
    // precision loss when the percent snaps to steps. See module docs for details.
    let current_position_px: StoredValue<Option<f64>> = StoredValue::new(None);

    // Use use_move hook for thumb dragging. This handles all the pointer event
    // management (pointerdown, pointermove, pointerup, pointercancel) automatically.
    let UseMoveReturn { props: move_props } = use_move(UseMoveInput {
        axis: Signal::derive(move || match orientation.get() {
            SliderOrientation::Horizontal => Some(MoveAxis::Horizontal),
            SliderOrientation::Vertical => Some(MoveAxis::Vertical),
        }),
        on_move_start: Callback::new(move |_: MoveStartEvent| {
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
        }),
        on_move: Callback::new(move |e: MoveEvent| {
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
        }),
        on_move_end: Callback::new(move |_: MoveEndEvent| {
            set_is_dragging.set(false);
            state.set_thumb_dragging.run((index, false));
            current_position_px.set_value(None);
        }),
    });

    // Destructure move_props to catch future type-changes / extensions early.
    let UseMoveProps { on_pointerdown } = move_props;

    // Focus the thumb element explicitly. use_move calls prevent_default()
    // on pointerdown which suppresses the browser's default focus behavior.
    let on_pointerdown = EventHandler::new(move |e: PointerEvent| {
        if let Some(target) = e.current_target() {
            if let Some(el) = target.dyn_ref::<web_sys::Element>() {
                focus_element(el, true);
            }
        }
    })
    .chain(on_pointerdown);

    // Handle keydown on thumb
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
    // Pass the thumb's focus/blur logic as callbacks so use_focus_ring chains them
    // with its internal focus tracking, making is_focus_visible work correctly.
    let UseFocusRingReturn {
        props: focus_ring_props,
        is_focus_visible,
        is_focused: _,
    } = use_focus_ring(UseFocusRingInput {
        disabled: is_disabled,
        within: false,
        auto_focus: false,
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
    let aria_invalid = if input.validation_state == ValidationState::Invalid {
        Some("true")
    } else {
        None
    };

    // Compute aria-disabled
    let aria_disabled = Signal::derive(move || {
        if is_disabled.get() {
            Some("true")
        } else {
            None
        }
    });

    // Compute aria-required
    let aria_required = if input.is_required {
        Some("true")
    } else {
        None
    };

    // Orientation string
    let aria_orientation = Signal::derive(move || match orientation.get() {
        SliderOrientation::Horizontal => "horizontal",
        SliderOrientation::Vertical => "vertical",
    });

    UseSliderThumbReturn {
        thumb_props: UseSliderThumbProps {
            id: thumb_id.clone(),
            role: "slider",
            tabindex: "0",
            aria_label: input.aria_label,
            aria_labelledby: input.aria_labelledby,
            aria_valuenow: value,
            aria_valuemin: thumb_min,
            aria_valuemax: thumb_max,
            aria_valuetext: display_value,
            aria_orientation,
            aria_invalid,
            aria_disabled,
            aria_required,
            aria_describedby: input.aria_describedby,
            aria_details: input.aria_details,
            aria_errormessage: input.aria_errormessage,
            on_keydown: EventHandler::new(handle_keydown),
            on_pointerdown,
            on_focus: focus_ring_props.on_focus,
            on_blur: focus_ring_props.on_blur,
            on_pointerenter: hover_props.on_pointerenter,
            on_pointerleave: hover_props.on_pointerleave,
            data_focus_visible: focus_ring_props.data_focus_visible,
        },
        input_props: UseSliderThumbInputProps {
            ty: "hidden",
            name: input.name,
            value,
            disabled: is_disabled,
            aria_hidden: "true",
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
