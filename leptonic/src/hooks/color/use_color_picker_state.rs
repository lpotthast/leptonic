// Upstream: react-stately/src/color/useColorPickerState.ts @ 99e6102368
// Upstream: react-aria-components/test/ColorPicker.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/color/ColorPicker.test.js @ 99e6102368
use leptos::prelude::*;

use crate::{ValueBinding, utils::color::Color};

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

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::{
        testing::with_owner,
        utils::color::{ColorFormat, HSV, OpaqueColor, RGB8},
    };

    fn color(text: &str) -> Color {
        text.parse().expect("a valid color")
    }

    /// The colors `on_change` was called with.
    fn recorder() -> (RwSignal<Vec<Color>>, Callback<Color>) {
        let changes = RwSignal::new(Vec::new());
        let on_change = Callback::new(move |color| changes.update(|c| c.push(color)));
        (changes, on_change)
    }

    // Upstream: "should have default value of black".
    #[test]
    fn defaults_to_black() {
        with_owner(|| {
            let state = use_color_picker_state(UseColorPickerStateInput::default());
            assert_that!(state.color.get_untracked().to_string_as(ColorFormat::Hexa))
                .is_equal_to("#000000FF".to_owned());
        });
    }

    // Upstream (react-aria-components): "renders", a part changing the shared color.
    #[test]
    fn set_color_changes_the_color_and_calls_on_change() {
        with_owner(|| {
            let (changes, on_change) = recorder();
            let state = use_color_picker_state(UseColorPickerStateInput {
                default_value: color("#f00"),
                value: None,
                on_change: Some(on_change),
            });
            assert_that!(state.color.get_untracked().to_string_as(ColorFormat::Hex))
                .is_equal_to("#FF0000".to_owned());

            state.set_color(color("#00f"));
            assert_that!(state.color.get_untracked().to_string_as(ColorFormat::Hex))
                .is_equal_to("#0000FF".to_owned());
            assert_that!(changes.get_untracked()).is_equal_to(vec![color("#00f")]);
        });
    }

    #[test]
    fn keeps_the_space_the_color_was_set_in() {
        with_owner(|| {
            let state = use_color_picker_state(UseColorPickerStateInput::default());
            // A gray set as HSV keeps its hue, which RGB would lose.
            let gray = HSV {
                hue: 120.0,
                saturation: 0.0,
                brightness: 0.5,
            };
            state.set_color(gray.into());
            assert_that!(state.color.get_untracked().color).is_equal_to(OpaqueColor::Hsv(gray));
            assert_that!(state.color.get_untracked().to::<HSV>().hue).is_equal_to(120.0);
        });
    }

    #[test]
    fn a_bound_color_follows_the_app_state() {
        with_owner(|| {
            let (changes, on_change) = recorder();
            let app = RwSignal::new(color("#f00"));
            let state = use_color_picker_state(UseColorPickerStateInput {
                default_value: color("#0f0"),
                value: Some(ValueBinding::from(app)),
                on_change: Some(on_change),
            });
            // The bound color, not `default_value`.
            assert_that!(state.color.get_untracked()).is_equal_to(color("#f00"));

            state.set_color(color("#00f"));
            assert_that!(app.get_untracked()).is_equal_to(color("#00f"));
            assert_that!(changes.get_untracked()).is_equal_to(vec![color("#00f")]);

            // Changed by the app: shown, and no change reported.
            app.set(RGB8 { r: 1, g: 2, b: 3 }.into());
            assert_that!(state.color.get_untracked().to::<RGB8>()).is_equal_to(RGB8 {
                r: 1,
                g: 2,
                b: 3,
            });
            assert_that!(changes.get_untracked().len()).is_equal_to(1);
        });
    }

    // Upstream's controlled `value` whose `onChange` doesn't update it.
    #[test]
    fn a_read_only_binding_keeps_its_color_and_reports_changes() {
        with_owner(|| {
            let (changes, on_change) = recorder();
            let state = use_color_picker_state(UseColorPickerStateInput {
                default_value: Color::default(),
                value: Some(ValueBinding::new(
                    Signal::stored(color("#f00")),
                    Callback::new(|_| {}),
                )),
                on_change: Some(on_change),
            });
            state.set_color(color("#00f"));
            assert_that!(state.color.get_untracked()).is_equal_to(color("#f00"));
            assert_that!(changes.get_untracked()).is_equal_to(vec![color("#00f")]);
        });
    }
}
