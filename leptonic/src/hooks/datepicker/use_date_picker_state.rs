use leptos::prelude::*;
use time::macros::format_description;

use super::use_time_field::TimeValue;
use crate::hooks::form::use_form_validation_state::{
    UseFormValidationStateInput, UseFormValidationStateReturn, ValidateFn, ValidationBehavior,
    use_form_validation_state,
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-stately/datepicker/src/useDatePickerState.ts

//
// ## OMITTED FEATURES
// - `granularity`: Uses a simpler `show_time: bool` flag instead of
//   day/hour/minute/second granularity. React-aria: `granularity` prop
//   with computed `hasTime`.
// - `getDateFormatter` / `formatValue` locale-awareness: Uses `time` crate
//   formatting (English month names) instead of ICU-based `DateFormatter`.
//   React-aria: locale-aware formatting via `@internationalized/date`.
// - Multi-calendar system support: Uses `time::OffsetDateTime` only.
//   React-aria: supports multiple calendar systems.
//
// ## LEPTOS-SPECIFIC ADAPTATIONS
// - Hook-owned state: All signals are created and owned internally. Callers
//   get read-only `Signal<T>` and semantic mutation callbacks. Replaces
//   React-aria's `useControlledState` pattern.
// - Validation via `use_form_validation_state` + inline builtin validation
//   for min/max/isDateUnavailable (React-aria uses `useFormValidationState`
//   with a separate `builtinValidation` prop).
//

/// Input parameters for [`use_date_picker_state`].
pub struct UseDatePickerStateInput {
    /// The initial date value.
    pub default_value: Option<time::OffsetDateTime>,

    /// The minimum allowed date.
    pub min: Option<time::OffsetDateTime>,

    /// The maximum allowed date.
    pub max: Option<time::OffsetDateTime>,

    /// Whether the picker includes a time portion.
    pub show_time: bool,

    /// Whether to auto-close the popover when a date is selected.
    /// Defaults to `true`.
    pub should_close_on_select: bool,

    /// Callback to check if a specific date is unavailable.
    pub is_date_unavailable: Option<Callback<time::Date, bool>>,

    /// Callback when the value changes.
    pub on_change: Option<Callback<Option<time::OffsetDateTime>>>,

    /// Callback when the open state changes.
    pub on_open_change: Option<Callback<bool>>,

    /// Whether the picker is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the picker is read-only.
    pub is_read_only: Signal<bool>,

    /// Whether the field is explicitly invalid (controlled).
    pub is_invalid: Option<Signal<bool>>,

    /// Custom validation function.
    pub validate: Option<ValidateFn<Option<time::OffsetDateTime>>>,

    /// Validation behavior mode.
    pub validation_behavior: ValidationBehavior,

    /// The field's name for form validation context matching.
    pub name: Option<String>,
}

impl Default for UseDatePickerStateInput {
    fn default() -> Self {
        Self {
            default_value: None,
            min: None,
            max: None,
            show_time: false,
            should_close_on_select: true,
            is_date_unavailable: None,
            on_change: None,
            on_open_change: None,
            is_disabled: Signal::derive(|| false),
            is_read_only: Signal::derive(|| false),
            is_invalid: None,
            validate: None,
            validation_behavior: ValidationBehavior::default(),
            name: None,
        }
    }
}

/// State for managing a date picker with staged value commitment.
///
/// Separates the committed `value` from staged `date_value`/`time_value`
/// which represent in-progress selection. When `show_time` is true, date
/// and time are staged independently and only committed when both are ready
/// or when the popover closes.
#[derive(Clone, Copy)]
pub struct UseDatePickerStateReturn {
    /// The committed value.
    pub value: Signal<Option<time::OffsetDateTime>>,

    /// Set the committed value directly.
    pub set_value: Callback<Option<time::OffsetDateTime>>,

    /// The effective date portion (committed value's date, or staged date).
    pub date_value: Signal<Option<time::Date>>,

    /// Set the date via calendar selection. Implements staged commit logic.
    pub set_date_value: Callback<time::Date>,

    /// The effective time portion (committed value's time, or staged time).
    pub time_value: Signal<Option<TimeValue>>,

    /// Set the time via time field. Commits if a date is already staged.
    pub set_time_value: Callback<TimeValue>,

    /// Whether the picker popover is open.
    pub is_open: Signal<bool>,

    /// Set the open state. Commits staged values when closing if needed.
    pub set_open: Callback<bool>,

    /// Open the picker.
    pub open: Callback<()>,

    /// Close the picker (commits staged values if applicable).
    pub close: Callback<()>,

    /// Clear the value and staged selections.
    pub clear: Callback<()>,

    /// Whether the picker includes a time portion.
    pub has_time: bool,

    /// Form validation state (for `commit_validation`, `reset_validation`, etc.).
    pub validation: UseFormValidationStateReturn,

    /// Whether the value is invalid (form + builtin validation combined).
    pub is_invalid: Signal<bool>,

    /// Validation error messages (form + builtin combined).
    pub validation_errors: Signal<Vec<String>>,

    /// The current value formatted as a human-readable string.
    /// Empty string when no value is set.
    pub formatted_value: Signal<String>,
}

/// Creates internal state for a date picker.
///
/// Manages the committed value, staged date/time selections, open state with
/// commit-on-close intercept, and form validation. Implements the staged
/// commit pattern from React-aria:
///
/// - **Date-only** (`show_time = false`): Calendar selection commits immediately.
/// - **Date+time** (`show_time = true`): Date and time are staged separately.
///   The value commits when both are set, when `should_close_on_select` triggers,
///   or when the popover closes with a staged date (using a placeholder time).
///
/// # Panics
///
/// Internal closures panic if `time::Date::with_hms` fails, which should not
/// happen since `TimeValue` constrains hour/minute/second to valid ranges.
#[allow(clippy::too_many_lines)]
pub fn use_date_picker_state(input: UseDatePickerStateInput) -> UseDatePickerStateReturn {
    let UseDatePickerStateInput {
        default_value,
        min,
        max,
        show_time,
        should_close_on_select,
        is_date_unavailable,
        on_change,
        on_open_change,
        is_disabled,
        is_read_only,
        is_invalid,
        validate,
        validation_behavior,
        name,
    } = input;

    let has_time = show_time;

    // ---- Value signal (hook-owned) ----
    let (value, set_value_signal) = signal(default_value);
    let value_signal: Signal<Option<time::OffsetDateTime>> = value.into();

    // ---- Staged date/time ----
    let (selected_date, set_selected_date) = signal::<Option<time::Date>>(None);
    let (selected_time, set_selected_time) = signal::<Option<TimeValue>>(None);

    // Derived: effective date/time (committed value takes precedence, then staged).
    let date_value = Signal::derive(move || {
        value
            .get()
            .map(time::OffsetDateTime::date)
            .or_else(|| selected_date.get())
    });
    let time_value = Signal::derive(move || {
        value
            .get()
            .map(|v| TimeValue::new(v.hour(), v.minute(), v.second()))
            .or_else(|| selected_time.get())
    });

    // ---- Open state ----
    let (is_open, set_is_open_signal) = signal(false);

    // ---- Emit helper ----
    let emit = move |new_value: Option<time::OffsetDateTime>| {
        set_value_signal.set(new_value);
        if let Some(cb) = on_change {
            cb.run(new_value);
        }
    };

    // ---- Commit helper: date + time → OffsetDateTime ----
    let commit_value = move |date: time::Date, time_val: TimeValue| {
        let offset = value
            .get_untracked()
            .or(default_value)
            .map_or(time::UtcOffset::UTC, time::OffsetDateTime::offset);
        let datetime = date
            .with_hms(time_val.hour, time_val.minute, time_val.second)
            .expect("TimeValue ensures valid time components")
            .assume_offset(offset);

        emit(Some(datetime));
        set_selected_date.set(None);
        set_selected_time.set(None);
    };

    let placeholder_time = move || {
        default_value
            .map(|v| TimeValue::new(v.hour(), v.minute(), v.second()))
            .unwrap_or_default()
    };

    // ---- Form validation ----
    let validation = use_form_validation_state(UseFormValidationStateInput {
        is_invalid,
        value: value_signal,
        validate,
        validation_behavior,
        name,
    });

    // ---- Builtin validation (min/max/unavailable) ----
    let builtin_invalid = Signal::derive(move || {
        let Some(v) = value.get() else {
            return false;
        };
        if min.is_some_and(|min_val| v < min_val) {
            return true;
        }
        if max.is_some_and(|max_val| v > max_val) {
            return true;
        }
        is_date_unavailable.is_some_and(|cb| cb.run(v.date()))
    });

    let builtin_errors = Signal::derive(move || {
        let Some(v) = value.get() else {
            return Vec::new();
        };
        let mut errors = Vec::new();
        if min.is_some_and(|min_val| v < min_val) {
            errors.push("Date is before the minimum allowed date.".to_string());
        }
        if max.is_some_and(|max_val| v > max_val) {
            errors.push("Date is after the maximum allowed date.".to_string());
        }
        if is_date_unavailable.is_some_and(|cb| cb.run(v.date())) {
            errors.push("Selected date is unavailable.".to_string());
        }
        errors
    });

    let combined_is_invalid =
        Signal::derive(move || validation.is_invalid.get() || builtin_invalid.get());
    let combined_errors = Signal::derive(move || {
        let mut errors = validation.validation_errors.get();
        errors.extend(builtin_errors.get());
        errors
    });

    // ---- Open state management ----
    let notify_open = move |new_open: bool| {
        set_is_open_signal.set(new_open);
        if let Some(cb) = on_open_change {
            cb.run(new_open);
        }
    };

    // Set open with commit-on-close intercept.
    // When closing with a staged date, no committed value, and has_time,
    // commit the staged date with a placeholder time.
    let do_set_open = move |new_open: bool| {
        if new_open && (is_disabled.get_untracked() || is_read_only.get_untracked()) {
            return;
        }

        if is_open.get_untracked() && !new_open && value.get_untracked().is_none() && has_time {
            if let Some(date) = selected_date.get_untracked() {
                commit_value(
                    date,
                    selected_time
                        .get_untracked()
                        .unwrap_or_else(placeholder_time),
                );
            }
        }

        notify_open(new_open);
    };

    // ---- Date setter (from calendar) ----
    let set_date_value = Callback::new(move |new_date: time::Date| {
        if has_time {
            let current_time = selected_time.get_untracked();
            if current_time.is_some() || should_close_on_select {
                commit_value(new_date, current_time.unwrap_or_else(placeholder_time));
            } else {
                set_selected_date.set(Some(new_date));
            }
        } else {
            let time_part = value.get_untracked().map_or_else(placeholder_time, |v| {
                TimeValue::new(v.hour(), v.minute(), v.second())
            });
            commit_value(new_date, time_part);
            validation.commit_validation.run(());
        }

        if should_close_on_select {
            do_set_open(false);
        }
    });

    // ---- Time setter (from time field) ----
    let set_time_value = Callback::new(move |new_time: TimeValue| {
        if let Some(date) = selected_date.get_untracked() {
            commit_value(date, new_time);
        } else {
            set_selected_time.set(Some(new_time));
        }
    });

    // ---- Public open/close/set_open ----
    let set_open = Callback::new(move |new_open: bool| do_set_open(new_open));
    let open = Callback::new(move |()| do_set_open(true));
    let close = Callback::new(move |()| do_set_open(false));

    // ---- Direct value setter ----
    let set_value = Callback::new(move |new_value: Option<time::OffsetDateTime>| {
        emit(new_value);
        set_selected_date.set(None);
        set_selected_time.set(None);
        validation.commit_validation.run(());
    });

    // ---- Clear ----
    let clear = Callback::new(move |()| {
        emit(None);
        set_selected_date.set(None);
        set_selected_time.set(None);
    });

    // ---- Formatted value (for accessibility description) ----
    let formatted_value = Signal::derive(move || {
        value.with(|v| match v {
            Some(dt) => {
                if has_time {
                    dt.format(format_description!(
                        "[month repr:long] [day padding:none], [year] at [hour]:[minute]"
                    ))
                    .unwrap_or_default()
                } else {
                    dt.format(format_description!(
                        "[month repr:long] [day padding:none], [year]"
                    ))
                    .unwrap_or_default()
                }
            }
            None => String::new(),
        })
    });

    UseDatePickerStateReturn {
        value: value_signal,
        set_value,
        date_value,
        set_date_value,
        time_value,
        set_time_value,
        is_open: is_open.into(),
        set_open,
        open,
        close,
        clear,
        has_time,
        validation,
        is_invalid: combined_is_invalid,
        validation_errors: combined_errors,
        formatted_value,
    }
}
