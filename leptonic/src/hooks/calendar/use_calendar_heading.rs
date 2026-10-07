// Upstream: react-aria/src/calendar/useCalendarHeading.ts @ 99e6102368
// Upstream: react-aria/src/calendar/useCalendarMonthPicker.ts @ 99e6102368
// Upstream: react-aria/src/calendar/useCalendarYearPicker.ts @ 99e6102368
use jiff::civil::Date;
use leptos::prelude::*;

use super::states::{CalendarStates, strings};
use crate::utils::{
    date::{DateDuration, DateExt},
    date_time_formatter::{DateTimeFormatOptions, DateTimeFormatter, MonthFormat, NumericFormat},
    i18n::use_locale,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The pickers return their items, value and a setter as signals and a callback (react-aria:
//   props for a `Select`); the value of the year picker is the year (react-aria: the item index,
//   as eras may change between years of other calendars).
//
// ## OMITTED FEATURES
// - Localized names: the pickers' labels are "Month" and "Year" (react-aria: the locale's names
//   of the date fields).
// - The heading of several visible days is "May 20, 2024 to May 26, 2024" (react-aria: the
//   locale's date range format).
//
// =============================================================================

/// How a calendar heading formats its date (react-aria's `CalendarHeadingFormatOptions`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalendarHeadingFormat {
    pub day: Option<NumericFormat>,
    pub month: MonthFormat,
    pub year: NumericFormat,
}

impl Default for CalendarHeadingFormat {
    fn default() -> Self {
        Self {
            day: None,
            month: MonthFormat::Long,
            year: NumericFormat::Numeric,
        }
    }
}

/// The heading of a calendar's month (or of the visible days): "May 2024". `offset` moves it to
/// a later month of a calendar showing several.
pub fn use_calendar_heading(
    state: &CalendarStates,
    offset: DateDuration,
    format: CalendarHeadingFormat,
) -> Signal<String> {
    let calendar = state.calendar();
    let locale = use_locale();
    let is_days = calendar.visible_duration.days != 0 || calendar.visible_duration.weeks != 0;
    Signal::derive(move || {
        let range = calendar.visible_range.get();
        let start = range.start.add(offset);
        let formatter = DateTimeFormatter::new(
            &locale.get(),
            DateTimeFormatOptions {
                day: format
                    .day
                    .or_else(|| is_days.then_some(NumericFormat::Numeric)),
                month: Some(format.month),
                year: Some(format.year),
                ..DateTimeFormatOptions::default()
            },
        );
        if is_days {
            strings::date_range(
                &formatter.format_date(start),
                &formatter.format_date(range.end),
            )
        } else {
            formatter.format_date(start)
        }
    })
}

/// An item of a month or year picker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalendarPickerItem {
    /// The month (1 to 12) or the year.
    pub id: i16,
    /// The focused date moved to this month or year.
    pub date: Date,
    /// The formatted month or year.
    pub formatted: String,
}

/// Return value of [`use_calendar_month_picker`] and [`use_calendar_year_picker`].
#[derive(Debug, Clone, Copy)]
pub struct UseCalendarPickerReturn {
    /// Names the picker: "Month" or "Year".
    pub aria_label: &'static str,
    /// The focused date's month or year.
    pub value: Signal<i16>,
    /// The items to pick from.
    pub items: Signal<Vec<CalendarPickerItem>>,
    /// Moves the focused date to the picked month or year (an item's id).
    pub on_change: Callback<i16>,
}

/// A picker of the focused date's month: the year's months, formatted with `format`.
pub fn use_calendar_month_picker(
    state: &CalendarStates,
    format: MonthFormat,
) -> UseCalendarPickerReturn {
    let calendar = state.calendar();
    let locale = use_locale();
    let items = Signal::derive(move || {
        let focused = calendar.focused_date.get();
        let formatter = DateTimeFormatter::new(
            &locale.get(),
            DateTimeFormatOptions {
                month: Some(format),
                ..DateTimeFormatOptions::default()
            },
        );
        (1..=12)
            .map(|month| {
                let date = focused.with_month(month);
                CalendarPickerItem {
                    id: i16::from(month),
                    date,
                    formatted: formatter.format_date(date),
                }
            })
            .collect::<Vec<_>>()
    });
    UseCalendarPickerReturn {
        aria_label: "Month",
        value: Signal::derive(move || i16::from(calendar.focused_date.get().month())),
        items,
        on_change: Callback::new(move |month: i16| {
            if let Some(item) = items
                .get_untracked()
                .into_iter()
                .find(|item| item.id == month)
            {
                calendar.set_focused_date(item.date);
            }
        }),
    }
}

/// A picker of the focused date's year: `visible_years` years around it (default 20), kept
/// within min and max.
pub fn use_calendar_year_picker(
    state: &CalendarStates,
    visible_years: Option<u8>,
    format: NumericFormat,
) -> UseCalendarPickerReturn {
    let calendar = state.calendar();
    let locale = use_locale();
    let visible_years = i32::from(visible_years.unwrap_or(20).max(1));
    let items = Signal::derive(move || {
        let focused = calendar.focused_date.get();
        let mut min = focused.subtract(DateDuration::years(visible_years / 2));
        let mut max = focused.add(DateDuration::years((visible_years + 1) / 2 - 1));
        if let Some(max_value) = calendar.max_value.get()
            && max > max_value
        {
            max = max_value;
            min = max.subtract(DateDuration::years(visible_years - 1));
        }
        if let Some(min_value) = calendar.min_value.get()
            && min < min_value
        {
            min = min_value;
            max = min.add(DateDuration::years(visible_years - 1));
            if let Some(max_value) = calendar.max_value.get()
                && max > max_value
            {
                max = max_value;
            }
        }
        let formatter = DateTimeFormatter::new(
            &locale.get(),
            DateTimeFormatOptions {
                year: Some(format),
                ..DateTimeFormatOptions::default()
            },
        );
        let mut items = Vec::new();
        let mut date = min;
        while date <= max || date.is_same_year(max) {
            let item_date = if date > max { max } else { date };
            items.push(CalendarPickerItem {
                id: item_date.year(),
                date: item_date,
                formatted: formatter.format_date(item_date),
            });
            let next = date.add(DateDuration::years(1));
            if next == date {
                break;
            }
            date = next;
        }
        items
    });
    UseCalendarPickerReturn {
        aria_label: "Year",
        value: Signal::derive(move || calendar.focused_date.get().year()),
        items,
        on_change: Callback::new(move |year: i16| {
            if let Some(item) = items
                .get_untracked()
                .into_iter()
                .find(|item| item.id == year)
            {
                calendar.set_focused_date(item.date);
            }
        }),
    }
}
