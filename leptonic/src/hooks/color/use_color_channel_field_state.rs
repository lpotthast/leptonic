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

/// Input of [`use_color_channel_field_state`].
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
        color.with_channel_value(channel, C::channel_range(channel).min_value)
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

    let range = C::channel_range(channel);
    let number = use_number_field_state(UseNumberFieldStateInput {
        value: Some(ValueBinding::new(
            Signal::derive(move || color_value.get().map(|c| c.channel_value(channel))),
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
        format_options: Signal::stored(C::channel_format_options(channel)),
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

#[cfg(test)]
mod tests {
    // Upstream: @adobe/react-spectrum/test/color/ColorField.test.js ("channel") and
    // react-aria-components/test/ColorField.test.js ("should support the channel prop").
    use assertr::prelude::*;

    use super::*;
    use crate::{
        testing::{flush_effects, with_owner},
        utils::color::{HSL, HslChannel, RGB8, RgbChannel},
    };

    /// The colors `on_change` was called with.
    fn recorder<C: ColorValue>() -> (RwSignal<Vec<Option<C>>>, Callback<Option<C>>) {
        let calls = RwSignal::new(Vec::new());
        let callback = Callback::new(move |color| calls.update(|c| c.push(color)));
        (calls, callback)
    }

    fn input<C: ColorValue>(
        default_value: Option<C>,
        channel: C::Channel,
    ) -> UseColorChannelFieldStateInput<C> {
        UseColorChannelFieldStateInput {
            default_value,
            value: None,
            channel,
            is_disabled: Signal::default(),
            is_read_only: Signal::default(),
            is_invalid: Signal::default(),
            validate: None,
            validation_behavior: ValidationBehavior::Aria,
            name: None,
            on_change: None,
        }
    }

    /// `#abc` in HSL: hsl(210, 25%, 73.33%).
    fn abc() -> HSL {
        RGB8 {
            r: 0xaa,
            g: 0xbb,
            b: 0xcc,
        }
        .into()
    }

    fn type_and_commit<C: ColorValue>(state: &ColorChannelFieldState<C>, text: &str) {
        state.number.set_input_value(text.to_owned());
        state.number.commit(None);
    }

    fn text<C: ColorValue>(state: &ColorChannelFieldState<C>) -> String {
        state.number.input_value.get_untracked()
    }

    // Upstream: "should support the channel prop".
    #[test]
    fn shows_the_channel_and_commits_a_typed_value() {
        with_owner(|| {
            let (changes, on_change) = recorder();
            let state = use_color_channel_field_state(UseColorChannelFieldStateInput {
                on_change: Some(on_change),
                ..input(Some(abc()), HslChannel::Hue)
            });
            assert_that!(text(&state)).is_equal_to("210°".to_owned());

            type_and_commit(&state, "100");
            let expected = HSL {
                hue: 100.0,
                saturation: 0.25,
                lightness: 0.7333,
            };
            assert_that!(changes.get_untracked()).is_equal_to(vec![Some(expected)]);
            assert_that!(state.color_value.get_untracked()).is_equal_to(Some(expected));
            assert_that!(text(&state)).is_equal_to("100°".to_owned());
        });
    }

    // Upstream: "should default to empty", "should support null value".
    #[test]
    fn defaults_to_empty() {
        with_owner(|| {
            let state = use_color_channel_field_state(input::<HSL>(None, HslChannel::Hue));
            assert_that!(text(&state)).is_equal_to(String::new());
            assert_that!(state.color_value.get_untracked()).is_none();
            assert_that!(state.number.value().get_untracked()).is_none();
        });
    }

    // Upstream: "should support clearing value".
    #[test]
    fn clearing_the_field_clears_the_color() {
        with_owner(|| {
            let (changes, on_change) = recorder();
            let state = use_color_channel_field_state(UseColorChannelFieldStateInput {
                on_change: Some(on_change),
                ..input(Some(abc()), HslChannel::Hue)
            });
            type_and_commit(&state, "");
            assert_that!(text(&state)).is_equal_to(String::new());
            assert_that!(state.color_value.get_untracked()).is_none();
            assert_that!(changes.get_untracked()).is_equal_to(vec![None]);
        });
    }

    // Upstream: `useConvertColor`'s black for an empty field.
    #[test]
    fn a_value_typed_into_an_empty_field_changes_black() {
        with_owner(|| {
            let state = use_color_channel_field_state(input::<RGB8>(None, RgbChannel::Red));
            type_and_commit(&state, "255");
            assert_that!(state.color_value.get_untracked()).is_equal_to(Some(RGB8 {
                r: 255,
                g: 0,
                b: 0,
            }));
        });
    }

    // Upstream (`useControlledState`): no change, no `onChange`.
    #[test]
    fn committing_the_same_value_reports_no_change() {
        with_owner(|| {
            let (changes, on_change) = recorder();
            let state = use_color_channel_field_state(UseColorChannelFieldStateInput {
                on_change: Some(on_change),
                ..input(Some(abc()), HslChannel::Hue)
            });
            type_and_commit(&state, "210");
            assert_that!(changes.get_untracked()).is_empty();
            assert_that!(text(&state)).is_equal_to("210°".to_owned());
        });
    }

    // Upstream: the channel's range, step and percent format (react-aria's `multiplier`; here
    // the channel is 0 to 1 already).
    #[test]
    fn percent_channels_step_by_a_percent_within_the_range() {
        with_owner(|| {
            let state = use_color_channel_field_state(input(Some(abc()), HslChannel::Saturation));
            assert_that!(text(&state)).is_equal_to("25%".to_owned());
            state.number.increment();
            assert_that!(text(&state)).is_equal_to("26%".to_owned());
            assert_that!(state.color_value.get_untracked().map(|c| c.saturation))
                .is_equal_to(Some(0.26));

            type_and_commit(&state, "150%");
            assert_that!(text(&state)).is_equal_to("100%".to_owned());
            assert_that!(state.color_value.get_untracked().map(|c| c.saturation))
                .is_equal_to(Some(1.0));
        });
    }

    // Upstream: "supports form reset".
    #[test]
    fn a_bound_color_follows_the_app_state_and_resets() {
        with_owner(|| {
            let yellow = RGB8 {
                r: 255,
                g: 255,
                b: 0,
            };
            let app = RwSignal::new(Some(yellow));
            let state = use_color_channel_field_state(UseColorChannelFieldStateInput {
                value: Some(ValueBinding::from(app)),
                ..input(None, RgbChannel::Red)
            });
            flush_effects();
            assert_that!(text(&state)).is_equal_to("255".to_owned());

            type_and_commit(&state, "0");
            assert_that!(app.get_untracked()).is_equal_to(Some(RGB8 { r: 0, ..yellow }));
            assert_that!(text(&state)).is_equal_to("0".to_owned());

            // The form reset: back to the initial color.
            assert_that!(state.default_color_value()).is_equal_to(Some(yellow));
            state.set_color_value(state.default_color_value());
            flush_effects();
            assert_that!(app.get_untracked()).is_equal_to(Some(yellow));
            assert_that!(text(&state)).is_equal_to("255".to_owned());

            // Changed by the app: the text follows.
            app.set(Some(RGB8 { r: 7, ..yellow }));
            flush_effects();
            assert_that!(text(&state)).is_equal_to("7".to_owned());
        });
    }
}
