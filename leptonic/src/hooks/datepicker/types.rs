// Upstream: react-stately/src/datepicker/types.ts @ 99e6102368
//! The values and segments of date and time fields.

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Values are `jiff` types behind the `DateValue` trait (`civil::Date`, `civil::DateTime`,
//   `Zoned`; react-aria: `@internationalized/date`'s `CalendarDate`, `CalendarDateTime`,
//   `ZonedDateTime`), times behind `TimeValue` (`civil::Time`, ...). A field returns the type it
//   was given (react-aria's `MappedDateValue`).
// - `Granularity`, `MaxGranularity`, `HourCycle`, `DateSegmentType`: enums (react-aria: strings
//   and numbers); `Era` is the Gregorian era of a proleptic year (react-aria: era strings of any
//   calendar).
//
// ## DIFFERENT BEHAVIOR
// - A local time a zoned value doesn't have once (`to_zoned`): a repeated time keeps its offset
//   if it has one there (else the earlier), a skipped one moves forward (`Temporal`'s
//   `compatible`, as `@internationalized/date`'s `toZoned` with `disambiguation: 'compatible'`).
//
// ## OMITTED FEATURES
// - Calendar systems other than the Gregorian.
//
// =============================================================================

use std::{cmp::Ordering, fmt::Debug};

use jiff::{
    Timestamp, Zoned,
    civil::{Date, DateTime, Time},
    tz::{AmbiguousOffset, Offset, TimeZone},
};

/// The finest unit a date field edits (react-aria's `granularity`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Granularity {
    Day,
    Hour,
    Minute,
    Second,
}

impl Granularity {
    /// Whether the field edits a time of day.
    pub fn has_time(self) -> bool {
        self != Self::Day
    }

    pub(crate) fn segment(self) -> DateSegmentType {
        match self {
            Self::Day => DateSegmentType::Day,
            Self::Hour => DateSegmentType::Hour,
            Self::Minute => DateSegmentType::Minute,
            Self::Second => DateSegmentType::Second,
        }
    }
}

/// The coarsest unit a date field edits (react-aria's `maxGranularity`), e.g. `Month` for a
/// field of a month and day.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum MaxGranularity {
    #[default]
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
}

impl MaxGranularity {
    pub(crate) fn segment(self) -> DateSegmentType {
        match self {
            Self::Year => DateSegmentType::Year,
            Self::Month => DateSegmentType::Month,
            Self::Day => DateSegmentType::Day,
            Self::Hour => DateSegmentType::Hour,
            Self::Minute => DateSegmentType::Minute,
            Self::Second => DateSegmentType::Second,
        }
    }
}

/// Whether times show 12 or 24 hours (react-aria's `hourCycle: 12 | 24`). Default: the locale's.
pub use crate::utils::date_time_formatter::HourCycle;

/// How hours are numbered (`Intl`'s resolved `hourCycle`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResolvedHourCycle {
    /// 0 to 11 with a day period (Japan).
    H11,
    /// 12, 1 to 11 with a day period.
    H12,
    /// 0 to 23.
    H23,
    /// 24, 1 to 23.
    H24,
}

impl ResolvedHourCycle {
    /// Whether hours come with a day period (AM/PM).
    pub fn is_12_hour(self) -> bool {
        matches!(self, Self::H11 | Self::H12)
    }
}

/// An era of the Gregorian calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Era {
    /// Before Christ: era year 1 is the proleptic year 0.
    Bc,
    /// Anno Domini.
    Ad,
}

impl Era {
    pub(crate) const ALL: [Self; 2] = [Self::Bc, Self::Ad];

    /// Era and era year of a proleptic year (`jiff`'s years: 0 is 1 BC).
    pub fn of(year: i16) -> (Self, i32) {
        if year <= 0 {
            (Self::Bc, 1 - i32::from(year))
        } else {
            (Self::Ad, i32::from(year))
        }
    }

    /// The proleptic year of an era year.
    pub fn proleptic_year(self, era_year: i32) -> i32 {
        match self {
            Self::Bc => 1 - era_year,
            Self::Ad => era_year,
        }
    }

    pub(crate) fn index(self) -> i32 {
        match self {
            Self::Bc => 0,
            Self::Ad => 1,
        }
    }
}

/// What a segment of a date field shows (react-stately's `DateSegmentType`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DateSegmentType {
    Era,
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    DayPeriod,
    Literal,
    TimeZoneName,
}

impl DateSegmentType {
    /// Whether the user edits this segment.
    pub fn is_editable(self) -> bool {
        !matches!(self, Self::Literal | Self::TimeZoneName)
    }

    /// The `data-type` of the segment's element.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Era => "era",
            Self::Year => "year",
            Self::Month => "month",
            Self::Day => "day",
            Self::Hour => "hour",
            Self::Minute => "minute",
            Self::Second => "second",
            Self::DayPeriod => "dayPeriod",
            Self::Literal => "literal",
            Self::TimeZoneName => "timeZoneName",
        }
    }
}

/// A segment of a date field: an editable part (year, month, ...) or a literal between them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateSegment {
    pub kind: DateSegmentType,
    /// The text shown: the formatted value, or the placeholder.
    pub text: String,
    /// The value: a number, the era's index (0: BC, 1: AD) or the day period (0: AM, 1: PM).
    pub value: Option<i32>,
    pub min_value: Option<i32>,
    pub max_value: Option<i32>,
    /// Whether the segment shows its placeholder (no value yet).
    pub is_placeholder: bool,
    pub placeholder: String,
    pub is_editable: bool,
}

impl DateSegment {
    pub(crate) fn literal(text: impl Into<String>) -> Self {
        Self {
            kind: DateSegmentType::Literal,
            text: text.into(),
            value: None,
            min_value: None,
            max_value: None,
            is_placeholder: false,
            placeholder: String::new(),
            is_editable: false,
        }
    }
}

/// A value of a date field: a [`Date`] (granularity day), a [`DateTime`] or a [`Zoned`]
/// (react-aria's `DateValue`, `CalendarDate | CalendarDateTime | ZonedDateTime`). Fields are
/// generic over it, so that they return the type they were given.
pub trait DateValue: Clone + PartialEq + Debug + Send + Sync + 'static {
    /// Whether the value has a time of day.
    const HAS_TIME: bool;

    fn date(&self) -> Date;

    /// The time of day (midnight for a date).
    fn time(&self) -> Time;

    /// The time zone of a zoned value.
    fn time_zone(&self) -> Option<&TimeZone>;

    /// The offset from UTC of a zoned value.
    fn offset(&self) -> Option<Offset>;

    /// This value with another date and time (a date ignores the time), in its time zone.
    /// `offset` picks the instant of a local time that occurs twice (DST).
    #[must_use]
    fn with_fields(&self, date: Date, time: Time, offset: Option<Offset>) -> Self;

    /// The order of two values (of zoned values: of their instants).
    fn compare(&self, other: &Self) -> Ordering;

    /// Today, at midnight for values with a time, in `time_zone` for zoned values (else in the
    /// system's): the default placeholder.
    fn today(time_zone: Option<&TimeZone>) -> Self;

    /// ISO 8601 (with the time zone's name for zoned values), e.g. for forms.
    fn to_iso_string(&self) -> String;

    /// The date and time.
    fn date_time(&self) -> DateTime {
        self.date().to_datetime(self.time())
    }
}

impl DateValue for Date {
    const HAS_TIME: bool = false;

    fn date(&self) -> Date {
        *self
    }

    fn time(&self) -> Time {
        Time::midnight()
    }

    fn time_zone(&self) -> Option<&TimeZone> {
        None
    }

    fn offset(&self) -> Option<Offset> {
        None
    }

    fn with_fields(&self, date: Date, _: Time, _: Option<Offset>) -> Self {
        date
    }

    fn compare(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }

    fn today(_: Option<&TimeZone>) -> Self {
        crate::utils::date::today()
    }

    fn to_iso_string(&self) -> String {
        self.to_string()
    }
}

impl DateValue for DateTime {
    const HAS_TIME: bool = true;

    fn date(&self) -> Date {
        DateTime::date(*self)
    }

    fn time(&self) -> Time {
        DateTime::time(*self)
    }

    fn time_zone(&self) -> Option<&TimeZone> {
        None
    }

    fn offset(&self) -> Option<Offset> {
        None
    }

    fn with_fields(&self, date: Date, time: Time, _: Option<Offset>) -> Self {
        date.to_datetime(time)
    }

    fn compare(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }

    fn today(_: Option<&TimeZone>) -> Self {
        crate::utils::date::today().to_datetime(Time::midnight())
    }

    fn to_iso_string(&self) -> String {
        self.to_string()
    }
}

impl DateValue for Zoned {
    const HAS_TIME: bool = true;

    fn date(&self) -> Date {
        Zoned::date(self)
    }

    fn time(&self) -> Time {
        Zoned::time(self)
    }

    fn time_zone(&self) -> Option<&TimeZone> {
        Some(Zoned::time_zone(self))
    }

    fn offset(&self) -> Option<Offset> {
        Some(Zoned::offset(self))
    }

    fn with_fields(&self, date: Date, time: Time, offset: Option<Offset>) -> Self {
        to_zoned(date.to_datetime(time), self.time_zone().clone(), offset)
            .unwrap_or_else(|| self.clone())
    }

    fn compare(&self, other: &Self) -> Ordering {
        self.timestamp().cmp(&other.timestamp())
    }

    fn today(time_zone: Option<&TimeZone>) -> Self {
        let time_zone = time_zone.cloned().unwrap_or_else(TimeZone::system);
        let today = Timestamp::now().to_zoned(time_zone.clone()).date();
        to_zoned(today.to_datetime(Time::midnight()), time_zone, None).unwrap_or_else(Zoned::now)
    }

    fn to_iso_string(&self) -> String {
        self.to_string()
    }
}

/// `date_time` in `time_zone`: a repeated local time at `offset` if it has one there (else the
/// earlier), a skipped one moved forward (as `Temporal`'s `compatible`).
pub(crate) fn to_zoned(
    date_time: DateTime,
    time_zone: TimeZone,
    offset: Option<Offset>,
) -> Option<Zoned> {
    let ambiguous = time_zone.to_ambiguous_zoned(date_time);
    if let (AmbiguousOffset::Fold { after, .. }, Some(offset)) = (ambiguous.offset(), offset)
        && after == offset
    {
        return ambiguous.later().ok();
    }
    ambiguous.compatible().ok()
}

/// A value of a time field: a [`Time`], or a [`DateTime`] or [`Zoned`] whose time is edited
/// (react-aria's `TimeValue`). The field edits it as a date value ([`TimeValue::Field`]).
pub trait TimeValue: Clone + PartialEq + Debug + Send + Sync + 'static {
    /// The date value the field edits.
    type Field: DateValue;

    /// As the field's value, on `date` if it has no date of its own.
    fn to_field(&self, date: Date) -> Self::Field;

    /// From the field's value.
    fn from_field(field: Self::Field) -> Self;
}

impl TimeValue for Time {
    type Field = DateTime;

    fn to_field(&self, date: Date) -> DateTime {
        date.to_datetime(*self)
    }

    fn from_field(field: DateTime) -> Self {
        field.time()
    }
}

impl TimeValue for DateTime {
    type Field = DateTime;

    fn to_field(&self, _: Date) -> DateTime {
        *self
    }

    fn from_field(field: DateTime) -> Self {
        field
    }
}

impl TimeValue for Zoned {
    type Field = Zoned;

    fn to_field(&self, _: Date) -> Zoned {
        self.clone()
    }

    fn from_field(field: Zoned) -> Self {
        field
    }
}

/// A range of values (react-aria's `RangeValue`), e.g. of a date range picker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangeValue<V> {
    pub start: V,
    pub end: V,
}
