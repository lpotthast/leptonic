// Upstream: internationalized/date/src/manipulation.ts @ 99e6102368
// Upstream: internationalized/date/src/queries.ts @ 99e6102368
//! Calendar dates: the parts of react-aria's `@internationalized/date` the calendar and date
//! hooks need, for the Gregorian calendar on `jiff::civil::Date`.

use std::cmp::Ordering;

use jiff::{
    Span,
    civil::{Date, Weekday},
};

use crate::utils::i18n::Locale;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Dates are `jiff::civil::Date` (Gregorian, years -9999 to 9999) with the [`DateExt`] methods;
//   react-aria's `CalendarDate` supports other calendar systems and eras.
// - The first day of the week is a `jiff::civil::Weekday` (react-aria: `'sun' | 'mon' | ...`).
//
// ## OMITTED FEATURES
// - Calendar systems other than the Gregorian (Japanese, Hebrew, ...), eras.
//
// =============================================================================

/// A length of time in calendar units (react-aria's `DateDuration`): years, months, weeks and
/// days, each possibly zero or negative.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct DateDuration {
    pub years: i32,
    pub months: i32,
    pub weeks: i32,
    pub days: i32,
}

impl DateDuration {
    #[must_use]
    pub const fn years(years: i32) -> Self {
        Self {
            years,
            months: 0,
            weeks: 0,
            days: 0,
        }
    }

    #[must_use]
    pub const fn months(months: i32) -> Self {
        Self {
            years: 0,
            months,
            weeks: 0,
            days: 0,
        }
    }

    #[must_use]
    pub const fn weeks(weeks: i32) -> Self {
        Self {
            years: 0,
            months: 0,
            weeks,
            days: 0,
        }
    }

    #[must_use]
    pub const fn days(days: i32) -> Self {
        Self {
            years: 0,
            months: 0,
            weeks: 0,
            days,
        }
    }

    /// The duration with every unit negated.
    #[must_use]
    pub const fn negated(self) -> Self {
        Self {
            years: -self.years,
            months: -self.months,
            weeks: -self.weeks,
            days: -self.days,
        }
    }

    /// One of each unit this duration uses (react-aria's `unitDuration`).
    #[must_use]
    pub const fn unit(self) -> Self {
        Self {
            years: if self.years != 0 { 1 } else { 0 },
            months: if self.months != 0 { 1 } else { 0 },
            weeks: if self.weeks != 0 { 1 } else { 0 },
            days: if self.days != 0 { 1 } else { 0 },
        }
    }
}

/// A range of dates, both ends included (react-aria's `RangeValue<CalendarDate>`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DateRange {
    pub start: Date,
    pub end: Date,
}

impl DateRange {
    /// The range between two dates in either order (react-aria's `makeRange`).
    #[must_use]
    pub fn between(a: Date, b: Date) -> Self {
        Self {
            start: min_date(a, b),
            end: max_date(a, b),
        }
    }

    /// Whether `date` lies in the range.
    #[must_use]
    pub fn contains(&self, date: Date) -> bool {
        self.start <= date && date <= self.end
    }
}

/// Calendar arithmetic and queries on `jiff::civil::Date` (react-aria's `CalendarDate` methods
/// and date queries).
pub trait DateExt: Sized {
    /// The date `duration` later: years and months together first (the day is constrained to the
    /// month's length: January 31 plus a month is February 29 in a leap year), then weeks and
    /// days. Clamped to the representable range.
    #[must_use]
    fn add(self, duration: DateDuration) -> Self;

    /// The date `duration` earlier.
    #[must_use]
    fn subtract(self, duration: DateDuration) -> Self;

    /// The date in the same year and month with day `day` (constrained to the month's length).
    #[must_use]
    fn with_day(self, day: i8) -> Self;

    /// The date with month `month` (1 to 12) of the same year, the day constrained.
    #[must_use]
    fn with_month(self, month: i8) -> Self;

    /// The first day of the date's week, with weeks starting on `first_day`.
    #[must_use]
    fn start_of_week(self, first_day: Weekday) -> Self;
    /// The last day of the date's week, with weeks starting on `first_day`.
    #[must_use]
    fn end_of_week(self, first_day: Weekday) -> Self;

    /// The date's position in its week, from 0 (`first_day`) to 6.
    fn day_of_week(self, first_day: Weekday) -> u8;

    /// How many (partial) weeks the date's month spans, with weeks starting on `first_day`.
    fn weeks_in_month(self, first_day: Weekday) -> u8;

    fn is_same_month(&self, other: Self) -> bool;
    fn is_same_year(&self, other: Self) -> bool;
}

impl DateExt for Date {
    fn add(self, duration: DateDuration) -> Self {
        let months = i64::from(duration.years) * 12 + i64::from(duration.months);
        let days = i64::from(duration.weeks) * 7 + i64::from(duration.days);
        let clamp = |amount: i64| if amount < 0 { Date::MIN } else { Date::MAX };
        // Years and months in one step: jiff would apply them separately and constrain the day
        // twice (February 29 plus a year and a month: March 29, not March 28).
        let date = if months == 0 {
            self
        } else {
            match Span::new()
                .try_months(months)
                .ok()
                .and_then(|span| self.checked_add(span).ok())
            {
                Some(date) => date,
                None => return clamp(months),
            }
        };
        if days == 0 {
            return date;
        }
        Span::new()
            .try_days(days)
            .ok()
            .and_then(|span| date.checked_add(span).ok())
            .unwrap_or_else(|| clamp(days))
    }

    fn subtract(self, duration: DateDuration) -> Self {
        self.add(duration.negated())
    }

    fn with_day(self, day: i8) -> Self {
        let day = day.clamp(1, self.days_in_month());
        self.with().day(day).build().unwrap_or(self)
    }

    fn with_month(self, month: i8) -> Self {
        let first = self
            .with()
            .day(1)
            .month(month.clamp(1, 12))
            .build()
            .unwrap_or(self);
        first.with_day(self.day())
    }

    fn start_of_week(self, first_day: Weekday) -> Self {
        self.subtract(DateDuration::days(i32::from(self.day_of_week(first_day))))
    }

    fn end_of_week(self, first_day: Weekday) -> Self {
        self.add(DateDuration::days(
            6 - i32::from(self.day_of_week(first_day)),
        ))
    }

    fn day_of_week(self, first_day: Weekday) -> u8 {
        let offset =
            (7 + self.weekday().to_monday_zero_offset() - first_day.to_monday_zero_offset()) % 7;
        u8::try_from(offset).unwrap_or_default()
    }

    fn weeks_in_month(self, first_day: Weekday) -> u8 {
        let days = u8::try_from(self.days_in_month()).unwrap_or(31);
        (self.first_of_month().day_of_week(first_day) + days).div_ceil(7)
    }

    fn is_same_month(&self, other: Self) -> bool {
        self.year() == other.year() && self.month() == other.month()
    }

    fn is_same_year(&self, other: Self) -> bool {
        self.year() == other.year()
    }
}

/// The earlier of two dates.
#[must_use]
pub fn min_date(a: Date, b: Date) -> Date {
    match a.cmp(&b) {
        Ordering::Greater => b,
        _ => a,
    }
}

/// The later of two dates.
#[must_use]
pub fn max_date(a: Date, b: Date) -> Date {
    match a.cmp(&b) {
        Ordering::Less => b,
        _ => a,
    }
}

/// The first day of the week in `locale` (from CLDR's week data, honoring a `-u-fw-` override),
/// e.g. Sunday in `en-US`, Monday in `de-DE`.
#[must_use]
pub fn first_day_of_week(locale: &Locale) -> Weekday {
    use icu_calendar::{types::Weekday as IcuWeekday, week::WeekInformation};

    let Ok(info) = WeekInformation::try_new(locale.icu_locale().into()) else {
        return Weekday::Monday;
    };
    match info.first_weekday {
        IcuWeekday::Monday => Weekday::Monday,
        IcuWeekday::Tuesday => Weekday::Tuesday,
        IcuWeekday::Wednesday => Weekday::Wednesday,
        IcuWeekday::Thursday => Weekday::Thursday,
        IcuWeekday::Friday => Weekday::Friday,
        IcuWeekday::Saturday => Weekday::Saturday,
        IcuWeekday::Sunday => Weekday::Sunday,
    }
}

/// Today in the system's time zone: the browser's in the browser, the server's on the server.
#[must_use]
pub fn today() -> Date {
    jiff::Zoned::now().date()
}

/// Today in the browser's time zone, for marking it: `None` on the server and until hydrated.
///
/// The server's today may be another date (its time zone), and hydration keeps server-rendered
/// attributes and text, so marks derived from it would stay wrong.
pub fn use_today() -> leptos::prelude::Signal<Option<Date>> {
    use leptos::prelude::{Effect, RwSignal, Set};

    let today_signal = RwSignal::new(None);
    Effect::new(move |_| today_signal.set(Some(today())));
    today_signal.into()
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;
    use jiff::civil::date;

    use super::*;

    #[test]
    fn adds_months_constraining_the_day() {
        assert_that!(date(2024, 1, 31).add(DateDuration::months(1))).is_equal_to(date(2024, 2, 29));
        assert_that!(date(2023, 1, 31).add(DateDuration::months(1))).is_equal_to(date(2023, 2, 28));
        assert_that!(date(2024, 3, 31).subtract(DateDuration::months(1)))
            .is_equal_to(date(2024, 2, 29));
        assert_that!(date(2024, 12, 15).add(DateDuration::months(1)))
            .is_equal_to(date(2025, 1, 15));
        assert_that!(date(2024, 2, 29).add(DateDuration::years(1))).is_equal_to(date(2025, 2, 28));
        // Years and months in one step (react-aria): March 29, not 28.
        assert_that!(date(2024, 2, 29).add(DateDuration {
            years: 1,
            months: 1,
            ..DateDuration::default()
        }))
        .is_equal_to(date(2025, 3, 29));
        assert_that!(date(2024, 1, 30).add(DateDuration {
            months: 1,
            days: 2,
            ..DateDuration::default()
        }))
        .is_equal_to(date(2024, 3, 2));
        assert_that!(Date::MAX.add(DateDuration::days(1))).is_equal_to(Date::MAX);
        assert_that!(Date::MIN.subtract(DateDuration::years(1))).is_equal_to(Date::MIN);
    }

    #[test]
    fn weeks_start_on_the_given_day() {
        // 2024-05-15 is a Wednesday.
        let wednesday = date(2024, 5, 15);
        assert_that!(wednesday.start_of_week(Weekday::Sunday)).is_equal_to(date(2024, 5, 12));
        assert_that!(wednesday.start_of_week(Weekday::Monday)).is_equal_to(date(2024, 5, 13));
        assert_that!(wednesday.end_of_week(Weekday::Monday)).is_equal_to(date(2024, 5, 19));
        assert_that!(wednesday.day_of_week(Weekday::Sunday)).is_equal_to(3);
        // May 2024 starts on a Wednesday: 5 rows; June 2024 (a Saturday, 30 days): 6 from
        // Sunday, 5 from Monday.
        assert_that!(wednesday.weeks_in_month(Weekday::Sunday)).is_equal_to(5);
        assert_that!(date(2024, 6, 1).weeks_in_month(Weekday::Sunday)).is_equal_to(6);
        assert_that!(date(2024, 6, 1).weeks_in_month(Weekday::Monday)).is_equal_to(5);
        assert_that!(date(2024, 5, 31).with_month(2)).is_equal_to(date(2024, 2, 29));
    }

    #[test]
    fn first_days_of_week_come_from_the_locale() {
        let locale = |tag: &str| tag.parse::<Locale>().expect("a locale");
        assert_that!(first_day_of_week(&locale("en-US"))).is_equal_to(Weekday::Sunday);
        assert_that!(first_day_of_week(&locale("de-DE"))).is_equal_to(Weekday::Monday);
        assert_that!(first_day_of_week(&locale("en-US-u-fw-wed"))).is_equal_to(Weekday::Wednesday);
    }
}
