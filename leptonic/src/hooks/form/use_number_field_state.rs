use leptos::prelude::*;

use crate::utils::{
    i18n::Locale,
    math::{decimal_precision, handle_decimal_operation, round_to_precision, snap_value_to_step},
    number_formatter::{NumberFormatOptions, NumberFormatter, NumberStyle},
    number_parser::NumberParser,
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-stately/numberfield/src/useNumberFieldState.ts

// REACT-ARIA DEVIATIONS
//
// 1. Hook-owned state: The hook always creates and owns its `WriteSignal`
//    internally. Callers get read-only `Signal<T>` and semantic mutation
//    callbacks. This replaces react-aria's `useControlledState` pattern.
//
// 2. `Option<f64>` for empty: We use `None` to represent an empty field
//    instead of react-aria's `NaN`.
//
// 3. ICU4X instead of `Intl.NumberFormat`: The `NumberParser` and
//    `NumberFormatter` use ICU4X for SSR safety. Behavior should be
//    equivalent for supported locales.

/// Input parameters for the `use_number_field_state` hook.
#[derive(Clone)]
pub struct UseNumberFieldStateInput {
    /// The initial value. `None` means the field starts empty.
    pub default_value: Option<f64>,

    /// Minimum allowed value.
    pub min_value: Option<f64>,

    /// Maximum allowed value.
    pub max_value: Option<f64>,

    /// Step size for increment/decrement. Default: `1.0`.
    pub step: f64,

    /// Number format options for display formatting and parsing.
    pub format_options: NumberFormatOptions,

    /// Locale for formatting and parsing.
    pub locale: Locale,

    /// Whether the field is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the field is read-only.
    pub is_read_only: Signal<bool>,

    /// Callback when the numeric value changes.
    pub on_change: Option<Callback<Option<f64>>>,
}

impl Default for UseNumberFieldStateInput {
    fn default() -> Self {
        Self {
            default_value: None,
            min_value: None,
            max_value: None,
            step: 1.0,
            format_options: NumberFormatOptions::default(),
            locale: Locale::new("en-US"),
            is_disabled: Signal::derive(|| false),
            is_read_only: Signal::derive(|| false),
            on_change: None,
        }
    }
}

/// Return value of the `use_number_field_state` hook.
///
/// Provides the dual-value model (`number_value` + `input_value`) and
/// semantic mutation callbacks for number field state management.
#[derive(Clone, Copy)]
pub struct UseNumberFieldStateReturn {
    /// The canonical numeric value. `None` means the field is empty.
    pub number_value: Signal<Option<f64>>,

    /// The string displayed in the input (formatted or user-typed).
    pub input_value: Signal<String>,

    /// Whether the value can be incremented.
    pub can_increment: Signal<bool>,

    /// Whether the value can be decremented.
    pub can_decrement: Signal<bool>,

    /// Set a new numeric value (formats and updates `input_value`).
    pub set_number_value: Callback<Option<f64>>,

    /// Set the raw input string during user typing. Does NOT parse to a number.
    pub set_input_value: Callback<String>,

    /// Commit: parse `input_value` → clamp → snap → format → update both signals.
    /// Called on blur and Enter key.
    pub commit: Callback<()>,

    /// Commit a specific string value (for paste handling).
    pub commit_value: Callback<String>,

    /// Increment by one step.
    pub increment: Callback<()>,

    /// Decrement by one step.
    pub decrement: Callback<()>,

    /// Set value to the maximum.
    pub increment_to_max: Callback<()>,

    /// Set value to the minimum.
    pub decrement_to_min: Callback<()>,

    /// Validate a string as a partial number input for the current locale.
    /// Returns `true` if the string is a valid partial number.
    pub validate: Callback<String, bool>,
}

impl UseNumberFieldStateReturn {
    /// Creates a placeholder return value with no-op callbacks.
    /// Used for `Default` implementation of `UseNumberFieldInput`.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            number_value: Signal::derive(|| None),
            input_value: Signal::derive(String::new),
            can_increment: Signal::derive(|| false),
            can_decrement: Signal::derive(|| false),
            set_number_value: Callback::new(|_| {}),
            set_input_value: Callback::new(|_: String| {}),
            commit: Callback::new(|()| {}),
            commit_value: Callback::new(|_: String| {}),
            increment: Callback::new(|()| {}),
            decrement: Callback::new(|()| {}),
            increment_to_max: Callback::new(|()| {}),
            decrement_to_min: Callback::new(|()| {}),
            validate: Callback::new(|_: String| true),
        }
    }
}

/// Creates the state for a number field component.
///
/// This is the state management layer that handles:
/// - Dual-value model (`number_value` / `input_value` separation)
/// - Locale-aware formatting and parsing
/// - Commit pipeline (parse → clamp → snap → format)
/// - Floating-point safe increment/decrement
/// - Partial input validation
///
/// # Example
///
/// ```ignore
/// let state = use_number_field_state(UseNumberFieldStateInput {
///     default_value: Some(50.0),
///     min_value: Some(0.0),
///     max_value: Some(100.0),
///     step: 1.0,
///     ..Default::default()
/// });
/// ```
#[allow(clippy::too_many_lines, clippy::similar_names)]
pub fn use_number_field_state(input: UseNumberFieldStateInput) -> UseNumberFieldStateReturn {
    let UseNumberFieldStateInput {
        default_value,
        min_value,
        max_value,
        step,
        format_options,
        locale,
        is_disabled,
        is_read_only,
        on_change,
    } = input;

    // For percent style with default step, use 0.01 (1%) instead of 1.0.
    #[allow(clippy::float_cmp)]
    let effective_step = if format_options.style == NumberStyle::Percent && step == 1.0 {
        0.01
    } else {
        step
    };

    let step_precision = decimal_precision(effective_step);

    // Create formatter and parser for the locale.
    let formatter = NumberFormatter::new(&locale, format_options.clone());
    let parser = NumberParser::new(&locale, &format_options);

    // Format a numeric value for display.
    let format = {
        let formatter = formatter.clone();
        move |value: Option<f64>| -> String { value.map_or(String::new(), |v| formatter.format(v)) }
    };

    // Clamp a value to min/max bounds.
    let clamp = move |v: f64| -> f64 {
        let mut result = v;
        if let Some(min) = min_value {
            result = result.max(min);
        }
        if let Some(max) = max_value {
            result = result.min(max);
        }
        result
    };

    // Snap a value to the step grid, respecting min/max.
    let snap = move |v: f64| -> f64 {
        let min = min_value.unwrap_or(0.0);
        let max = max_value.unwrap_or(f64::MAX);
        snap_value_to_step(v, min, max, effective_step, step_precision)
    };

    // -- State signals --
    let initial_formatted = format(default_value);
    let (number_value, set_number_value_raw) = signal(default_value);
    let (input_value, set_input_value_raw) = signal(initial_formatted);

    // Fire `on_change` and update the number value signal.
    let update_number_value = move |new_value: Option<f64>| {
        set_number_value_raw.set(new_value);
        if let Some(on_change) = on_change {
            on_change.run(new_value);
        }
    };

    // -- can_increment / can_decrement --
    let can_increment = Signal::derive(move || {
        if is_disabled.get() || is_read_only.get() {
            return false;
        }
        let current = number_value.get();
        match (current, max_value) {
            (None, _) | (Some(_), None) => true,
            (Some(v), Some(max)) => {
                // Check if snapping would move the value upward.
                let min = min_value.unwrap_or(0.0);
                let snapped = snap_value_to_step(v, min, max, effective_step, step_precision);
                if snapped > v {
                    return true;
                }
                // Check if adding a step would stay within bounds.
                handle_decimal_operation('+', v, effective_step) <= max
            }
        }
    });

    let can_decrement = Signal::derive(move || {
        if is_disabled.get() || is_read_only.get() {
            return false;
        }
        let current = number_value.get();
        match (current, min_value) {
            (None, _) | (Some(_), None) => true,
            (Some(v), Some(min)) => {
                let max = max_value.unwrap_or(f64::MAX);
                let snapped = snap_value_to_step(v, min, max, effective_step, step_precision);
                if snapped < v {
                    return true;
                }
                handle_decimal_operation('-', v, effective_step) >= min
            }
        }
    });

    // -- Commit pipeline --
    // Shared commit logic extracted as a function to avoid closure move issues.
    let commit_text = {
        let format_fn = format.clone();
        let parser_for_commit = parser.clone();
        move |text: String| {
            if text.is_empty() {
                update_number_value(None);
                set_input_value_raw.set(String::new());
                return;
            }

            let parsed = parser_for_commit.parse(&text);
            if let Some(v) = parsed {
                let clamped = clamp(v);
                let snapped = snap(clamped);
                let rounded = round_to_precision(snapped, step_precision);

                let formatted = format_fn(Some(rounded));
                set_input_value_raw.set(formatted);
                update_number_value(Some(rounded));
            } else {
                // Unparseable input → revert to previous formatted value.
                let formatted = format_fn(number_value.get_untracked());
                set_input_value_raw.set(formatted);
            }
        }
    };

    // -- Safe next step (following react-aria's safeNextStep) --
    let safe_next_step = move |op: char, fallback: Option<f64>| -> f64 {
        let current = number_value.get_untracked();

        match current {
            None => {
                // Start from fallback (minValue for increment, maxValue for decrement) or 0.
                let start = fallback.unwrap_or(0.0);
                snap(start)
            }
            Some(v) => {
                let min = min_value.unwrap_or(0.0);
                let max = max_value.unwrap_or(f64::MAX);

                // First try snapping the current value.
                let snapped = snap_value_to_step(v, min, max, effective_step, step_precision);

                // If snap moved us in the desired direction, use it.
                if (op == '+' && snapped > v) || (op == '-' && snapped < v) {
                    return snapped;
                }

                // Otherwise: add/subtract step, then snap.
                let stepped = handle_decimal_operation(op, v, effective_step);
                let result = snap_value_to_step(stepped, min, max, effective_step, step_precision);
                round_to_precision(result, step_precision)
            }
        }
    };

    let format_for_inc = format.clone();
    let do_increment = move || {
        if !can_increment.get_untracked() {
            return;
        }
        let new_value = safe_next_step('+', min_value);
        let formatted = format_for_inc(Some(new_value));
        set_input_value_raw.set(formatted);
        update_number_value(Some(new_value));
    };

    let format_for_dec = format.clone();
    let do_decrement = move || {
        if !can_decrement.get_untracked() {
            return;
        }
        let new_value = safe_next_step('-', max_value);
        let formatted = format_for_dec(Some(new_value));
        set_input_value_raw.set(formatted);
        update_number_value(Some(new_value));
    };

    let format_for_max = format.clone();
    let do_increment_to_max = move || {
        if let Some(max) = max_value {
            let snapped = snap(max);
            let formatted = format_for_max(Some(snapped));
            set_input_value_raw.set(formatted);
            update_number_value(Some(snapped));
        }
    };

    let format_for_min = format.clone();
    let do_decrement_to_min = move || {
        if let Some(min) = min_value {
            let formatted = format_for_min(Some(min));
            set_input_value_raw.set(formatted);
            update_number_value(Some(min));
        }
    };

    // -- set_number_value (public callback) --
    let format_for_set = format.clone();
    let public_set_number_value = move |value: Option<f64>| {
        let formatted = format_for_set(value);
        set_input_value_raw.set(formatted);
        update_number_value(value);
    };

    UseNumberFieldStateReturn {
        number_value: number_value.into(),
        input_value: input_value.into(),
        can_increment,
        can_decrement,
        set_number_value: Callback::new(public_set_number_value),
        set_input_value: Callback::new(move |text: String| {
            set_input_value_raw.set(text);
        }),
        commit: Callback::new({
            let commit_text = commit_text.clone();
            move |()| {
                let text = input_value.get_untracked();
                commit_text(text);
            }
        }),
        commit_value: Callback::new(move |text: String| {
            commit_text(text);
        }),
        increment: Callback::new(move |()| {
            do_increment();
        }),
        decrement: Callback::new(move |()| {
            do_decrement();
        }),
        increment_to_max: Callback::new(move |()| {
            do_increment_to_max();
        }),
        decrement_to_min: Callback::new(move |()| {
            do_decrement_to_min();
        }),
        validate: Callback::new(move |text: String| {
            parser.is_valid_partial_number(&text, min_value, max_value)
        }),
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;
    use leptos::prelude::*;

    use super::*;

    fn with_owner<T>(f: impl FnOnce() -> T) -> T {
        let owner = Owner::new();
        owner.with(f)
    }

    #[test]
    fn test_default_empty() {
        with_owner(|| {
            let state = use_number_field_state(UseNumberFieldStateInput::default());
            assert_that(state.number_value.get_untracked()).is_equal_to(None);
            assert_that(state.input_value.get_untracked()).is_equal_to(String::new());
        });
    }

    #[test]
    fn test_default_with_value() {
        with_owner(|| {
            let state = use_number_field_state(UseNumberFieldStateInput {
                default_value: Some(42.0),
                ..Default::default()
            });
            assert_that(state.number_value.get_untracked()).is_equal_to(Some(42.0));
            assert_that(state.input_value.get_untracked()).is_equal_to("42".to_string());
        });
    }

    #[test]
    fn test_increment_from_empty() {
        with_owner(|| {
            let state = use_number_field_state(UseNumberFieldStateInput {
                min_value: Some(0.0),
                max_value: Some(10.0),
                step: 1.0,
                ..Default::default()
            });
            state.increment.run(());
            assert_that(state.number_value.get_untracked()).is_equal_to(Some(0.0));
        });
    }

    #[test]
    fn test_increment_basic() {
        with_owner(|| {
            let state = use_number_field_state(UseNumberFieldStateInput {
                default_value: Some(5.0),
                min_value: Some(0.0),
                max_value: Some(10.0),
                step: 1.0,
                ..Default::default()
            });
            state.increment.run(());
            assert_that(state.number_value.get_untracked()).is_equal_to(Some(6.0));
        });
    }

    #[test]
    fn test_decrement_from_empty() {
        with_owner(|| {
            let state = use_number_field_state(UseNumberFieldStateInput {
                min_value: Some(0.0),
                max_value: Some(10.0),
                step: 1.0,
                ..Default::default()
            });
            state.decrement.run(());
            assert_that(state.number_value.get_untracked()).is_equal_to(Some(10.0));
        });
    }

    #[test]
    fn test_increment_respects_max() {
        with_owner(|| {
            let state = use_number_field_state(UseNumberFieldStateInput {
                default_value: Some(10.0),
                min_value: Some(0.0),
                max_value: Some(10.0),
                step: 1.0,
                ..Default::default()
            });
            // At max, can_increment should be false.
            assert_that(state.can_increment.get_untracked()).is_false();
            state.increment.run(());
            // Value shouldn't change.
            assert_that(state.number_value.get_untracked()).is_equal_to(Some(10.0));
        });
    }

    #[test]
    fn test_decrement_respects_min() {
        with_owner(|| {
            let state = use_number_field_state(UseNumberFieldStateInput {
                default_value: Some(0.0),
                min_value: Some(0.0),
                max_value: Some(10.0),
                step: 1.0,
                ..Default::default()
            });
            assert_that(state.can_decrement.get_untracked()).is_false();
            state.decrement.run(());
            assert_that(state.number_value.get_untracked()).is_equal_to(Some(0.0));
        });
    }

    #[test]
    fn test_floating_point_precision() {
        with_owner(|| {
            let state = use_number_field_state(UseNumberFieldStateInput {
                default_value: Some(0.0),
                step: 0.1,
                ..Default::default()
            });
            state.increment.run(());
            state.increment.run(());
            state.increment.run(());
            // Should be exactly 0.3, not 0.30000000000000004.
            assert_that(state.number_value.get_untracked()).is_equal_to(Some(0.3));
        });
    }

    #[test]
    fn test_commit_clamps_and_snaps() {
        with_owner(|| {
            let state = use_number_field_state(UseNumberFieldStateInput {
                min_value: Some(0.0),
                max_value: Some(100.0),
                step: 5.0,
                ..Default::default()
            });
            // Simulate typing a value that's not on a step boundary.
            state.set_input_value.run("37".to_string());
            state.commit.run(());
            // Should snap to nearest step boundary: 35 or 40.
            let val = state.number_value.get_untracked().unwrap();
            assert_that(val == 35.0 || val == 40.0).is_true();
        });
    }

    #[test]
    fn test_commit_empty() {
        with_owner(|| {
            let state = use_number_field_state(UseNumberFieldStateInput {
                default_value: Some(42.0),
                ..Default::default()
            });
            state.set_input_value.run(String::new());
            state.commit.run(());
            assert_that(state.number_value.get_untracked()).is_equal_to(None);
        });
    }

    #[test]
    fn test_commit_invalid_reverts() {
        with_owner(|| {
            let state = use_number_field_state(UseNumberFieldStateInput {
                default_value: Some(42.0),
                ..Default::default()
            });
            state.set_input_value.run("abc".to_string());
            state.commit.run(());
            // Should revert to previous formatted value.
            assert_that(state.number_value.get_untracked()).is_equal_to(Some(42.0));
            assert_that(state.input_value.get_untracked()).is_equal_to("42".to_string());
        });
    }

    #[test]
    fn test_validate_partial() {
        with_owner(|| {
            let state = use_number_field_state(UseNumberFieldStateInput::default());
            assert_that(state.validate.run(String::new())).is_true();
            assert_that(state.validate.run("1".to_string())).is_true();
            assert_that(state.validate.run("1.".to_string())).is_true();
            assert_that(state.validate.run("-".to_string())).is_true();
            assert_that(state.validate.run("abc".to_string())).is_false();
        });
    }

    #[test]
    fn test_increment_to_max() {
        with_owner(|| {
            let state = use_number_field_state(UseNumberFieldStateInput {
                default_value: Some(5.0),
                min_value: Some(0.0),
                max_value: Some(100.0),
                ..Default::default()
            });
            state.increment_to_max.run(());
            assert_that(state.number_value.get_untracked()).is_equal_to(Some(100.0));
        });
    }

    #[test]
    fn test_decrement_to_min() {
        with_owner(|| {
            let state = use_number_field_state(UseNumberFieldStateInput {
                default_value: Some(50.0),
                min_value: Some(0.0),
                max_value: Some(100.0),
                ..Default::default()
            });
            state.decrement_to_min.run(());
            assert_that(state.number_value.get_untracked()).is_equal_to(Some(0.0));
        });
    }

    #[test]
    fn test_commit_value_for_paste() {
        with_owner(|| {
            let state = use_number_field_state(UseNumberFieldStateInput {
                min_value: Some(0.0),
                max_value: Some(100.0),
                step: 1.0,
                ..Default::default()
            });
            state.commit_value.run("42.5".to_string());
            // Should parse, clamp, snap.
            let val = state.number_value.get_untracked().unwrap();
            assert_that(val == 42.0 || val == 43.0).is_true();
        });
    }
}
