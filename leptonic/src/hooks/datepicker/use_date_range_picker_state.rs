// Upstream: react-stately/src/datepicker/useDateRangePickerState.ts @ 99e6102368
use std::sync::Arc;

use jiff::civil::{Date, Time};
use leptos::prelude::*;

use super::{
    format::{DateFormatter, FormatOptions},
    types::{DateValue, Era, Granularity, HourCycle, MaxGranularity, RangeValue},
    use_date_field_state::{resolve_granularity, validation_result},
};
use crate::{
    hooks::{
        OverlayTriggerState, UseOverlayTriggerStateInput,
        form::{
            FormValidationState, UseFormValidationStateInput, VALID_VALIDITY_STATE, ValidateFn,
            ValidationBehavior, ValidationResult, ValidityStateSnapshot, merge_validation,
            use_form_validation_state,
        },
        use_overlay_trigger_state,
    },
    utils::{
        ValueBinding,
        date::DateRange,
        i18n::{Locale, use_locale},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Generic over the value type (`V: DateValue`); the calendar's range is a `DateRange` of
//   `civil::Date`s and the times `civil::Time`s.
// - Closing commits a selected range through the overlay state's `on_open_change`.
// - The format options and the placeholder are signals (C11).
// - `is_date_unavailable` takes the date only (react-aria: also the anchor date, always `null`
//   for the validation).
//
// ## OMITTED FEATURES
// - Range formatting with shared fields ("June 1 – 15, 2024", `formatRange`): ICU4X has none
//   yet; start and end are formatted apart.
// - Localized validation messages: English.
// - Server errors under the end's name (react-aria validates under both names); the start's
//   name (else the end's) applies.
//
// =============================================================================

/// Which end of a range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RangePart {
    Start,
    End,
}

/// Input of [`use_date_range_picker_state`].
pub struct UseDateRangePickerStateInput<V: DateValue> {
    pub default_value: Option<RangeValue<V>>,
    pub value: Option<ValueBinding<Option<RangeValue<V>>>>,
    pub on_change: Option<Callback<Option<RangeValue<V>>>>,
    /// The value the fields start from when edited, the time of ranges selected in the
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
    /// Whether selecting a range closes the popover. Default: `true`.
    pub should_close_on_select: Signal<bool>,
    pub default_open: bool,
    pub is_open: Option<ValueBinding<bool>>,
    pub on_open_change: Option<Callback<bool>>,
    pub is_invalid: Signal<bool>,
    pub validate: Option<ValidateFn<Option<RangeValue<V>>>>,
    pub validation_behavior: ValidationBehavior,
    /// The names of the ends in forms (server errors for the start's name apply).
    pub start_name: Option<String>,
    pub end_name: Option<String>,
}

impl<V: DateValue> Default for UseDateRangePickerStateInput<V> {
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
            start_name: None,
            end_name: None,
        }
    }
}

/// The state of a date range picker: the range (complete, or its ends while incomplete), the
/// range and times selected in the popover, and whether it is open.
pub struct DateRangePickerState<V: DateValue> {
    /// The complete range (`None` while an end is missing).
    pub value: Signal<Option<RangeValue<V>>>,
    /// The start shown (also of an incomplete range).
    pub start: Signal<Option<V>>,
    /// The end shown.
    pub end: Signal<Option<V>>,
    /// The calendar's range: the value's, else the one selected in the popover (react-aria: a
    /// complete value replaces the selection).
    pub date_range: Signal<Option<DateRange>>,
    pub granularity: Signal<Granularity>,
    pub has_time: Signal<bool>,
    pub overlay: OverlayTriggerState,
    pub is_invalid: Signal<bool>,
    pub validation: FormValidationState,
    binding: ValueBinding<Option<RangeValue<V>>>,
    partial: RwSignal<(Option<V>, Option<V>)>,
    selected_range: RwSignal<Option<DateRange>>,
    selected_times: RwSignal<(Option<Time>, Option<Time>)>,
    placeholder: Memo<V>,
    placeholder_time: Signal<Time>,
    should_close_on_select: Signal<bool>,
    format_options: Memo<FormatOptions>,
    locale: Signal<Locale>,
}

// Derived, it would require a `Copy` value type.
#[allow(clippy::expl_impl_clone_on_copy)]
impl<V: DateValue> Clone for DateRangePickerState<V> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<V: DateValue> Copy for DateRangePickerState<V> {}

impl<V: DateValue> DateRangePickerState<V> {
    /// Sets the range's ends: committed once both are there.
    pub fn set_value(&self, start: Option<V>, end: Option<V>) {
        self.partial.set((start.clone(), end.clone()));
        match (start, end) {
            (Some(start), Some(end)) => self.binding.set(Some(RangeValue { start, end })),
            _ => self.binding.set(None),
        }
    }

    /// Sets one end (from its field).
    pub fn set_date_time(&self, part: RangePart, value: Option<V>) {
        let (start, end) = (self.start.get_untracked(), self.end.get_untracked());
        match part {
            RangePart::Start => self.set_value(value, end),
            RangePart::End => self.set_value(start, value),
        }
    }

    fn commit(&self, range: DateRange, times: (Time, Time)) {
        let base = |value: Option<V>| value.unwrap_or_else(|| self.placeholder.get_untracked());
        let start = base(self.start.get_untracked()).with_fields(range.start, times.0, None);
        let end = base(self.end.get_untracked()).with_fields(range.end, times.1, None);
        self.set_value(Some(start), Some(end));
        self.selected_range.set(None);
        self.selected_times.set((None, None));
        self.validation.commit_validation();
    }

    /// Selects a range in the calendar (keeping the times).
    pub fn select_range(&self, range: DateRange) {
        let should_close = self.should_close_on_select.get_untracked();
        if self.has_time.get_untracked() {
            let (start_time, end_time) = self.times();
            if should_close || (start_time.is_some() && end_time.is_some()) {
                let placeholder_time = self.placeholder_time.get_untracked();
                self.commit(
                    range,
                    (
                        start_time.unwrap_or(placeholder_time),
                        end_time.unwrap_or(placeholder_time),
                    ),
                );
            } else {
                self.selected_range.set(Some(range));
            }
        } else {
            self.commit(range, (Time::midnight(), Time::midnight()));
        }
        if should_close {
            self.overlay.set_open(false);
        }
    }

    /// The times: a complete value's, else the ones selected in the popover (react-aria: a
    /// complete value replaces the selection).
    fn times(&self) -> (Option<Time>, Option<Time>) {
        if V::HAS_TIME
            && let Some(RangeValue { start, end }) = self.value.get_untracked()
        {
            return (Some(start.time()), Some(end.time()));
        }
        self.selected_times.get_untracked()
    }

    /// Selects a time of one end (committed with a selected range).
    pub fn select_time(&self, part: RangePart, time: Time) {
        let (mut start, mut end) = self.times();
        match part {
            RangePart::Start => start = Some(time),
            RangePart::End => end = Some(time),
        }
        match (self.date_range.get_untracked(), start, end) {
            (Some(range), Some(start), Some(end)) => self.commit(range, (start, end)),
            _ => self.selected_times.set((start, end)),
        }
    }

    pub fn set_open(&self, is_open: bool) {
        self.overlay.set_open(is_open);
    }

    /// A date as a value: on the time and in the zone of the start, else of the placeholder.
    pub fn date_to_value(&self, date: Date) -> V {
        let base = self
            .start
            .get_untracked()
            .unwrap_or_else(|| self.placeholder.get_untracked());
        base.with_fields(date, base.time(), None)
    }

    /// The start and end formatted for descriptions ("June 1, 2024", "June 15, 2024").
    pub fn format_value(&self) -> Option<(String, String)> {
        let RangeValue { start, end } = self.value.get()?;
        let formatter = DateFormatter::long(&self.locale.get(), &self.format_options.get());
        Some((formatter.format(&start), formatter.format(&end)))
    }
}

/// The validation of a range: of each end, and that the end isn't before the start
/// (react-stately's `getRangeValidationResult`).
fn range_validation<V: DateValue>(
    start: Option<&V>,
    end: Option<&V>,
    min: Option<&V>,
    max: Option<&V>,
    is_date_unavailable: Option<Callback<V, bool>>,
    formatter: &DateFormatter,
) -> ValidationResult {
    let start_result = validation_result(start, min, max, is_date_unavailable, formatter);
    let end_result = validation_result(end, min, max, is_date_unavailable, formatter);
    let mut result = merge_validation(&[start_result, end_result]);
    if let (Some(start), Some(end)) = (start, end)
        && end.compare(start).is_lt()
    {
        result = merge_validation(&[
            result,
            ValidationResult {
                is_invalid: true,
                validation_errors: vec!["Start date must be before end date.".to_owned()],
                validation_details: ValidityStateSnapshot {
                    range_underflow: true,
                    range_overflow: true,
                    valid: false,
                    ..VALID_VALIDITY_STATE
                },
            },
        ]);
    }
    result
}

/// State of a date range picker (react-stately's `useDateRangePickerState`): two date fields
/// (start and end) with a popover range calendar (and times); the range is committed once
/// both ends are there, and validated as a whole.
#[allow(clippy::too_many_lines)]
pub fn use_date_range_picker_state<V: DateValue>(
    input: UseDateRangePickerStateInput<V>,
) -> DateRangePickerState<V> {
    let UseDateRangePickerStateInput {
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
        start_name,
        end_name,
    } = input;
    let locale = use_locale();

    let owned_value =
        value.unwrap_or_else(|| ValueBinding::from(RwSignal::new(default_value.clone())));
    let binding = ValueBinding::new(
        owned_value.value,
        Callback::new(move |value: Option<RangeValue<V>>| {
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
    // The ends of an incomplete range (react-aria's placeholder value): reset when the value
    // becomes empty from outside.
    let partial = RwSignal::new(
        binding
            .value
            .get_untracked()
            .map_or((None, None), |range| (Some(range.start), Some(range.end))),
    );
    Effect::watch(
        move || binding.value.get(),
        move |value, _, _| {
            if value.is_none()
                && partial.with_untracked(|(start, end)| start.is_some() && end.is_some())
            {
                partial.set((None, None));
            }
        },
        false,
    );
    let start = Signal::derive(move || {
        binding
            .value
            .get()
            .map(|range| range.start)
            .or_else(|| partial.with(|(start, _)| start.clone()))
    });
    let end = Signal::derive(move || {
        binding
            .value
            .get()
            .map(|range| range.end)
            .or_else(|| partial.with(|(_, end)| end.clone()))
    });

    let granularity = Signal::derive(move || resolve_granularity::<V>(granularity.get()));
    let has_time = Signal::derive(move || granularity.get().has_time());
    // The time of ranges selected without one (react-aria's `getPlaceholderTime`).
    let placeholder_time = Signal::derive(move || {
        placeholder_value
            .get()
            .filter(|_| V::HAS_TIME)
            .map_or(Time::midnight(), |value| value.time())
    });
    let time_zone = Memo::new(move |_| {
        start
            .get()
            .or_else(|| end.get())
            .and_then(|value| value.time_zone().cloned())
            .or_else(|| {
                placeholder_value
                    .with(|value| value.as_ref().and_then(|value| value.time_zone().cloned()))
            })
    });
    let placeholder = Memo::new(move |_| {
        placeholder_value
            .get()
            .unwrap_or_else(|| V::today(time_zone.get().as_ref()))
    });

    let selected_range = RwSignal::new(None::<DateRange>);
    let selected_times = RwSignal::new((None::<Time>, None::<Time>));
    let date_range = Signal::derive(move || {
        binding
            .value
            .get()
            .map(|range| DateRange {
                start: range.start.date(),
                end: range.end.date(),
            })
            .or_else(|| selected_range.get())
    });

    let format_options = Memo::new(move |_| FormatOptions {
        granularity: granularity.get(),
        max_granularity: MaxGranularity::Year,
        time_zone: time_zone.get(),
        hide_time_zone: hide_time_zone.get(),
        hour_cycle: hour_cycle.get(),
        show_era: [start.get(), end.get()]
            .into_iter()
            .flatten()
            .any(|value| Era::of(value.date().year()).0 == Era::Bc),
        should_force_leading_zeros: should_force_leading_zeros.get(),
    });
    let is_date_unavailable = StoredValue::new(is_date_unavailable);
    let builtin_validation = Signal::derive(move || {
        let (start, end) = (start.get(), end.get());
        let (min, max) = (min_value.get(), max_value.get());
        let formatter = DateFormatter::new(&locale.get(), &format_options.get());
        Some(range_validation(
            start.as_ref(),
            end.as_ref(),
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
        // One name per validation state: the start's (react-aria: both).
        name: start_name.or(end_name),
    });

    // Closing commits a range selected without times, with the placeholder time.
    let commit_on_close: Arc<dyn Fn() + Send + Sync> = Arc::new(move || {
        if has_time.get_untracked()
            && binding.value.get_untracked().is_none()
            && let Some(range) = selected_range.get_untracked()
        {
            let (start_time, end_time) = selected_times.get_untracked();
            let placeholder_time = placeholder_time.get_untracked();
            let base = placeholder.get_untracked();
            binding.set(Some(RangeValue {
                start: base.with_fields(range.start, start_time.unwrap_or(placeholder_time), None),
                end: base.with_fields(range.end, end_time.unwrap_or(placeholder_time), None),
            }));
            selected_range.set(None);
            selected_times.set((None, None));
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

    DateRangePickerState {
        value: binding.value,
        start,
        end,
        date_range,
        granularity,
        has_time,
        overlay,
        is_invalid: validation.is_invalid,
        validation,
        binding,
        partial,
        selected_range,
        selected_times,
        placeholder,
        placeholder_time,
        should_close_on_select,
        format_options,
        locale,
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;
    use jiff::civil::{DateTime, date, time};

    use super::*;

    #[test]
    fn commits_complete_ranges_only() {
        let owner = Owner::new();
        owner.with(|| {
            let state =
                use_date_range_picker_state(UseDateRangePickerStateInput::<Date>::default());
            state.set_date_time(RangePart::Start, Some(date(2024, 6, 1)));
            assert_that!(state.value.get_untracked()).is_none();
            assert_that!(state.start.get_untracked()).is_equal_to(Some(date(2024, 6, 1)));
            state.set_date_time(RangePart::End, Some(date(2024, 6, 5)));
            assert_that!(state.value.get_untracked()).is_equal_to(Some(RangeValue {
                start: date(2024, 6, 1),
                end: date(2024, 6, 5),
            }));
        });
    }

    /// react-stately's `useDateRangePickerState`: a complete value replaces a range (and times)
    /// selected in the popover.
    #[test]
    fn a_complete_value_replaces_the_selection() {
        let owner = Owner::new();
        owner.with(|| {
            let state = use_date_range_picker_state(UseDateRangePickerStateInput::<DateTime> {
                should_close_on_select: Signal::stored(false),
                ..UseDateRangePickerStateInput::default()
            });
            state.set_open(true);
            // Waits for times.
            state.select_range(DateRange {
                start: date(2024, 6, 10),
                end: date(2024, 6, 12),
            });
            assert_that!(state.value.get_untracked()).is_none();
            state.set_value(
                Some(date(2024, 7, 1).at(8, 0, 0, 0)),
                Some(date(2024, 7, 3).at(9, 0, 0, 0)),
            );
            assert_that!(state.date_range.get_untracked()).is_equal_to(Some(DateRange {
                start: date(2024, 7, 1),
                end: date(2024, 7, 3),
            }));
            // A time selected now goes with the value's range and other time.
            state.select_time(RangePart::End, time(18, 0, 0, 0));
            assert_that!(state.value.get_untracked()).is_equal_to(Some(RangeValue {
                start: date(2024, 7, 1).at(8, 0, 0, 0),
                end: date(2024, 7, 3).at(18, 0, 0, 0),
            }));
        });
    }

    /// RAC `DateRangePicker.test.js`: closing with a range selected but no times commits it with
    /// the placeholder's time.
    #[test]
    fn closing_commits_the_placeholder_time() {
        let owner = Owner::new();
        owner.with(|| {
            let state = use_date_range_picker_state(UseDateRangePickerStateInput::<DateTime> {
                should_close_on_select: Signal::stored(false),
                placeholder_value: Signal::stored(Some(date(2024, 6, 1).at(10, 30, 0, 0))),
                ..UseDateRangePickerStateInput::default()
            });
            state.set_open(true);
            state.select_range(DateRange {
                start: date(2024, 6, 10),
                end: date(2024, 6, 12),
            });
            assert_that!(state.value.get_untracked()).is_none();
            state.set_open(false);
            assert_that!(state.value.get_untracked()).is_equal_to(Some(RangeValue {
                start: date(2024, 6, 10).at(10, 30, 0, 0),
                end: date(2024, 6, 12).at(10, 30, 0, 0),
            }));
        });
    }

    #[test]
    fn selects_a_range_and_validates_its_order() {
        let owner = Owner::new();
        owner.with(|| {
            let state =
                use_date_range_picker_state(UseDateRangePickerStateInput::<Date>::default());
            state.set_open(true);
            state.select_range(DateRange {
                start: date(2024, 6, 10),
                end: date(2024, 6, 12),
            });
            assert_that!(state.overlay.is_open.get_untracked()).is_false();
            assert_that!(state.value.get_untracked().map(|range| range.end))
                .is_equal_to(Some(date(2024, 6, 12)));
            state.set_date_time(RangePart::End, Some(date(2024, 6, 1)));
            let validation = state.validation.realtime_validation.get_untracked();
            assert_that!(validation.validation_errors)
                .is_equal_to(vec!["Start date must be before end date.".to_owned()]);
        });
    }
}
