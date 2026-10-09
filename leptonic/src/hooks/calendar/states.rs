// Upstream: react-aria/src/calendar/utils.ts @ 99e6102368
// Upstream: @adobe/react-spectrum/test/calendar/RangeCalendar.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/calendar/Calendar.test.js @ 99e6102368
//! What the calendar hooks share: either calendar state, the calendar's data for its grids and
//! cells, and the descriptions of dates and ranges.

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Either state is a `CalendarStates` enum (react-aria: a `CalendarState | RangeCalendarState`
//   union told apart by its fields); the calendar's data for its parts is `CalendarData`
//   (react-aria: `hookData`, a `WeakMap` keyed by the state).
// - The formatters of the labels are kept per locale in `CalendarData` (react-aria memoizes them
//   per hook): cells share them.
//
// ## OMITTED FEATURES
// - Range formatting with shared fields ("June 1 – 15, 2024", `formatRange`): ICU4X has none
//   yet; a range is "start to end".
//
// =============================================================================

use jiff::civil::Date;
use leptos::prelude::*;

use super::{use_calendar_state::CalendarState, use_range_calendar_state::RangeCalendarState};
use crate::utils::{
    date::DateRange,
    date_time_formatter::{
        DateTimeFormat, DateTimeFormatOptions, DateTimeFormatter, MonthFormat, NumericFormat,
    },
    i18n::{Locale, use_locale},
    intl_strings::{CalendarStrings, DateRangeArgs, LocalizedStrings},
};

/// A calendar's state: a single date or a range (react-aria: `CalendarState | RangeCalendarState`).
#[derive(Clone, Copy)]
pub enum CalendarStates {
    Single(CalendarState),
    Range(RangeCalendarState),
}

impl From<CalendarState> for CalendarStates {
    fn from(state: CalendarState) -> Self {
        Self::Single(state)
    }
}

impl From<RangeCalendarState> for CalendarStates {
    fn from(state: RangeCalendarState) -> Self {
        Self::Range(state)
    }
}

impl CalendarStates {
    /// The calendar part (focus, visible range, navigation).
    pub fn calendar(&self) -> CalendarState {
        match self {
            Self::Single(state) => *state,
            Self::Range(state) => state.calendar,
        }
    }

    /// The range state, for a range calendar.
    pub fn range(&self) -> Option<RangeCalendarState> {
        match self {
            Self::Single(_) => None,
            Self::Range(state) => Some(*state),
        }
    }

    pub fn is_value_invalid(&self) -> Signal<bool> {
        match self {
            Self::Single(state) => state.is_value_invalid,
            Self::Range(state) => state.is_value_invalid,
        }
    }

    pub fn is_selected(&self, date: Date) -> bool {
        match self {
            Self::Single(state) => state.is_selected(date),
            Self::Range(state) => state.is_selected(date),
        }
    }

    pub fn is_invalid(&self, date: Date) -> bool {
        match self {
            Self::Single(state) => state.is_invalid(date),
            Self::Range(state) => state.is_invalid(date),
        }
    }

    /// Whether `date` can't be focused or selected (a range calendar also checks the available
    /// range around the anchor).
    pub fn is_cell_disabled(&self, date: Date) -> bool {
        let calendar = self.calendar();
        calendar.is_disabled.get()
            || !calendar.visible_range.get().contains(date)
            || self.is_invalid(date)
    }

    pub fn select_focused_date(&self) {
        match self {
            Self::Single(state) => state.select_focused_date(),
            Self::Range(state) => state.select_focused_date(),
        }
    }

    pub fn select_date(&self, date: Date) {
        match self {
            Self::Single(state) => state.select_date(date),
            Self::Range(state) => state.select_date(date),
        }
    }
}

/// What a calendar passes to its grids and cells (react-aria's `hookData`).
#[derive(Clone)]
pub struct CalendarData {
    pub state: CalendarStates,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
    /// The id of the calendar's error message while it is rendered (cells with invalid dates
    /// refer to it).
    pub error_message_id: Signal<Option<String>>,
    /// A description of the selection, e.g. "Selected Date: Monday, May 20, 2024".
    pub selected_date_description: Signal<String>,
    /// The formatters of the labels, shared by the cells.
    pub(crate) formatters: CalendarFormatters,
}

/// The era format of a date: the short era for dates before Christ (react-aria's
/// `getEraFormat`), else none.
pub(crate) fn era_format(date: Date) -> Option<DateTimeFormat> {
    (date.year() <= 0).then_some(DateTimeFormat::Short)
}

/// Options for full dates ("Monday, May 20, 2024").
fn full_date_options(era: Option<DateTimeFormat>) -> DateTimeFormatOptions {
    DateTimeFormatOptions {
        weekday: Some(DateTimeFormat::Long),
        month: Some(MonthFormat::Long),
        day: Some(NumericFormat::Numeric),
        year: Some(NumericFormat::Numeric),
        era,
        ..DateTimeFormatOptions::default()
    }
}

/// The formatters of a calendar's labels, kept per locale.
#[derive(Clone, Copy)]
pub(crate) struct CalendarFormatters {
    full_date: Memo<DateTimeFormatter>,
    full_date_with_era: Memo<DateTimeFormatter>,
    day: Memo<DateTimeFormatter>,
}

impl CalendarFormatters {
    pub(crate) fn new() -> Self {
        let locale = use_locale();
        let formatter = move |options: fn() -> DateTimeFormatOptions| {
            Memo::new(move |_| DateTimeFormatter::new(&locale.get(), options()))
        };
        Self {
            full_date: formatter(|| full_date_options(None)),
            full_date_with_era: formatter(|| full_date_options(Some(DateTimeFormat::Short))),
            day: formatter(|| DateTimeFormatOptions {
                day: Some(NumericFormat::Numeric),
                ..DateTimeFormatOptions::default()
            }),
        }
    }

    /// "Monday, May 20, 2024" (with the era before Christ).
    pub(crate) fn full_date(&self, date: Date) -> String {
        let formatter = if era_format(date).is_some() {
            self.full_date_with_era
        } else {
            self.full_date
        };
        formatter.with(|formatter| formatter.format_date(date))
    }

    /// The day number: "20".
    pub(crate) fn day(&self, date: Date) -> String {
        self.day.with(|formatter| formatter.format_date(date))
    }
}

/// The description of the selection (react-aria's `useSelectedDateDescription`).
pub(crate) fn selected_date_description(state: &CalendarStates, locale: &Locale) -> String {
    let (start, end) = match *state {
        CalendarStates::Single(state) => {
            let value = state.value.get();
            (value, value)
        }
        CalendarStates::Range(state) => {
            if state.anchor_date.get().is_some() {
                return String::new();
            }
            let range = state.highlighted_range.get();
            (range.map(|r| r.start), range.map(|r| r.end))
        }
    };
    let (Some(start), Some(end)) = (start, end) else {
        return String::new();
    };
    let formatter = DateTimeFormatter::new(
        locale,
        full_date_options(era_format(start).or_else(|| era_format(end))),
    );
    let strings = CalendarStrings::for_locale(locale.clone());
    if start == end {
        strings.selected_date_description(&formatter.format_date(start))
    } else {
        strings.selected_range_description(&strings.date_range(DateRangeArgs {
            start_date: &formatter.format_date(start),
            end_date: &formatter.format_date(end),
        }))
    }
}

/// The description of the visible range (react-aria's `useVisibleRangeDescription`): "May 2024"
/// for a month, "May 2024 to July 2024" for months, else the dates.
pub(crate) fn visible_range_description(range: DateRange, locale: &Locale) -> String {
    let era = era_format(range.start).or_else(|| era_format(range.end));
    let strings = CalendarStrings::for_locale(locale.clone());
    let months = DateTimeFormatter::new(
        locale,
        DateTimeFormatOptions {
            month: Some(MonthFormat::Long),
            year: Some(NumericFormat::Numeric),
            era,
            ..DateTimeFormatOptions::default()
        },
    );
    if range.start == range.start.first_of_month() {
        if range.end == range.start.last_of_month() {
            return months.format_date(range.start);
        }
        if range.end == range.end.last_of_month() {
            return strings.date_range(DateRangeArgs {
                start_date: &months.format_date(range.start),
                end_date: &months.format_date(range.end),
            });
        }
    }
    let dates = DateTimeFormatter::new(
        locale,
        DateTimeFormatOptions {
            month: Some(MonthFormat::Long),
            day: Some(NumericFormat::Numeric),
            year: Some(NumericFormat::Numeric),
            era,
            ..DateTimeFormatOptions::default()
        },
    );
    strings.date_range(DateRangeArgs {
        start_date: &dates.format_date(range.start),
        end_date: &dates.format_date(range.end),
    })
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;
    use jiff::civil::date;

    use super::*;
    use crate::testing::with_owner;

    fn locale(tag: &str) -> Locale {
        tag.parse().expect("a locale")
    }

    #[test]
    fn describes_visible_ranges() {
        let en = locale("en-US");
        let month = DateRange {
            start: date(2024, 5, 1),
            end: date(2024, 5, 31),
        };
        assert_that!(visible_range_description(month, &en)).is_equal_to("May 2024".to_owned());
        let months = DateRange {
            start: date(2024, 5, 1),
            end: date(2024, 7, 31),
        };
        assert_that!(visible_range_description(months, &en))
            .is_equal_to("May 2024 to July 2024".to_owned());
        let days = DateRange {
            start: date(2019, 6, 2),
            end: date(2019, 6, 15),
        };
        assert_that!(visible_range_description(days, &en))
            .is_equal_to("June 2, 2019 to June 15, 2019".to_owned());
        assert_that!(visible_range_description(month, &locale("de-DE")))
            .is_equal_to("Mai 2024".to_owned());
    }

    /// Dates before Christ get their era (react-aria's `getEraFormat`), as in
    /// `RangeCalendar.test.js` ("March 5 BC").
    #[test]
    fn names_the_era_before_christ() {
        let en = locale("en-US");
        // The proleptic year -1 is 2 BC.
        let month = DateRange {
            start: date(-1, 3, 1),
            end: date(-1, 3, 31),
        };
        assert_that!(visible_range_description(month, &en)).is_equal_to("March 2 BC".to_owned());
        with_owner(|| {
            let formatters = CalendarFormatters::new();
            assert_that!(formatters.full_date(date(-1, 3, 5)))
                .is_equal_to("Friday, March 5, 2 BC".to_owned());
            assert_that!(formatters.full_date(date(2024, 5, 20)))
                .is_equal_to("Monday, May 20, 2024".to_owned());
            assert_that!(formatters.day(date(2024, 5, 20))).is_equal_to("20".to_owned());
        });
    }
}
