// Upstream: react-stately/src/color/useColorChannelFieldState.ts @ 99e6102368
use leptos::prelude::*;

use crate::{
    hooks::form::{
        use_form_validation_state::{ValidateFn, ValidationBehavior},
        use_number_field_state::{
            NumberFieldState, UseNumberFieldStateInput, use_number_field_state,
        },
    },
    utils::{ValueBinding, color::ColorValue},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Generic over the color type (`ColorValue`), the color space is the type.
// - Hook-owned value (C4): `default_value` + `on_change`, or `value` bound to app state.
// - A `Copy` struct (C3) holding the channel's `NumberFieldState` (react-aria: the number field
//   state spread into the channel field state).
//
// =============================================================================

/// Input of [`use_color_channel_field_state`]. Start from
/// [`UseColorChannelFieldStateInput::new`].
#[derive(Clone)]
pub struct UseColorChannelFieldStateInput<C: ColorValue> {
    /// The initial color (`None`: empty).
    pub default_value: Option<C>,
    /// The color as app state, replacing `default_value`.
    pub value: Option<ValueBinding<Option<C>>>,
    /// The channel the field edits.
    pub channel: C::Channel,
    pub is_disabled: Signal<bool>,
    pub is_read_only: Signal<bool>,
    pub is_invalid: Signal<bool>,
    pub validate: Option<ValidateFn<Option<f64>>>,
    pub validation_behavior: ValidationBehavior,
    /// The field's name, matching server errors.
    pub name: Option<String>,
    /// Called with the color when the channel's value is committed.
    pub on_change: Option<Callback<Option<C>>>,
}

impl<C: ColorValue> UseColorChannelFieldStateInput<C> {
    /// An empty field for `channel`.
    pub fn new(channel: C::Channel) -> Self {
        Self {
            default_value: None,
            value: None,
            channel,
            is_disabled: Signal::stored(false),
            is_read_only: Signal::stored(false),
            is_invalid: Signal::stored(false),
            validate: None,
            validation_behavior: ValidationBehavior::default(),
            name: None,
            on_change: None,
        }
    }
}

/// The state of a field editing one channel of a color.
#[derive(Clone, Copy)]
pub struct ColorChannelFieldState<C: ColorValue> {
    /// The color.
    pub color_value: Signal<Option<C>>,
    /// The channel the field edits.
    pub channel: C::Channel,
    /// The number field of the channel's value.
    pub number: NumberFieldState<f64>,
    binding: ValueBinding<Option<C>>,
    default_color_value: StoredValue<Option<C>>,
}

impl<C: ColorValue> ColorChannelFieldState<C> {
    /// Sets the color.
    pub fn set_color_value(&self, color: Option<C>) {
        self.binding.set(color);
    }

    /// The color to reset to (for form resets).
    pub fn default_color_value(&self) -> Option<C> {
        self.default_color_value.get_value()
    }
}

/// The darkest color of the type: every channel at its minimum (react-aria uses black for an
/// empty field's channel changes).
fn black<C: ColorValue + Default>() -> C {
    C::channels().iter().fold(C::default(), |color, &channel| {
        color.with_channel_value(channel, C::get_channel_range(channel).min_value)
    })
}

/// Creates the state of a field editing one channel of a color (a number field).
pub fn use_color_channel_field_state<C: ColorValue + Default>(
    input: UseColorChannelFieldStateInput<C>,
) -> ColorChannelFieldState<C> {
    let UseColorChannelFieldStateInput {
        default_value,
        value,
        channel,
        is_disabled,
        is_read_only,
        is_invalid,
        validate,
        validation_behavior,
        name,
        on_change,
    } = input;

    let binding = value.unwrap_or_else(|| ValueBinding::from(RwSignal::new(default_value)));
    let default_color_value = StoredValue::new(binding.value.get_untracked());
    let color_value = binding.value;
    let binding = ValueBinding::new(
        color_value,
        Callback::new(move |color: Option<C>| {
            binding.set(color);
            if let Some(on_change) = on_change {
                on_change.run(color);
            }
        }),
    );

    let range = C::get_channel_range(channel);
    let number = use_number_field_state(UseNumberFieldStateInput {
        value: Some(ValueBinding::new(
            Signal::derive(move || color_value.get().map(|c| c.get_channel_value(channel))),
            Callback::new(move |value: Option<f64>| {
                binding.set(value.map(|value| {
                    color_value
                        .get_untracked()
                        .unwrap_or_else(black::<C>)
                        .with_channel_value(channel, value)
                }));
            }),
        )),
        min_value: Signal::stored(Some(range.min_value)),
        max_value: Signal::stored(Some(range.max_value)),
        step: Signal::stored(Some(range.step)),
        format_options: Signal::stored(C::get_channel_format_options(channel)),
        is_disabled,
        is_read_only,
        is_invalid,
        validate,
        validation_behavior,
        name,
        ..UseNumberFieldStateInput::default()
    });

    ColorChannelFieldState {
        color_value,
        channel,
        number,
        binding,
        default_color_value,
    }
}
