// Upstream: react-stately/src/color/useColorFieldState.ts @ 99e6102368
// Upstream: react-stately/test/color/useColorFieldState.test.js @ 99e6102368
use leptos::prelude::*;

use crate::{
    ValueBinding,
    hooks::form::use_form_validation_state::{
        FormValidationState, UseFormValidationStateInput, ValidateFn, ValidationBehavior,
        use_form_validation_state,
    },
    utils::color::{Color, ColorValue, RGB8},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Generic over the color type (`ColorValue`, default `RGB8`), as react-aria's field takes any
//   color: the text is the color's hex form, and typed hex is converted into the type.
// - Hook-owned value (C4): `default_value` + `on_change`, or `value` bound to app state.
// - A `Copy` struct with signals and methods (C3).
//
// =============================================================================

const MIN_COLOR: RGB8 = RGB8::from_hex_int(0x00_00_00);
const MAX_COLOR: RGB8 = RGB8::from_hex_int(0xFF_FF_FF);

/// Input of [`use_color_field_state`].
#[derive(Clone)]
pub struct UseColorFieldStateInput<C: ColorValue = RGB8> {
    /// The initial color (`None`: empty).
    pub default_value: Option<C>,
    /// The color as app state, replacing `default_value`.
    pub value: Option<ValueBinding<Option<C>>>,
    pub is_invalid: Signal<bool>,
    pub validate: Option<ValidateFn<Option<C>>>,
    pub validation_behavior: ValidationBehavior,
    /// The field's name, matching server errors.
    pub name: Option<String>,
    /// Called with the committed color.
    pub on_change: Option<Callback<Option<C>>>,
}

impl<C: ColorValue> Default for UseColorFieldStateInput<C> {
    fn default() -> Self {
        Self {
            default_value: None,
            value: None,
            is_invalid: Signal::default(),
            validate: None,
            validation_behavior: ValidationBehavior::default(),
            name: None,
            on_change: None,
        }
    }
}

/// The state of a hex color field: the typed text and the committed color.
#[derive(Clone, Copy)]
pub struct ColorFieldState<C: ColorValue = RGB8> {
    /// The text in the field.
    pub input_value: Signal<String>,
    /// The committed color.
    pub color_value: Signal<Option<C>>,
    pub validation: FormValidationState,
    pub validation_behavior: ValidationBehavior,
    text: RwSignal<String>,
    binding: ValueBinding<Option<C>>,
    is_bound: bool,
    default_color_value: StoredValue<Option<C>>,
}

/// The `#RRGGBB` text of a color.
fn hex<C: ColorValue>(color: Option<C>) -> String {
    color.map_or_else(String::new, |color| color.to_rgb8().to_string())
}

/// The color of type `C` of an RGB color.
fn from_rgb<C: ColorValue>(color: RGB8) -> C {
    C::from(Color::from(color))
}

impl<C: ColorValue> ColorFieldState<C> {
    /// The color to reset to (for form resets).
    pub fn default_color_value(&self) -> Option<C> {
        self.default_color_value.get_value()
    }

    /// Sets the committed color (the text follows).
    pub fn set_color_value(&self, color: Option<C>) {
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

    /// The color of the typed text: hex digits with or without `#`, three or six of them, or
    /// four (`#rgba`, with alpha; react-stately's `parseColor`).
    fn parsed_value(&self) -> Option<Color> {
        self.text.with_untracked(|text| {
            if text.starts_with('#') {
                text.parse::<Color>()
            } else {
                format!("#{text}").parse::<Color>()
            }
            .ok()
        })
    }

    /// Sets the color only if it is a different one (by its hex value; react-stately's
    /// `safelySetColorValue`, and `useControlledState` reports no change from empty to empty).
    fn safely_set_color_value(&self, color: Option<C>) {
        let current = self.color_value.get_untracked();
        let changed = match (current, color) {
            (Some(current), Some(color)) => {
                current.to_rgb8().to_hex_int() != color.to_rgb8().to_hex_int()
            }
            (None, None) => false,
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
        self.safely_set_color_value(Some(C::from(parsed)));
        // Bound app state may keep its color: show what it holds.
        self.text.set(hex(self.color_value.get_untracked()));
        self.validation.commit_validation();
    }

    fn step(&self, delta: i64) {
        let color = self.parsed_value().map_or(MIN_COLOR, Color::to::<RGB8>);
        let int = i64::from(color.to_hex_int());
        let clamped = (int + delta).clamp(
            i64::from(MIN_COLOR.to_hex_int()),
            i64::from(MAX_COLOR.to_hex_int()),
        );
        let new_color = u32::try_from(clamped).map_or(color, RGB8::from_hex_int);
        // The same color as before: the text may show something else (react-aria).
        if self
            .color_value
            .get_untracked()
            .map(|color| color.to_rgb8())
            == Some(new_color)
        {
            self.text.set(hex(Some(new_color)));
        }
        self.safely_set_color_value(Some(from_rgb(new_color)));
        self.validation.commit_validation();
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
        self.safely_set_color_value(Some(from_rgb(MAX_COLOR)));
    }

    /// Black.
    pub fn decrement_to_min(&self) {
        self.safely_set_color_value(Some(from_rgb(MIN_COLOR)));
    }
}

/// Creates the state of a field for a color as hex text.
pub fn use_color_field_state<C: ColorValue>(
    input: UseColorFieldStateInput<C>,
) -> ColorFieldState<C> {
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
        Callback::new(move |color: Option<C>| {
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
        names: name.into_iter().collect(),
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
    use crate::{
        testing::{flush_effects, with_owner},
        utils::color::{Alpha, HSV},
    };

    const AABBCC: RGB8 = RGB8::from_hex_int(0xAA_BB_CC);

    fn field(color: Option<RGB8>) -> ColorFieldState {
        use_color_field_state(UseColorFieldStateInput {
            default_value: color,
            ..UseColorFieldStateInput::default()
        })
    }

    /// A field whose `on_change` calls are recorded.
    fn recorded(input: UseColorFieldStateInput) -> (ColorFieldState, RwSignal<Vec<Option<RGB8>>>) {
        let changes = RwSignal::new(Vec::new());
        let state = use_color_field_state(UseColorFieldStateInput {
            on_change: Some(Callback::new(move |color| {
                changes.update(|changes| changes.push(color));
            })),
            ..input
        });
        // The Effects' first runs (the text follows color changes after them).
        flush_effects();
        (state, changes)
    }

    fn uncontrolled(color: Option<RGB8>) -> (ColorFieldState, RwSignal<Vec<Option<RGB8>>>) {
        recorded(UseColorFieldStateInput {
            default_value: color,
            ..UseColorFieldStateInput::default()
        })
    }

    #[test]
    fn commits_typed_hex_and_shorthand() {
        with_owner(|| {
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

    /// Upstream: "should be in empty state if no initial value is provided".
    #[test]
    fn without_a_color_the_field_is_empty() {
        with_owner(|| {
            let state = field(None);
            assert_that!(state.color_value.get_untracked()).is_none();
            assert_that!(state.input_value.get_untracked()).is_equal_to(String::new());
        });
    }

    /// Upstream: "should accept 6-length hex string as value" (controlled), and the other
    /// controlled variants: the text is the bound color's hex.
    #[test]
    fn a_bound_color_shows_as_hex() {
        with_owner(|| {
            let state = use_color_field_state(UseColorFieldStateInput {
                value: Some(ValueBinding::from(RwSignal::new(Some(AABBCC)))),
                ..UseColorFieldStateInput::default()
            });
            assert_that!(state.input_value.get_untracked()).is_equal_to("#AABBCC".to_owned());
        });
    }

    /// Upstream: "should increment not increment beyond max value", "should not call onChange
    /// on increment when value is already at max".
    #[test]
    fn incrementing_white_stays_white_without_a_change() {
        with_owner(|| {
            let (state, changes) = uncontrolled(Some(MAX_COLOR));
            state.increment();
            flush_effects();
            assert_that!(state.color_value.get_untracked()).is_equal_to(Some(MAX_COLOR));
            assert_that!(state.input_value.get_untracked()).is_equal_to("#FFFFFF".to_owned());
            assert_that!(changes.get_untracked()).is_empty();
        });
    }

    /// Upstream: "should incrementToMax increment to max value", "should not call onChange on
    /// incrementToMax when value is already at max".
    #[test]
    fn increment_to_max_sets_white_once() {
        with_owner(|| {
            let (state, changes) = uncontrolled(Some(AABBCC));
            state.increment_to_max();
            flush_effects();
            assert_that!(state.input_value.get_untracked()).is_equal_to("#FFFFFF".to_owned());
            state.increment_to_max();
            assert_that!(changes.get_untracked()).is_equal_to(vec![Some(MAX_COLOR)]);
        });
    }

    /// Upstream: "should decrement not decrement beyond min value", "should not call onChange
    /// on decrement when value is already at min".
    #[test]
    fn decrementing_black_stays_black_without_a_change() {
        with_owner(|| {
            let (state, changes) = uncontrolled(Some(MIN_COLOR));
            state.decrement();
            flush_effects();
            assert_that!(state.color_value.get_untracked()).is_equal_to(Some(MIN_COLOR));
            assert_that!(state.input_value.get_untracked()).is_equal_to("#000000".to_owned());
            assert_that!(changes.get_untracked()).is_empty();
        });
    }

    /// Upstream: "should decrementToMin decrement to min value", "should not call onChange on
    /// decrementToMin when value is already at min".
    #[test]
    fn decrement_to_min_sets_black_once() {
        with_owner(|| {
            let (state, changes) = uncontrolled(Some(AABBCC));
            state.decrement_to_min();
            flush_effects();
            assert_that!(state.input_value.get_untracked()).is_equal_to("#000000".to_owned());
            state.decrement_to_min();
            assert_that!(changes.get_untracked()).is_equal_to(vec![Some(MIN_COLOR)]);
        });
    }

    /// Upstream: "should revert to last valid input value", "should not accept invalid
    /// characters": text that is no color reverts to the color, without a change.
    #[test]
    fn invalid_text_reverts_without_a_change() {
        with_owner(|| {
            let (state, changes) = uncontrolled(Some(AABBCC));
            for text in ["ab", "invalidColor"] {
                state.set_input_value(text.to_owned());
                state.commit();
                assert_that!(state.input_value.get_untracked()).is_equal_to("#AABBCC".to_owned());
            }
            assert_that!(changes.get_untracked()).is_empty();
        });
    }

    /// Upstream: "should update colorValue (uncontrolled)".
    #[test]
    fn a_committed_color_changes_the_field() {
        with_owner(|| {
            let (state, changes) = uncontrolled(Some(AABBCC));
            state.set_input_value("#cba".to_owned());
            state.commit();
            flush_effects();
            let cba = RGB8::from_hex_int(0xCC_BB_AA);
            assert_that!(changes.get_untracked()).is_equal_to(vec![Some(cba)]);
            assert_that!(state.color_value.get_untracked()).is_equal_to(Some(cba));
            assert_that!(state.input_value.get_untracked()).is_equal_to("#CCBBAA".to_owned());
        });
    }

    /// Upstream: "should not update colorValue (controlled)": app state that keeps its color
    /// gets the change reported, and the field shows the kept color.
    #[test]
    fn app_state_that_keeps_its_color_keeps_the_field() {
        with_owner(|| {
            let (state, changes) = recorded(UseColorFieldStateInput {
                value: Some(ValueBinding::new(
                    Signal::stored(Some(AABBCC)),
                    Callback::new(|_| {}),
                )),
                ..UseColorFieldStateInput::default()
            });
            state.set_input_value("#cba".to_owned());
            state.commit();
            flush_effects();
            assert_that!(changes.get_untracked())
                .is_equal_to(vec![Some(RGB8::from_hex_int(0xCC_BB_AA))]);
            assert_that!(state.color_value.get_untracked()).is_equal_to(Some(AABBCC));
            assert_that!(state.input_value.get_untracked()).is_equal_to("#AABBCC".to_owned());
        });
    }

    /// Upstream: "should call onChange when input is cleared".
    #[test]
    fn clearing_the_text_clears_the_color() {
        with_owner(|| {
            let (state, changes) = uncontrolled(Some(AABBCC));
            state.set_input_value(String::new());
            state.commit();
            flush_effects();
            assert_that!(changes.get_untracked()).is_equal_to(vec![None]);
            assert_that!(state.color_value.get_untracked()).is_none();
            assert_that!(state.input_value.get_untracked()).is_equal_to(String::new());
        });
    }

    /// Committing an empty field again reports no change (react-stately's `useControlledState`
    /// calls `onChange` only for a new value).
    #[test]
    fn committing_an_empty_field_reports_no_change() {
        with_owner(|| {
            let (state, changes) = uncontrolled(None);
            state.commit();
            state.commit();
            assert_that!(changes.get_untracked()).is_empty();
        });
    }

    /// Upstream: "should not call onChange when new value has the same color value".
    #[test]
    fn the_same_color_in_other_text_is_no_change() {
        with_owner(|| {
            let (state, changes) = uncontrolled(Some(RGB8::from_hex_int(0xBB_BB_BB)));
            state.set_input_value("#BBBBBB".to_owned());
            state.commit();
            assert_that!(changes.get_untracked()).is_empty();
        });
    }

    /// Four hex digits are a color with alpha (react-stately parses the text with
    /// `parseColor`): `#abcd` is `#AABBCC` at alpha `0xDD`.
    #[test]
    fn four_digits_are_a_color_with_alpha() {
        with_owner(|| {
            let state = use_color_field_state(UseColorFieldStateInput::<Alpha<RGB8>> {
                default_value: None,
                ..UseColorFieldStateInput::default()
            });
            state.set_input_value("abcd".to_owned());
            state.commit();
            assert_that!(state.color_value.get_untracked()).is_equal_to(Some(Alpha {
                color: AABBCC,
                alpha: f64::from(0xDD_u8) / 255.0,
            }));
            // An opaque field takes the color without the alpha.
            let (opaque, _) = uncontrolled(None);
            opaque.set_input_value("#abcd".to_owned());
            opaque.commit();
            assert_that!(opaque.color_value.get_untracked()).is_equal_to(Some(AABBCC));
        });
    }

    #[test]
    fn invalid_text_reverts_and_empty_text_clears() {
        with_owner(|| {
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
        with_owner(|| {
            let state = field(Some(RGB8 { r: 0, g: 0, b: 0 }));
            state.set_input_value("#0000FE".to_owned());
            state.increment();
            assert_that!(state.color_value.get_untracked()).is_equal_to(Some(RGB8 {
                r: 0,
                g: 0,
                b: 0xFF,
            }));
            assert_that!(state.validate("#12345")).is_true();
            assert_that!(state.validate("#1234567")).is_false();
            assert_that!(state.validate("xyz")).is_false();
        });
    }

    #[test]
    fn other_color_types_take_typed_hex_converted() {
        with_owner(|| {
            let state = use_color_field_state(UseColorFieldStateInput {
                default_value: Some(HSV::new()),
                ..UseColorFieldStateInput::default()
            });
            assert_that!(state.input_value.get_untracked()).is_equal_to("#FF0000".to_owned());
            state.set_input_value("#00FF00".to_owned());
            state.commit();
            assert_that!(state.color_value.get_untracked()).is_equal_to(Some(HSV {
                hue: 120.0,
                saturation: 1.0,
                brightness: 1.0,
            }));
        });
    }
}
