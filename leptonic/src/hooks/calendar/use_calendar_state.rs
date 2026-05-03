use leptos::prelude::*;
use time::macros::format_description;
use uuid::Uuid;

use crate::utils::{
    live_announcer::try_use_live_announcer,
    time::{
        Day, InMonth, Month, SaveReplaceYear, Week, Year, is_in_range, start_of_next_month,
        start_of_previous_month, whole_days_in,
    },
};

//
// 1. Focus/selection separation: Matches react-aria. `focused_date` is the
//    keyboard cursor; `value` is the confirmed selection. Arrow keys move
//    focus without selecting.
//
// 2. Year/month picker grids: Leptonic-specific addition for navigating
//    years/months via grid UI. Not present in react-aria.
//
// 3. No calendar system abstraction: We use `time::OffsetDateTime` directly
//    rather than react-aria's `@internationalized/date` calendar system.
//
// 4. No multi-month visible duration, selection alignment, locale-aware
//    formatting, autoFocus, or pageBehavior.
//
// 5. English-only month names: `format_description!("[month repr:long]")`
//    produces English month names only. i18n would need a custom formatter.
//
// 6. Simplified validation model: `previous_available_date` searches backward
//    up to `min` or 365 days. React-aria uses a more sophisticated search
//    with calendar-system-aware iteration.
//

/// Input parameters for the `use_calendar_state` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseCalendarStateInput {
    /// The initial selected value. `None` means no date is selected.
    pub default_value: Option<time::OffsetDateTime>,

    /// The minimum selectable date.
    pub min: Option<time::OffsetDateTime>,

    /// The maximum selectable date.
    pub max: Option<time::OffsetDateTime>,

    /// Whether the calendar is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the calendar is read-only (can focus but not select).
    pub is_read_only: Signal<bool>,

    /// Callback to check if a specific date is unavailable.
    pub is_date_unavailable: Option<Callback<time::OffsetDateTime, bool>>,

    /// Called when the selected value changes.
    pub on_change: Option<Callback<Option<time::OffsetDateTime>>>,

    /// Called when the focused date changes.
    pub on_focus_change: Option<Callback<time::OffsetDateTime>>,

    /// The initial focused date. Defaults to `default_value` or now.
    pub default_focused_value: Option<time::OffsetDateTime>,

    /// External validity signal (e.g. from form validation).
    pub is_invalid: Option<Signal<bool>>,

    /// The first day of the week. Defaults to Monday.
    pub first_day_of_week: time::Weekday,
}

impl Default for UseCalendarStateInput {
    fn default() -> Self {
        Self {
            default_value: None,
            min: None,
            max: None,
            is_disabled: Signal::derive(|| false),
            is_read_only: Signal::derive(|| false),
            is_date_unavailable: None,
            on_change: None,
            on_focus_change: None,
            default_focused_value: None,
            is_invalid: None,
            first_day_of_week: time::Weekday::Monday,
        }
    }
}

/// The return value of the `use_calendar_state` hook.
#[derive(Clone, Copy)]
pub struct UseCalendarStateReturn {
    // Core state
    /// The currently selected date, if any.
    pub value: Signal<Option<time::OffsetDateTime>>,
    /// The currently focused date (keyboard cursor). Always set.
    pub focused_date: Signal<time::OffsetDateTime>,
    /// Whether the calendar grid currently has focus.
    pub is_focused: Signal<bool>,
    /// Whether the calendar is disabled.
    pub is_disabled: Signal<bool>,
    /// Whether the calendar is read-only.
    pub is_read_only: Signal<bool>,
    /// Whether the current value is invalid (out of range, unavailable, or externally invalid).
    pub is_value_invalid: Signal<bool>,

    // Derived display
    /// The year of the focused date.
    pub focused_year: Memo<i32>,
    /// The month name of the focused date.
    pub focused_month_name: Memo<String>,

    // Pre-computed grids
    /// The weeks grid for the focused month.
    pub weeks: Signal<Vec<Week>>,
    /// The months grid for year selection.
    pub months: Signal<Vec<Month>>,
    /// The years grid.
    pub years: Signal<Vec<Year>>,
    /// A display string for the years range (e.g. "2020 - 2031").
    pub years_range: Signal<String>,

    // Boundary checks
    /// Whether navigating to the previous visible range (month) is invalid (past min).
    pub is_previous_visible_range_invalid: Signal<bool>,
    /// Whether navigating to the next visible range (month) is invalid (past max).
    pub is_next_visible_range_invalid: Signal<bool>,

    // Selection
    /// Set the selected value directly.
    pub set_value: Callback<Option<time::OffsetDateTime>>,
    /// Select the currently focused date.
    pub select_focused_date: Callback<()>,
    /// Select a specific date.
    pub select_date: Callback<time::OffsetDateTime>,

    // Focus navigation
    /// Set the focused date directly.
    pub set_focused_date: Callback<time::OffsetDateTime>,
    /// Set whether the calendar has focus.
    pub set_focused: Callback<bool>,
    /// Move focus to the next day (+1 day).
    pub focus_next_day: Callback<()>,
    /// Move focus to the previous day (-1 day).
    pub focus_previous_day: Callback<()>,
    /// Move focus to the next row (+7 days).
    pub focus_next_row: Callback<()>,
    /// Move focus to the previous row (-7 days).
    pub focus_previous_row: Callback<()>,
    /// Move focus to the next page (+1 month).
    pub focus_next_page: Callback<()>,
    /// Move focus to the previous page (-1 month).
    pub focus_previous_page: Callback<()>,
    /// Move focus to the start of the current section (first day of month).
    pub focus_section_start: Callback<()>,
    /// Move focus to the end of the current section (last day of month).
    pub focus_section_end: Callback<()>,
    /// Move focus to the next section. If `larger` (bool arg) is true, +1 year; else +1 month.
    pub focus_next_section: Callback<bool>,
    /// Move focus to the previous section. If `larger` (bool arg) is true, -1 year; else -1 month.
    pub focus_previous_section: Callback<bool>,

    // Year/month picker (leptonic-specific)
    /// Navigate the years grid backward by one page.
    pub navigate_years_backward: Callback<()>,
    /// Navigate the years grid forward by one page.
    pub navigate_years_forward: Callback<()>,
    /// Focus a specific year (updates focused date to that year).
    pub focus_year: Callback<i32>,
    /// Focus a specific month (updates focused date to that month, 1-based index).
    pub focus_month: Callback<u8>,

    // Query methods
    /// Check if a date is the currently selected date.
    pub is_selected: Callback<time::OffsetDateTime, bool>,
    /// Check if a date is the currently focused date.
    pub is_cell_focused: Callback<time::OffsetDateTime, bool>,
    /// Check if a date is disabled (out of min/max range or calendar disabled).
    pub is_cell_disabled: Callback<time::OffsetDateTime, bool>,
    /// Check if a date is unavailable (from `is_date_unavailable` callback).
    pub is_cell_unavailable: Callback<time::OffsetDateTime, bool>,
}

/// Clamp a date to the [min, max] range.
fn constrain_value(
    date: time::OffsetDateTime,
    min: Option<time::OffsetDateTime>,
    max: Option<time::OffsetDateTime>,
) -> time::OffsetDateTime {
    let date = if let Some(min) = min {
        if date < min { min } else { date }
    } else {
        date
    };
    if let Some(max) = max {
        if date > max { max } else { date }
    } else {
        date
    }
}

/// Creates internal state for a calendar, separating focus (keyboard cursor)
/// from selection (confirmed value).
#[allow(
    clippy::too_many_lines,
    clippy::needless_pass_by_value,
    clippy::missing_panics_doc
)]
pub fn use_calendar_state(input: UseCalendarStateInput) -> UseCalendarStateReturn {
    let UseCalendarStateInput {
        default_value,
        min,
        max,
        is_disabled,
        is_read_only,
        is_date_unavailable,
        on_change,
        on_focus_change,
        default_focused_value,
        is_invalid,
        first_day_of_week,
    } = input;

    // --- Core state ---

    let (value, set_value_signal) = signal(default_value);

    let initial_focused = default_focused_value
        .or(default_value)
        .unwrap_or_else(now_local_or_utc);
    let initial_focused = constrain_value(initial_focused, min, max);
    let (focused_date, set_focused_date_signal) = signal(initial_focused);

    let (is_focused_signal, set_is_focused_signal) = signal(false);

    // Years grid start offset
    let years_start = RwSignal::new(initial_focused.year() - 4);

    // --- Live announcement on month/year navigation ---
    let prev_month = StoredValue::new(initial_focused.month());
    let prev_year = StoredValue::new(initial_focused.year());
    Effect::new(move |_| {
        let fd = focused_date.get();
        let new_month = fd.month();
        let new_year = fd.year();
        if new_month != prev_month.get_value() || new_year != prev_year.get_value() {
            prev_month.set_value(new_month);
            prev_year.set_value(new_year);
            if let Some(announcer) = try_use_live_announcer() {
                announcer.announce_polite(format!("{new_month} {new_year}"));
            }
        }
    });

    // --- Helpers ---

    let update_focused = move |new_date: time::OffsetDateTime| {
        let constrained = constrain_value(new_date, min, max);
        set_focused_date_signal.set(constrained);
        // Keep years grid in sync
        years_start.set(constrained.year() - 4);
        if let Some(cb) = on_focus_change {
            cb.run(constrained);
        }
    };

    let try_select = move |date: time::OffsetDateTime| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }
        if !is_in_range(&date, min.as_ref(), max.as_ref()) {
            return;
        }
        // If the requested date is unavailable, find the nearest previous available date.
        let date = if is_date_unavailable.is_some_and(|cb| cb.run(date)) {
            let Some(available) = previous_available_date(date, min, is_date_unavailable) else {
                return;
            };
            available
        } else {
            date
        };
        set_value_signal.set(Some(date));
        if let Some(cb) = on_change {
            cb.run(Some(date));
        }
    };

    // --- Derived display ---

    let focused_year = Memo::new(move |_| focused_date.get().year());
    let focused_month_name = Memo::new(move |_| focused_date.get().month().to_string());

    // --- Pre-computed grids ---

    let years = Signal::derive(move || {
        let focused = focused_date.get();
        let selected = value.get();
        create_years(
            focused,
            selected.as_ref(),
            years_start.get(),
            min.as_ref(),
            max.as_ref(),
        )
    });
    let years_range = Signal::derive(move || {
        years.with(|years| {
            if years.is_empty() {
                "ERR: no years".to_owned()
            } else {
                format!("{} - {}", years[0].number, years[years.len() - 1].number)
            }
        })
    });
    let months = Signal::derive(move || {
        let focused = focused_date.get();
        let selected = value.get();
        create_months(focused, selected.as_ref(), min.as_ref(), max.as_ref())
    });
    let weeks = Signal::derive(move || {
        let focused = focused_date.get();
        let selected = value.get();
        create_weeks(
            &focused,
            selected.as_ref(),
            min.as_ref(),
            max.as_ref(),
            is_date_unavailable,
            first_day_of_week,
        )
    });

    // --- Boundary checks ---

    let is_previous_visible_range_invalid = Signal::derive(move || {
        if let Some(min) = min {
            let focused = focused_date.get();
            let first_of_month = focused.replace_day(1).unwrap();
            first_of_month <= min
        } else {
            false
        }
    });

    let is_next_visible_range_invalid = Signal::derive(move || {
        if let Some(max) = max {
            let focused = focused_date.get();
            let days_in_month = whole_days_in(focused.year(), focused.month());
            let last_of_month = focused.replace_day(days_in_month).unwrap();
            last_of_month >= max
        } else {
            false
        }
    });

    // --- Validation ---

    let is_value_invalid = Signal::derive(move || {
        if let Some(is_invalid) = is_invalid {
            if is_invalid.get() {
                return true;
            }
        }
        if let Some(val) = value.get() {
            if !is_in_range(&val, min.as_ref(), max.as_ref()) {
                return true;
            }
            if let Some(ref unavailable) = is_date_unavailable {
                if unavailable.run(val) {
                    return true;
                }
            }
        }
        false
    });

    // --- Selection callbacks ---

    let set_value_cb = Callback::new(move |new_value: Option<time::OffsetDateTime>| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }
        set_value_signal.set(new_value);
        if let Some(cb) = on_change {
            cb.run(new_value);
        }
    });

    let select_focused_date = Callback::new(move |()| {
        let date = focused_date.get_untracked();
        try_select(date);
    });

    let select_date = Callback::new(move |date: time::OffsetDateTime| {
        // Also move focus to the selected date
        update_focused(date);
        try_select(date);
    });

    // --- Focus navigation callbacks ---

    let set_focused_date_cb = Callback::new(move |date: time::OffsetDateTime| {
        update_focused(date);
    });

    let set_focused_cb = Callback::new(move |focused: bool| {
        set_is_focused_signal.set(focused);
    });

    let focus_next_day = Callback::new(move |()| {
        let current = focused_date.get_untracked();
        update_focused(current + time::Duration::days(1));
    });

    let focus_previous_day = Callback::new(move |()| {
        let current = focused_date.get_untracked();
        update_focused(current - time::Duration::days(1));
    });

    let focus_next_row = Callback::new(move |()| {
        let current = focused_date.get_untracked();
        update_focused(current + time::Duration::days(7));
    });

    let focus_previous_row = Callback::new(move |()| {
        let current = focused_date.get_untracked();
        update_focused(current - time::Duration::days(7));
    });

    let focus_next_page = Callback::new(move |()| {
        let current = focused_date.get_untracked();
        update_focused(start_of_next_month(current));
    });

    let focus_previous_page = Callback::new(move |()| {
        let current = focused_date.get_untracked();
        update_focused(start_of_previous_month(current));
    });

    let focus_section_start = Callback::new(move |()| {
        let current = focused_date.get_untracked();
        update_focused(current.replace_day(1).unwrap());
    });

    let focus_section_end = Callback::new(move |()| {
        let current = focused_date.get_untracked();
        let last_day = whole_days_in(current.year(), current.month());
        update_focused(current.replace_day(last_day).unwrap());
    });

    let focus_next_section = Callback::new(move |larger: bool| {
        let current = focused_date.get_untracked();
        if larger {
            // +1 year
            update_focused(current.save_replace_year(current.year() + 1).unwrap());
        } else {
            // +1 month
            update_focused(start_of_next_month(current));
        }
    });

    let focus_previous_section = Callback::new(move |larger: bool| {
        let current = focused_date.get_untracked();
        if larger {
            // -1 year
            update_focused(current.save_replace_year(current.year() - 1).unwrap());
        } else {
            // -1 month
            update_focused(start_of_previous_month(current));
        }
    });

    // --- Year/month picker (leptonic-specific) ---

    let navigate_years_backward = Callback::new(move |()| {
        years.with(|years| {
            years_start.update(|starting| {
                *starting = match years.len() {
                    0 => focused_date.get_untracked().year() - 5,
                    _ => years[0].number - 12,
                };
            });
        });
    });

    let navigate_years_forward = Callback::new(move |()| {
        years.with(|years| {
            years_start.update(|starting| {
                *starting = match years.len() {
                    0 => focused_date.get_untracked().year() + 1,
                    _ => years[years.len() - 1].number + 1,
                };
            });
        });
    });

    let focus_year_cb = Callback::new(move |year: i32| {
        let current = focused_date.get_untracked();
        update_focused(current.save_replace_year(year).unwrap());
    });

    let focus_month_cb = Callback::new(move |month_index: u8| {
        let current = focused_date.get_untracked();
        update_focused(
            current
                .save_replace_month(time::Month::try_from(month_index).unwrap())
                .unwrap(),
        );
    });

    // --- Query methods ---

    let is_selected_cb = Callback::new(move |date: time::OffsetDateTime| -> bool {
        value.with(|val| {
            val.is_some_and(|v| {
                v.year() == date.year() && v.month() == date.month() && v.day() == date.day()
            })
        })
    });

    let is_cell_focused_cb = Callback::new(move |date: time::OffsetDateTime| -> bool {
        focused_date
            .with(|f| f.year() == date.year() && f.month() == date.month() && f.day() == date.day())
    });

    let is_cell_disabled_cb = Callback::new(move |date: time::OffsetDateTime| -> bool {
        is_disabled.get_untracked() || !is_in_range(&date, min.as_ref(), max.as_ref())
    });

    let is_cell_unavailable_cb = Callback::new(move |date: time::OffsetDateTime| -> bool {
        is_date_unavailable.is_some_and(|cb| cb.run(date))
    });

    UseCalendarStateReturn {
        value: value.into(),
        focused_date: focused_date.into(),
        is_focused: is_focused_signal.into(),
        is_disabled,
        is_read_only,
        is_value_invalid,

        focused_year,
        focused_month_name,

        weeks,
        months,
        years,
        years_range,

        is_previous_visible_range_invalid,
        is_next_visible_range_invalid,

        set_value: set_value_cb,
        select_focused_date,
        select_date,

        set_focused_date: set_focused_date_cb,
        set_focused: set_focused_cb,
        focus_next_day,
        focus_previous_day,
        focus_next_row,
        focus_previous_row,
        focus_next_page,
        focus_previous_page,
        focus_section_start,
        focus_section_end,
        focus_next_section,
        focus_previous_section,

        navigate_years_backward,
        navigate_years_forward,
        focus_year: focus_year_cb,
        focus_month: focus_month_cb,

        is_selected: is_selected_cb,
        is_cell_focused: is_cell_focused_cb,
        is_cell_disabled: is_cell_disabled_cb,
        is_cell_unavailable: is_cell_unavailable_cb,
    }
}

/// Tries local timezone; falls back to UTC in SSR or sandboxed environments.
fn now_local_or_utc() -> time::OffsetDateTime {
    time::OffsetDateTime::now_local().unwrap_or_else(|_| time::OffsetDateTime::now_utc())
}

/// Find the nearest available date on or before `date`, searching back up to `min` or 365 days.
fn previous_available_date(
    date: time::OffsetDateTime,
    min: Option<time::OffsetDateTime>,
    is_date_unavailable: Option<Callback<time::OffsetDateTime, bool>>,
) -> Option<time::OffsetDateTime> {
    let is_unavailable =
        |d: &time::OffsetDateTime| -> bool { is_date_unavailable.is_some_and(|cb| cb.run(*d)) };

    if !is_unavailable(&date) {
        return Some(date);
    }

    let min_date = min.unwrap_or_else(|| date - time::Duration::days(365));
    let mut current = date;
    while current >= min_date {
        if !is_unavailable(&current) {
            return Some(current);
        }
        current -= time::Duration::days(1);
    }
    None
}

pub fn create_years(
    focused: time::OffsetDateTime,
    selected: Option<&time::OffsetDateTime>,
    starting_year: i32,
    min: Option<&time::OffsetDateTime>,
    max: Option<&time::OffsetDateTime>,
) -> Vec<Year> {
    let amount = 3 * 4; // 4 rows of 3 year numbers each.
    let mut years = Vec::<Year>::with_capacity(amount);
    let now = now_local_or_utc();
    let this_year = now.year();
    let focused_year = focused.year();
    let selected_year = selected.map(|s| s.year());
    let min_year = min.map_or(i32::MIN, |it| it.year());
    let max_year = max.map_or(i32::MAX, |it| it.year());

    for i in 0..amount {
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        let year_number = starting_year + i as i32;
        years.push(Year {
            number: year_number,
            is_focused: year_number == focused_year,
            is_selected: selected_year == Some(year_number),
            is_now: year_number == this_year,
            disabled: year_number < min_year || year_number > max_year,
        });
    }
    years
}

/// # Panics
///
/// Panics if a month index cannot be converted or if date replacement fails.
pub fn create_months(
    focused: time::OffsetDateTime,
    selected: Option<&time::OffsetDateTime>,
    min: Option<&time::OffsetDateTime>,
    max: Option<&time::OffsetDateTime>,
) -> Vec<Month> {
    let now = now_local_or_utc();
    let this_year = now.year();
    let this_month = now.month();
    let focused_year = focused.year();
    let focused_month = focused.month();
    let mut months = Vec::<Month>::with_capacity(12);
    for i in 1..=12u8 {
        let month: time::OffsetDateTime = focused
            .save_replace_month(time::Month::try_from(i).unwrap())
            .unwrap();
        let month_year = month.year();
        let month_month = month.month();
        months.push(Month {
            index: i,
            // English-only; i18n would need a custom formatter
            name: month
                .format(format_description!("[month repr:long]"))
                .unwrap(),
            is_focused: focused_year == month_year && focused_month == month_month,
            is_selected: selected
                .is_some_and(|s| s.year() == month_year && s.month() == month_month),
            is_now: this_year == month_year && this_month == month_month,
            disabled: !is_in_range(&month, min, max),
        });
    }
    assert_eq!(months.len(), 12);
    months
}

/// # Panics
///
/// Panics if a day replacement on a date fails.
pub fn create_weeks(
    focused: &time::OffsetDateTime,
    selected: Option<&time::OffsetDateTime>,
    min: Option<&time::OffsetDateTime>,
    max: Option<&time::OffsetDateTime>,
    is_date_unavailable: Option<Callback<time::OffsetDateTime, bool>>,
    first_day_of_week: time::Weekday,
) -> Vec<Week> {
    const WEEKS_TO_DISPLAY: u8 = 6;
    const DAYS_PER_WEEK: u8 = 7;

    let now = now_local_or_utc();

    let current_year = now.year();
    let current_month = now.month();
    let current_day = now.day();
    let focused_day = focused.day();

    // Calculate offset from Monday for the configured first day of week
    let first_day_offset = first_day_of_week.number_days_from_monday(); // 0 = Monday, 6 = Sunday

    let raw_weekday_index = (*focused)
        .replace_day(1)
        .unwrap()
        .weekday()
        .number_days_from_monday(); // in range [0..6]
    // Adjust for first_day_of_week
    let first_weekday_index = (raw_weekday_index + 7 - first_day_offset) % 7;

    let number_of_days_in_month = whole_days_in(focused.year(), focused.month());
    let index_of_last_day_in_month = first_weekday_index + number_of_days_in_month;

    let prev_month = start_of_previous_month(*focused);
    let this_month = focused;
    let next_month = start_of_next_month(*focused);

    let days_in_previous_month = whole_days_in(prev_month.year(), prev_month.month());

    let mut weeks = Vec::<Week>::with_capacity(WEEKS_TO_DISPLAY as usize);
    for w in 0..WEEKS_TO_DISPLAY {
        let mut week = Week {
            id: Uuid::new_v4(),
            days: Vec::with_capacity(DAYS_PER_WEEK as usize),
        };
        for d in 0..DAYS_PER_WEEK {
            let i = d + w * DAYS_PER_WEEK;

            let in_month = if i < first_weekday_index {
                InMonth::Previous
            } else if i < index_of_last_day_in_month {
                InMonth::Current
            } else {
                InMonth::Next
            };

            // base 1 (!)
            let day_in_month = match in_month {
                InMonth::Previous => days_in_previous_month - first_weekday_index + i + 1,
                InMonth::Current => i - first_weekday_index + 1,
                InMonth::Next => i - index_of_last_day_in_month + 1,
            };

            let relevant_month = match in_month {
                InMonth::Previous => &prev_month,
                InMonth::Current => this_month,
                InMonth::Next => &next_month,
            };
            let date_time: time::OffsetDateTime = relevant_month.replace_day(day_in_month).unwrap();
            let disabled = !is_in_range(&date_time, min, max);
            let unavailable = is_date_unavailable.is_some_and(|cb| cb.run(date_time));
            let is_focused_day = in_month == InMonth::Current && day_in_month == focused_day;
            let is_selected_day = selected.is_some_and(|s| {
                s.year() == date_time.year()
                    && s.month() == date_time.month()
                    && s.day() == date_time.day()
            });

            week.days.push(Day {
                id: Uuid::new_v4(),
                index: day_in_month,
                in_month,
                date_time,
                disabled,
                unavailable,
                highlighted: false,
                is_focused: is_focused_day,
                is_selected: is_selected_day,
                is_now: current_month == relevant_month.month()
                    && current_year == relevant_month.year()
                    && current_day == day_in_month,
            });
        }
        weeks.push(week);
    }
    weeks
}
