// Upstream: react-aria/src/calendar/useCalendarHeading.ts @ 99e6102368
// Upstream: react-aria/src/calendar/useCalendarMonthPicker.ts @ 99e6102368
// Upstream: react-aria/src/calendar/useCalendarYearPicker.ts @ 99e6102368
use jiff::civil::Date;
use leptos::prelude::*;

use super::states::{CalendarStates, era_format};
use crate::utils::{
    date::{DateDuration, DateExt},
    date_time_formatter::{
        DateTimeFormat, DateTimeFormatOptions, DateTimeFormatter, MonthFormat, NumericFormat,
    },
    i18n::use_locale,
    intl_strings::{CalendarStrings, DateRangeArgs, LocalizedStrings},
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
// ## OMITTED FEATURES
// - Localized names: the pickers' labels are "month" and "year" (react-aria: the locale's names
//   of the date fields, `Intl.DisplayNames`; ICU4X has none yet).
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
    Memo::new(move |_| {
        let duration = calendar.visible_duration.get();
        let is_days = duration.days != 0 || duration.weeks != 0;
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
                era: era_format(start).or_else(|| era_format(range.end)),
                ..DateTimeFormatOptions::default()
            },
        );
        if is_days {
            CalendarStrings::for_locale(locale.get()).date_range(DateRangeArgs {
                start_date: &formatter.format_date(start),
                end_date: &formatter.format_date(range.end),
            })
        } else {
            formatter.format_date(start)
        }
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
    /// Names the picker: "month" or "year".
    pub aria_label: &'static str,
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
    UseCalendarPickerReturn {
        aria_label: "month",
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
    let items = Memo::new(move |_| {
        let focused = calendar.focused_date.get();
        let formatter = DateTimeFormatter::new(
            &locale.get(),
            DateTimeFormatOptions {
                year: Some(format.year),
                era: format.era.or_else(|| era_format(focused)),
                ..DateTimeFormatOptions::default()
            },
        );
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
    UseCalendarPickerReturn {
        aria_label: "year",
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
    use crate::hooks::calendar::{UseCalendarStateInput, use_calendar_state};

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
        Owner::new().with(|| {
            let state = calendar(date(2026, 4, 1));
            let months = use_calendar_month_picker(UseCalendarMonthPickerInput {
                state,
                format: MonthFormat::Short,
            });
            assert_that!(months.aria_label).is_equal_to("month");
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

    #[test]
    fn year_pickers_name_the_era_before_christ() {
        Owner::new().with(|| {
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
