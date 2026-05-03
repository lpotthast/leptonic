use leptos::prelude::*;

use crate::{
    hooks::form::{
        use_form_validation_state::ValidationBehavior,
        use_number_field::{UseNumberFieldInput, UseNumberFieldReturn, use_number_field},
        use_number_field_state::{UseNumberFieldStateInput, use_number_field_state},
    },
    utils::{
        color::ColorValue, i18n::use_locale_or_default, number_formatter::NumberFormatOptions,
    },
};

use super::use_color_channel_field_state::UseColorChannelFieldStateReturn;

// This is based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/color/src/useColorChannelField.ts

// ## INTENTIONAL DEVIATIONS
//
// - Delegates to `use_number_field_state` + `use_number_field` with
//   channel-derived parameters.
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
    let locale = use_locale_or_default();

    // Create a number field state that syncs with the color channel state.
    let initial_value = state.channel_value.get_untracked();

    let set_channel_value = state.set_channel_value;
    let number_state = use_number_field_state(UseNumberFieldStateInput {
        default_value: initial_value,
        min_value: Some(state.min_value),
        max_value: Some(state.max_value),
        step: state.step,
        format_options: NumberFormatOptions::default(),
        locale,
        is_disabled,
        is_read_only,
        on_change: Some(Callback::new(move |val: Option<f64>| {
            set_channel_value.run(val);
        })),
    });

    // Sync external color changes back into the number field.
    // Uses an Effect because `use_number_field_state` owns its internal
    // signals — there is no way to derive them from an external source.
    // The equality guard prevents reactive loops: number field on_change
    // → set_channel_value → channel_value updates → Effect fires →
    // set_number_value → but value is unchanged → no-op.
    let set_number_value = number_state.set_number_value;
    let number_value = number_state.number_value;
    Effect::new(move |_| {
        let channel_val = state.channel_value.get();
        if channel_val != number_value.get_untracked() {
            set_number_value.run(channel_val);
        }
    });

    use_number_field(UseNumberFieldInput {
        state: number_state,
        on_focus: None,
        on_blur: None,
        is_disabled,
        is_read_only,
        is_required: false,
        is_invalid: None,
        validate: None,
        validation_behavior: ValidationBehavior::Aria,
        default_value: None,
        placeholder: None,
        aria_label: Some(label),
        name: None,
        label: None,
        description: None,
        min_value: Some(state.min_value),
        max_value: Some(state.max_value),
        step: state.step,
        auto_focus: false,
        is_wheel_disabled: false,
        increment_aria_label: None,
        decrement_aria_label: None,
    })
}
