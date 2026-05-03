use std::fmt;

use leptos::prelude::*;

use crate::utils::color::ColorValue;

// This is based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-stately/color/src/useColorPickerState.ts

// ## INTENTIONAL DEVIATIONS
//
// - Hook-owned state: The hook owns its WriteSignal internally and exposes a
//   read-only Signal<C>. React-aria uses useControlledState for
//   controlled/uncontrolled support.

/// Input parameters for `use_color_picker_state`.
#[derive(Debug, Clone)]
pub struct UseColorPickerStateInput<C: ColorValue> {
    /// The initial color value.
    pub default_value: C,

    /// Callback fired when the color changes.
    pub on_change: Option<Callback<C>>,
}

/// Return value of `use_color_picker_state`.
pub struct UseColorPickerStateReturn<C: ColorValue> {
    /// The current color (read-only).
    pub color: Signal<C>,

    /// Update the color value.
    pub set_color: Callback<C>,
}

impl<C: ColorValue> Clone for UseColorPickerStateReturn<C> {
    fn clone(&self) -> Self {
        Self {
            color: self.color,
            set_color: self.set_color,
        }
    }
}

impl<C: ColorValue> fmt::Debug for UseColorPickerStateReturn<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseColorPickerStateReturn")
            .field("color", &self.color)
            .finish_non_exhaustive()
    }
}

/// Creates top-level state for a color picker.
///
/// This is the simplest state hook — it wraps a single color value and
/// provides a setter callback. Typically used to compose with more specialized
/// hooks like `use_color_area_state` or `use_color_slider_state`.
pub fn use_color_picker_state<C: ColorValue>(
    input: &UseColorPickerStateInput<C>,
) -> UseColorPickerStateReturn<C> {
    let UseColorPickerStateInput {
        default_value,
        on_change,
    } = *input;

    let (color, set_color_signal) = signal(default_value);

    let set_color = Callback::new(move |new_color: C| {
        set_color_signal.set(new_color);
        if let Some(cb) = on_change {
            cb.run(new_color);
        }
    });

    UseColorPickerStateReturn {
        color: color.into(),
        set_color,
    }
}
