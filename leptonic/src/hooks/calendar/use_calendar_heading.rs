// Upstream: react-aria/src/calendar/useCalendarHeading.ts @ 99e6102368
// Upstream: react-aria/src/calendar/useCalendarMonthPicker.ts @ 99e6102368
// Upstream: react-aria/src/calendar/useCalendarYearPicker.ts @ 99e6102368
// Upstream: react-aria-components/test/Calendar.test.js @ 99e6102368
// Upstream: react-aria-components/test/RangeCalendar.test.tsx @ 99e6102368
use jiff::civil::Date;
use leptos::prelude::*;

use super::states::{CalendarStates, era_format};
use crate::utils::{
    date::{DateDuration, DateExt},
    date_time_formatter::{
        DateTimeFormat, DateTimeFormatOptions, DateTimeFormatter, MonthFormat, NumericFormat,
    },
    i18n::use_locale,
    intl_strings::{CalendarStrings, DatePickerStrings, DateRangeArgs, use_localized_strings},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - One input with the calendar's state (C8): `UseCalendarHeadingInput`,
//   `UseCalendarMonthPickerInput`, `UseCalendarYearPickerInput`.
// - The pickers return their items, value and a setter as signals and a callback (react-aria:
//   props for a `Select`); the value of the year picker is the year (react-aria: the item index,
//   as eras may change between years of other calendars).
//
// ## DIFFERENT BEHAVIOR
// - The pickers' labels are the date picker's messages for the fields ("month", "Monat"),
//   react-aria's fallback when the browser has no `Intl.DisplayNames` (ICU4X has no display
//   names for date fields).
//
// ## OMITTED FEATURES
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

/// Input of [`use_calendar_heading`].
#[derive(Clone, Copy)]
pub struct UseCalendarHeadingInput {
    pub state: CalendarStates,
    /// Moves the heading to a later month of a calendar showing several.
    pub offset: DateDuration,
    pub format: CalendarHeadingFormat,
}

/// The heading of a calendar's month (or of the visible days): "May 2024".
pub fn use_calendar_heading(input: UseCalendarHeadingInput) -> Signal<String> {
    let UseCalendarHeadingInput {
        state,
        offset,
        format,
    } = input;
    let calendar = state.calendar();
    let locale = use_locale();
    let is_days = Memo::new(move |_| {
        let duration = calendar.visible_duration.get();
        duration.days != 0 || duration.weeks != 0
    });
    // Kept while the locale and options stay (not rebuilt per focused date).
    let options = Memo::new(move |_| {
        let range = calendar.visible_range.get();
        let start = range.start.add(offset);
        DateTimeFormatOptions {
            day: format
                .day
                .or_else(|| is_days.get().then_some(NumericFormat::Numeric)),
            month: Some(format.month),
            year: Some(format.year),
            era: era_format(start).or_else(|| era_format(range.end)),
            ..DateTimeFormatOptions::default()
        }
    });
    let formatter = Memo::new(move |_| DateTimeFormatter::new(&locale.get(), options.get()));
    let strings = use_localized_strings::<CalendarStrings>();
    Memo::new(move |_| {
        let range = calendar.visible_range.get();
        let start = range.start.add(offset);
        formatter.with(|formatter| {
            if is_days.get() {
                strings.read().date_range(DateRangeArgs {
                    start_date: &formatter.format_date(start),
                    end_date: &formatter.format_date(range.end),
                })
            } else {
                formatter.format_date(start)
            }
        })
    })
    .into()
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
    /// Names the picker: "month" or "year", in the locale's language.
    pub aria_label: Signal<String>,
    /// The focused date's month or year.
    pub value: Signal<i16>,
    /// The items to pick from.
    pub items: Signal<Vec<CalendarPickerItem>>,
    /// Moves the focused date to the picked month or year (an item's id).
    pub on_change: Callback<i16>,
}

/// Input of [`use_calendar_month_picker`].
#[derive(Clone, Copy)]
pub struct UseCalendarMonthPickerInput {
    pub state: CalendarStates,
    /// How the months are formatted (react-aria's default: short, "Jan").
    pub format: MonthFormat,
}

/// A picker of the focused date's month: the year's months.
pub fn use_calendar_month_picker(input: UseCalendarMonthPickerInput) -> UseCalendarPickerReturn {
    let UseCalendarMonthPickerInput { state, format } = input;
    let calendar = state.calendar();
    let locale = use_locale();
    let formatter = Memo::new(move |_| {
        DateTimeFormatter::new(
            &locale.get(),
            DateTimeFormatOptions {
                month: Some(format),
                ..DateTimeFormatOptions::default()
            },
        )
    });
    let items = Memo::new(move |_| {
        let focused = calendar.focused_date.get();
        formatter.with(|formatter| {
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
        })
    });
    let strings = use_localized_strings::<DatePickerStrings>();
    UseCalendarPickerReturn {
        aria_label: Signal::derive(move || strings.read().month()),
        value: Signal::derive(move || i16::from(calendar.focused_date.get().month())),
        items: items.into(),
        on_change: Callback::new(move |month: i16| {
            if let Some(item) =
                items.with_untracked(|items| items.iter().find(|item| item.id == month).cloned())
            {
                calendar.set_focused_date(item.date);
            }
        }),
    }
}

/// How a year picker formats its years (react-aria's `CalendarYearPickerFormatOptions`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalendarYearPickerFormat {
    pub year: NumericFormat,
    /// The era. `None`: the short era for years before Christ.
    pub era: Option<DateTimeFormat>,
}

impl Default for CalendarYearPickerFormat {
    fn default() -> Self {
        Self {
            year: NumericFormat::Numeric,
            era: None,
        }
    }
}

/// Input of [`use_calendar_year_picker`].
#[derive(Clone, Copy)]
pub struct UseCalendarYearPickerInput {
    pub state: CalendarStates,
    /// How many years to offer around the focused date's (react-aria's default: 20).
    pub visible_years: u8,
    pub format: CalendarYearPickerFormat,
}

/// A picker of the focused date's year: `visible_years` years around it, kept within min and
/// max.
pub fn use_calendar_year_picker(input: UseCalendarYearPickerInput) -> UseCalendarPickerReturn {
    let UseCalendarYearPickerInput {
        state,
        visible_years,
        format,
    } = input;
    let calendar = state.calendar();
    let locale = use_locale();
    let visible_years = i32::from(visible_years.max(1));
    // Kept while the locale and era stay (not rebuilt per focused date).
    let era = Memo::new(move |_| {
        format
            .era
            .or_else(|| era_format(calendar.focused_date.get()))
    });
    let formatter = Memo::new(move |_| {
        DateTimeFormatter::new(
            &locale.get(),
            DateTimeFormatOptions {
                year: Some(format.year),
                era: era.get(),
                ..DateTimeFormatOptions::default()
            },
        )
    });
    let items = Memo::new(move |_| {
        let focused = calendar.focused_date.get();
        let formatter = formatter.get();
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
    let strings = use_localized_strings::<DatePickerStrings>();
    UseCalendarPickerReturn {
        aria_label: Signal::derive(move || strings.read().year()),
        value: Signal::derive(move || calendar.focused_date.get().year()),
        items: items.into(),
        on_change: Callback::new(move |year: i16| {
            if let Some(item) =
                items.with_untracked(|items| items.iter().find(|item| item.id == year).cloned())
            {
                calendar.set_focused_date(item.date);
            }
        }),
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;
    use jiff::civil::date;

    use super::*;
    use crate::{
        hooks::calendar::{UseCalendarStateInput, use_calendar_state},
        testing::with_owner,
    };

    fn calendar(focused: Date) -> CalendarStates {
        use_calendar_state(UseCalendarStateInput {
            default_focused_value: Some(focused),
            ..UseCalendarStateInput::default()
        })
        .into()
    }

    /// RAC `Calendar.test.js`, "should support month and year dropdowns".
    #[test]
    fn picks_months_and_years() {
        with_owner(|| {
            let state = calendar(date(2026, 4, 1));
            let months = use_calendar_month_picker(UseCalendarMonthPickerInput {
                state,
                format: MonthFormat::Short,
            });
            assert_that!(months.aria_label.get_untracked()).is_equal_to("month".to_owned());
            assert_that!(months.value.get_untracked()).is_equal_to(4);
            let names: Vec<String> = months
                .items
                .get_untracked()
                .into_iter()
                .map(|item| item.formatted)
                .collect();
            assert_that!(names).is_equal_to(
                [
                    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov",
                    "Dec",
                ]
                .map(str::to_owned)
                .to_vec(),
            );
            months.on_change.run(6);
            assert_that!(state.calendar().focused_date.get_untracked())
                .is_equal_to(date(2026, 6, 1));

            let years = use_calendar_year_picker(UseCalendarYearPickerInput {
                state,
                visible_years: 20,
                format: CalendarYearPickerFormat::default(),
            });
            let names = |years: &UseCalendarPickerReturn| -> Vec<String> {
                years
                    .items
                    .get_untracked()
                    .into_iter()
                    .map(|item| item.formatted)
                    .collect()
            };
            assert_that!(names(&years)).is_equal_to(
                (2016..2036)
                    .map(|year| year.to_string())
                    .collect::<Vec<_>>(),
            );
            years.on_change.run(2030);
            assert_that!(state.calendar().focused_date.get_untracked())
                .is_equal_to(date(2030, 6, 1));
            assert_that!(names(&years)).is_equal_to(
                (2020..2040)
                    .map(|year| year.to_string())
                    .collect::<Vec<_>>(),
            );
        });
    }

    /// The years of a year picker of a calendar with limits.
    fn limited_years(
        min_value: Option<Date>,
        max_value: Option<Date>,
        focused: Date,
        visible_years: u8,
    ) -> Vec<CalendarPickerItem> {
        let state: CalendarStates = use_calendar_state(UseCalendarStateInput {
            default_focused_value: Some(focused),
            min_value: Signal::stored(min_value),
            max_value: Signal::stored(max_value),
            ..UseCalendarStateInput::default()
        })
        .into();
        use_calendar_year_picker(UseCalendarYearPickerInput {
            state,
            visible_years,
            format: CalendarYearPickerFormat::default(),
        })
        .items
        .get_untracked()
    }

    fn year_names(items: &[CalendarPickerItem]) -> Vec<String> {
        items.iter().map(|item| item.formatted.clone()).collect()
    }

    /// RAC `Calendar.test.js`, "supports minValue and maxValue": the years end at max, start at
    /// min, stay between both, and a year cut by a limit moves to it.
    #[test]
    fn year_pickers_stay_within_min_and_max() {
        with_owner(|| {
            let years = |range: std::ops::RangeInclusive<i32>| -> Vec<String> {
                range.map(|year| year.to_string()).collect()
            };
            // Upstream focuses today, here a fixed date of its time (results independent of the
            // day the test runs).
            let today = date(2026, 10, 9);
            let items = limited_years(None, Some(date(2026, 6, 30)), today, 20);
            assert_that!(year_names(&items)).is_equal_to(years(2007..=2026));
            let items = limited_years(Some(date(2020, 6, 30)), None, today, 20);
            assert_that!(year_names(&items)).is_equal_to(years(2020..=2039));
            let items = limited_years(
                Some(date(2020, 6, 30)),
                Some(date(2026, 6, 30)),
                date(2022, 6, 30),
                20,
            );
            assert_that!(year_names(&items)).is_equal_to(years(2020..=2026));
            let items = limited_years(
                Some(date(2024, 8, 3)),
                Some(date(2025, 2, 3)),
                date(2025, 2, 1),
                20,
            );
            assert_that!(year_names(&items)).is_equal_to(years(2024..=2025));
            assert_that!(items[1].date).is_equal_to(date(2025, 2, 3));
            let items = limited_years(None, None, date(2026, 6, 30), 1);
            assert_that!(year_names(&items)).is_equal_to(years(2026..=2026));
        });
    }

    #[test]
    fn year_pickers_name_the_era_before_christ() {
        with_owner(|| {
            let state = calendar(date(-1, 4, 1));
            let years = use_calendar_year_picker(UseCalendarYearPickerInput {
                state,
                visible_years: 2,
                format: CalendarYearPickerFormat::default(),
            });
            let names: Vec<String> = years
                .items
                .get_untracked()
                .into_iter()
                .map(|item| item.formatted)
                .collect();
            assert_that!(names).is_equal_to(vec!["3 BC".to_owned(), "2 BC".to_owned()]);
        });
    }
}
