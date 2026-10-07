// Upstream: react-stately/src/color/useColorFieldState.ts @ 99e6102368
use leptos::prelude::*;

use crate::{
    hooks::form::use_form_validation_state::{
        UseFormValidationStateInput, UseFormValidationStateReturn, ValidateFn, ValidationBehavior,
        use_form_validation_state,
    },
    utils::{ValueBinding, color::RGB8},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The color is an `RGB8` (hex input is always RGB; react-aria: any `Color`).
// - Hook-owned value (C4): `default_value` + `on_change`, or `value` bound to app state.
// - A `Copy` struct with signals and methods (C3).
//
// =============================================================================

const MIN_COLOR: RGB8 = RGB8::from_hex_int(0x00_00_00);
const MAX_COLOR: RGB8 = RGB8::from_hex_int(0xFF_FF_FF);

/// Input of [`use_color_field_state`].
#[derive(Clone, Default)]
pub struct UseColorFieldStateInput {
    /// The initial color (`None`: empty).
    pub default_value: Option<RGB8>,
    /// The color as app state, replacing `default_value`.
    pub value: Option<ValueBinding<Option<RGB8>>>,
    pub is_invalid: Signal<bool>,
    pub validate: Option<ValidateFn<Option<RGB8>>>,
    pub validation_behavior: ValidationBehavior,
    /// The field's name, matching server errors.
    pub name: Option<String>,
    /// Called with the committed color.
    pub on_change: Option<Callback<Option<RGB8>>>,
}

/// The state of a hex color field: the typed text and the committed color.
#[derive(Clone, Copy)]
pub struct ColorFieldState {
    /// The text in the field.
    pub input_value: Signal<String>,
    /// The committed color.
    pub color_value: Signal<Option<RGB8>>,
    pub validation: UseFormValidationStateReturn,
    pub validation_behavior: ValidationBehavior,
    text: RwSignal<String>,
    binding: ValueBinding<Option<RGB8>>,
    is_bound: bool,
    default_color_value: StoredValue<Option<RGB8>>,
}

/// The `#RRGGBB` text of a color.
fn hex(color: Option<RGB8>) -> String {
    color.map_or_else(String::new, |color| format!("#{color:X}"))
}

impl ColorFieldState {
    /// The color to reset to (for form resets).
    pub fn default_color_value(&self) -> Option<RGB8> {
        self.default_color_value.get_value()
    }

    /// Sets the committed color (the text follows).
    pub fn set_color_value(&self, color: Option<RGB8>) {
        self.binding.set(color);
    }

    /// Sets the text (validate it first, see [`ColorFieldState::validate`]).
    pub fn set_input_value(&self, text: String) {
        self.text.set(text);
    }

    /// Whether `text` may be typed: empty, or up to six hex digits after an optional `#`.
    pub fn validate(&self, text: &str) -> bool {
        let digits = text.strip_prefix('#').unwrap_or(text);
        digits.len() <= 6 && digits.chars().all(|c| c.is_ascii_hexdigit())
    }

    /// The color of the typed text.
    fn parsed_value(&self) -> Option<RGB8> {
        self.text.with_untracked(|text| RGB8::from_hex(text))
    }

    /// Sets the color only if it is a different one.
    fn safely_set_color_value(&self, color: Option<RGB8>) {
        let current = self.color_value.get_untracked();
        let changed = match (current, color) {
            (Some(current), Some(color)) => current.to_hex_int() != color.to_hex_int(),
            _ => true,
        };
        if changed {
            self.set_color_value(color);
        }
    }

    /// Commits the text: an empty text clears the color, an invalid one is replaced by the
    /// current color's text.
    pub fn commit(&self) {
        let current = self.color_value.get_untracked();
        if self.text.with_untracked(String::is_empty) {
            self.safely_set_color_value(None);
            // Bound to a color, the text shows it again (react-aria).
            self.text.set(if self.is_bound {
                hex(current)
            } else {
                String::new()
            });
            return;
        }
        let Some(parsed) = self.parsed_value() else {
            self.text.set(hex(current));
            return;
        };
        self.safely_set_color_value(Some(parsed));
        // Bound app state may keep its color: show what it holds.
        self.text.set(hex(self.color_value.get_untracked()));
        self.validation.commit_validation.run(());
    }

    fn step(&self, delta: i64) {
        let color = self.parsed_value().unwrap_or(MIN_COLOR);
        let int = i64::from(color.to_hex_int());
        let clamped = (int + delta).clamp(
            i64::from(MIN_COLOR.to_hex_int()),
            i64::from(MAX_COLOR.to_hex_int()),
        );
        let new_color = u32::try_from(clamped).map_or(color, RGB8::from_hex_int);
        // The same color as before: the text may show something else (react-aria).
        if Some(new_color) == self.color_value.get_untracked() {
            self.text.set(hex(Some(new_color)));
        }
        self.safely_set_color_value(Some(new_color));
        self.validation.commit_validation.run(());
    }

    /// The typed color plus one.
    pub fn increment(&self) {
        self.step(1);
    }

    /// The typed color minus one.
    pub fn decrement(&self) {
        self.step(-1);
    }

    /// White.
    pub fn increment_to_max(&self) {
        self.safely_set_color_value(Some(MAX_COLOR));
    }

    /// Black.
    pub fn decrement_to_min(&self) {
        self.safely_set_color_value(Some(MIN_COLOR));
    }
}

/// Creates the state of a field for a color as hex text.
pub fn use_color_field_state(input: UseColorFieldStateInput) -> ColorFieldState {
    let UseColorFieldStateInput {
        default_value,
        value,
        is_invalid,
        validate,
        validation_behavior,
        name,
        on_change,
    } = input;

    let is_bound = value.is_some();
    let binding = value.unwrap_or_else(|| ValueBinding::from(RwSignal::new(default_value)));
    let default_color_value = StoredValue::new(binding.value.get_untracked());
    let color_value = binding.value;
    let binding = ValueBinding::new(
        color_value,
        Callback::new(move |color: Option<RGB8>| {
            binding.set(color);
            if let Some(on_change) = on_change {
                on_change.run(color);
            }
        }),
    );

    let text = RwSignal::new(hex(color_value.get_untracked()));
    // The text follows the color (also when it changes from outside).
    Effect::new(move |previous: Option<()>| {
        let color = color_value.get();
        if previous.is_some() {
            text.set(hex(color));
        }
    });

    let validation = use_form_validation_state(UseFormValidationStateInput {
        builtin_validation: Signal::default(),
        is_invalid,
        value: color_value,
        validate,
        validation_behavior,
        name,
    });

    ColorFieldState {
        input_value: text.into(),
        color_value,
        validation,
        validation_behavior,
        text,
        binding,
        is_bound,
        default_color_value,
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    fn field(color: Option<RGB8>) -> ColorFieldState {
        use_color_field_state(UseColorFieldStateInput {
            default_value: color,
            ..UseColorFieldStateInput::default()
        })
    }

    #[test]
    fn commits_typed_hex_and_shorthand() {
        Owner::new().with(|| {
            let state = field(None);
            state.set_input_value("f0a".to_owned());
            state.commit();
            assert_that!(state.color_value.get_untracked()).is_equal_to(Some(RGB8 {
                r: 0xFF,
                g: 0x00,
                b: 0xAA,
            }));
            assert_that!(state.input_value.get_untracked()).is_equal_to("#FF00AA".to_owned());
        });
    }

    #[test]
    fn invalid_text_reverts_and_empty_text_clears() {
        Owner::new().with(|| {
            let state = field(Some(RGB8 { r: 1, g: 2, b: 3 }));
            state.set_input_value("12".to_owned());
            state.commit();
            assert_that!(state.input_value.get_untracked()).is_equal_to("#010203".to_owned());
            state.set_input_value(String::new());
            state.commit();
            assert_that!(state.color_value.get_untracked()).is_none();
        });
    }

    #[test]
    fn steps_from_the_typed_color_within_bounds() {
        Owner::new().with(|| {
            let state = field(Some(RGB8 { r: 0, g: 0, b: 0 }));
            state.set_input_value("#0000FE".to_owned());
            state.increment();
            assert_that!(state.color_value.get_untracked()).is_equal_to(Some(RGB8 {
                r: 0,
                g: 0,
                b: 0xFF,
            }));
            // At the maximum, incrementing stays there (the text follows the color in an effect,
            // which unit tests don't run: a fresh field).
            let at_max = field(Some(MAX_COLOR));
            at_max.increment();
            assert_that!(at_max.color_value.get_untracked()).is_equal_to(Some(MAX_COLOR));
            assert_that!(state.validate("#12345")).is_true();
            assert_that!(state.validate("#1234567")).is_false();
            assert_that!(state.validate("xyz")).is_false();
        });
    }
}
