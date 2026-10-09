// Upstream: react-aria/src/color/useColorChannelField.ts @ 99e6102368
// Upstream: @adobe/react-spectrum/test/color/ColorField.test.js @ 99e6102368
use leptos::prelude::*;
use web_sys::FocusEvent;

use super::use_color_channel_field_state::ColorChannelFieldState;
use crate::{
    hooks::{
        form::use_number_field::{UseNumberFieldInput, UseNumberFieldReturn, use_number_field},
        interactions::use_keyboard::KeyboardEventWrapper,
    },
    utils::{color::ColorValue, i18n::use_locale},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Takes the state and the number field's settings (react-aria: the props object, minus the
//   value props); the number field's state is the channel state's.
//
// =============================================================================

/// Input of [`use_color_channel_field`]: the state, and the settings of its number field.
#[derive(Clone)]
pub struct UseColorChannelFieldInput<C: ColorValue> {
    pub state: ColorChannelFieldState<C>,
    /// The input's id. Generated when `None`.
    pub id: Option<String>,
    /// Whether a visible label is rendered (with `label_props`).
    pub has_label: Signal<bool>,
    /// Names the field. Without any label, the channel's name does.
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Option<String>,
    pub is_required: Signal<bool>,
    pub placeholder: MaybeProp<String>,
    pub auto_focus: bool,
    /// Whether the scroll wheel leaves the value alone (it steps while the field has focus).
    pub is_wheel_disabled: bool,
    /// Replaces "Increase `<field label>`".
    pub increment_aria_label: MaybeProp<String>,
    /// Replaces "Decrease `<field label>`".
    pub decrement_aria_label: MaybeProp<String>,
    pub on_focus: Option<Callback<FocusEvent>>,
    pub on_blur: Option<Callback<FocusEvent>>,
    pub on_focus_change: Option<Callback<bool>>,
    pub on_key_down: Option<Callback<KeyboardEventWrapper>>,
    pub on_key_up: Option<Callback<KeyboardEventWrapper>>,
}

/// Behavior and accessibility of a field editing one channel of a color: a number field whose
/// label defaults to the channel's name when there is no other label.
pub fn use_color_channel_field<C: ColorValue>(
    input: UseColorChannelFieldInput<C>,
) -> UseNumberFieldReturn {
    let UseColorChannelFieldInput {
        state,
        id,
        has_label,
        aria_label,
        aria_labelledby,
        aria_describedby,
        is_required,
        placeholder,
        auto_focus,
        is_wheel_disabled,
        increment_aria_label,
        decrement_aria_label,
        on_focus,
        on_blur,
        on_focus_change,
        on_key_down,
        on_key_up,
    } = input;
    let channel = state.channel;
    let has_labelledby = aria_labelledby.is_some();
    let locale = use_locale();
    use_number_field(UseNumberFieldInput {
        state: state.number,
        id,
        has_label,
        aria_label: MaybeProp::derive(move || {
            aria_label.get().or_else(|| {
                (!has_label.get() && !has_labelledby)
                    .then(|| C::channel_name(channel, &locale.get()))
            })
        }),
        aria_labelledby,
        aria_describedby,
        is_required,
        placeholder,
        auto_focus,
        is_wheel_disabled,
        increment_aria_label,
        decrement_aria_label,
        on_focus,
        on_blur,
        on_focus_change,
        on_key_down,
        on_key_up,
    })
}
