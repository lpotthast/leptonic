// Upstream: react-stately/src/datepicker/useDateFieldState.ts @ 99e6102368
// Upstream: react-stately/src/datepicker/utils.ts @ 99e6102368
// Upstream: @adobe/react-spectrum/test/datepicker/DateField.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/datepicker/DatePicker.test.js @ 99e6102368
use jiff::tz::TimeZone;
use leptos::prelude::*;

use super::{
    format::{FormatOptions, Formatters, SegmentNumbers, resolve_hour_cycle, segments},
    incomplete_date::IncompleteDate,
    types::{
        DateSegment, DateSegmentType, DateValue, Era, Granularity, HourCycle, MaxGranularity,
        ResolvedHourCycle,
    },
};
use crate::{
    ValueBinding,
    hooks::form::{
        FormValidationState, UseFormValidationStateInput, VALID_VALIDITY_STATE, ValidateFn,
        ValidationBehavior, ValidationResult, ValidityStateSnapshot, use_form_validation_state,
    },
    utils::{
        i18n::use_locale,
        intl_strings::{DateValidationStrings, use_localized_strings},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Generic over the value type (`V: DateValue`: `civil::Date`, `civil::DateTime`, `Zoned`), so
//   that the field returns the type it was given (react-aria: `MappedDateValue<T>`); min, max and
//   the placeholder have that type as well.
// - Hook-owned value (C4): `default_value` + `on_change`, or a binding to app state.
// - A `Copy` struct with signals and methods (C3); `set_segment` takes the era as its index.
// - `hour_cycle`, `granularity`, `max_granularity`: enums (react-aria: numbers and strings).
// - The format options (`placeholder_value`, `granularity`, `max_granularity`, `hour_cycle`,
//   `hide_time_zone`, `should_force_leading_zeros`) are signals (C11).
// - `is_invalid` only adds invalidity (C4): react-aria's `isInvalid={false}` hiding native and
//   built-in errors ("should use controlled validation first") has no counterpart.
//
// ## DIFFERENT BEHAVIOR
// - A granularity finer than the value type has (a time for a `civil::Date`) is the day
//   (react-aria throws).
// - The displayed value is derived, not reset in an effect: an edit in progress is kept while
//   the value and the hour cycle it was made for stay.
//
// ## OMITTED FEATURES
// - Calendar systems other than the Gregorian (`createCalendar`).
// - A `maxGranularity` of minute or second with a time: the hour always shows.
//
// =============================================================================

/// The page steps of the segments (Page Up/Down).
fn page_step(kind: DateSegmentType) -> i32 {
    match kind {
        DateSegmentType::Year => 5,
        DateSegmentType::Month | DateSegmentType::Hour => 2,
        DateSegmentType::Day => 7,
        DateSegmentType::Minute | DateSegmentType::Second => 15,
        _ => 1,
    }
}

/// Input of [`use_date_field_state`].
pub struct UseDateFieldStateInput<V: DateValue> {
    /// The initial value.
    pub default_value: Option<V>,
    /// The value as app state, replacing `default_value`.
    pub value: Option<ValueBinding<Option<V>>>,
    /// Called with each new value.
    pub on_change: Option<Callback<Option<V>>>,
    /// The value the segments start from when edited (e.g. its time). Default: today, midnight.
    pub placeholder_value: Signal<Option<V>>,
    pub min_value: Signal<Option<V>>,
    pub max_value: Signal<Option<V>>,
    /// Whether a date can't be chosen (it makes the value invalid).
    pub is_date_unavailable: Option<Callback<V, bool>>,
    /// The finest unit. Default: the minute for values with a time, else the day.
    pub granularity: Signal<Option<Granularity>>,
    /// The coarsest unit. Default: the year.
    pub max_granularity: Signal<MaxGranularity>,
    /// 12 or 24 hours. Default: the locale's.
    pub hour_cycle: Signal<Option<HourCycle>>,
    /// Hides the time zone of zoned values.
    pub hide_time_zone: Signal<bool>,
    /// Pads months, days and hours to two digits.
    pub should_force_leading_zeros: Signal<bool>,
    pub is_disabled: Signal<bool>,
    pub is_read_only: Signal<bool>,
    pub is_required: Signal<bool>,
    pub is_invalid: Signal<bool>,
    pub validate: Option<ValidateFn<Option<V>>>,
    pub validation_behavior: ValidationBehavior,
    pub name: Option<String>,
    /// The validation of an enclosing date picker, which owns it (react-aria's private
    /// validation state prop): used instead of the field's own.
    pub validation: Option<FormValidationState>,
}

impl<V: DateValue> Default for UseDateFieldStateInput<V> {
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
            max_granularity: Signal::stored(MaxGranularity::Year),
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
            validation: None,
        }
    }
}

/// An edit in progress: the incomplete value, and the revision of the value and the hour cycle
/// it was made for (react-aria resets the display whenever the value changes, also back to an
/// earlier one).
#[derive(Clone, PartialEq)]
struct Edit {
    revision: u64,
    hour_cycle: ResolvedHourCycle,
    display: IncompleteDate,
}

/// The state of a date field: the value, the segments shown, and how they are edited.
pub struct DateFieldState<V: DateValue> {
    /// The value (`None` while empty or incomplete).
    pub value: Signal<Option<V>>,
    /// The segments, in the locale's order.
    pub segments: Signal<Vec<DateSegment>>,
    /// The shown value completed by the placeholder (for descriptions).
    pub date_value: Signal<V>,
    pub granularity: Signal<Granularity>,
    pub max_granularity: Signal<MaxGranularity>,
    pub is_disabled: Signal<bool>,
    pub is_read_only: Signal<bool>,
    pub is_required: Signal<bool>,
    /// Whether the shown validation fails.
    pub is_invalid: Signal<bool>,
    pub validation: FormValidationState,
    pub validation_behavior: ValidationBehavior,
    name: StoredValue<Option<String>>,
    binding: ValueBinding<Option<V>>,
    edit: RwSignal<Option<Edit>>,
    /// Counts the value's changes.
    revision: Memo<u64>,
    display: Memo<IncompleteDate>,
    placeholder: Memo<V>,
    display_segments: Memo<Vec<DateSegmentType>>,
    hour_cycle: Memo<ResolvedHourCycle>,
    formatters: Memo<Formatters>,
    format_options: Memo<FormatOptions>,
    default_value: StoredValue<Option<V>>,
}

// Derived, it would require a `Copy` value type.
#[allow(clippy::expl_impl_clone_on_copy)]
impl<V: DateValue> Clone for DateFieldState<V> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<V: DateValue> Copy for DateFieldState<V> {}

/// What to set a date field to.
enum NewValue<V> {
    Value(Option<V>),
    Incomplete(IncompleteDate),
}

impl<V: DateValue> DateFieldState<V> {
    /// Sets the value (`None` clears the field).
    pub fn set_value(&self, value: Option<V>) {
        self.update(NewValue::Value(value));
    }

    /// Whether the state still exists (not disposed with its field), for blur handlers.
    pub(crate) fn is_alive(&self) -> bool {
        self.edit.try_with_untracked(|_| ()).is_some()
    }

    /// The field's name in forms.
    pub fn name(&self) -> Option<String> {
        self.name.get_value()
    }

    /// The value the field started with (restored on form reset).
    pub fn default_value(&self) -> Option<V> {
        self.default_value.get_value()
    }

    fn update(&self, new_value: NewValue<V>) {
        if self.is_disabled.get_untracked() || self.is_read_only.get_untracked() {
            return;
        }
        let segments = self.display_segments.get_untracked();
        match new_value {
            NewValue::Value(None) => {
                self.edit.set(None);
                self.binding.set(None);
            }
            NewValue::Incomplete(display) if display.is_cleared(&segments) => {
                self.edit.set(None);
                self.binding.set(None);
            }
            NewValue::Value(Some(value)) => {
                self.edit.set(None);
                self.binding.set(Some(value));
            }
            NewValue::Incomplete(display) => {
                let value = self.binding.value.get_untracked();
                // A complete and valid value is set right away; an incomplete or invalid one
                // (February 30) when the field is left.
                if display.is_complete(&segments) {
                    let base = value
                        .clone()
                        .unwrap_or_else(|| self.placeholder.get_untracked());
                    let date_value = display.to_value(&base);
                    if display.validate(&date_value, &segments)
                        && value
                            .as_ref()
                            .is_none_or(|value| value.compare(&date_value).is_ne())
                    {
                        self.edit.set(None);
                        self.binding.set(Some(date_value));
                        return;
                    }
                }
                self.edit.set(Some(Edit {
                    revision: self.revision.get_untracked(),
                    hour_cycle: self.hour_cycle.get_untracked(),
                    display,
                }));
            }
        }
    }

    fn adjust(&self, kind: DateSegmentType, amount: i32) {
        let display = self.display.get_untracked().cycle(
            kind,
            amount,
            &self.placeholder.get_untracked(),
            &self.display_segments.get_untracked(),
        );
        self.update(NewValue::Incomplete(display));
    }

    pub fn increment(&self, kind: DateSegmentType) {
        self.adjust(kind, 1);
    }

    pub fn decrement(&self, kind: DateSegmentType) {
        self.adjust(kind, -1);
    }

    pub fn increment_page(&self, kind: DateSegmentType) {
        self.adjust(kind, page_step(kind));
    }

    pub fn decrement_page(&self, kind: DateSegmentType) {
        self.adjust(kind, -page_step(kind));
    }

    /// Sets a segment to its maximum (End).
    pub fn increment_to_max(&self, kind: DateSegmentType) {
        let display = self.display.get_untracked();
        let max = if kind == DateSegmentType::Hour && display.hour_cycle() == ResolvedHourCycle::H12
        {
            11
        } else {
            display
                .segment_limits(kind)
                .map_or(0, |limits| limits.max_value)
        };
        self.set_segment(kind, max);
    }

    /// Sets a segment to its minimum (Home).
    pub fn decrement_to_min(&self, kind: DateSegmentType) {
        let display = self.display.get_untracked();
        let min = if kind == DateSegmentType::Hour && display.hour_cycle() == ResolvedHourCycle::H12
        {
            12
        } else {
            display
                .segment_limits(kind)
                .map_or(0, |limits| limits.min_value)
        };
        self.set_segment(kind, min);
    }

    /// Sets a segment (the era as its index, the day period as 0: AM, 1: PM).
    pub fn set_segment(&self, kind: DateSegmentType, value: i32) {
        let display =
            self.display
                .get_untracked()
                .set(kind, value, &self.placeholder.get_untracked());
        self.update(NewValue::Incomplete(display));
    }

    /// Commits a complete but invalid shown value, constrained (when the field is left).
    pub fn confirm_placeholder(&self) {
        if self.is_disabled.get_untracked() || self.is_read_only.get_untracked() {
            return;
        }
        let display = self.display.get_untracked();
        if display.is_complete(&self.display_segments.get_untracked()) {
            let value = self.binding.value.get_untracked();
            let base = value
                .clone()
                .unwrap_or_else(|| self.placeholder.get_untracked());
            let date_value = display.to_value(&base);
            if value
                .as_ref()
                .is_none_or(|value| value.compare(&date_value).is_ne())
            {
                self.binding.set(Some(date_value));
            }
            self.edit.set(None);
        }
    }

    /// Clears a segment.
    pub fn clear_segment(&self, kind: DateSegmentType) {
        let display = self.display.get_untracked();
        let display = if matches!(
            kind,
            DateSegmentType::TimeZoneName | DateSegmentType::Literal
        ) {
            display
        } else {
            display.clear(kind)
        };
        self.update(NewValue::Incomplete(display));
    }

    /// The value formatted for descriptions, months by name ("June 15, 2024"); empty without a
    /// value (react-stately's `formatValue({month: 'long'})`).
    pub fn format_value(&self) -> String {
        let Some(value) = self.binding.value.get() else {
            return String::new();
        };
        self.formatters
            .with(|formatters| formatters.long().format(&value))
    }

    /// The field's options for other formatting of its values (e.g. a picker's description).
    pub(crate) fn format_options(&self) -> FormatOptions {
        self.format_options.get()
    }
}

/// The granularity of a value type: as given if the type has it (a time for a `civil::Date` is
/// the day), else the minute for values with a time, the day for dates.
pub(crate) fn resolve_granularity<V: DateValue>(granularity: Option<Granularity>) -> Granularity {
    match granularity {
        Some(granularity) if granularity.has_time() && !V::HAS_TIME => Granularity::Day,
        Some(granularity) => granularity,
        None if V::HAS_TIME => Granularity::Minute,
        None => Granularity::Day,
    }
}

/// The validation of a value against min, max and unavailable dates (react-stately's
/// `getValidationResult`). `format` formats a violated limit for its message (only then).
pub(crate) fn validation_result<V: DateValue>(
    value: Option<&V>,
    min_value: Option<&V>,
    max_value: Option<&V>,
    is_date_unavailable: Option<Callback<V, bool>>,
    format: &dyn Fn(&V) -> String,
    strings: &DateValidationStrings,
) -> ValidationResult {
    let Some(value) = value else {
        return valid();
    };
    let range_overflow = max_value.is_some_and(|max| value.compare(max).is_gt());
    let range_underflow = min_value.is_some_and(|min| value.compare(min).is_lt());
    let is_unavailable =
        is_date_unavailable.is_some_and(|unavailable| unavailable.run(value.clone()));
    let is_invalid = range_overflow || range_underflow || is_unavailable;
    let mut errors = Vec::new();
    if let Some(min) = min_value.filter(|_| range_underflow) {
        errors.push(strings.range_underflow(&format(min)));
    }
    if let Some(max) = max_value.filter(|_| range_overflow) {
        errors.push(strings.range_overflow(&format(max)));
    }
    if is_unavailable {
        errors.push(strings.unavailable_date());
    }
    ValidationResult {
        is_invalid,
        validation_errors: errors,
        validation_details: ValidityStateSnapshot {
            bad_input: is_unavailable,
            range_overflow,
            range_underflow,
            valid: !is_invalid,
            ..VALID_VALIDITY_STATE
        },
    }
}

fn valid() -> ValidationResult {
    ValidationResult {
        is_invalid: false,
        validation_errors: Vec::new(),
        validation_details: VALID_VALIDITY_STATE,
    }
}

/// The segments a field edits, from `max_granularity` to `granularity` (the day period with
/// the hour in 12-hour cycles).
fn display_segments(
    max_granularity: MaxGranularity,
    granularity: Granularity,
    hour_cycle: ResolvedHourCycle,
) -> Vec<DateSegmentType> {
    let mut all = vec![
        DateSegmentType::Era,
        DateSegmentType::Year,
        DateSegmentType::Month,
        DateSegmentType::Day,
        DateSegmentType::Hour,
    ];
    if hour_cycle.is_12_hour() {
        all.push(DateSegmentType::DayPeriod);
    }
    all.extend([DateSegmentType::Minute, DateSegmentType::Second]);
    let first = if max_granularity == MaxGranularity::Year {
        DateSegmentType::Era
    } else {
        max_granularity.segment()
    };
    let last = if granularity == Granularity::Hour && hour_cycle.is_12_hour() {
        DateSegmentType::DayPeriod
    } else {
        granularity.segment()
    };
    let start = all
        .iter()
        .position(|segment| *segment == first)
        .unwrap_or(0);
    let end = all
        .iter()
        .position(|segment| *segment == last)
        .unwrap_or(all.len() - 1);
    all.get(start..=end).map(<[_]>::to_vec).unwrap_or_default()
}

/// State of a date field: the value, its segments in the locale's order and their editing
/// (typing, stepping, clearing), with validation against min, max and unavailable dates.
#[allow(clippy::too_many_lines)]
pub fn use_date_field_state<V: DateValue>(input: UseDateFieldStateInput<V>) -> DateFieldState<V> {
    let UseDateFieldStateInput {
        default_value,
        value,
        on_change,
        placeholder_value,
        min_value,
        max_value,
        is_date_unavailable,
        granularity,
        max_granularity,
        hour_cycle: hour_cycle_preference,
        hide_time_zone,
        should_force_leading_zeros,
        is_disabled,
        is_read_only,
        is_required,
        is_invalid,
        validate,
        validation_behavior,
        name,
        validation: picker_validation,
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
    let initial_value = binding.value.get_untracked();

    // The finest unit: as given, if the value type has it.
    let granularity_signal = Signal::derive(move || resolve_granularity::<V>(granularity.get()));
    // The time zone of zoned values: the last value's (or the placeholder's).
    let last_time_zone = StoredValue::new(None::<TimeZone>);
    let time_zone = Memo::new(move |_| {
        let value = binding.value.get();
        let time_zone = value
            .as_ref()
            .and_then(|value| value.time_zone().cloned())
            .or_else(|| {
                placeholder_value
                    .with(|value| value.as_ref().and_then(|value| value.time_zone().cloned()))
            })
            .or_else(|| last_time_zone.get_value());
        last_time_zone.set_value(time_zone.clone());
        time_zone
    });

    let hour_cycle =
        Memo::new(move |_| resolve_hour_cycle(&locale.get(), hour_cycle_preference.get()));
    let display_segments = Memo::new(move |_| {
        display_segments(
            max_granularity.get(),
            granularity_signal.get(),
            hour_cycle.get(),
        )
    });
    let placeholder = Memo::new(move |_| {
        placeholder_value
            .get()
            .unwrap_or_else(|| V::today(time_zone.get().as_ref()))
    });

    let edit = RwSignal::new(None::<Edit>);
    let revision = Memo::new(move |previous: Option<&u64>| {
        binding.value.track();
        previous.map_or(0, |revision| revision + 1)
    });
    // The shown value: an edit in progress for the current value and hour cycle, else the value.
    let display = Memo::new(move |_| {
        let value = binding.value.get();
        let revision = revision.get();
        let hour_cycle = hour_cycle.get();
        edit.with(|edit| {
            edit.as_ref()
                .filter(|edit| edit.revision == revision && edit.hour_cycle == hour_cycle)
                .map(|edit| edit.display.clone())
        })
        .unwrap_or_else(|| IncompleteDate::new(hour_cycle, value.as_ref()))
    });

    let format_options = Memo::new(move |_| FormatOptions {
        granularity: granularity_signal.get(),
        max_granularity: max_granularity.get(),
        time_zone: time_zone.get(),
        hide_time_zone: hide_time_zone.get(),
        hour_cycle: hour_cycle_preference.get(),
        show_era: display.with(|display| display.era == Some(Era::Bc)),
        should_force_leading_zeros: should_force_leading_zeros.get(),
    });
    let formatters = Memo::new(move |_| Formatters::new(locale.get(), format_options.get()));
    let numbers = Memo::new(move |_| SegmentNumbers::new(&locale.get()));

    let date_value = Memo::new(move |_| {
        let base = binding.value.get().unwrap_or_else(|| placeholder.get());
        display.with(|display| display.to_value(&base))
    });
    let segment_list = Memo::new(move |_| {
        let date_value = date_value.get();
        display.with(|display| {
            formatters.with(|formatters| {
                numbers.with(|numbers| {
                    locale.with(|locale| {
                        segments(
                            &date_value,
                            display,
                            formatters.short(),
                            numbers,
                            locale,
                            granularity_signal.get(),
                        )
                    })
                })
            })
        })
    });

    let is_date_unavailable = StoredValue::new(is_date_unavailable);
    let strings = use_localized_strings::<DateValidationStrings>();
    let builtin_validation = Memo::new(move |_| {
        let value = binding.value.get();
        let (min, max) = (min_value.get(), max_value.get());
        Some(validation_result(
            value.as_ref(),
            min.as_ref(),
            max.as_ref(),
            is_date_unavailable.get_value(),
            &|limit| formatters.with(|formatters| formatters.short().format(limit)),
            &strings.read(),
        ))
    });
    let validation = picker_validation.unwrap_or_else(|| {
        use_form_validation_state(UseFormValidationStateInput {
            is_invalid,
            value: binding.value,
            validate,
            builtin_validation: builtin_validation.into(),
            validation_behavior,
            names: name.clone().into_iter().collect(),
        })
    });

    DateFieldState {
        value: binding.value,
        segments: segment_list.into(),
        date_value: date_value.into(),
        granularity: granularity_signal,
        max_granularity,
        is_disabled,
        is_read_only,
        is_required,
        is_invalid: validation.is_invalid,
        validation,
        validation_behavior,
        name: StoredValue::new(name),
        binding,
        edit,
        revision,
        display,
        placeholder,
        display_segments,
        hour_cycle,
        formatters,
        format_options,
        default_value: StoredValue::new(initial_value),
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;
    use jiff::civil::{Date, date};

    use super::*;
    use crate::testing::with_owner;

    fn texts(state: &DateFieldState<Date>) -> Vec<String> {
        state
            .segments
            .get_untracked()
            .into_iter()
            .filter(|segment| segment.is_editable)
            .map(|segment| segment.text)
            .collect()
    }

    #[test]
    fn edits_an_empty_field_segment_by_segment() {
        with_owner(|| {
            let changes = RwSignal::new(Vec::<Option<Date>>::new());
            let state = use_date_field_state(UseDateFieldStateInput {
                placeholder_value: Signal::stored(Some(date(2024, 1, 1))),
                on_change: Some(Callback::new(move |value| {
                    changes.update(|changes| changes.push(value));
                })),
                ..UseDateFieldStateInput::default()
            });
            assert_that!(texts(&state))
                .is_equal_to(["mm", "dd", "yyyy"].map(str::to_owned).to_vec());
            state.set_segment(DateSegmentType::Month, 2);
            state.set_segment(DateSegmentType::Day, 30);
            assert_that!(texts(&state))
                .is_equal_to(["2", "30", "yyyy"].map(str::to_owned).to_vec());
            // Complete but invalid (February 30): shown, not committed until confirmed.
            state.set_segment(DateSegmentType::Year, 2023);
            assert_that!(state.value.get_untracked()).is_none();
            assert_that!(texts(&state))
                .is_equal_to(["2", "30", "2023"].map(str::to_owned).to_vec());
            state.confirm_placeholder();
            assert_that!(state.value.get_untracked()).is_equal_to(Some(date(2023, 2, 28)));
            // Valid edits are committed right away.
            state.increment(DateSegmentType::Month);
            assert_that!(state.value.get_untracked()).is_equal_to(Some(date(2023, 3, 28)));
            assert_that!(changes.get_untracked().len()).is_equal_to(2);
            // Clearing a segment leaves an incomplete value: the value stays until all are clear.
            state.clear_segment(DateSegmentType::Day);
            assert_that!(texts(&state)[1].as_str()).is_equal_to("dd");
            state.clear_segment(DateSegmentType::Month);
            state.clear_segment(DateSegmentType::Year);
            assert_that!(state.value.get_untracked()).is_none();
        });
    }

    #[test]
    fn validates_against_min_and_max() {
        with_owner(|| {
            let state = use_date_field_state(UseDateFieldStateInput {
                default_value: Some(date(2024, 6, 5)),
                min_value: Signal::stored(Some(date(2024, 7, 1))),
                ..UseDateFieldStateInput::default()
            });
            let validation = state.validation.realtime_validation.get_untracked();
            assert_that!(validation.is_invalid).is_true();
            assert_that!(validation.validation_errors)
                .is_equal_to(vec!["Value must be 7/1/2024 or later.".to_owned()]);
        });
    }

    #[test]
    fn lists_the_edited_segments() {
        assert_that!(display_segments(
            MaxGranularity::Year,
            Granularity::Day,
            ResolvedHourCycle::H12
        ))
        .is_equal_to(vec![
            DateSegmentType::Era,
            DateSegmentType::Year,
            DateSegmentType::Month,
            DateSegmentType::Day,
        ]);
        assert_that!(display_segments(
            MaxGranularity::Hour,
            Granularity::Hour,
            ResolvedHourCycle::H12
        ))
        .is_equal_to(vec![DateSegmentType::Hour, DateSegmentType::DayPeriod]);
        assert_that!(display_segments(
            MaxGranularity::Hour,
            Granularity::Minute,
            ResolvedHourCycle::H23
        ))
        .is_equal_to(vec![DateSegmentType::Hour, DateSegmentType::Minute]);
    }
}
