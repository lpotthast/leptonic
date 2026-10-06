// Upstream: react-aria/src/color/useColorChannelField.ts @ 6f664fe911
use leptos::prelude::*;

use super::use_color_channel_field_state::UseColorChannelFieldStateReturn;
use crate::{
    hooks::form::{
        use_form_validation_state::ValidationBehavior,
        use_number_field::{UseNumberFieldInput, UseNumberFieldReturn, use_number_field},
        use_number_field_state::{UseNumberFieldStateInput, use_number_field_state},
    },
    utils::{ValueBinding, color::ColorValue},
};

// This is based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/react-aria/src/color/useColorChannelField.ts

// ## INTENTIONAL DEVIATIONS
//
// - Delegates to `use_number_field_state` (the channel value bound as its value) +
//   `use_number_field` with channel-derived parameters.
//   Auto-generates aria-label from channel name if none provided.

/// Input parameters for `use_color_channel_field`.
#[derive(Clone)]
pub struct UseColorChannelFieldInput<C: ColorValue> {
    /// The channel field state (from `use_color_channel_field_state`).
    pub state: UseColorChannelFieldStateReturn<C>,

    /// Whether the field is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the field is read-only.
    pub is_read_only: Signal<bool>,

    /// An optional aria-label. If not provided, the channel name is used.
    pub aria_label: Option<&'static str>,
}

/// Creates behavior and ARIA props for a single-channel numeric input.
///
/// This is a thin wrapper around `use_number_field_state` + `use_number_field`
/// that provides channel-specific default labels and range parameters.
pub fn use_color_channel_field<C: ColorValue>(
    input: UseColorChannelFieldInput<C>,
) -> UseNumberFieldReturn {
    let UseColorChannelFieldInput {
        state,
        is_disabled,
        is_read_only,
        aria_label,
    } = input;

    let label = aria_label.unwrap_or_else(|| C::get_channel_name(state.channel));

    // The number field edits the channel value directly.
    let number_state = use_number_field_state(UseNumberFieldStateInput {
        value: Some(ValueBinding::new(
            state.channel_value,
            state.set_channel_value,
        )),
        min_value: Signal::stored(Some(state.min_value)),
        max_value: Signal::stored(Some(state.max_value)),
        step: Signal::stored(Some(state.step)),
        is_disabled,
        is_read_only,
        validation_behavior: ValidationBehavior::Aria,
        ..UseNumberFieldStateInput::default()
    });

    use_number_field(UseNumberFieldInput {
        aria_label: label.into(),
        ..UseNumberFieldInput::new(number_state)
    })
}
