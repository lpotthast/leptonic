// Upstream: react-stately/src/color/useColorPickerState.ts @ 99e6102368
use leptos::prelude::*;

use crate::utils::{ValueBinding, color::Color};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Hook-owned value (C4): `default_value` + `on_change`, or `value` bound to app state.
// - A `Copy` struct with a signal and a method (C3).
//
// =============================================================================

/// Input of [`use_color_picker_state`].
#[derive(Debug, Clone, Default)]
pub struct UseColorPickerStateInput {
    /// The initial color. Default: black.
    pub default_value: Color,
    /// The color as app state, replacing `default_value`.
    pub value: Option<ValueBinding<Color>>,
    /// Called with every new color.
    pub on_change: Option<Callback<Color>>,
}

/// The color that a color picker's parts share.
#[derive(Debug, Clone, Copy)]
pub struct ColorPickerState {
    /// The color, in the space it was last set in.
    pub color: Signal<Color>,
    binding: ValueBinding<Color>,
    on_change: Option<Callback<Color>>,
}

impl ColorPickerState {
    /// Sets the color.
    pub fn set_color(&self, color: Color) {
        self.binding.set(color);
        if let Some(on_change) = self.on_change {
            on_change.run(color);
        }
    }
}

/// Creates the state of a color picker: one color its parts (area, sliders, fields, swatches)
/// show and change.
pub fn use_color_picker_state(input: UseColorPickerStateInput) -> ColorPickerState {
    let UseColorPickerStateInput {
        default_value,
        value,
        on_change,
    } = input;
    let binding = value.unwrap_or_else(|| ValueBinding::from(RwSignal::new(default_value)));
    ColorPickerState {
        color: binding.value,
        binding,
        on_change,
    }
}
