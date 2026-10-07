// Upstream: react-stately/src/datepicker/useDatePickerState.ts @ 99e6102368
use std::sync::Arc;

use jiff::civil::{Date, Time};
use leptos::prelude::*;

use super::{
    format::{DateFormatter, FormatOptions},
    types::{DateValue, Era, Granularity, HourCycle, MaxGranularity},
    use_date_field_state::{resolve_granularity, validation_result},
};
use crate::{
    hooks::{
        OverlayTriggerState, UseOverlayTriggerStateInput,
        form::{
            FormValidationState, UseFormValidationStateInput, ValidateFn, ValidationBehavior,
            use_form_validation_state,
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
// - The format options and the placeholder are signals (C11).
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
    /// The value the field starts from when edited, its time for dates selected in the
    /// calendar, and the month the calendar opens on.
    pub placeholder_value: Signal<Option<V>>,
    pub min_value: Signal<Option<V>>,
    pub max_value: Signal<Option<V>>,
    pub is_date_unavailable: Option<Callback<V, bool>>,
    /// The finest unit. Default: the minute for values with a time, else the day.
    pub granularity: Signal<Option<Granularity>>,
    /// 12 or 24 hours. Default: the locale's.
    pub hour_cycle: Signal<Option<HourCycle>>,
    pub hide_time_zone: Signal<bool>,
    pub should_force_leading_zeros: Signal<bool>,
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
            placeholder_value: Signal::stored(None),
            min_value: Signal::stored(None),
            max_value: Signal::stored(None),
            is_date_unavailable: None,
            granularity: Signal::stored(None),
            hour_cycle: Signal::stored(None),
            hide_time_zone: Signal::stored(false),
            should_force_leading_zeros: Signal::stored(false),
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
    pub granularity: Signal<Granularity>,
    /// Whether the value has a time (selecting a date then waits for one, unless it closes).
    pub has_time: Signal<bool>,
    pub overlay: OverlayTriggerState,
    pub is_invalid: Signal<bool>,
    pub validation: FormValidationState,
    pub(crate) binding: ValueBinding<Option<V>>,
    pub(crate) format_options: Memo<FormatOptions>,
    selected_date: RwSignal<Option<Date>>,
    selected_time: RwSignal<Option<Time>>,
    placeholder: Memo<V>,
    placeholder_time: Signal<Time>,
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
        self.validation.commit_validation();
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
        if self.has_time.get_untracked() {
            match self.time_value.get_untracked() {
                Some(time) => self.commit(date, time),
                None if should_close => self.commit(date, self.placeholder_time.get_untracked()),
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

    let granularity = Signal::derive(move || resolve_granularity::<V>(granularity.get()));
    let has_time = Signal::derive(move || granularity.get().has_time());
    let default_value = StoredValue::new(default_value);
    // The time of dates selected in the calendar without one: the default value's or the
    // placeholder's (react-aria's `getPlaceholderTime`), else midnight.
    let placeholder_time = Signal::derive(move || {
        default_value
            .get_value()
            .or_else(|| placeholder_value.get())
            .filter(|_| V::HAS_TIME)
            .map_or(Time::midnight(), |value| value.time())
    });
    let default_time_zone = Signal::derive(move || {
        default_value
            .get_value()
            .or_else(|| placeholder_value.get())
            .and_then(|value| value.time_zone().cloned())
    });
    // The time zone of zoned values: the value's, else the last one's (react-aria's
    // `useDefaultProps`), else the default value's or placeholder's.
    let time_zone = Memo::new(move |previous: Option<&Option<jiff::tz::TimeZone>>| {
        binding
            .value
            .get()
            .and_then(|value| value.time_zone().cloned())
            .or_else(|| previous.cloned().flatten())
            .or_else(|| default_time_zone.get())
    });
    let placeholder = Memo::new(move |_| {
        placeholder_value
            .get()
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
        granularity: granularity.get(),
        max_granularity: MaxGranularity::Year,
        time_zone: time_zone.get(),
        hide_time_zone: hide_time_zone.get(),
        hour_cycle: hour_cycle.get(),
        show_era: binding
            .value
            .get()
            .is_some_and(|value| Era::of(value.date().year()).0 == Era::Bc),
        should_force_leading_zeros: should_force_leading_zeros.get(),
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
        if has_time.get_untracked()
            && binding.value.get_untracked().is_none()
            && let Some(date) = selected_date.get_untracked()
        {
            let time = selected_time
                .get_untracked()
                .unwrap_or_else(|| placeholder_time.get_untracked());
            let base = placeholder.get_untracked();
            binding.set(Some(base.with_fields(date, time, None)));
            selected_date.set(None);
            selected_time.set(None);
            validation.commit_validation();
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
