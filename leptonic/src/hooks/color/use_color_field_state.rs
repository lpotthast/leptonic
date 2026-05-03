use std::fmt;

use leptos::prelude::*;

use crate::utils::color::RGB8;

// This is based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-stately/color/src/useColorFieldState.ts

// ## INTENTIONAL DEVIATIONS
//
// - Hook-owned state: The hook owns its WriteSignal internally.
//
// - Uses `RGB8` directly instead of a polymorphic Color object.
//   Hex input always operates in RGB space.
//
// - No form validation state integration (useFormValidationState).
//   Can be added later if needed.

const MIN_HEX: u32 = 0x00_00_00;
const MAX_HEX: u32 = 0xFF_FF_FF;

/// Input parameters for `use_color_field_state`.
#[derive(Debug, Clone)]
pub struct UseColorFieldStateInput {
    /// The initial color value (None for empty field).
    pub default_value: Option<RGB8>,

    /// Callback fired when the color changes.
    pub on_change: Option<Callback<Option<RGB8>>>,
}

/// Return value of `use_color_field_state`.
pub struct UseColorFieldStateReturn {
    /// The current text in the input field.
    pub input_value: Signal<String>,

    /// Set the input text (does not parse until commit).
    pub set_input_value: Callback<String>,

    /// The currently parsed color value (None if invalid/empty).
    pub color_value: Signal<Option<RGB8>>,

    /// Set the color value programmatically (also updates input text).
    pub set_color_value: Callback<Option<RGB8>>,

    /// Commit the current input text: parse it and update the color.
    /// Called on blur. Reverts to previous valid text if parsing fails.
    pub commit: Callback<()>,

    /// Validate whether a string is a valid partial or complete hex color.
    pub validate: Callback<String, bool>,

    /// Increment the hex integer value by one step.
    pub increment: Callback<()>,

    /// Decrement the hex integer value by one step.
    pub decrement: Callback<()>,

    /// Jump to the maximum value (#FFFFFF).
    pub increment_to_max: Callback<()>,

    /// Jump to the minimum value (#000000).
    pub decrement_to_min: Callback<()>,
}

impl Clone for UseColorFieldStateReturn {
    fn clone(&self) -> Self {
        Self {
            input_value: self.input_value,
            set_input_value: self.set_input_value,
            color_value: self.color_value,
            set_color_value: self.set_color_value,
            commit: self.commit,
            validate: self.validate,
            increment: self.increment,
            decrement: self.decrement,
            increment_to_max: self.increment_to_max,
            decrement_to_min: self.decrement_to_min,
        }
    }
}

impl fmt::Debug for UseColorFieldStateReturn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseColorFieldStateReturn").finish()
    }
}

/// Check if a string is a valid partial or complete hex color.
fn is_valid_hex_partial(s: &str) -> bool {
    let s = s.strip_prefix('#').unwrap_or(s);
    if s.len() > 6 {
        return false;
    }
    s.chars().all(|c| c.is_ascii_hexdigit())
}

/// Format an `RGB8` as a hex string with # prefix.
fn format_hex(color: RGB8) -> String {
    format!("#{color:X}")
}

/// Creates state for a hex color text input field.
///
/// Manages dual tracking of input text and parsed color value.
/// Validates hex strings during typing and commits on blur.
#[allow(clippy::needless_pass_by_value)]
pub fn use_color_field_state(input: UseColorFieldStateInput) -> UseColorFieldStateReturn {
    let UseColorFieldStateInput {
        default_value,
        on_change,
    } = input;

    let initial_text = default_value.map_or_else(String::new, format_hex);

    let (input_value, set_input_value_signal) = signal(initial_text);
    let (color_value, set_color_value_signal) = signal(default_value);

    let fire_change = move |color: Option<RGB8>| {
        if let Some(cb) = on_change {
            cb.run(color);
        }
    };

    let set_input_value = Callback::new(move |text: String| {
        if is_valid_hex_partial(&text) {
            set_input_value_signal.set(text);
        }
    });

    let set_color_value = Callback::new(move |color: Option<RGB8>| {
        set_color_value_signal.set(color);
        let text = color.map_or_else(String::new, format_hex);
        set_input_value_signal.set(text);
        fire_change(color);
    });

    let commit = Callback::new(move |()| {
        let text = input_value.get_untracked();
        if text.is_empty() {
            set_color_value_signal.set(None);
            fire_change(None);
            return;
        }
        // Try to parse the hex string.
        if let Some(parsed) = RGB8::from_hex(&text) {
            set_color_value_signal.set(Some(parsed));
            set_input_value_signal.set(format_hex(parsed));
            fire_change(Some(parsed));
        } else {
            // Revert to previous valid color text.
            let prev = color_value.get_untracked();
            let text = prev.map_or_else(String::new, format_hex);
            set_input_value_signal.set(text);
        }
    });

    let validate = Callback::new(move |text: String| -> bool { is_valid_hex_partial(&text) });

    let adjust = move |delta: i64| {
        let current = color_value.get_untracked().unwrap_or(RGB8::new());
        let int = i64::from(current.to_hex_int());
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let new_int = (int + delta).clamp(i64::from(MIN_HEX), i64::from(MAX_HEX)) as u32;
        let new_color = RGB8::from_hex_int(new_int);
        set_color_value_signal.set(Some(new_color));
        set_input_value_signal.set(format_hex(new_color));
        fire_change(Some(new_color));
    };

    let increment = Callback::new(move |()| adjust(1));
    let decrement = Callback::new(move |()| adjust(-1));

    let increment_to_max = Callback::new(move |()| {
        let max = RGB8::from_hex_int(MAX_HEX);
        set_color_value_signal.set(Some(max));
        set_input_value_signal.set(format_hex(max));
        fire_change(Some(max));
    });

    let decrement_to_min = Callback::new(move |()| {
        let min = RGB8::from_hex_int(MIN_HEX);
        set_color_value_signal.set(Some(min));
        set_input_value_signal.set(format_hex(min));
        fire_change(Some(min));
    });

    UseColorFieldStateReturn {
        input_value: input_value.into(),
        set_input_value,
        color_value: color_value.into(),
        set_color_value,
        commit,
        validate,
        increment,
        decrement,
        increment_to_max,
        decrement_to_min,
    }
}
