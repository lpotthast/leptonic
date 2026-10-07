// Upstream: react-stately/src/datepicker/IncompleteDate.ts @ 99e6102368
// Upstream: @internationalized/date/src/manipulation.ts @ 99e6102368 (`cycleValue`, zoned hours)
//! The value a date field shows while it is edited: each field may be missing, and the fields
//! may form an invalid date (February 30) until the field is left.

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Gregorian only: eras are `Era::Bc`/`Era::Ad`, months 1 to 12, days 1 to 31.
//
// ## DIFFERENT BEHAVIOR
// - A zoned hour cycles within its half of the day in both 12-hour cycles (react-aria: only for
//   `h12`; `h11` cycles through all 24 hours, changing the day period).
//
// =============================================================================

use jiff::{
    SignedDuration, Zoned,
    civil::{Date, Time},
    tz::Offset,
};

use super::types::{DateSegmentType, DateValue, Era, ResolvedHourCycle};

/// The limits of a segment's value and its current value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SegmentLimits {
    pub value: Option<i32>,
    pub min_value: i32,
    pub max_value: i32,
}

const MAX_YEAR: i32 = 9999;
const MAX_MONTHS: i32 = 12;
const MAX_DAYS: i32 = 31;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct IncompleteDate {
    hour_cycle: ResolvedHourCycle,
    pub era: Option<Era>,
    /// The year in its era.
    pub year: Option<i32>,
    pub month: Option<i32>,
    pub day: Option<i32>,
    /// In the hour cycle (e.g. 12 for midnight in `H12`).
    pub hour: Option<i32>,
    /// 0: AM, 1: PM (12-hour cycles only).
    pub day_period: Option<i32>,
    pub minute: Option<i32>,
    pub second: Option<i32>,
    pub nanosecond: Option<i32>,
    /// The UTC offset of a zoned value, to keep a repeated local time (DST) apart.
    pub offset: Option<Offset>,
}

impl IncompleteDate {
    pub fn new<V: DateValue>(hour_cycle: ResolvedHourCycle, value: Option<&V>) -> Self {
        let mut date = Self {
            hour_cycle,
            era: None,
            year: None,
            month: None,
            day: None,
            hour: None,
            day_period: None,
            minute: None,
            second: None,
            nanosecond: None,
            offset: None,
        };
        if let Some(value) = value {
            let (era, year) = Era::of(value.date().year());
            date.era = Some(era);
            date.year = Some(year);
            date.month = Some(i32::from(value.date().month()));
            date.day = Some(i32::from(value.date().day()));
            if V::HAS_TIME {
                let time = value.time();
                let (day_period, hour) = to_hour_cycle(i32::from(time.hour()), hour_cycle);
                date.hour = Some(hour);
                date.day_period = day_period;
                date.minute = Some(i32::from(time.minute()));
                date.second = Some(i32::from(time.second()));
                date.nanosecond = Some(time.subsec_nanosecond());
            }
            date.offset = value.offset();
        }
        date
    }

    pub fn hour_cycle(&self) -> ResolvedHourCycle {
        self.hour_cycle
    }

    /// A segment's value (the era as its index).
    pub fn get(&self, field: DateSegmentType) -> Option<i32> {
        match field {
            DateSegmentType::Era => self.era.map(Era::index),
            DateSegmentType::Year => self.year,
            DateSegmentType::Month => self.month,
            DateSegmentType::Day => self.day,
            DateSegmentType::Hour => self.hour,
            DateSegmentType::DayPeriod => self.day_period,
            DateSegmentType::Minute => self.minute,
            DateSegmentType::Second => self.second,
            DateSegmentType::Literal | DateSegmentType::TimeZoneName => None,
        }
    }

    fn put(&mut self, field: DateSegmentType, value: Option<i32>) {
        match field {
            DateSegmentType::Era => {
                self.era = value.map(|index| if index <= 0 { Era::Bc } else { Era::Ad });
            }
            DateSegmentType::Year => self.year = value,
            DateSegmentType::Month => self.month = value,
            DateSegmentType::Day => self.day = value,
            DateSegmentType::Hour => self.hour = value,
            DateSegmentType::DayPeriod => self.day_period = value,
            DateSegmentType::Minute => self.minute = value,
            DateSegmentType::Second => self.second = value,
            DateSegmentType::Literal | DateSegmentType::TimeZoneName => {}
        }
    }

    /// Whether all `segments` have a value.
    pub fn is_complete(&self, segments: &[DateSegmentType]) -> bool {
        segments.iter().all(|segment| self.get(*segment).is_some())
    }

    /// Whether `value` shows these fields unchanged (it wasn't constrained, e.g. February 30).
    pub fn validate<V: DateValue>(&self, value: &V, segments: &[DateSegmentType]) -> bool {
        let fields = Self::new(self.hour_cycle, Some(value));
        segments.iter().all(|segment| match segment {
            DateSegmentType::Hour | DateSegmentType::DayPeriod if V::HAS_TIME => {
                self.day_period == fields.day_period && self.hour == fields.hour
            }
            segment => self.get(*segment) == fields.get(*segment),
        })
    }

    /// Whether none of `segments` has a value.
    pub fn is_cleared(&self, segments: &[DateSegmentType]) -> bool {
        segments.iter().all(|segment| self.get(*segment).is_none())
    }

    /// With `field` set to `value`.
    #[must_use]
    pub fn set<V: DateValue>(&self, field: DateSegmentType, value: i32, placeholder: &V) -> Self {
        let mut result = self.clone();
        result.put(field, Some(value));
        if field == DateSegmentType::Hour && result.day_period.is_none() && V::HAS_TIME {
            result.day_period =
                to_hour_cycle(i32::from(placeholder.time().hour()), self.hour_cycle).0;
        }
        if field == DateSegmentType::Year && result.era.is_none() {
            result.era = Some(Era::of(placeholder.date().year()).0);
        }
        // A changed date or time may not have this offset any more.
        if !matches!(
            field,
            DateSegmentType::Second | DateSegmentType::Literal | DateSegmentType::TimeZoneName
        ) {
            result.offset = None;
        }
        result
    }

    /// With `field` cleared.
    #[must_use]
    pub fn clear(&self, field: DateSegmentType) -> Self {
        let mut result = self.clone();
        result.put(field, None);
        if field == DateSegmentType::Year {
            result.era = None;
        }
        result.offset = None;
        result
    }

    /// With `field` moved by `amount`, wrapping around (a missing field takes the placeholder's).
    #[must_use]
    pub fn cycle<V: DateValue>(
        &self,
        field: DateSegmentType,
        amount: i32,
        placeholder: &V,
        display_segments: &[DateSegmentType],
    ) -> Self {
        let mut result = self.clone();
        let (placeholder_era, placeholder_year) = Era::of(placeholder.date().year());

        if result.get(field).is_none()
            && !matches!(field, DateSegmentType::DayPeriod | DateSegmentType::Era)
        {
            let placeholder_fields = Self::new(self.hour_cycle, Some(placeholder));
            if field == DateSegmentType::Hour && V::HAS_TIME {
                result.day_period = placeholder_fields.day_period;
                result.hour = placeholder_fields.hour;
            } else {
                result.put(field, placeholder_fields.get(field));
            }
            if field == DateSegmentType::Year && result.era.is_none() {
                result.era = Some(placeholder_era);
            }
            return result;
        }

        match field {
            DateSegmentType::Era => {
                let index = result.era.map_or(1, Era::index);
                result.era = Some(
                    Era::ALL[usize::from(
                        cycle_value(i64::from(index), i64::from(amount), Some(0), 1, false) == 1,
                    )],
                );
            }
            DateSegmentType::Year => {
                // As a date, so that 1 AD and 1 BC go into each other.
                let era = self.era.unwrap_or(placeholder_era);
                let year = self.year.unwrap_or(placeholder_year);
                // BC years count backwards.
                let amount = if era == Era::Bc { -amount } else { amount };
                let cycled = cycle_value(
                    i64::from(year),
                    i64::from(amount),
                    None,
                    i64::from(MAX_YEAR),
                    true,
                );
                let cycled = i32::try_from(cycled).unwrap_or(1);
                let proleptic = era.proleptic_year(cycled);
                let (era, year) = Era::of(i16::try_from(proleptic).unwrap_or(1));
                result.era = Some(era);
                result.year = Some(year);
            }
            DateSegmentType::Month => {
                result.month = Some(cycle_i32(
                    result.month.unwrap_or(1),
                    amount,
                    1,
                    MAX_MONTHS,
                    false,
                ));
            }
            DateSegmentType::Day => {
                // Up to the most days of any month.
                result.day = Some(cycle_i32(
                    result.day.unwrap_or(1),
                    amount,
                    1,
                    MAX_DAYS,
                    false,
                ));
            }
            DateSegmentType::Hour => {
                let has_date_segments = display_segments.iter().any(|segment| {
                    matches!(
                        segment,
                        DateSegmentType::Year | DateSegmentType::Month | DateSegmentType::Day
                    )
                });
                let zoned = placeholder.time_zone().is_some()
                    && (!has_date_segments
                        || (result.year.is_some()
                            && result.month.is_some()
                            && result.day.is_some()));
                let cycled = zoned.then(|| self.to_value(placeholder)).and_then(|value| {
                    // The value's instant (its offset tells a repeated local time apart).
                    let time_zone = value.time_zone()?.clone();
                    let timestamp = value.offset()?.to_timestamp(value.date_time()).ok()?;
                    cycle_zoned_hour(
                        &timestamp.to_zoned(time_zone),
                        amount,
                        self.hour_cycle.is_12_hour(),
                    )
                });
                if let Some(cycled) = cycled {
                    let (day_period, hour) =
                        to_hour_cycle(i32::from(cycled.hour()), self.hour_cycle);
                    result.hour = Some(hour);
                    result.day_period = day_period;
                    result.offset = Some(cycled.offset());
                } else {
                    let limits = self.segment_limits(DateSegmentType::Hour);
                    let (min, max) =
                        limits.map_or((0, 23), |limits| (limits.min_value, limits.max_value));
                    result.hour =
                        Some(cycle_i32(result.hour.unwrap_or(0), amount, min, max, false));
                    if result.day_period.is_none() && V::HAS_TIME {
                        result.day_period =
                            to_hour_cycle(i32::from(placeholder.time().hour()), self.hour_cycle).0;
                    }
                }
            }
            DateSegmentType::DayPeriod => {
                result.day_period = Some(cycle_i32(
                    result.day_period.unwrap_or(0),
                    amount,
                    0,
                    1,
                    false,
                ));
            }
            DateSegmentType::Minute => {
                result.minute = Some(cycle_i32(result.minute.unwrap_or(0), amount, 0, 59, true));
            }
            DateSegmentType::Second => {
                result.second = Some(cycle_i32(result.second.unwrap_or(0), amount, 0, 59, true));
            }
            DateSegmentType::Literal | DateSegmentType::TimeZoneName => {}
        }
        result
    }

    /// A value of `value`'s type with these fields (missing ones from `value`); a day beyond
    /// its month is constrained to the month's last day.
    pub fn to_value<V: DateValue>(&self, value: &V) -> V {
        let (value_era, value_year) = Era::of(value.date().year());
        let era = self.era.unwrap_or(value_era);
        let year = era.proleptic_year(self.year.unwrap_or(value_year));
        let month = self.month.unwrap_or(i32::from(value.date().month()));
        let day = self.day.unwrap_or(i32::from(value.date().day()));
        let date = constrained_date(year, month, day).unwrap_or_else(|| value.date());
        if !V::HAS_TIME {
            return value.with_fields(date, Time::midnight(), None);
        }
        let hour = match self.hour {
            Some(hour) => Some(from_hour_cycle(
                hour,
                self.day_period.unwrap_or(0),
                self.hour_cycle,
            )),
            None if self.hour_cycle.is_12_hour() => {
                Some(if self.day_period == Some(1) { 12 } else { 0 })
            }
            None => None,
        };
        let time = value.time();
        let time = Time::new(
            hour.and_then(|hour| i8::try_from(hour).ok())
                .unwrap_or(time.hour()),
            self.minute
                .and_then(|minute| i8::try_from(minute).ok())
                .unwrap_or(time.minute()),
            self.second
                .and_then(|second| i8::try_from(second).ok())
                .unwrap_or(time.second()),
            self.nanosecond.unwrap_or(time.subsec_nanosecond()),
        )
        .unwrap_or(time);
        value.with_fields(date, time, self.offset)
    }

    /// The limits of a segment (`None` for literals and the time zone).
    pub fn segment_limits(&self, kind: DateSegmentType) -> Option<SegmentLimits> {
        let limits = |value, min_value, max_value| {
            Some(SegmentLimits {
                value,
                min_value,
                max_value,
            })
        };
        match kind {
            DateSegmentType::Era => limits(Some(self.era.map_or(1, Era::index)), 0, 1),
            DateSegmentType::Year => limits(self.year, 1, MAX_YEAR),
            DateSegmentType::Month => limits(self.month, 1, MAX_MONTHS),
            DateSegmentType::Day => limits(self.day, 1, MAX_DAYS),
            DateSegmentType::DayPeriod => limits(self.day_period, 0, 1),
            DateSegmentType::Hour => {
                let (min_value, max_value) = match self.hour_cycle {
                    ResolvedHourCycle::H12 => (1, 12),
                    ResolvedHourCycle::H11 => (0, 11),
                    ResolvedHourCycle::H23 | ResolvedHourCycle::H24 => (0, 23),
                };
                limits(self.hour, min_value, max_value)
            }
            DateSegmentType::Minute => limits(self.minute, 0, 59),
            DateSegmentType::Second => limits(self.second, 0, 59),
            DateSegmentType::Literal | DateSegmentType::TimeZoneName => None,
        }
    }
}

/// The date, the day constrained to the month's length.
fn constrained_date(year: i32, month: i32, day: i32) -> Option<Date> {
    let year = i16::try_from(year).ok()?;
    let month = i8::try_from(month.clamp(1, MAX_MONTHS)).ok()?;
    let first = Date::new(year, month, 1).ok()?;
    let day = i8::try_from(day.clamp(1, i32::from(first.days_in_month()))).ok()?;
    Date::new(year, month, day).ok()
}

fn cycle_i32(value: i32, amount: i32, min: i32, max: i32, round: bool) -> i32 {
    let cycled = cycle_value(
        i64::from(value),
        i64::from(amount),
        Some(i64::from(min)),
        i64::from(max),
        round,
    );
    i32::try_from(cycled).unwrap_or(min)
}

/// `value` moved by `amount` within `min` (`None`: unbounded) and `max`, wrapping around;
/// rounding to a multiple of `amount` (react-aria's page steps).
fn cycle_value(value: i64, amount: i64, min: Option<i64>, max: i64, round: bool) -> i64 {
    let below = |value: i64| min.is_some_and(|min| value < min);
    // Below an unbounded minimum, the value wraps to 1 (as `-Infinity` upstream).
    let wrap_min = min.unwrap_or(1);
    if round {
        let mut value = value + amount.signum();
        if below(value) {
            value = max;
        }
        let div = amount.abs().max(1);
        value = if amount > 0 {
            value.div_euclid(div) * div + if value.rem_euclid(div) == 0 { 0 } else { div }
        } else {
            value.div_euclid(div) * div
        };
        if value > max {
            value = wrap_min;
        }
        value
    } else {
        let value = value + amount;
        match min {
            Some(min) if value < min => max - (min - value - 1),
            _ if value > max => wrap_min + (value - max - 1),
            _ => value,
        }
    }
}

/// Cycles the hour of a zoned value through the hours of its day (in absolute time, so that a
/// day with a DST change has one more or one less hour); with `twelve_hour`, within its half.
fn cycle_zoned_hour(value: &Zoned, amount: i32, twelve_hour: bool) -> Option<Zoned> {
    const HOUR: i64 = 3_600;
    let (min, max) = if twelve_hour {
        if value.hour() >= 12 {
            (12, 23)
        } else {
            (0, 11)
        }
    } else {
        (0, 23)
    };
    let time_zone = value.time_zone().clone();
    let date = value.date();
    // The instants of an hour of the day (two if it repeats), within the day.
    let instants = |hour: i8| -> Vec<jiff::Timestamp> {
        let Ok(time) = Time::new(hour, 0, 0, 0) else {
            return Vec::new();
        };
        let ambiguous = time_zone.to_ambiguous_timestamp(date.to_datetime(time));
        [ambiguous.earlier().ok(), ambiguous.later().ok()]
            .into_iter()
            .flatten()
            .filter(|timestamp| timestamp.to_zoned(time_zone.clone()).date() == date)
            .collect()
    };
    let min_absolute = *instants(min).first()?;
    let max_absolute = *instants(max).last()?;
    let seconds = value.timestamp().as_second();
    let hours = seconds.div_euclid(HOUR);
    let remainder = value.timestamp().as_duration() - SignedDuration::from_secs(hours * HOUR);
    let cycled = cycle_value(
        hours,
        i64::from(amount),
        Some(min_absolute.as_second().div_euclid(HOUR)),
        max_absolute.as_second().div_euclid(HOUR),
        false,
    );
    let timestamp =
        jiff::Timestamp::from_duration(SignedDuration::from_secs(cycled * HOUR) + remainder)
            .ok()?;
    Some(timestamp.to_zoned(time_zone))
}

/// A 24-hour hour in the hour cycle: (day period, hour).
pub(crate) fn to_hour_cycle(hour: i32, hour_cycle: ResolvedHourCycle) -> (Option<i32>, i32) {
    let day_period = Some(i32::from(hour >= 12));
    match hour_cycle {
        ResolvedHourCycle::H11 => (day_period, if hour >= 12 { hour - 12 } else { hour }),
        ResolvedHourCycle::H12 => (
            day_period,
            match hour {
                0 => 12,
                hour if hour > 12 => hour - 12,
                hour => hour,
            },
        ),
        ResolvedHourCycle::H23 => (None, hour),
        ResolvedHourCycle::H24 => (None, hour + 1),
    }
}

/// An hour of the hour cycle as a 24-hour hour.
pub(crate) fn from_hour_cycle(hour: i32, day_period: i32, hour_cycle: ResolvedHourCycle) -> i32 {
    match hour_cycle {
        ResolvedHourCycle::H11 => hour + if day_period == 1 { 12 } else { 0 },
        ResolvedHourCycle::H12 => {
            (if hour == 12 { 0 } else { hour }) + if day_period == 1 { 12 } else { 0 }
        }
        ResolvedHourCycle::H23 => hour,
        ResolvedHourCycle::H24 => hour - 1,
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;
    use jiff::civil::{date, datetime};

    use super::*;

    const DATE: [DateSegmentType; 3] = [
        DateSegmentType::Month,
        DateSegmentType::Day,
        DateSegmentType::Year,
    ];

    #[test]
    fn cycles_values_with_and_without_rounding() {
        assert_that!(cycle_value(59, 1, Some(0), 59, false)).is_equal_to(0);
        assert_that!(cycle_value(0, -1, Some(0), 59, false)).is_equal_to(59);
        assert_that!(cycle_value(7, 15, Some(0), 59, true)).is_equal_to(15);
        assert_that!(cycle_value(15, 15, Some(0), 59, true)).is_equal_to(30);
        assert_that!(cycle_value(50, 15, Some(0), 59, true)).is_equal_to(0);
        assert_that!(cycle_value(7, -15, Some(0), 59, true)).is_equal_to(0);
        assert_that!(cycle_value(0, -15, Some(0), 59, true)).is_equal_to(45);
        assert_that!(cycle_value(9999, 1, None, 9999, true)).is_equal_to(1);
    }

    #[test]
    fn converts_hour_cycles() {
        assert_that!(to_hour_cycle(0, ResolvedHourCycle::H12)).is_equal_to((Some(0), 12));
        assert_that!(to_hour_cycle(13, ResolvedHourCycle::H12)).is_equal_to((Some(1), 1));
        assert_that!(to_hour_cycle(12, ResolvedHourCycle::H11)).is_equal_to((Some(1), 0));
        assert_that!(to_hour_cycle(0, ResolvedHourCycle::H24)).is_equal_to((None, 1));
        for hour in 0..24 {
            for cycle in [
                ResolvedHourCycle::H11,
                ResolvedHourCycle::H12,
                ResolvedHourCycle::H23,
                ResolvedHourCycle::H24,
            ] {
                let (day_period, converted) = to_hour_cycle(hour, cycle);
                assert_that!(from_hour_cycle(converted, day_period.unwrap_or(0), cycle))
                    .is_equal_to(hour);
            }
        }
    }

    #[test]
    fn edits_fields_and_builds_values() {
        let placeholder = date(2024, 1, 1);
        let empty = IncompleteDate::new::<Date>(ResolvedHourCycle::H12, None);
        assert_that!(empty.is_cleared(&DATE)).is_true();
        let partial = empty.set(DateSegmentType::Month, 2, &placeholder).set(
            DateSegmentType::Day,
            30,
            &placeholder,
        );
        assert_that!(partial.is_complete(&DATE)).is_false();
        let complete = partial.set(DateSegmentType::Year, 2023, &placeholder);
        assert_that!(complete.era).is_equal_to(Some(Era::Ad));
        assert_that!(complete.is_complete(&DATE)).is_true();
        // February 30 is constrained, so it doesn't validate.
        let value = complete.to_value(&placeholder);
        assert_that!(value).is_equal_to(date(2023, 2, 28));
        assert_that!(complete.validate(&value, &DATE)).is_false();
        let valid = complete.set(DateSegmentType::Day, 28, &placeholder);
        assert_that!(valid.validate(&valid.to_value(&placeholder), &DATE)).is_true();
    }

    #[test]
    fn cycles_from_the_placeholder_and_across_eras() {
        let placeholder = date(2024, 5, 20);
        let empty = IncompleteDate::new::<Date>(ResolvedHourCycle::H12, None);
        // A missing field takes the placeholder's value first.
        let year = empty.cycle(DateSegmentType::Year, 1, &placeholder, &DATE);
        assert_that!(year.year).is_equal_to(Some(2024));
        let day = empty.cycle(DateSegmentType::Day, -1, &placeholder, &DATE);
        assert_that!(day.day).is_equal_to(Some(20));
        // 1 AD - 1 is 1 BC; 1 BC + 1 is 1 AD.
        let ad1 = IncompleteDate::new(ResolvedHourCycle::H12, Some(&date(1, 1, 1)));
        let bc1 = ad1.cycle(DateSegmentType::Year, -1, &placeholder, &DATE);
        assert_that!((bc1.era, bc1.year)).is_equal_to((Some(Era::Bc), Some(1)));
        assert_that!(bc1.to_value(&placeholder).year()).is_equal_to(0);
        let back = bc1.cycle(DateSegmentType::Year, 1, &placeholder, &DATE);
        assert_that!((back.era, back.year)).is_equal_to((Some(Era::Ad), Some(1)));
    }

    #[test]
    fn keeps_times_in_the_hour_cycle() {
        let value = datetime(2024, 5, 20, 0, 30, 0, 0);
        let fields = IncompleteDate::new(ResolvedHourCycle::H12, Some(&value));
        assert_that!((fields.hour, fields.day_period)).is_equal_to((Some(12), Some(0)));
        let segments = [DateSegmentType::Hour, DateSegmentType::DayPeriod];
        let next = fields.cycle(DateSegmentType::Hour, 1, &value, &segments);
        assert_that!(next.hour).is_equal_to(Some(1));
        let pm = next.cycle(DateSegmentType::DayPeriod, 1, &value, &segments);
        assert_that!(pm.to_value(&value)).is_equal_to(datetime(2024, 5, 20, 13, 30, 0, 0));
    }

    #[test]
    fn cycles_zoned_hours_through_a_dst_change() {
        // New York falls back on 2024-11-03: 1 AM occurs twice.
        let value = date(2024, 11, 3)
            .at(0, 30, 0, 0)
            .in_tz("America/New_York")
            .expect("a zoned value");
        let segments = [DateSegmentType::Hour, DateSegmentType::Minute];
        let fields = IncompleteDate::new(ResolvedHourCycle::H23, Some(&value));
        let first = fields.cycle(DateSegmentType::Hour, 1, &value, &segments);
        assert_that!(first.hour).is_equal_to(Some(1));
        let first_value = first.to_value(&value);
        let second = IncompleteDate::new(ResolvedHourCycle::H23, Some(&first_value)).cycle(
            DateSegmentType::Hour,
            1,
            &first_value,
            &segments,
        );
        // The repeated 1 AM, at the standard offset.
        assert_that!(second.hour).is_equal_to(Some(1));
        assert_that!(second.offset).is_not_equal_to(first.offset);
        let second_value = second.to_value(&first_value);
        assert_that!(second_value.timestamp().as_second() - first_value.timestamp().as_second())
            .is_equal_to(3600);
    }
}
