// Upstream: react-stately/src/datepicker/useDatePickerState.ts @ 99e6102368
use std::sync::Arc;

use jiff::civil::{Date, Time};
use leptos::prelude::*;

use super::{
    format::{DateFormatter, FormatOptions},
    types::{DateValue, Era, Granularity, HourCycle, MaxGranularity},
    use_date_field_state::validation_result,
};
use crate::{
    hooks::{
        OverlayTriggerState, UseOverlayTriggerStateInput,
        form::{
            UseFormValidationStateInput, UseFormValidationStateReturn, ValidateFn,
            ValidationBehavior, use_form_validation_state,
        },
        use_overlay_trigger_state,
    },
    utils::{ValueBinding, i18n::use_locale},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Generic over the value type (`V: DateValue`), as the date field.
// - The calendar's selection is a `civil::Date` and the time a `civil::Time` (react-aria: date
//   values); the value is built with the type and time zone of the value or placeholder.
// - Closing commits a selected date through the overlay state's `on_open_change` (react-aria
//   wraps `setOpen`), so that a popover closing itself commits as well.
// - Hook-owned value (C4): `default_value` + `on_change`, or a binding to app state.
//
// ## OMITTED FEATURES
// - `shouldCloseOnSelect` as a function: a signal.
//
// =============================================================================

/// Input of [`use_date_picker_state`].
pub struct UseDatePickerStateInput<V: DateValue> {
    pub default_value: Option<V>,
    pub value: Option<ValueBinding<Option<V>>>,
    pub on_change: Option<Callback<Option<V>>>,
    pub placeholder_value: Option<V>,
    pub min_value: Signal<Option<V>>,
    pub max_value: Signal<Option<V>>,
    pub is_date_unavailable: Option<Callback<V, bool>>,
    pub granularity: Option<Granularity>,
    pub hour_cycle: Option<HourCycle>,
    pub hide_time_zone: bool,
    pub should_force_leading_zeros: bool,
    /// Whether selecting a date closes the popover. Default: `true`.
    pub should_close_on_select: Signal<bool>,
    pub default_open: bool,
    pub is_open: Option<ValueBinding<bool>>,
    pub on_open_change: Option<Callback<bool>>,
    pub is_invalid: Signal<bool>,
    pub validate: Option<ValidateFn<Option<V>>>,
    pub validation_behavior: ValidationBehavior,
    pub name: Option<String>,
}

impl<V: DateValue> Default for UseDatePickerStateInput<V> {
    fn default() -> Self {
        Self {
            default_value: None,
            value: None,
            on_change: None,
            placeholder_value: None,
            min_value: Signal::stored(None),
            max_value: Signal::stored(None),
            is_date_unavailable: None,
            granularity: None,
            hour_cycle: None,
            hide_time_zone: false,
            should_force_leading_zeros: false,
            should_close_on_select: Signal::stored(true),
            default_open: false,
            is_open: None,
            on_open_change: None,
            is_invalid: Signal::stored(false),
            validate: None,
            validation_behavior: ValidationBehavior::default(),
            name: None,
        }
    }
}

/// The state of a date picker: its value, the date and time selected in its popover, and
/// whether the popover is open.
pub struct DatePickerState<V: DateValue> {
    pub value: Signal<Option<V>>,
    /// The calendar's date: the one selected in the popover, else the value's.
    pub date_value: Signal<Option<Date>>,
    /// The time: the one selected in the popover, else the value's.
    pub time_value: Signal<Option<Time>>,
    pub granularity: Granularity,
    /// Whether the value has a time (selecting a date then waits for one, unless it closes).
    pub has_time: bool,
    pub overlay: OverlayTriggerState,
    pub is_invalid: Signal<bool>,
    pub validation: UseFormValidationStateReturn,
    pub(crate) binding: ValueBinding<Option<V>>,
    pub(crate) format_options: Memo<FormatOptions>,
    selected_date: RwSignal<Option<Date>>,
    selected_time: RwSignal<Option<Time>>,
    placeholder: Memo<V>,
    placeholder_time: Time,
    should_close_on_select: Signal<bool>,
    locale: Signal<crate::utils::i18n::Locale>,
}

// Derived, it would require a `Copy` value type.
#[allow(clippy::expl_impl_clone_on_copy)]
impl<V: DateValue> Clone for DatePickerState<V> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<V: DateValue> Copy for DatePickerState<V> {}

impl<V: DateValue> DatePickerState<V> {
    pub fn set_value(&self, value: Option<V>) {
        self.binding.set(value);
    }

    fn commit(&self, date: Date, time: Time) {
        let base = self
            .binding
            .value
            .get_untracked()
            .unwrap_or_else(|| self.placeholder.get_untracked());
        self.binding.set(Some(base.with_fields(date, time, None)));
        self.selected_date.set(None);
        self.selected_time.set(None);
        self.validation.commit_validation.run(());
    }

    /// A date as a value: on the time and in the zone of the value, else of the placeholder.
    pub fn date_to_value(&self, date: Date) -> V {
        let base = self
            .binding
            .value
            .get_untracked()
            .unwrap_or_else(|| self.placeholder.get_untracked());
        base.with_fields(date, base.time(), None)
    }

    /// Selects a date in the calendar (keeping the time).
    pub fn select_date(&self, date: Date) {
        let should_close = self.should_close_on_select.get_untracked();
        if self.has_time {
            match self.time_value.get_untracked() {
                Some(time) => self.commit(date, time),
                None if should_close => self.commit(date, self.placeholder_time),
                None => self.selected_date.set(Some(date)),
            }
        } else {
            self.commit(date, Time::midnight());
        }
        if should_close {
            self.overlay.set_open(false);
        }
    }

    /// Selects a time (committed with a selected date).
    pub fn select_time(&self, time: Time) {
        match self.date_value.get_untracked() {
            Some(date) => self.commit(date, time),
            None => self.selected_time.set(Some(time)),
        }
    }

    /// Opens or closes the popover (closing commits a date selected without a time).
    pub fn set_open(&self, is_open: bool) {
        self.overlay.set_open(is_open);
    }

    /// The value formatted for descriptions ("June 15, 2024"); empty without a value.
    pub fn format_value(&self) -> String {
        let Some(value) = self.value.get() else {
            return String::new();
        };
        DateFormatter::long(&self.locale.get(), &self.format_options.get()).format(&value)
    }
}

/// State of a date picker (react-stately's `useDatePickerState`): a date field's value with a
/// popover calendar (and time) to pick it; validated as the field.
#[allow(clippy::too_many_lines)]
pub fn use_date_picker_state<V: DateValue>(
    input: UseDatePickerStateInput<V>,
) -> DatePickerState<V> {
    let UseDatePickerStateInput {
        default_value,
        value,
        on_change,
        placeholder_value,
        min_value,
        max_value,
        is_date_unavailable,
        granularity,
        hour_cycle,
        hide_time_zone,
        should_force_leading_zeros,
        should_close_on_select,
        default_open,
        is_open,
        on_open_change,
        is_invalid,
        validate,
        validation_behavior,
        name,
    } = input;
    let locale = use_locale();

    let owned_value =
        value.unwrap_or_else(|| ValueBinding::from(RwSignal::new(default_value.clone())));
    let binding = ValueBinding::new(
        owned_value.value,
        Callback::new(move |value: Option<V>| {
            // Only changes (react-aria's `useControlledState`).
            if owned_value
                .value
                .with_untracked(|current| *current == value)
            {
                return;
            }
            owned_value.set(value.clone());
            if let Some(on_change) = on_change {
                on_change.run(value);
            }
        }),
    );

    let granularity = match granularity {
        Some(granularity) if granularity.has_time() && !V::HAS_TIME => Granularity::Day,
        Some(granularity) => granularity,
        None if V::HAS_TIME => Granularity::Minute,
        None => Granularity::Day,
    };
    let has_time = granularity.has_time();
    let placeholder_time = default_value
        .as_ref()
        .or(placeholder_value.as_ref())
        .filter(|_| V::HAS_TIME)
        .map_or(Time::midnight(), DateValue::time);
    let default_time_zone = default_value
        .as_ref()
        .or(placeholder_value.as_ref())
        .and_then(|value| value.time_zone().cloned());
    let placeholder_value = StoredValue::new(placeholder_value);
    // The time zone of zoned values: the value's, else the last one's (react-aria's
    // `useDefaultProps`), else the default value's or placeholder's.
    let time_zone = Memo::new(move |previous: Option<&Option<jiff::tz::TimeZone>>| {
        binding
            .value
            .get()
            .and_then(|value| value.time_zone().cloned())
            .or_else(|| previous.cloned().flatten())
            .or_else(|| default_time_zone.clone())
    });
    let placeholder = Memo::new(move |_| {
        placeholder_value
            .get_value()
            .unwrap_or_else(|| V::today(time_zone.get().as_ref()))
    });

    let selected_date = RwSignal::new(None::<Date>);
    let selected_time = RwSignal::new(None::<Time>);
    // The value's date and time, else the selected ones (react-aria: a value replaces the
    // selection).
    let date_value = Signal::derive(move || {
        binding
            .value
            .get()
            .map(|value| value.date())
            .or_else(|| selected_date.get())
    });
    let time_value = Signal::derive(move || {
        binding
            .value
            .get()
            .filter(|_| V::HAS_TIME)
            .map(|value| value.time())
            .or_else(|| selected_time.get())
    });

    let format_options = Memo::new(move |_| FormatOptions {
        granularity,
        max_granularity: MaxGranularity::Year,
        time_zone: time_zone.get(),
        hide_time_zone,
        hour_cycle,
        show_era: binding
            .value
            .get()
            .is_some_and(|value| Era::of(value.date().year()).0 == Era::Bc),
        should_force_leading_zeros,
    });
    let is_date_unavailable = StoredValue::new(is_date_unavailable);
    let builtin_validation = Signal::derive(move || {
        let value = binding.value.get();
        let (min, max) = (min_value.get(), max_value.get());
        let formatter = DateFormatter::new(&locale.get(), &format_options.get());
        Some(validation_result(
            value.as_ref(),
            min.as_ref(),
            max.as_ref(),
            is_date_unavailable.get_value(),
            &formatter,
        ))
    });
    let validation = use_form_validation_state(UseFormValidationStateInput {
        is_invalid,
        value: binding.value,
        validate,
        builtin_validation,
        validation_behavior,
        name,
    });

    // Closing commits a date selected without a time, with the placeholder time (a time selected
    // without a date stays until the popover opens again).
    let commit_on_close: Arc<dyn Fn() + Send + Sync> = Arc::new(move || {
        if has_time
            && binding.value.get_untracked().is_none()
            && let Some(date) = selected_date.get_untracked()
        {
            let time = selected_time.get_untracked().unwrap_or(placeholder_time);
            let base = placeholder.get_untracked();
            binding.set(Some(base.with_fields(date, time, None)));
            selected_date.set(None);
            selected_time.set(None);
            validation.commit_validation.run(());
        }
    });
    let overlay = use_overlay_trigger_state(UseOverlayTriggerStateInput {
        default_open,
        value: is_open,
        on_open_change: Some(Callback::new(move |open: bool| {
            if !open {
                commit_on_close();
            }
            if let Some(on_open_change) = on_open_change {
                on_open_change.run(open);
            }
        })),
    });

    DatePickerState {
        value: binding.value,
        date_value,
        time_value,
        granularity,
        has_time,
        overlay,
        is_invalid: validation.is_invalid,
        validation,
        binding,
        format_options,
        selected_date,
        selected_time,
        placeholder,
        placeholder_time,
        should_close_on_select,
        locale,
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;
    use jiff::civil::{DateTime, date, time};

    use super::*;

    #[test]
    fn selects_a_date_and_closes() {
        let owner = Owner::new();
        owner.with(|| {
            let state = use_date_picker_state(UseDatePickerStateInput::<Date>::default());
            state.set_open(true);
            state.select_date(date(2024, 6, 5));
            assert_that!(state.value.get_untracked()).is_equal_to(Some(date(2024, 6, 5)));
            assert_that!(state.overlay.is_open.get_untracked()).is_false();
        });
    }

    #[test]
    fn keeps_the_time_of_a_date_time() {
        let owner = Owner::new();
        owner.with(|| {
            let state = use_date_picker_state(UseDatePickerStateInput::<DateTime> {
                default_value: Some(date(2024, 6, 5).at(14, 30, 0, 0)),
                ..UseDatePickerStateInput::default()
            });
            state.select_date(date(2024, 6, 20));
            assert_that!(state.value.get_untracked())
                .is_equal_to(Some(date(2024, 6, 20).at(14, 30, 0, 0)));
        });
    }

    #[test]
    fn waits_for_a_time_unless_closing() {
        let owner = Owner::new();
        owner.with(|| {
            let state = use_date_picker_state(UseDatePickerStateInput::<DateTime> {
                should_close_on_select: Signal::stored(false),
                ..UseDatePickerStateInput::default()
            });
            state.set_open(true);
            state.select_date(date(2024, 6, 20));
            assert_that!(state.value.get_untracked()).is_none();
            assert_that!(state.date_value.get_untracked()).is_equal_to(Some(date(2024, 6, 20)));
            state.select_time(time(9, 15, 0, 0));
            assert_that!(state.value.get_untracked())
                .is_equal_to(Some(date(2024, 6, 20).at(9, 15, 0, 0)));
        });
    }
}
