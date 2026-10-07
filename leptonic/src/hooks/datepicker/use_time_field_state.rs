// Upstream: react-stately/src/datepicker/useTimeFieldState.ts @ 99e6102368
use std::sync::Arc;

use jiff::civil::{Date, Time};
use leptos::prelude::*;

use super::{
    types::{DateValue, Granularity, HourCycle, MaxGranularity, TimeValue},
    use_date_field_state::{DateFieldState, UseDateFieldStateInput, use_date_field_state},
};
use crate::{
    hooks::form::{ValidateFn, ValidationBehavior},
    utils::{ValueBinding, date::today},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Generic over the value type (`T: TimeValue`: `civil::Time`, `civil::DateTime`, `Zoned`);
//   min, max and the placeholder have that type as well. The date field state it is built on
//   is `field` (react-aria: one merged state).
// - Hook-owned value (C4): `default_value` + `on_change`, or a binding to app state.
// - The format options are signals (C11), as in `use_date_field_state`.
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
    pub min_value: Signal<Option<T>>,
    pub max_value: Signal<Option<T>>,
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
    let field = use_date_field_state(UseDateFieldStateInput {
        default_value: None,
        value: Some(field_binding),
        on_change: None,
        placeholder_value: Signal::derive(move || {
            Some(placeholder_value.get().map_or_else(
                || T::Field::today(time_zone.as_ref()).with_fields(day, Time::midnight(), None),
                |placeholder| placeholder.to_field(day),
            ))
        }),
        min_value: Signal::derive(move || to_field(min_value.get())),
        max_value: Signal::derive(move || to_field(max_value.get())),
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
    use jiff::civil::time;

    use super::*;
    use crate::hooks::datepicker::DateSegmentType;

    #[test]
    fn edits_a_time() {
        let owner = Owner::new();
        owner.with(|| {
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
