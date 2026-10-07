// Upstream: react-aria/src/slider/useSliderThumb.ts @ 99e6102368
use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
    tachys::html::property::{Property, prop},
};
use send_wrapper::SendWrapper;
use web_sys::{Event, FocusEvent, KeyboardEvent, PointerEvent};

use crate::{
    hooks::{
        FocusableContextAttr, IntoAttrs, MoveEndEvent, MoveEvent, MoveStartEvent, PropsWithStyles,
        UseFocusableInput, UseFocusableReturn, UseFormResetInput, UseLabelInput, UseLabelProps,
        UseLabelReturn, UseMoveInput,
        slider::{SliderData, SliderState},
        use_focusable, use_form_reset, use_label, use_move,
    },
    utils::{
        EventAccessors, EventHandler, EventTargetExt,
        aria::{AriaInvalid, AriaOrientation, AriaRequired},
        css::TouchAction,
        element_capture::{CapturedElement, ElementCaptureAttr},
        event_listeners::{Listener, listen_to},
        focus::focus_safely,
        i18n::use_direction,
        keyboard_shortcut::{KeyboardShortcuts, Shortcut},
        locale::WritingDirection,
        number_value::NumberValue,
        orientation::Orientation,
        pointer_type::PointerType,
        style::TouchActionProperty,
        styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The slider's `SliderData` and track come in as fields of `UseSliderThumbInput`; the input
//   element is captured by its props instead of an `inputRef`.
// - The thumb's orientation is the slider's (react-aria allows overriding it per thumb).
// - `isRequired`/`isInvalid` are signals; no `validationState`.
// - The input's `min`, `max`, `step` and `value` are the values' exact decimals (C15; react-aria:
//   JS numbers); typed values parse through a decimal too.
//
// ## DIFFERENT BEHAVIOR
// - Thumb presses start on `pointerdown` only (PointerEvent is always available).
//
// =============================================================================

/// Input of [`use_slider_thumb`].
#[derive(Debug)]
pub struct UseSliderThumbInput<T: NumberValue> {
    pub state: SliderState<T>,
    pub slider: SliderData,
    /// The slider's track (`UseSliderReturn::track_element`).
    pub track: CapturedElement,
    /// The thumb's index in the slider's values.
    pub index: usize,
    /// Disables this thumb only (the slider's `is_disabled` disables all).
    pub is_disabled: Signal<bool>,
    pub is_required: Signal<bool>,
    pub is_invalid: Signal<bool>,
    /// The input's name, for forms.
    pub name: Option<String>,
    /// The id of the form the input belongs to, if not its ancestor.
    pub form: Option<String>,
    /// Whether the thumb has a visible label of its own (with `label_props`).
    pub has_label: Signal<bool>,
    /// Names this thumb (next to the slider's label), e.g. "Minimum".
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
    /// Further elements describing the thumb (next to the slider's description).
    pub aria_describedby: Option<String>,
    /// The element with the thumb's error message.
    pub aria_errormessage: Option<String>,
    /// Further elements with details about the thumb (next to the slider's).
    pub aria_details: Option<String>,
}

/// Return value of [`use_slider_thumb`].
#[derive(Debug)]
pub struct UseSliderThumbReturn {
    /// For the thumb element, positioned on the track (`position: absolute`).
    pub thumb_props: PropsWithStyles<UseSliderThumbProps>,
    /// For the `<input type="range">` inside the thumb (visually hidden): focus, keyboard and
    /// assistive technology.
    pub input_props: UseSliderThumbInputProps,
    /// For the thumb's own label, if it has one.
    pub label_props: UseLabelProps,
    pub is_dragging: Signal<bool>,
    pub is_disabled: Signal<bool>,
    pub is_focused: Signal<bool>,
}

#[derive(Debug)]
pub struct UseSliderThumbProps {
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub element_capture: ElementCaptureAttr,
}

pub type UseSliderThumbAttrs = (
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    ElementCaptureAttr,
);

impl IntoAttrs for UseSliderThumbProps {
    type Attrs = UseSliderThumbAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.on_pointerdown.into_on(ev::pointerdown),
            self.on_keydown.into_on(ev::keydown),
            self.element_capture,
        )
    }
}

#[derive(Debug)]
pub struct UseSliderThumbInputProps {
    pub id: String,
    pub tabindex: Signal<Option<i32>>,
    pub min: Signal<String>,
    pub max: Signal<String>,
    pub step: Signal<String>,
    pub value: Signal<String>,
    pub name: Option<String>,
    pub form: Option<String>,
    pub disabled: Signal<bool>,
    pub aria_orientation: Signal<AriaOrientation>,
    pub aria_valuetext: Signal<String>,
    pub aria_required: Signal<Option<AriaRequired>>,
    pub aria_invalid: Signal<Option<AriaInvalid>>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<String>>,
    pub aria_describedby: Signal<Option<String>>,
    pub aria_errormessage: Option<String>,
    pub aria_details: Option<String>,
    pub element_capture: ElementCaptureAttr,
    pub on_input: EventHandler<Event>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_keyup: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub context_attrs: FocusableContextAttr,
}

pub type UseSliderThumbInputAttrs = (
    (
        Attr<attr::Type, &'static str>,
        Attr<attr::Id, String>,
        Attr<attr::Tabindex, Signal<Option<i32>>>,
        Attr<attr::Min, Signal<String>>,
        Attr<attr::Max, Signal<String>>,
        Attr<attr::Step, Signal<String>>,
        Attr<attr::Value, Signal<String>>,
        Property<&'static str, Signal<String>>,
        Attr<attr::Name, Option<String>>,
        Attr<attr::Form, Option<String>>,
        Attr<attr::Disabled, Signal<bool>>,
    ),
    (
        Attr<attr::AriaOrientation, Signal<AriaOrientation>>,
        Attr<attr::AriaValuetext, Signal<String>>,
        Attr<attr::AriaRequired, Signal<Option<AriaRequired>>>,
        Attr<attr::AriaInvalid, Signal<Option<AriaInvalid>>>,
        Attr<attr::AriaLabel, MaybeProp<String>>,
        Attr<attr::AriaLabelledby, Signal<Option<String>>>,
        Attr<attr::AriaDescribedby, Signal<Option<String>>>,
        Attr<attr::AriaErrormessage, Option<String>>,
        Attr<attr::AriaDetails, Option<String>>,
        ElementCaptureAttr,
    ),
    (
        On<ev::input, SharedEventCallback<Event>>,
        On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
        On<ev::keyup, SharedEventCallback<KeyboardEvent>>,
        On<ev::focus, SharedEventCallback<FocusEvent>>,
        On<ev::blur, SharedEventCallback<FocusEvent>>,
        FocusableContextAttr,
    ),
);

impl IntoAttrs for UseSliderThumbInputProps {
    type Attrs = UseSliderThumbInputAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            (
                Attr(attr::Type, "range"),
                Attr(attr::Id, self.id),
                Attr(attr::Tabindex, self.tabindex),
                Attr(attr::Min, self.min),
                Attr(attr::Max, self.max),
                Attr(attr::Step, self.step),
                Attr(attr::Value, self.value),
                // The attribute is the initial value only.
                prop("value", self.value),
                Attr(attr::Name, self.name),
                Attr(attr::Form, self.form),
                Attr(attr::Disabled, self.disabled),
            ),
            (
                Attr(attr::AriaOrientation, self.aria_orientation),
                Attr(attr::AriaValuetext, self.aria_valuetext),
                Attr(attr::AriaRequired, self.aria_required),
                Attr(attr::AriaInvalid, self.aria_invalid),
                Attr(attr::AriaLabel, self.aria_label),
                Attr(attr::AriaLabelledby, self.aria_labelledby),
                Attr(attr::AriaDescribedby, self.aria_describedby),
                Attr(attr::AriaErrormessage, self.aria_errormessage),
                Attr(attr::AriaDetails, self.aria_details),
                self.element_capture,
            ),
            (
                self.on_input.into_on(ev::input),
                self.on_keydown.into_on(ev::keydown),
                self.on_keyup.into_on(ev::keyup),
                self.on_focus.into_on(ev::focus),
                self.on_blur.into_on(ev::blur),
                self.context_attrs,
            ),
        )
    }
}

/// The value of a range input's text: exact through a decimal (C15), else the closest value.
fn parse_value<T: NumberValue>(text: &str) -> Option<T> {
    fixed_decimal::Decimal::try_from_str(text.trim())
        .ok()
        .and_then(|decimal| T::from_decimal(&decimal))
        .or_else(|| text.trim().parse::<f64>().ok().and_then(T::from_f64))
}

/// A thumb of a slider: drag it, or focus its range input and use the arrow keys (Shift: by a
/// page), PageUp/PageDown, Home and End.
#[allow(clippy::too_many_lines)]
pub fn use_slider_thumb<T: NumberValue>(input: UseSliderThumbInput<T>) -> UseSliderThumbReturn {
    let UseSliderThumbInput {
        state,
        slider,
        track,
        index,
        is_disabled,
        is_required,
        is_invalid,
        name,
        form,
        has_label,
        aria_label,
        aria_labelledby,
        aria_describedby,
        aria_errormessage,
        aria_details,
    } = input;

    if index >= state.values.with_untracked(Vec::len) {
        crate::utils::dev_warn!(
            "Slider thumb {index} has no value: the slider has {} values.",
            state.values.with_untracked(Vec::len)
        );
    }
    let is_disabled = Signal::derive(move || is_disabled.get() || state.is_disabled.get());
    let orientation = state.orientation;
    let direction = use_direction();
    let is_vertical = move || orientation.get_untracked() == Orientation::Vertical;
    let reverse_x = move || direction.get_untracked() == WritingDirection::Rtl;

    let UseLabelReturn {
        label_props,
        field_props,
    } = use_label(UseLabelInput {
        id: Some(slider.thumb_id(index)),
        has_label,
        aria_label,
        // Labelled by the slider in any case (also keeps `use_label` from warning); the labelling
        // itself follows the slider's label below.
        aria_labelledby: Some(slider.labelled_by.get_untracked()),
        ..UseLabelInput::default()
    });
    // As `use_label` with `"{slider labelled by} {aria_labelledby}"`, but following whether the
    // slider's label is rendered: the thumb's own label, the slider's, then the given ids, and
    // the thumb itself (first) next to an `aria-label`.
    let thumb_labelled_by = {
        let (thumb_label_id, thumb_id) = (label_props.id.clone(), field_props.id.clone());
        let slider_labelled_by = slider.labelled_by;
        Signal::derive(move || {
            let mut ids = Vec::new();
            if aria_label.with(Option::is_some) {
                ids.push(thumb_id.clone());
            }
            if has_label.get() {
                ids.push(thumb_label_id.clone());
            }
            ids.push(slider_labelled_by.get());
            ids.extend(
                aria_labelledby
                    .iter()
                    .flat_map(|ids| ids.split_whitespace())
                    .map(str::to_owned),
            );
            crate::hooks::form::use_label::dedup_ids(&mut ids);
            Some(ids.join(" "))
        })
    };

    let input_element = CapturedElement::new();
    let focus_input = move || {
        if let Some(el) = input_element.get_untracked() {
            focus_safely(&el);
        }
    };
    let is_focused = Signal::derive(move || state.focused_thumb.get() == Some(index));
    Effect::new(move |_| {
        if is_focused.get() {
            focus_input();
        }
    });

    // Keyboard changes count as a drag, so that `on_change_end` follows them.
    let keyboard_update = move |update: &dyn Fn()| {
        state.set_thumb_dragging(index, true);
        update();
        state.set_thumb_dragging(index, false);
    };
    let shortcuts = KeyboardShortcuts::new()
        .on(Shortcut::key("PageUp"), move |_| {
            keyboard_update(&|| state.increment_thumb(index, Some(untrack(|| state.page_size()))));
        })
        .on(Shortcut::key("PageDown"), move |_| {
            keyboard_update(&|| state.decrement_thumb(index, Some(untrack(|| state.page_size()))));
        })
        .on(Shortcut::key("Home"), move |_| {
            keyboard_update(&|| {
                state.set_thumb_value(index, untrack(|| state.thumb_min_value(index)));
            });
        })
        .on(Shortcut::key("End"), move |_| {
            keyboard_update(&|| {
                state.set_thumb_value(index, untrack(|| state.thumb_max_value(index)));
            });
        });

    // Dragging accumulates the pointer's deltas in pixels, so that movements smaller than a step
    // aren't lost when the value snaps.
    let position = StoredValue::new(None::<f64>);
    let thumb_move = use_move(UseMoveInput {
        is_disabled,
        on_move_start: Some(Callback::new(move |_: MoveStartEvent| {
            position.set_value(None);
            state.set_thumb_dragging(index, true);
        })),
        on_move: Some(Callback::new(move |e: MoveEvent| {
            let Some(track_el) = track.get_untracked() else {
                return;
            };
            let rect = track_el.get_bounding_client_rect();
            let size = if is_vertical() {
                rect.height()
            } else {
                rect.width()
            };
            if e.pointer_type == PointerType::Keyboard {
                let step = Some(if e.modifiers.shift_key {
                    untrack(|| state.page_size())
                } else {
                    state.step.get_untracked()
                });
                if (e.delta_x > 0.0 && reverse_x())
                    || (e.delta_x < 0.0 && !reverse_x())
                    || e.delta_y > 0.0
                {
                    state.decrement_thumb(index, step);
                } else {
                    state.increment_thumb(index, step);
                }
                return;
            }
            let current = position
                .get_value()
                .unwrap_or_else(|| untrack(|| state.thumb_percent(index)) * size);
            let mut delta = if is_vertical() { e.delta_y } else { e.delta_x };
            if is_vertical() || reverse_x() {
                delta = -delta;
            }
            let current = current + delta;
            position.set_value(Some(current));
            state.set_thumb_percent(index, (current / size).clamp(0.0, 1.0));
        })),
        on_move_end: Some(Callback::new(move |_: MoveEndEvent| {
            state.set_thumb_dragging(index, false);
        })),
    });

    // Register the thumb's editability with the state (react-aria: during render).
    Effect::new(move |_| state.set_thumb_editable(index, !is_disabled.get()));
    state.set_thumb_editable(index, !is_disabled.get_untracked());

    let UseFocusableReturn {
        props: focusable_props,
        ..
    } = use_focusable(UseFocusableInput {
        is_disabled,
        on_focus: Some(Callback::new(move |_| state.set_focused_thumb(Some(index)))),
        on_blur: Some(Callback::new(move |_| state.set_focused_thumb(None))),
        shortcuts: Some(shortcuts),
        allow_shortcut_repeats: true,
        ..UseFocusableInput::default()
    });

    // A press on the thumb focuses its input and drags until the pointer is released.
    let release_listeners: StoredValue<Option<SendWrapper<Vec<Listener>>>> = StoredValue::new(None);
    let on_down = move |e: PointerEvent| {
        if is_disabled.get_untracked()
            || e.button() != 0
            || e.alt_key()
            || e.ctrl_key()
            || e.meta_key()
        {
            return;
        }
        focus_input();
        state.set_thumb_dragging(index, true);
        let pointer_id = e.pointer_id();
        if let Some(document) = e.expect_current_target().get_owner_document() {
            let on_up = listen_to(&document, ev::pointerup, false, move |e: PointerEvent| {
                if e.pointer_id() == pointer_id {
                    focus_input();
                    state.set_thumb_dragging(index, false);
                    release_listeners.set_value(None);
                }
            });
            release_listeners.set_value(Some(SendWrapper::new(vec![on_up])));
        }
    };
    on_cleanup(move || {
        release_listeners.try_update_value(Option::take);
    });

    use_form_reset(UseFormResetInput {
        element: input_element,
        initial_value: state
            .default_values()
            .get(index)
            .copied()
            .unwrap_or(T::ZERO),
        on_reset: Callback::new(move |value| state.set_thumb_value(index, value)),
    });

    let percent = Signal::derive(move || {
        let percent = state.thumb_percent(index);
        if orientation.get() == Orientation::Vertical || direction.get() == WritingDirection::Rtl {
            1.0 - percent
        } else {
            percent
        }
    });
    let side = move || {
        if orientation.get() == Orientation::Vertical {
            "top"
        } else {
            "left"
        }
    };
    let thumb_styles = Styles::new()
        .add_unchecked("position", "absolute")
        .add_unchecked("transform", "translate(-50%, -50%)")
        .add(TouchActionProperty.declare(TouchAction::None))
        .add_optional_unchecked("left", move || {
            (side() == "left").then(|| format!("{}%", percent.get() * 100.0))
        })
        .add_optional_unchecked("top", move || {
            (side() == "top").then(|| format!("{}%", percent.get() * 100.0))
        });

    // Exact (C15): through the value's decimal, not `f64` (f32 0.1 would be "0.10000000149011612").
    let to_string = |value: T| {
        value
            .to_decimal()
            .map_or_else(|| value.to_f64().to_string(), |decimal| decimal.to_string())
    };
    UseSliderThumbReturn {
        thumb_props: PropsWithStyles::new(
            UseSliderThumbProps {
                on_pointerdown: EventHandler::new(on_down).chain(thumb_move.props.on_pointerdown),
                on_keydown: thumb_move.props.on_keydown,
                element_capture: thumb_move.props.element_capture,
            },
            thumb_styles,
        ),
        input_props: UseSliderThumbInputProps {
            id: field_props.id,
            tabindex: Signal::derive(move || (!is_disabled.get()).then_some(0)),
            min: Signal::derive(move || to_string(state.thumb_min_value(index))),
            max: Signal::derive(move || to_string(state.thumb_max_value(index))),
            step: Signal::derive(move || to_string(state.step.get())),
            value: Signal::derive(move || to_string(state.thumb_value(index))),
            name,
            form,
            disabled: is_disabled,
            aria_orientation: Signal::derive(move || orientation.get().into()),
            aria_valuetext: Signal::derive(move || state.thumb_value_label(index)),
            aria_required: Signal::derive(move || is_required.get().then_some(AriaRequired::True)),
            aria_invalid: Signal::derive(move || is_invalid.get().then_some(AriaInvalid::True)),
            aria_label: field_props.aria_label,
            aria_labelledby: thumb_labelled_by,
            aria_describedby: {
                let slider_describedby = slider.aria_describedby;
                Signal::derive(move || {
                    let ids: Vec<String> = slider_describedby
                        .get()
                        .into_iter()
                        .chain(aria_describedby.clone())
                        .collect();
                    (!ids.is_empty()).then(|| ids.join(" "))
                })
            },
            aria_errormessage,
            aria_details: {
                let ids: Vec<String> = slider
                    .aria_details
                    .into_iter()
                    .chain(aria_details)
                    .collect();
                (!ids.is_empty()).then(|| ids.join(" "))
            },
            element_capture: focusable_props.element_capture.chain(input_element.attr()),
            on_input: EventHandler::new(move |e: Event| {
                if let Some(value) = parse_value::<T>(&event_target_value(&e)) {
                    state.set_thumb_value(index, value);
                }
            }),
            on_keydown: focusable_props.on_keydown,
            on_keyup: focusable_props.on_keyup,
            on_focus: focusable_props.on_focus,
            on_blur: focusable_props.on_blur,
            context_attrs: FocusableContextAttr(focusable_props.context_attrs),
        },
        label_props,
        is_dragging: Signal::derive(move || state.is_thumb_dragging(index)),
        is_disabled,
        is_focused,
    }
}
