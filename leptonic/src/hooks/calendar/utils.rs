// Upstream: react-stately/src/calendar/utils.ts @ 99e6102368
// Upstream: react-aria/test/calendar/useCalendar.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/calendar/CalendarBase.test.js @ 99e6102368
//! Aligning and constraining the visible range of a calendar.

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Dates are `jiff::civil::Date`, durations `DateDuration` (react-aria: `CalendarDate`,
//   `DateDuration`).
// - The `align_*` functions take the first day of the week (react-aria: the locale, from which
//   `startOfWeek` finds it): calendars may override it (`first_day_of_week`).
//
// =============================================================================

use jiff::civil::{Date, Weekday};

use crate::utils::date::{DateDuration, DateExt, max_date, min_date};

/// Whether `date` lies outside `min`..=`max`.
pub(crate) fn is_invalid(date: Date, min: Option<Date>, max: Option<Date>) -> bool {
    min.is_some_and(|min| date < min) || max.is_some_and(|max| date > max)
}

/// The first visible date when `date` is centered in a range of `duration`.
pub(crate) fn align_center(
    date: Date,
    duration: DateDuration,
    first_day: Weekday,
    min: Option<Date>,
    max: Option<Date>,
) -> Date {
    let half = |units: i32| {
        let half = units / 2;
        if half > 0 && units % 2 == 0 {
            half - 1
        } else {
            half
        }
    };
    let half_duration = DateDuration {
        years: half(duration.years),
        months: half(duration.months),
        weeks: half(duration.weeks),
        days: half(duration.days),
    };
    let aligned = align_start(date, duration, first_day, None, None).subtract(half_duration);
    constrain_start(date, aligned, duration, first_day, min, max)
}

/// The first visible date of a range of `duration` starting at `date`'s largest unit.
pub(crate) fn align_start(
    date: Date,
    duration: DateDuration,
    first_day: Weekday,
    min: Option<Date>,
    max: Option<Date>,
) -> Date {
    let aligned = if duration.years != 0 {
        date.first_of_year()
    } else if duration.months != 0 {
        date.first_of_month()
    } else if duration.weeks != 0 || duration.days > 7 {
        date.start_of_week(first_day)
    } else {
        date
    };
    constrain_start(date, aligned, duration, first_day, min, max)
}

/// The first visible date of a range of `duration` ending with `date`.
pub(crate) fn align_end(
    date: Date,
    duration: DateDuration,
    first_day: Weekday,
    min: Option<Date>,
    max: Option<Date>,
) -> Date {
    // One less of the smallest unit.
    let mut shortened = duration;
    if shortened.days != 0 {
        shortened.days -= 1;
    } else if shortened.weeks != 0 {
        shortened.weeks -= 1;
    } else if shortened.months != 0 {
        shortened.months -= 1;
    } else if shortened.years != 0 {
        shortened.years -= 1;
    }
    let aligned = align_start(date, duration, first_day, None, None).subtract(shortened);
    constrain_start(date, aligned, duration, first_day, min, max)
}

/// `aligned`, moved so that the visible range stays within `min`..=`max`.
pub(crate) fn constrain_start(
    date: Date,
    aligned: Date,
    duration: DateDuration,
    first_day: Weekday,
    min: Option<Date>,
    max: Option<Date>,
) -> Date {
    let mut aligned = aligned;
    if let Some(min) = min
        && date >= min
    {
        aligned = max_date(aligned, align_start(min, duration, first_day, None, None));
    }
    if let Some(max) = max
        && date <= max
    {
        aligned = min_date(aligned, align_end(max, duration, first_day, None, None));
    }
    aligned
}

/// `date` within `min`..=`max`.
pub(crate) fn constrain_value(date: Date, min: Option<Date>, max: Option<Date>) -> Date {
    let date = min.map_or(date, |min| max_date(date, min));
    max.map_or(date, |max| min_date(date, max))
}

/// The latest available date at or before `date`, not before `min`.
pub(crate) fn previous_available_date(
    date: Date,
    min: Date,
    is_unavailable: Option<&dyn Fn(Date) -> bool>,
) -> Option<Date> {
    let Some(is_unavailable) = is_unavailable else {
        return Some(date);
    };
    let mut date = date;
    while date >= min && is_unavailable(date) {
        let previous = date.subtract(DateDuration::days(1));
        if previous == date {
            break;
        }
        date = previous;
    }
    (date >= min && !is_unavailable(date)).then_some(date)
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;
    use jiff::civil::date;

    use super::*;

    const MONTH: DateDuration = DateDuration::months(1);

    #[test]
    fn aligns_months() {
        let may15 = date(2024, 5, 15);
        assert_that!(align_start(may15, MONTH, Weekday::Sunday, None, None))
            .is_equal_to(date(2024, 5, 1));
        assert_that!(align_center(may15, MONTH, Weekday::Sunday, None, None))
            .is_equal_to(date(2024, 5, 1));
        // Three months, centered: the month before, this month, the month after.
        assert_that!(align_center(
            may15,
            DateDuration::months(3),
            Weekday::Sunday,
            None,
            None
        ))
        .is_equal_to(date(2024, 4, 1));
        assert_that!(align_end(
            may15,
            DateDuration::months(3),
            Weekday::Sunday,
            None,
            None
        ))
        .is_equal_to(date(2024, 3, 1));
        // A week view starts on the first day of the week.
        assert_that!(align_start(
            may15,
            DateDuration::weeks(1),
            Weekday::Monday,
            None,
            None
        ))
        .is_equal_to(date(2024, 5, 13));
    }

    #[test]
    fn constrains_to_min_and_max() {
        let min = Some(date(2024, 5, 10));
        let max = Some(date(2024, 6, 20));
        assert_that!(constrain_value(date(2024, 5, 1), min, max)).is_equal_to(date(2024, 5, 10));
        assert_that!(constrain_value(date(2024, 7, 1), min, max)).is_equal_to(date(2024, 6, 20));
        // Three months centered on May would start in April, before min's month.
        assert_that!(align_center(
            date(2024, 5, 15),
            DateDuration::months(3),
            Weekday::Sunday,
            min,
            None
        ))
        .is_equal_to(date(2024, 5, 1));
    }

    #[test]
    fn finds_the_previous_available_date() {
        let weekend =
            |date: Date| date.weekday() == Weekday::Saturday || date.weekday() == Weekday::Sunday;
        // 2024-05-19 is a Sunday.
        assert_that!(previous_available_date(
            date(2024, 5, 19),
            date(2024, 5, 1),
            Some(&weekend)
        ))
        .is_equal_to(Some(date(2024, 5, 17)));
        assert_that!(previous_available_date(
            date(2024, 5, 19),
            date(2024, 5, 18),
            Some(&weekend)
        ))
        .is_none();
    }
}
