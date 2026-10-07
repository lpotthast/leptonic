// Upstream: react-stately/src/numberfield/useNumberFieldState.ts @ 99e6102368
use leptos::prelude::*;

use super::use_form_validation_state::{
    UseFormValidationStateInput, UseFormValidationStateReturn, ValidateFn, ValidationBehavior,
    use_form_validation_state,
};
use crate::utils::{
    NumberValue, ValueBinding,
    i18n::{Locale, use_locale},
    number_formatter::{NumberFormatOptions, NumberFormatter, NumberStyle},
    number_parser::NumberParser,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Generic over the value type `T: NumberValue` (react-aria: JS numbers). Reason: exact integer
//   values of every width (C15); `None` instead of `NaN` for an empty field.
// - Hook-owned state (C4): `default_value` + `on_change`, or `value` bound to app state
//   (`ValueBinding::from(rw_signal)`), instead of a controlled `value`. The field follows changes
//   of the bound value (re-formatting its text), as a controlled react-aria field does.
// - The state is a `Copy` struct of signals and methods (C3); `commit(Some(text))` replaces
//   `commit(overrideValue)`.
// - `min_value`, `max_value` and `step` are signals. Integer types additionally never step
//   beyond their own bounds, and `increment_to_max`/`decrement_to_min` (End/Home) use the type's
//   bounds when no explicit ones are set.
// - The locale comes from the i18n context (C5).
//
// =============================================================================

/// How a number field treats a typed value outside its range or step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CommitBehavior {
    /// Clamp it to `min..=max` and round it to the nearest step.
    #[default]
    Snap,
    /// Keep it and report it as invalid (range overflow/underflow, step mismatch).
    Validate,
}

/// Input of [`use_number_field_state`].
pub struct UseNumberFieldStateInput<T: NumberValue> {
    /// The initial value; `None` starts empty. Ignored when `value` is bound.
    pub default_value: Option<T>,
    /// The value as app state, replacing `default_value`.
    pub value: Option<ValueBinding<Option<T>>>,
    /// Called when a committed value changes.
    pub on_change: Option<Callback<Option<T>>>,
    pub min_value: Signal<Option<T>>,
    pub max_value: Signal<Option<T>>,
    /// The step of increments; `None` is 1 (0.01 for percentages).
    pub step: Signal<Option<T>>,
    pub format_options: Signal<NumberFormatOptions>,
    pub commit_behavior: CommitBehavior,
    pub is_disabled: Signal<bool>,
    pub is_read_only: Signal<bool>,
    /// Marks the value invalid, regardless of `validate`.
    pub is_invalid: Signal<bool>,
    pub validate: Option<ValidateFn<Option<T>>>,
    pub validation_behavior: ValidationBehavior,
    /// The field's `name`, matching server errors of a surrounding form.
    pub name: Option<String>,
}

impl<T: NumberValue> Clone for UseNumberFieldStateInput<T> {
    fn clone(&self) -> Self {
        Self {
            default_value: self.default_value,
            value: self.value,
            on_change: self.on_change,
            min_value: self.min_value,
            max_value: self.max_value,
            step: self.step,
            format_options: self.format_options,
            commit_behavior: self.commit_behavior,
            is_disabled: self.is_disabled,
            is_read_only: self.is_read_only,
            is_invalid: self.is_invalid,
            validate: self.validate.clone(),
            validation_behavior: self.validation_behavior,
            name: self.name.clone(),
        }
    }
}

impl<T: NumberValue> Default for UseNumberFieldStateInput<T> {
    fn default() -> Self {
        Self {
            default_value: None,
            value: None,
            on_change: None,
            min_value: Signal::stored(None),
            max_value: Signal::stored(None),
            step: Signal::stored(None),
            format_options: Signal::stored(NumberFormatOptions::default()),
            commit_behavior: CommitBehavior::default(),
            is_disabled: Signal::stored(false),
            is_read_only: Signal::stored(false),
            is_invalid: Signal::stored(false),
            validate: None,
            validation_behavior: ValidationBehavior::default(),
            name: None,
        }
    }
}

/// The state of a number field: the committed value, the typed text and the operations on them.
#[derive(Clone, Copy)]
pub struct NumberFieldState<T: NumberValue> {
    /// The value of the typed text (react-aria's `numberValue`); `None` while it is empty or
    /// unparsable.
    pub number_value: Signal<Option<T>>,
    /// The text in the input: the formatted value, or what the user is typing.
    pub input_value: Signal<String>,
    pub can_increment: Signal<bool>,
    pub can_decrement: Signal<bool>,
    pub min_value: Signal<Option<T>>,
    pub max_value: Signal<Option<T>>,
    /// The effective step (`step`, else 1, else 0.01 for percentages).
    pub step: Signal<T>,
    pub format_options: Signal<NumberFormatOptions>,
    pub commit_behavior: CommitBehavior,
    pub is_disabled: Signal<bool>,
    pub is_read_only: Signal<bool>,
    pub validation: UseFormValidationStateReturn,
    pub validation_behavior: ValidationBehavior,
    /// The value form resets restore.
    pub default_number_value: Option<T>,
    /// The committed value.
    value: Signal<Option<T>>,
    set_value: Callback<Option<T>>,
    set_input_value: Callback<String>,
    commit: Callback<Option<String>>,
    increment: Callback<()>,
    decrement: Callback<()>,
    increment_to_max: Callback<()>,
    decrement_to_min: Callback<()>,
    validate: Callback<String, bool>,
}

impl<T: NumberValue> NumberFieldState<T> {
    /// The committed value.
    pub fn value(&self) -> Signal<Option<T>> {
        self.value
    }

    /// Sets the value (formatting the text), e.g. from a form reset.
    pub fn set_number_value(&self, value: Option<T>) {
        self.set_value.run(value);
    }

    /// Sets the typed text without committing it.
    pub fn set_input_value(&self, text: String) {
        self.set_input_value.run(text);
    }

    /// Commits the typed text, or `text` (a paste): parses it, snaps it (with
    /// `CommitBehavior::Snap`) and formats it. Unparsable text reverts to the committed value.
    pub fn commit(&self, text: Option<String>) {
        self.commit.run(text);
    }

    pub fn increment(&self) {
        self.increment.run(());
    }

    pub fn decrement(&self) {
        self.decrement.run(());
    }

    /// To `max_value` (or the type's maximum).
    pub fn increment_to_max(&self) {
        self.increment_to_max.run(());
    }

    /// To `min_value` (or the type's minimum).
    pub fn decrement_to_min(&self) {
        self.decrement_to_min.run(());
    }

    /// Whether `text` may be typed: a valid number, or the beginning of one.
    pub fn validate(&self, text: String) -> bool {
        self.validate.run(text)
    }
}

/// The number formatter and parser of a locale and format options.
#[derive(Clone)]
struct Formatting {
    formatter: NumberFormatter,
    parser: NumberParser,
}

/// Creates the state of a number field holding values of type `T`.
///
/// ```ignore
/// let state = use_number_field_state(UseNumberFieldStateInput::<u32> {
///     default_value: Some(1),
///     max_value: Signal::stored(Some(99)),
///     ..UseNumberFieldStateInput::default()
/// });
/// state.increment();
/// ```
#[allow(clippy::too_many_lines)]
pub fn use_number_field_state<T: NumberValue>(
    input: UseNumberFieldStateInput<T>,
) -> NumberFieldState<T> {
    let UseNumberFieldStateInput {
        default_value,
        value: binding,
        on_change,
        min_value,
        max_value,
        step,
        format_options,
        commit_behavior,
        is_disabled,
        is_read_only,
        is_invalid,
        validate,
        validation_behavior,
        name,
    } = input;

    // Formatter and parser, rebuilt when the locale or the options (by value) change.
    let locale = use_locale();
    let formatting_key: Memo<(Locale, NumberFormatOptions)> =
        Memo::new(move |_| (locale.get(), format_options.get()));
    let formatting = Memo::new_with_compare(
        move |_| {
            formatting_key.with(|(locale, options)| Formatting {
                formatter: NumberFormatter::new(locale, options.clone()),
                parser: NumberParser::new(locale, options),
            })
        },
        // Always a change: the key memo already compares by value.
        |_, _| true,
    );
    let format = move |value: Option<T>| {
        value.map_or_else(String::new, |value| {
            formatting.with_untracked(|f| f.formatter.format(value))
        })
    };
    let parse = move |text: &str| formatting.with_untracked(|f| f.parser.parse::<T>(text));

    let explicit_step = step;
    let step = Signal::derive(move || {
        explicit_step.get().unwrap_or_else(|| {
            let is_percent = format_options.with(|o| o.style == NumberStyle::Percent);
            let hundredth = || {
                fixed_decimal::Decimal::try_from_str("0.01")
                    .ok()
                    .and_then(|d| T::from_decimal(&d))
            };
            if is_percent {
                hundredth().unwrap_or(T::ONE)
            } else {
                T::ONE
            }
        })
    });
    // Stepping snaps to the effective step.
    let snap = move |value: T| {
        value.snap_to_step(
            min_value.get_untracked(),
            max_value.get_untracked(),
            step.get_untracked(),
        )
    };
    // Committed values snap to an explicit step only; without one, they are only clamped.
    let snap_committed = move |value: T| match explicit_step.get_untracked() {
        Some(step) => {
            value.snap_to_step(min_value.get_untracked(), max_value.get_untracked(), step)
        }
        None => value.clamp_to(min_value.get_untracked(), max_value.get_untracked()),
    };

    // The committed value: bound app state, or owned.
    let (value, store_value): (Signal<Option<T>>, Callback<Option<T>>) =
        if let Some(binding) = binding {
            (binding.value, Callback::new(move |v| binding.set(v)))
        } else {
            let owned = RwSignal::new(match commit_behavior {
                CommitBehavior::Snap => default_value.map(snap_committed),
                CommitBehavior::Validate => default_value,
            });
            (owned.into(), Callback::new(move |v| owned.set(v)))
        };
    let set_value = Callback::new(move |new: Option<T>| {
        if value.get_untracked() != new {
            store_value.run(new);
            if let Some(on_change) = on_change {
                on_change.run(new);
            }
        }
    });
    let default_number_value = value.get_untracked();

    let input_value = RwSignal::new(format(value.get_untracked()));
    // The text follows the committed value (also when changed from outside), the locale and the
    // format options.
    Effect::new(move |previous: Option<()>| {
        let value = value.get();
        formatting.track();
        if previous.is_some() {
            input_value.set(format(value));
        }
    });

    // The value of the typed text.
    let number_value = Signal::derive(move || {
        formatting.track();
        input_value.with(|text| parse(text))
    });

    let validation = use_form_validation_state(UseFormValidationStateInput {
        builtin_validation: Signal::default(),
        is_invalid,
        value,
        validate,
        validation_behavior,
        name,
    });

    let commit = Callback::new(move |text: Option<String>| {
        let text = text.unwrap_or_else(|| input_value.get_untracked());
        if text.is_empty() {
            set_value.run(None);
            input_value.set(String::new());
            return;
        }
        let Some(parsed) = parse(&text) else {
            // Unparsable: back to the committed value.
            input_value.set(format(value.get_untracked()));
            return;
        };
        let committed = match commit_behavior {
            CommitBehavior::Snap => snap_committed(parsed),
            CommitBehavior::Validate => parsed,
        };
        // As displayed (rounded to the format's fraction digits).
        let committed = parse(&format(Some(committed))).unwrap_or(committed);
        let should_validate = value.get_untracked() != Some(committed);
        set_value.run(Some(committed));
        // What the store holds (a bound value may reject the change).
        input_value.set(format(value.get_untracked()));
        if should_validate {
            validation.commit_validation.run(());
        }
    });

    // react-aria's `safeNextStep`: from the typed value (snapped first, if that moves in the
    // direction), else from `start` (min/max) or zero.
    let next_step = move |up: bool, start: Option<T>| -> T {
        let step = step.get_untracked();
        let Some(previous) = number_value.get_untracked() else {
            return snap(start.unwrap_or(T::ZERO));
        };
        let snapped = snap(previous);
        if (up && snapped > previous) || (!up && snapped < previous) {
            return snapped;
        }
        let stepped = if up {
            previous.checked_add(step)
        } else {
            previous.checked_sub(step)
        };
        // At the type's bounds, stay there.
        stepped.map_or(previous, snap)
    };
    let set_stepped = move |new: T| {
        set_value.run(Some(new));
        // Also when the value didn't change (the text may differ from it), and what the store
        // holds (a bound value may reject the change).
        input_value.set(format(value.get_untracked()));
        validation.commit_validation.run(());
    };
    let increment = Callback::new(move |()| {
        set_stepped(next_step(true, min_value.get_untracked()));
    });
    let decrement = Callback::new(move |()| {
        set_stepped(next_step(false, max_value.get_untracked()));
    });
    let increment_to_max = Callback::new(move |()| {
        if let Some(max) = T::upper_bound(max_value.get_untracked()) {
            set_stepped(snap(max));
        }
    });
    let decrement_to_min = Callback::new(move |()| {
        if let Some(min) = T::lower_bound(min_value.get_untracked()) {
            set_stepped(min);
        }
    });

    let can_step = move |up: bool| {
        if is_disabled.get() || is_read_only.get() {
            return false;
        }
        let Some(current) = number_value.get() else {
            return true;
        };
        let bound = if up {
            T::upper_bound(max_value.get())
        } else {
            T::lower_bound(min_value.get())
        };
        let Some(bound) = bound else {
            return true;
        };
        let snapped = current.snap_to_step(min_value.get(), max_value.get(), step.get());
        if (up && snapped > current) || (!up && snapped < current) {
            return true;
        }
        let stepped = if up {
            current.checked_add(step.get())
        } else {
            current.checked_sub(step.get())
        };
        stepped.is_some_and(|stepped| {
            if up {
                stepped <= bound
            } else {
                stepped >= bound
            }
        })
    };

    NumberFieldState {
        number_value,
        input_value: input_value.into(),
        can_increment: Signal::derive(move || can_step(true)),
        can_decrement: Signal::derive(move || can_step(false)),
        min_value,
        max_value,
        step,
        format_options,
        commit_behavior,
        is_disabled,
        is_read_only,
        validation,
        validation_behavior,
        default_number_value,
        value,
        set_value: Callback::new(move |new: Option<T>| {
            set_value.run(new);
            input_value.set(format(value.get_untracked()));
        }),
        set_input_value: Callback::new(move |text: String| input_value.set(text)),
        commit,
        increment,
        decrement,
        increment_to_max,
        decrement_to_min,
        validate: Callback::new(move |text: String| {
            formatting.with_untracked(|f| {
                f.parser.is_valid_partial_number(
                    &text,
                    min_value.get_untracked(),
                    max_value.get_untracked(),
                )
            })
        }),
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    fn with_state<T: NumberValue, R>(
        input: UseNumberFieldStateInput<T>,
        f: impl FnOnce(NumberFieldState<T>) -> R,
    ) -> R {
        Owner::new().with(|| f(use_number_field_state(input)))
    }

    #[test]
    fn starts_empty_or_formatted() {
        with_state(UseNumberFieldStateInput::<f64>::default(), |state| {
            assert_that!(state.value().get_untracked()).is_none();
            assert_that!(state.input_value.get_untracked()).is_equal_to(String::new());
        });
        with_state(
            UseNumberFieldStateInput {
                default_value: Some(1234.5_f64),
                format_options: Signal::stored(NumberFormatOptions {
                    use_grouping: true,
                    ..NumberFormatOptions::default()
                }),
                ..UseNumberFieldStateInput::default()
            },
            |state| {
                assert_that!(state.input_value.get_untracked()).is_equal_to("1,234.5".to_owned());
            },
        );
    }

    #[test]
    fn incrementing_an_empty_field_starts_at_min_or_zero() {
        with_state(UseNumberFieldStateInput::<i32>::default(), |state| {
            state.increment();
            assert_that!(state.value().get_untracked()).is_equal_to(Some(0));
            assert_that!(state.input_value.get_untracked()).is_equal_to("0".to_owned());
        });
        with_state(
            UseNumberFieldStateInput::<i32> {
                min_value: Signal::stored(Some(5)),
                ..UseNumberFieldStateInput::default()
            },
            |state| {
                state.increment();
                assert_that!(state.value().get_untracked()).is_equal_to(Some(5));
            },
        );
        with_state(
            UseNumberFieldStateInput::<i32> {
                max_value: Signal::stored(Some(-5)),
                ..UseNumberFieldStateInput::default()
            },
            |state| {
                state.decrement();
                assert_that!(state.value().get_untracked()).is_equal_to(Some(-5));
            },
        );
    }

    #[test]
    fn steps_snap_first_when_that_moves_in_the_direction() {
        with_state(
            UseNumberFieldStateInput::<i32> {
                default_value: Some(7),
                step: Signal::stored(Some(5)),
                commit_behavior: CommitBehavior::Validate,
                ..UseNumberFieldStateInput::default()
            },
            |state| {
                // 7 snaps down to 5; incrementing goes to 10, decrementing to 5.
                state.increment();
                assert_that!(state.value().get_untracked()).is_equal_to(Some(10));
                state.set_input_value("7".to_owned());
                state.decrement();
                assert_that!(state.value().get_untracked()).is_equal_to(Some(5));
            },
        );
    }

    #[test]
    fn steps_stop_at_bounds() {
        with_state(
            UseNumberFieldStateInput::<u8> {
                default_value: Some(254),
                ..UseNumberFieldStateInput::default()
            },
            |state| {
                state.increment();
                assert_that!(state.value().get_untracked()).is_equal_to(Some(255));
                assert_that!(state.can_increment.get_untracked()).is_false();
                state.increment();
                assert_that!(state.value().get_untracked()).is_equal_to(Some(255));
            },
        );
        with_state(
            UseNumberFieldStateInput::<f64> {
                default_value: Some(9.0),
                max_value: Signal::stored(Some(10.0)),
                ..UseNumberFieldStateInput::default()
            },
            |state| {
                state.increment();
                assert_that!(state.value().get_untracked()).is_equal_to(Some(10.0));
                assert_that!(state.can_increment.get_untracked()).is_false();
            },
        );
    }

    #[test]
    fn floats_step_without_binary_rounding_errors() {
        with_state(
            UseNumberFieldStateInput::<f64> {
                default_value: Some(0.1),
                step: Signal::stored(Some(0.1)),
                ..UseNumberFieldStateInput::default()
            },
            |state| {
                state.increment();
                state.increment();
                assert_that!(state.value().get_untracked()).is_equal_to(Some(0.3));
            },
        );
    }

    #[test]
    fn integers_are_exact_beyond_f64() {
        with_state(
            UseNumberFieldStateInput::<u64> {
                default_value: Some(u64::MAX - 1),
                ..UseNumberFieldStateInput::default()
            },
            |state| {
                state.increment();
                assert_that!(state.value().get_untracked()).is_equal_to(Some(u64::MAX));
                assert_that!(state.input_value.get_untracked())
                    .is_equal_to("18,446,744,073,709,551,615".to_owned());
            },
        );
    }

    #[test]
    fn commit_snaps_clamps_and_formats() {
        with_state(
            UseNumberFieldStateInput::<f64> {
                min_value: Signal::stored(Some(0.0)),
                max_value: Signal::stored(Some(100.0)),
                step: Signal::stored(Some(5.0)),
                ..UseNumberFieldStateInput::default()
            },
            |state| {
                state.set_input_value("42".to_owned());
                assert_that!(state.number_value.get_untracked()).is_equal_to(Some(42.0));
                state.commit(None);
                assert_that!(state.value().get_untracked()).is_equal_to(Some(40.0));
                state.commit(Some("250".to_owned()));
                assert_that!(state.value().get_untracked()).is_equal_to(Some(100.0));
                assert_that!(state.input_value.get_untracked()).is_equal_to("100".to_owned());
            },
        );
    }

    #[test]
    fn commit_with_validate_behavior_keeps_the_value() {
        with_state(
            UseNumberFieldStateInput::<i32> {
                max_value: Signal::stored(Some(10)),
                commit_behavior: CommitBehavior::Validate,
                ..UseNumberFieldStateInput::default()
            },
            |state| {
                state.commit(Some("12".to_owned()));
                assert_that!(state.value().get_untracked()).is_equal_to(Some(12));
            },
        );
    }

    #[test]
    fn commit_of_empty_text_clears_and_of_invalid_text_reverts() {
        with_state(
            UseNumberFieldStateInput::<i32> {
                default_value: Some(3),
                ..UseNumberFieldStateInput::default()
            },
            |state| {
                state.commit(Some("abc".to_owned()));
                assert_that!(state.value().get_untracked()).is_equal_to(Some(3));
                assert_that!(state.input_value.get_untracked()).is_equal_to("3".to_owned());
                state.commit(Some(String::new()));
                assert_that!(state.value().get_untracked()).is_none();
            },
        );
    }

    #[test]
    fn integer_fields_reject_fractions_while_typing() {
        with_state(UseNumberFieldStateInput::<u16>::default(), |state| {
            assert_that!(state.validate("12".to_owned())).is_true();
            assert_that!(state.validate("1.".to_owned())).is_false();
            assert_that!(state.validate("-".to_owned())).is_false();
        });
        with_state(UseNumberFieldStateInput::<f32>::default(), |state| {
            assert_that!(state.validate("-1.".to_owned())).is_true();
        });
    }

    #[test]
    fn end_and_home_use_the_type_bounds_without_explicit_ones() {
        with_state(UseNumberFieldStateInput::<i8>::default(), |state| {
            state.increment_to_max();
            assert_that!(state.value().get_untracked()).is_equal_to(Some(i8::MAX));
            state.decrement_to_min();
            assert_that!(state.value().get_untracked()).is_equal_to(Some(i8::MIN));
        });
    }

    #[test]
    fn on_change_reports_committed_changes_only() {
        let changes = RwSignal::new(Vec::new());
        with_state(
            UseNumberFieldStateInput::<i32> {
                default_value: Some(1),
                on_change: Some(Callback::new(move |v| changes.update(|c| c.push(v)))),
                ..UseNumberFieldStateInput::default()
            },
            |state| {
                state.set_input_value("2".to_owned());
                state.commit(None);
                state.commit(None);
                assert_that!(changes.get_untracked()).is_equal_to(vec![Some(2)]);
            },
        );
    }

    #[test]
    fn a_bound_value_is_read_and_written() {
        Owner::new().with(|| {
            let app = RwSignal::new(Some(4_i64));
            let state = use_number_field_state(UseNumberFieldStateInput {
                value: Some(ValueBinding::from(app)),
                ..UseNumberFieldStateInput::default()
            });
            assert_that!(state.input_value.get_untracked()).is_equal_to("4".to_owned());
            state.increment();
            assert_that!(app.get_untracked()).is_equal_to(Some(5));
        });
    }
}
