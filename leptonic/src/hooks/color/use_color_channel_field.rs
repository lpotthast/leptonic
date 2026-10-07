// Upstream: react-aria/src/color/useColorChannelField.ts @ 99e6102368
use leptos::prelude::*;

use super::use_color_channel_field_state::ColorChannelFieldState;
use crate::{
    hooks::form::use_number_field::{UseNumberFieldInput, UseNumberFieldReturn, use_number_field},
    utils::color::ColorValue,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Takes the number field's input around the state's number field (react-aria: the props
//   object, minus the value props).
//
// =============================================================================

/// Input of [`use_color_channel_field`]: the state, and the number field's other settings.
#[derive(Clone)]
pub struct UseColorChannelFieldInput<C: ColorValue> {
    pub state: ColorChannelFieldState<C>,
    /// The number field's settings (its `state` is replaced by the channel's).
    pub field: UseNumberFieldInput<f64>,
}

/// Behavior and accessibility of a field editing one channel of a color: a number field whose
/// label defaults to the channel's name when there is no other label.
pub fn use_color_channel_field<C: ColorValue>(
    input: UseColorChannelFieldInput<C>,
) -> UseNumberFieldReturn {
    let UseColorChannelFieldInput { state, field } = input;
    let channel = state.channel;
    let aria_label = field.aria_label;
    let has_label = field.has_label;
    let has_labelledby = field.aria_labelledby.is_some();
    use_number_field(UseNumberFieldInput {
        state: state.number,
        aria_label: MaybeProp::derive(move || {
            aria_label.get().or_else(|| {
                (!has_label.get() && !has_labelledby).then(|| C::channel_name(channel).to_owned())
            })
        }),
        ..field
    })
}
