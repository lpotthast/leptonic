// Upstream: react-stately/src/datepicker/useTimeFieldState.ts @ 99e6102368
// Upstream: react-aria-components/test/TimeField.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/datepicker/TimeField.test.js @ 99e6102368
use std::sync::Arc;

use jiff::civil::{Date, Time};
use leptos::prelude::*;

use super::{
    types::{DateValue, Granularity, HourCycle, MaxGranularity, TimeBound, TimeValue},
    use_date_field_state::{DateFieldState, UseDateFieldStateInput, use_date_field_state},
};
use crate::{
    ValueBinding,
    hooks::form::{ValidateFn, ValidationBehavior},
    utils::date::today,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Generic over the value type (`T: TimeValue`: `civil::Time`, `civil::DateTime`, `Zoned`);
//   the placeholder has that type as well. The date field state it is built on is `field`
//   (react-aria: one merged state).
// - Hook-owned value (C4): `default_value` + `on_change`, or a binding to app state.
// - The format options are signals (C11), as in `use_date_field_state`.
// - `min_value`/`max_value` are `TimeBound`s (react-aria: any `TimeValue`, told apart at
//   runtime): a time of day (`TimeOfDay`, react-aria's `Time` bound, on the value's day) or an
//   absolute bound of the value type (`Absolute`, react-aria's `CalendarDateTime` and
//   `ZonedDateTime` bounds). A field of times takes only times of day, and a date-time bound has
//   the field's value type (react-aria also compares a zoned value with a `CalendarDateTime`).
//
// =============================================================================

/// Input of [`use_time_field_state`].
pub struct UseTimeFieldStateInput<T: TimeValue> {
    pub default_value: Option<T>,
    /// The value as app state, replacing `default_value`.
    pub value: Option<ValueBinding<Option<T>>>,
    pub on_change: Option<Callback<Option<T>>>,
    /// The time the segments start from when edited. Default: midnight.
    pub placeholder_value: Signal<Option<T>>,
    /// The earliest valid value: a time of day (on the value's day) or a date and time.
    pub min_value: Signal<Option<TimeBound<T>>>,
    /// The latest valid value: a time of day (on the value's day) or a date and time.
    pub max_value: Signal<Option<TimeBound<T>>>,
    /// The finest unit: hour, minute (default) or second.
    pub granularity: Signal<Option<Granularity>>,
    /// 12 or 24 hours. Default: the locale's.
    pub hour_cycle: Signal<Option<HourCycle>>,
    pub hide_time_zone: Signal<bool>,
    pub should_force_leading_zeros: Signal<bool>,
    pub is_disabled: Signal<bool>,
    pub is_read_only: Signal<bool>,
    pub is_required: Signal<bool>,
    pub is_invalid: Signal<bool>,
    pub validate: Option<ValidateFn<Option<T>>>,
    pub validation_behavior: ValidationBehavior,
    pub name: Option<String>,
}

impl<T: TimeValue> Default for UseTimeFieldStateInput<T> {
    fn default() -> Self {
        Self {
            default_value: None,
            value: None,
            on_change: None,
            placeholder_value: Signal::stored(None),
            min_value: Signal::stored(None),
            max_value: Signal::stored(None),
            granularity: Signal::stored(None),
            hour_cycle: Signal::stored(None),
            hide_time_zone: Signal::stored(false),
            should_force_leading_zeros: Signal::stored(false),
            is_disabled: Signal::stored(false),
            is_read_only: Signal::stored(false),
            is_required: Signal::stored(false),
            is_invalid: Signal::stored(false),
            validate: None,
            validation_behavior: ValidationBehavior::default(),
            name: None,
        }
    }
}

/// The state of a time field: its value, and the date field state that edits it.
pub struct TimeFieldState<T: TimeValue> {
    pub value: Signal<Option<T>>,
    /// The time of the value.
    pub time_value: Signal<Option<Time>>,
    /// The segments and their editing (the value as a date value).
    pub field: DateFieldState<T::Field>,
}

// Derived, it would require a `Copy` value type.
#[allow(clippy::expl_impl_clone_on_copy)]
impl<T: TimeValue> Clone for TimeFieldState<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: TimeValue> Copy for TimeFieldState<T> {}

/// State of a time field: a date field of hours to seconds (with a day period in 12-hour
/// cycles), whose value is a time (or the time of a date value).
pub fn use_time_field_state<T: TimeValue>(input: UseTimeFieldStateInput<T>) -> TimeFieldState<T> {
    let UseTimeFieldStateInput {
        default_value,
        value,
        on_change,
        placeholder_value,
        min_value,
        max_value,
        granularity,
        hour_cycle,
        hide_time_zone,
        should_force_leading_zeros,
        is_disabled,
        is_read_only,
        is_required,
        is_invalid,
        validate,
        validation_behavior,
        name,
    } = input;

    // The time zone of zoned values: the value's, else the default value's.
    let time_zone = value
        .as_ref()
        .and_then(|value| value.value.get_untracked())
        .or_else(|| default_value.clone())
        .and_then(|value| value.to_field(today()).time_zone().cloned());
    let owned_value = value.unwrap_or_else(|| ValueBinding::from(RwSignal::new(default_value)));
    // Times are edited on today's date (dates and zoned values on their own).
    let day: Date = today();
    let to_field = move |value: Option<T>| value.map(|value| value.to_field(day));
    let field_binding = ValueBinding::new(
        Signal::derive(move || to_field(owned_value.value.get())),
        Callback::new(move |field: Option<T::Field>| {
            let value = field.map(T::from_field);
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
    let validate = validate.map(|validate| {
        Arc::new(move |field: &Option<T::Field>| validate(&field.clone().map(T::from_field)))
            as ValidateFn<Option<T::Field>>
    });
    let placeholder = Signal::derive(move || {
        placeholder_value.get().map_or_else(
            || T::Field::today(time_zone.as_ref()).with_fields(day, Time::midnight(), None),
            |placeholder| placeholder.to_field(day),
        )
    });
    // A bound as the field's value: a time of day on the value's day (else the placeholder's),
    // in its zone, a date-time as is (react-aria's `convertValue(minValue, day)`).
    let bound = move |bound: Option<TimeBound<T>>| {
        bound.map(|bound| {
            bound.to_field(|| {
                field_binding
                    .value
                    .get()
                    .unwrap_or_else(|| placeholder.get())
            })
        })
    };
    let field = use_date_field_state(UseDateFieldStateInput {
        default_value: None,
        value: Some(field_binding),
        on_change: None,
        placeholder_value: Signal::derive(move || Some(placeholder.get())),
        min_value: Signal::derive(move || bound(min_value.get())),
        max_value: Signal::derive(move || bound(max_value.get())),
        is_date_unavailable: None,
        granularity: Signal::derive(move || Some(granularity.get().unwrap_or(Granularity::Minute))),
        max_granularity: Signal::stored(MaxGranularity::Hour),
        hour_cycle,
        hide_time_zone,
        should_force_leading_zeros,
        is_disabled,
        is_read_only,
        is_required,
        is_invalid,
        validate,
        validation_behavior,
        name,
        validation: None,
    });

    TimeFieldState {
        value: owned_value.value,
        time_value: Signal::derive(move || {
            owned_value
                .value
                .get()
                .map(|value| value.to_field(day).time())
        }),
        field,
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;
    use jiff::{
        Zoned,
        civil::{DateTime, date, time},
    };

    use super::*;
    use crate::{hooks::datepicker::DateSegmentType, testing::with_owner};

    /// The realtime validation of a time field: whether it is invalid, and its errors.
    fn validation<T: TimeValue>(state: &TimeFieldState<T>) -> (bool, Vec<String>) {
        let validation = state.field.validation.realtime_validation.get_untracked();
        (validation.is_invalid, validation.validation_errors)
    }

    fn valid() -> (bool, Vec<String>) {
        (false, Vec::new())
    }

    fn invalid(error: &str) -> (bool, Vec<String>) {
        (true, vec![error.to_owned()])
    }

    fn zoned(text: &str) -> Zoned {
        text.parse().expect("a zoned value")
    }

    /// react-spectrum's `TimeField.test.js`, "supports minValue and maxValue"
    /// (`validationBehavior=aria`): 8:00 is before 9:00, stepping the hour up makes it valid,
    /// stepping to 21:00 is not constrained but invalid.
    #[test]
    fn validates_times_against_times_of_day() {
        with_owner(|| {
            let state = use_time_field_state(UseTimeFieldStateInput::<Time> {
                default_value: Some(time(8, 0, 0, 0)),
                min_value: Signal::stored(Some(time(9, 0, 0, 0).into())),
                max_value: Signal::stored(Some(time(17, 0, 0, 0).into())),
                ..UseTimeFieldStateInput::default()
            });
            assert_that!(validation(&state))
                .is_equal_to(invalid("Value must be 9:00\u{202f}AM or later."));
            state.field.increment(DateSegmentType::Hour);
            assert_that!(state.value.get_untracked()).is_equal_to(Some(time(9, 0, 0, 0)));
            assert_that!(validation(&state)).is_equal_to(valid());
            state.field.increment(DateSegmentType::DayPeriod);
            assert_that!(state.value.get_untracked()).is_equal_to(Some(time(21, 0, 0, 0)));
            assert_that!(validation(&state))
                .is_equal_to(invalid("Value must be 5:00\u{202f}PM or earlier."));
            state.field.decrement(DateSegmentType::DayPeriod);
            assert_that!(validation(&state)).is_equal_to(valid());
        });
    }

    /// A time field of date-times bounded by times of day: "not before 9:00" on any day
    /// (react-stately's `useTimeFieldState`, `convertValue(minValue, day)`).
    #[test]
    fn times_of_day_bound_the_values_day() {
        with_owner(|| {
            let value = RwSignal::new(Some(date(2024, 6, 5).at(8, 0, 0, 0)));
            let state = use_time_field_state(UseTimeFieldStateInput::<DateTime> {
                value: Some(ValueBinding::from(value)),
                min_value: Signal::stored(Some(TimeBound::TimeOfDay(time(9, 0, 0, 0)))),
                max_value: Signal::stored(Some(TimeBound::TimeOfDay(time(17, 0, 0, 0)))),
                ..UseTimeFieldStateInput::default()
            });
            assert_that!(validation(&state))
                .is_equal_to(invalid("Value must be 9:00\u{202f}AM or later."));
            value.set(Some(date(2030, 1, 1).at(10, 0, 0, 0)));
            assert_that!(validation(&state)).is_equal_to(valid());
            value.set(Some(date(2030, 1, 1).at(17, 30, 0, 0)));
            assert_that!(validation(&state))
                .is_equal_to(invalid("Value must be 5:00\u{202f}PM or earlier."));
        });
    }

    /// A date-time bound is absolute (react-stately's `convertValue` keeps a value with a day):
    /// 8:00 is valid on a later day than the minimum's, invalid on its day.
    #[test]
    fn date_times_bound_absolutely() {
        with_owner(|| {
            let value = RwSignal::new(Some(date(2024, 6, 6).at(8, 0, 0, 0)));
            let state = use_time_field_state(UseTimeFieldStateInput::<DateTime> {
                value: Some(ValueBinding::from(value)),
                min_value: Signal::stored(Some(date(2024, 6, 5).at(9, 0, 0, 0).into())),
                max_value: Signal::stored(Some(TimeBound::Absolute(
                    date(2024, 6, 7).at(17, 0, 0, 0),
                ))),
                ..UseTimeFieldStateInput::default()
            });
            assert_that!(validation(&state)).is_equal_to(valid());
            value.set(Some(date(2024, 6, 5).at(8, 0, 0, 0)));
            assert_that!(validation(&state))
                .is_equal_to(invalid("Value must be 9:00\u{202f}AM or later."));
            value.set(Some(date(2024, 6, 7).at(16, 0, 0, 0)));
            assert_that!(validation(&state)).is_equal_to(valid());
            value.set(Some(date(2024, 6, 7).at(17, 30, 0, 0)));
            assert_that!(validation(&state))
                .is_equal_to(invalid("Value must be 5:00\u{202f}PM or earlier."));
        });
    }

    /// Zoned values: a time of day is on the value's day in its time zone, a zoned bound is
    /// compared by its instant, whatever its time zone (`ZonedDateTime.compare`).
    #[test]
    fn bounds_zoned_values() {
        with_owner(|| {
            let value = RwSignal::new(Some(zoned("2024-06-05T08:30[America/New_York]")));
            let time_of_day = use_time_field_state(UseTimeFieldStateInput::<Zoned> {
                value: Some(ValueBinding::from(value)),
                min_value: Signal::stored(Some(time(9, 0, 0, 0).into())),
                ..UseTimeFieldStateInput::default()
            });
            // 15:00 in Berlin is 9:00 in New York.
            let absolute = use_time_field_state(UseTimeFieldStateInput::<Zoned> {
                value: Some(ValueBinding::from(value)),
                min_value: Signal::stored(Some(zoned("2024-06-05T15:00[Europe/Berlin]").into())),
                ..UseTimeFieldStateInput::default()
            });
            assert_that!(validation(&time_of_day).0).is_true();
            assert_that!(validation(&absolute).0).is_true();
            value.set(Some(zoned("2024-06-05T09:30[America/New_York]")));
            assert_that!(validation(&time_of_day)).is_equal_to(valid());
            assert_that!(validation(&absolute)).is_equal_to(valid());
            // 8:30 on the next day: after the absolute minimum, before 9:00 of that day.
            value.set(Some(zoned("2024-06-06T08:30[America/New_York]")));
            assert_that!(validation(&time_of_day).0).is_true();
            assert_that!(validation(&absolute)).is_equal_to(valid());
        });
    }

    #[test]
    fn edits_a_time() {
        with_owner(|| {
            let state = use_time_field_state(UseTimeFieldStateInput::<Time> {
                hour_cycle: Signal::stored(Some(HourCycle::H24)),
                ..UseTimeFieldStateInput::default()
            });
            let texts = || -> Vec<String> {
                state
                    .field
                    .segments
                    .get_untracked()
                    .into_iter()
                    .filter(|segment| segment.is_editable)
                    .map(|segment| segment.text)
                    .collect()
            };
            assert_that!(texts()).is_equal_to(vec!["––".to_owned(), "––".to_owned()]);
            state.field.set_segment(DateSegmentType::Hour, 14);
            assert_that!(state.value.get_untracked()).is_none();
            state.field.set_segment(DateSegmentType::Minute, 30);
            assert_that!(state.value.get_untracked()).is_equal_to(Some(time(14, 30, 0, 0)));
            state.field.increment_page(DateSegmentType::Minute);
            assert_that!(state.value.get_untracked()).is_equal_to(Some(time(14, 45, 0, 0)));
        });
    }
}
