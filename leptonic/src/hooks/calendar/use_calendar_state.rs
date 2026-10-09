// Upstream: react-stately/src/calendar/useCalendarState.ts @ 99e6102368
// Upstream: react-stately/test/calendar/useCalendarState.test.ts @ 99e6102368
// Upstream: react-aria/test/calendar/useCalendar.test.js @ 99e6102368
// Upstream: react-aria-components/test/Calendar.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/calendar/CalendarBase.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/calendar/Calendar.test.js @ 99e6102368
use jiff::civil::{Date, Weekday};
use leptos::prelude::*;

use super::utils::{
    align_center, align_end, align_start, constrain_start, constrain_value, is_invalid,
    previous_available_date,
};
use crate::{
    ValueBinding,
    utils::{
        date::{DateDuration, DateExt, DateRange, first_day_of_week, min_date, today},
        i18n::use_locale,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Dates are `jiff::civil::Date` (Gregorian; see `utils::date`); the value has no time part
//   (react-aria keeps a `CalendarDateTime`'s time).
// - Hook-owned value and focused date (C4): `default_value`/`default_focused_value` +
//   `on_change`/`on_focus_change`, or bindings to app state.
// - A `Copy` struct with signals and methods (C3).
// - `page_behavior`, `selection_alignment`: enums (react-aria: strings).
//
// - The layout props (`visible_duration`, `page_behavior`, `selection_alignment`,
//   `first_day_of_week`, `weeks_in_month`) are signals (C11); a changed visible duration re-aligns
//   the visible range around the focused date, as react-aria does.
//
// ## LEPTOS-SPECIFIC ADAPTATIONS
// - Without a value or focused date, the calendar starts at today. The server's today may be
//   another date than the browser's (its time zone), and hydration keeps the server's markup: the
//   browser hydrates with the server's date (a `SharedValue`), then moves the focused date to its
//   own today after mounting, unless it was moved already. (The today marks are set in the
//   browser, see `utils::date::use_today`.)
//
// ## OMITTED FEATURES
// - `selectionMode: 'multiple'` (several selected dates).
// - Calendar systems other than the Gregorian.
//
// =============================================================================

/// How the previous and next buttons page (react-aria's `pageBehavior`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PageBehavior {
    /// By the whole visible duration (e.g. three months).
    #[default]
    Visible,
    /// By one unit of it (e.g. one month).
    Single,
}

/// Where the initially focused date sits in the visible range (react-aria's
/// `selectionAlignment`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SelectionAlignment {
    Start,
    #[default]
    Center,
    End,
}

/// Input of [`use_calendar_state`].
#[derive(Clone)]
pub struct UseCalendarStateInput {
    /// The initially selected date.
    pub default_value: Option<Date>,
    /// The selected date as app state, replacing `default_value`.
    pub value: Option<ValueBinding<Option<Date>>>,
    /// Called with the newly selected date.
    pub on_change: Option<Callback<Option<Date>>>,
    pub min_value: Signal<Option<Date>>,
    pub max_value: Signal<Option<Date>>,
    /// Whether a date can't be selected (e.g. booked out).
    pub is_date_unavailable: Option<Callback<Date, bool>>,
    pub is_disabled: Signal<bool>,
    pub is_read_only: Signal<bool>,
    pub is_invalid: Signal<bool>,
    /// Whether the calendar starts focused.
    pub auto_focus: bool,
    /// The initially focused date. Default: the value, else today (within min and max).
    pub default_focused_value: Option<Date>,
    /// The focused date as app state, replacing `default_focused_value`.
    pub focused_value: Option<ValueBinding<Date>>,
    pub on_focus_change: Option<Callback<Date>>,
    /// How much is visible at once. Default: one month. A change re-aligns the visible range
    /// around the focused date.
    pub visible_duration: Signal<DateDuration>,
    pub page_behavior: Signal<PageBehavior>,
    /// Where the focused date is placed when the visible range is aligned (initially, and when
    /// the visible duration changes).
    pub selection_alignment: Signal<SelectionAlignment>,
    /// The first day of the week. `None`: the locale's.
    pub first_day_of_week: Signal<Option<Weekday>>,
    /// A fixed number of week rows per month (e.g. 6, so the calendar keeps its height).
    pub weeks_in_month: Signal<Option<u8>>,
}

impl Default for UseCalendarStateInput {
    fn default() -> Self {
        Self {
            default_value: None,
            value: None,
            on_change: None,
            min_value: Signal::stored(None),
            max_value: Signal::stored(None),
            is_date_unavailable: None,
            is_disabled: Signal::stored(false),
            is_read_only: Signal::stored(false),
            is_invalid: Signal::stored(false),
            auto_focus: false,
            default_focused_value: None,
            focused_value: None,
            on_focus_change: None,
            visible_duration: Signal::stored(DateDuration::months(1)),
            page_behavior: Signal::stored(PageBehavior::Visible),
            selection_alignment: Signal::stored(SelectionAlignment::Center),
            first_day_of_week: Signal::stored(None),
            weeks_in_month: Signal::stored(None),
        }
    }
}

/// The state of a calendar: the selected date, the focused date (the keyboard cursor) and the
/// visible range, which follows the focused date.
#[derive(Clone, Copy)]
pub struct CalendarState {
    /// The selected date.
    pub value: Signal<Option<Date>>,
    /// The focused date: the date the keyboard moves from.
    pub focused_date: Signal<Date>,
    /// The visible dates.
    pub visible_range: Signal<DateRange>,
    /// Whether the calendar's grid has focus.
    pub is_focused: Signal<bool>,
    pub is_disabled: Signal<bool>,
    pub is_read_only: Signal<bool>,
    /// Whether the value is invalid: out of range, unavailable, or marked invalid.
    pub is_value_invalid: Signal<bool>,
    pub min_value: Signal<Option<Date>>,
    pub max_value: Signal<Option<Date>>,
    /// How much is visible at once.
    pub visible_duration: Signal<DateDuration>,
    /// The first day of the week.
    pub first_day_of_week: Signal<Weekday>,
    binding: ValueBinding<Option<Date>>,
    focus: ValueBinding<Date>,
    start: RwSignal<Date>,
    focused: RwSignal<bool>,
    is_date_unavailable: Option<Callback<Date, bool>>,
    page_behavior: Signal<PageBehavior>,
    selection_alignment: Signal<SelectionAlignment>,
    weeks_in_month: Signal<Option<u8>>,
}

/// The last visible date of a range starting at `start`.
fn end_of(start: Date, duration: DateDuration) -> Date {
    let mut shortened = duration;
    if shortened.days == 0 {
        shortened.days = -1;
    } else {
        shortened.days -= 1;
    }
    start.add(shortened)
}

impl CalendarState {
    fn bounds(&self) -> (Option<Date>, Option<Date>) {
        (
            self.min_value.get_untracked(),
            self.max_value.get_untracked(),
        )
    }

    fn first_day(&self) -> Weekday {
        self.first_day_of_week.get_untracked()
    }

    fn duration(&self) -> DateDuration {
        self.visible_duration.get_untracked()
    }

    /// The first visible date with `focused` placed per the selection alignment.
    fn aligned_start(&self, focused: Date) -> Date {
        let (min, max) = self.bounds();
        let (duration, first_day) = (self.duration(), self.first_day());
        match self.selection_alignment.get_untracked() {
            SelectionAlignment::Start => align_start(focused, duration, first_day, min, max),
            SelectionAlignment::End => align_end(focused, duration, first_day, min, max),
            SelectionAlignment::Center => align_center(focused, duration, first_day, min, max),
        }
    }

    /// Aligns the visible range around the focused date (react-aria: when the visible duration
    /// changes).
    fn realign(&self) {
        self.start
            .set(self.aligned_start(self.focused_date.get_untracked()));
    }

    /// Moves the visible range so that it shows `focused`.
    fn show(&self, focused: Date) {
        let (min, max) = self.bounds();
        let start = self.start.get_untracked();
        let duration = self.duration();
        if focused < start {
            self.start
                .set(align_end(focused, duration, self.first_day(), min, max));
        } else if focused > end_of(start, duration) {
            self.start
                .set(align_start(focused, duration, self.first_day(), min, max));
        }
    }

    /// Focuses `date` (within min and max) and shows the focused date: a controlled focused
    /// value may not take it.
    fn focus_cell(&self, date: Date) {
        let (min, max) = self.bounds();
        self.focus.set(constrain_value(date, min, max));
        self.show(self.focus.value.get_untracked());
    }

    /// `date` as a value: within min and max, else the previous available date (react-aria's
    /// `normalizeValue`).
    fn normalize(&self, date: Date) -> Option<Date> {
        let (min, max) = self.bounds();
        let constrained = constrain_value(date, min, max);
        let lower = min.unwrap_or_else(|| min_date(constrained, self.start.get_untracked()));
        let is_unavailable = self
            .is_date_unavailable
            .map(|callback| move |date: Date| callback.run(date));
        previous_available_date(
            constrained,
            lower,
            is_unavailable.as_ref().map(|f| f as &dyn Fn(Date) -> bool),
        )
    }

    /// Sets the value (unless disabled or read-only): a date within min and max, else the
    /// previous available one; `None` clears it.
    pub fn set_value(&self, value: Option<Date>) {
        if self.is_disabled.get_untracked() || self.is_read_only.get_untracked() {
            return;
        }
        match value {
            None => self.binding.set(None),
            Some(date) => {
                if let Some(date) = self.normalize(date) {
                    self.binding.set(Some(date));
                }
            }
        }
    }

    /// Focuses `date` (within min and max).
    pub fn set_focused_date(&self, date: Date) {
        self.focus_cell(date);
    }

    pub fn focus_next_day(&self) {
        self.focus_cell(self.focused_date.get_untracked().add(DateDuration::days(1)));
    }

    pub fn focus_previous_day(&self) {
        self.focus_cell(
            self.focused_date
                .get_untracked()
                .subtract(DateDuration::days(1)),
        );
    }

    /// The date a week later (in a day view: the next page).
    pub fn focus_next_row(&self) {
        let duration = self.duration();
        if duration.days != 0 {
            self.focus_next_page();
        } else if duration.weeks != 0 || duration.months != 0 || duration.years != 0 {
            self.focus_cell(
                self.focused_date
                    .get_untracked()
                    .add(DateDuration::weeks(1)),
            );
        }
    }

    /// The date a week earlier (in a day view: the previous page).
    pub fn focus_previous_row(&self) {
        let duration = self.duration();
        if duration.days != 0 {
            self.focus_previous_page();
        } else if duration.weeks != 0 || duration.months != 0 || duration.years != 0 {
            self.focus_cell(
                self.focused_date
                    .get_untracked()
                    .subtract(DateDuration::weeks(1)),
            );
        }
    }

    /// Shows the next page and moves the focused date by the same amount (keeping the day).
    pub fn focus_next_page(&self) {
        self.page(self.page_duration());
    }

    /// Shows the previous page and moves the focused date by the same amount (keeping the day).
    pub fn focus_previous_page(&self) {
        self.page(self.page_duration().negated());
    }

    /// How far the previous and next buttons page.
    fn page_duration(&self) -> DateDuration {
        let duration = self.duration();
        match self.page_behavior.get_untracked() {
            PageBehavior::Visible => duration,
            PageBehavior::Single => duration.unit(),
        }
    }

    fn page(&self, duration: DateDuration) {
        let (min, max) = self.bounds();
        let focused = self.focused_date.get_untracked();
        let start = self.start.get_untracked().add(duration);
        self.focus
            .set(constrain_value(focused.add(duration), min, max));
        let page =
            if duration.years < 0 || duration.months < 0 || duration.weeks < 0 || duration.days < 0
            {
                duration.negated()
            } else {
                duration
            };
        self.start.set(align_start(
            constrain_start(focused, start, page, self.first_day(), min, max),
            page,
            self.first_day(),
            None,
            None,
        ));
    }

    /// The first date of the focused section (week or month).
    pub fn focus_section_start(&self) {
        let duration = self.duration();
        let focused = self.focused_date.get_untracked();
        if duration.days != 0 {
            self.focus_cell(self.start.get_untracked());
        } else if duration.weeks != 0 {
            self.focus_cell(focused.start_of_week(self.first_day()));
        } else if duration.months != 0 || duration.years != 0 {
            self.focus_cell(focused.first_of_month());
        }
    }

    /// The last date of the focused section (week or month).
    pub fn focus_section_end(&self) {
        let duration = self.duration();
        let focused = self.focused_date.get_untracked();
        if duration.days != 0 {
            self.focus_cell(end_of(self.start.get_untracked(), duration));
        } else if duration.weeks != 0 {
            self.focus_cell(focused.end_of_week(self.first_day()));
        } else if duration.months != 0 || duration.years != 0 {
            self.focus_cell(focused.last_of_month());
        }
    }

    /// The same date one section later: a month (`larger`: a year) in a month view, a week (a
    /// month) in a week view, a page in a day view.
    pub fn focus_next_section(&self, larger: bool) {
        self.section(larger, 1);
    }

    /// The same date one section earlier.
    pub fn focus_previous_section(&self, larger: bool) {
        self.section(larger, -1);
    }

    fn section(&self, larger: bool, direction: i32) {
        let duration = self.duration();
        let focused = self.focused_date.get_untracked();
        let by =
            |step: DateDuration| focused.add(if direction < 0 { step.negated() } else { step });
        if !larger && duration.days == 0 {
            self.focus_cell(by(duration.unit()));
        } else if duration.days != 0 {
            if direction < 0 {
                self.focus_previous_page();
            } else {
                self.focus_next_page();
            }
        } else if duration.weeks != 0 {
            self.focus_cell(by(DateDuration::months(1)));
        } else if duration.months != 0 || duration.years != 0 {
            self.focus_cell(by(DateDuration::years(1)));
        }
    }

    /// Selects the focused date, unless it is unavailable.
    pub fn select_focused_date(&self) {
        let focused = self.focused_date.get_untracked();
        if !self.is_cell_unavailable_untracked(focused) {
            self.select_date(focused);
        }
    }

    /// Selects `date` (unless disabled or read-only).
    pub fn select_date(&self, date: Date) {
        if self.is_disabled.get_untracked() || self.is_read_only.get_untracked() {
            return;
        }
        self.set_value(Some(date));
    }

    /// Whether the state still exists (not disposed with its calendar), for blur handlers.
    pub(crate) fn is_alive(&self) -> bool {
        self.focused.try_with_untracked(|_| ()).is_some()
    }

    /// Sets whether the calendar's grid has focus.
    pub fn set_focused(&self, focused: bool) {
        self.focused.set(focused);
    }

    /// Whether `date` lies outside min and max.
    pub fn is_invalid(&self, date: Date) -> bool {
        is_invalid(date, self.min_value.get(), self.max_value.get())
    }

    /// Whether `date` is the selected date (and selectable).
    pub fn is_selected(&self, date: Date) -> bool {
        self.value.get() == Some(date)
            && !self.is_cell_disabled(date)
            && !self.is_cell_unavailable(date)
    }

    /// Whether `date` is focused (the grid has focus and it is the focused date).
    pub fn is_cell_focused(&self, date: Date) -> bool {
        self.is_focused.get() && self.focused_date.get() == date
    }

    /// Whether `date` can't be focused or selected: the calendar is disabled, or the date is not
    /// visible or outside min and max.
    pub fn is_cell_disabled(&self, date: Date) -> bool {
        let range = self.visible_range.get();
        self.is_disabled.get() || !range.contains(date) || self.is_invalid(date)
    }

    /// Whether `date` is unavailable (`is_date_unavailable`).
    pub fn is_cell_unavailable(&self, date: Date) -> bool {
        self.is_date_unavailable
            .is_some_and(|is_unavailable| is_unavailable.run(date))
    }

    fn is_cell_unavailable_untracked(&self, date: Date) -> bool {
        untrack(|| self.is_cell_unavailable(date))
    }

    /// Whether the previous page would show only dates before min.
    pub fn is_previous_visible_range_invalid(&self) -> bool {
        let start = self.visible_range.get().start;
        let previous = start.subtract(DateDuration::days(1));
        previous == start || self.is_invalid(previous)
    }

    /// Whether the next page would show only dates after max.
    pub fn is_next_visible_range_invalid(&self) -> bool {
        let end = self.visible_range.get().end;
        let next = end.add(DateDuration::days(1));
        next == end || self.is_invalid(next)
    }

    /// The dates of the week `week_index` weeks after `from` (default: the visible range's start),
    /// `None` where the representable range ends.
    pub fn dates_in_week(&self, week_index: u8, from: Option<Date>) -> Vec<Option<Date>> {
        let from = from.unwrap_or_else(|| self.visible_range.get().start);
        let mut date = from.add(DateDuration::weeks(i32::from(week_index)));
        let days = match self.visible_duration.get().days {
            days @ 1..7 => usize::try_from(days).unwrap_or(7),
            _ => 7,
        };
        let mut dates = Vec::with_capacity(days);
        if days == 7 {
            let first_day = self.first_day_of_week.get();
            date = date.start_of_week(first_day);
            // The representable range may start in the middle of a week.
            dates.extend((0..date.day_of_week(first_day)).map(|_| None));
        }
        while dates.len() < days {
            dates.push(Some(date));
            let next = date.add(DateDuration::days(1));
            if next == date {
                break;
            }
            date = next;
        }
        dates.resize(days, None);
        dates
    }

    /// The number of week rows of the month of `from` (default: the visible range's start).
    pub fn weeks_in_month(&self, from: Option<Date>) -> u8 {
        let duration = self.visible_duration.get();
        if duration.weeks != 0 || duration.days != 0 {
            let days_weeks = u8::try_from((duration.days.max(0) + 6) / 7).unwrap_or(0);
            return u8::try_from(duration.weeks.max(0)).unwrap_or(0) + days_weeks;
        }
        self.weeks_in_month.get().unwrap_or_else(|| {
            let from = from.unwrap_or_else(|| self.visible_range.get().start);
            from.weeks_in_month(self.first_day_of_week.get())
        })
    }
}

/// Creates the state of a calendar: the selected date, the focused date and the visible range
/// following it.
pub fn use_calendar_state(input: UseCalendarStateInput) -> CalendarState {
    let UseCalendarStateInput {
        default_value,
        value,
        on_change,
        min_value,
        max_value,
        is_date_unavailable,
        is_disabled,
        is_read_only,
        is_invalid: is_marked_invalid,
        auto_focus,
        default_focused_value,
        focused_value,
        on_focus_change,
        visible_duration,
        page_behavior,
        selection_alignment,
        first_day_of_week: first_day_override,
        weeks_in_month,
    } = input;

    let locale = use_locale();
    let first_day_of_week = Signal::derive(move || {
        first_day_override
            .get()
            .unwrap_or_else(|| first_day_of_week(&locale.get()))
    });

    let owned_value = value.unwrap_or_else(|| ValueBinding::from(RwSignal::new(default_value)));
    let binding = ValueBinding::new(
        owned_value.value,
        Callback::new(move |date: Option<Date>| {
            owned_value.set(date);
            if let Some(on_change) = on_change {
                on_change.run(date);
            }
        }),
    );

    let (min, max) = (min_value.get_untracked(), max_value.get_untracked());
    // Without a value or focused date: today, the server's while hydrating (see the deviations).
    let starts_today = default_focused_value.is_none()
        && focused_value.is_none()
        && binding.value.get_untracked().is_none();
    let initial_today = starts_today.then(|| SharedValue::new_str(today).into_inner());
    let initial_focus = constrain_value(
        default_focused_value
            .or_else(|| binding.value.get_untracked())
            .or(initial_today)
            .unwrap_or_else(today),
        min,
        max,
    );
    // A controlled value may be read-only: constrain what the calendar reads rather than
    // relying on its setter to accept an initial or subsequent min/max correction.
    let owned_focus = focused_value.map_or_else(
        || ValueBinding::from(RwSignal::new(initial_focus)),
        |binding| {
            ValueBinding::new(
                Signal::derive(move || {
                    constrain_value(binding.value.get(), min_value.get(), max_value.get())
                }),
                Callback::new(move |date| binding.set(date)),
            )
        },
    );
    let focus = ValueBinding::new(
        owned_focus.value,
        Callback::new(move |date: Date| {
            if owned_focus.value.get_untracked() != date {
                owned_focus.set(date);
                if let Some(on_focus_change) = on_focus_change {
                    on_focus_change.run(date);
                }
            }
        }),
    );

    let first_day = first_day_of_week.get_untracked();
    let focused_now = focus.value.get_untracked();
    let duration = visible_duration.get_untracked();
    let start = RwSignal::new(match selection_alignment.get_untracked() {
        SelectionAlignment::Start => align_start(focused_now, duration, first_day, min, max),
        SelectionAlignment::End => align_end(focused_now, duration, first_day, min, max),
        SelectionAlignment::Center => align_center(focused_now, duration, first_day, min, max),
    });
    let visible_range = Signal::derive(move || {
        let start = start.get();
        DateRange {
            start,
            end: end_of(start, visible_duration.get()),
        }
    });
    let focused = RwSignal::new(auto_focus);

    let is_value_invalid = Signal::derive(move || {
        is_marked_invalid.get()
            || binding.value.get().is_some_and(|date| {
                is_date_unavailable.is_some_and(|is_unavailable| is_unavailable.run(date))
                    || is_invalid(date, min_value.get(), max_value.get())
            })
    });

    let state = CalendarState {
        value: binding.value,
        focused_date: focus.value,
        visible_range,
        is_focused: focused.into(),
        is_disabled,
        is_read_only,
        is_value_invalid,
        min_value,
        max_value,
        visible_duration,
        first_day_of_week,
        binding,
        focus,
        start,
        focused,
        is_date_unavailable,
        page_behavior,
        selection_alignment,
        weeks_in_month,
    };

    // A changed visible duration re-aligns the visible range around the focused date.
    Effect::watch(
        move || visible_duration.get(),
        move |_, _, _| state.realign(),
        false,
    );

    // Hydrated with the server's today: the browser's, once mounted (unless moved meanwhile).
    if let Some(server_today) = initial_today {
        Effect::new(move |_| {
            untrack(|| {
                let browser_today = today();
                if browser_today != server_today && state.focused_date.get() == initial_focus {
                    let (min, max) = state.bounds();
                    state.focus.set(constrain_value(browser_today, min, max));
                    state.realign();
                }
            });
        });
    }

    // A focused date moved from outside (app state, min/max changes) stays within min and max
    // and visible (react-aria checks this while rendering).
    Effect::new(move |_| {
        let date = focus.value.get();
        let (min, max) = (min_value.get(), max_value.get());
        if is_invalid(date, min, max) {
            focus.set(constrain_value(date, min, max));
        } else {
            untrack(|| state.show(date));
        }
    });

    state
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;
    use jiff::civil::date;

    use super::*;
    use crate::testing::with_owner;

    fn state(input: UseCalendarStateInput) -> CalendarState {
        use_calendar_state(UseCalendarStateInput {
            first_day_of_week: Signal::stored(Some(Weekday::Sunday)),
            ..input
        })
    }

    #[test]
    fn shows_the_month_of_the_value() {
        with_owner(|| {
            let calendar = state(UseCalendarStateInput {
                default_value: Some(date(2024, 5, 15)),
                ..UseCalendarStateInput::default()
            });
            assert_that!(calendar.focused_date.get_untracked()).is_equal_to(date(2024, 5, 15));
            assert_that!(calendar.visible_range.get_untracked()).is_equal_to(DateRange {
                start: date(2024, 5, 1),
                end: date(2024, 5, 31),
            });
            assert_that!(calendar.weeks_in_month(None)).is_equal_to(5);
            assert_that!(calendar.dates_in_week(0, None)[0]).is_equal_to(Some(date(2024, 4, 28)));
        });
    }

    #[test]
    fn constrains_a_controlled_focused_date_without_mutating_it() {
        with_owner(|| {
            let focused = RwSignal::new(date(2019, 6, 5));
            let min = RwSignal::new(Some(date(2019, 7, 5)));
            let calendar = state(UseCalendarStateInput {
                focused_value: Some(ValueBinding::new(focused.into(), Callback::new(|_| {}))),
                min_value: min.into(),
                ..UseCalendarStateInput::default()
            });
            assert_that!(calendar.focused_date.get_untracked()).is_equal_to(date(2019, 7, 5));
            assert_that!(calendar.visible_range.get_untracked().start)
                .is_equal_to(date(2019, 7, 1));
            assert_that!(focused.get_untracked()).is_equal_to(date(2019, 6, 5));
            min.set(Some(date(2019, 8, 5)));
            crate::testing::flush_effects();
            assert_that!(calendar.focused_date.get_untracked()).is_equal_to(date(2019, 8, 5));
            assert_that!(calendar.visible_range.get_untracked().start)
                .is_equal_to(date(2019, 8, 1));
        });
    }

    #[test]
    fn paging_keeps_the_day() {
        with_owner(|| {
            let calendar = state(UseCalendarStateInput {
                default_value: Some(date(2024, 5, 15)),
                ..UseCalendarStateInput::default()
            });
            calendar.focus_next_page();
            assert_that!(calendar.focused_date.get_untracked()).is_equal_to(date(2024, 6, 15));
            assert_that!(calendar.visible_range.get_untracked().start)
                .is_equal_to(date(2024, 6, 1));
            calendar.focus_previous_section(true);
            assert_that!(calendar.focused_date.get_untracked()).is_equal_to(date(2023, 6, 15));
            assert_that!(calendar.visible_range.get_untracked().start)
                .is_equal_to(date(2023, 6, 1));
            calendar.focus_section_end();
            assert_that!(calendar.focused_date.get_untracked()).is_equal_to(date(2023, 6, 30));
            calendar.focus_next_day();
            assert_that!(calendar.visible_range.get_untracked().start)
                .is_equal_to(date(2023, 7, 1));
        });
    }

    #[test]
    fn selection_respects_min_max_and_unavailable_dates() {
        with_owner(|| {
            let weekend = Callback::new(|date: Date| {
                matches!(date.weekday(), Weekday::Saturday | Weekday::Sunday)
            });
            let calendar = state(UseCalendarStateInput {
                default_focused_value: Some(date(2024, 5, 15)),
                min_value: Signal::stored(Some(date(2024, 5, 10))),
                is_date_unavailable: Some(weekend),
                ..UseCalendarStateInput::default()
            });
            // An unavailable date selects the previous available one (react-aria's
            // `normalizeValue`); keyboard selection of an unavailable date does nothing.
            calendar.select_date(date(2024, 5, 19));
            assert_that!(calendar.value.get_untracked()).is_equal_to(Some(date(2024, 5, 17)));
            calendar.set_focused_date(date(2024, 5, 18));
            calendar.select_focused_date();
            assert_that!(calendar.value.get_untracked()).is_equal_to(Some(date(2024, 5, 17)));
            // Below min: constrained.
            calendar.set_focused_date(date(2024, 5, 1));
            assert_that!(calendar.focused_date.get_untracked()).is_equal_to(date(2024, 5, 10));
            assert_that!(calendar.is_previous_visible_range_invalid()).is_true();
        });
    }

    /// From react-stately's `useCalendarState.test.ts` ("selectDate").
    #[test]
    fn selects_dates_outside_the_visible_range_and_the_nearest_available() {
        with_owner(|| {
            let selected = date(2026, 4, 15);
            let never = Callback::new(|_: Date| false);
            for forward in [true, false] {
                let calendar = state(UseCalendarStateInput {
                    is_date_unavailable: Some(never),
                    default_focused_value: Some(selected),
                    ..UseCalendarStateInput::default()
                });
                if forward {
                    calendar.focus_next_page();
                    assert_that!(calendar.visible_range.get_untracked().start > selected).is_true();
                } else {
                    calendar.focus_previous_page();
                    assert_that!(calendar.visible_range.get_untracked().end < selected).is_true();
                }
                calendar.select_date(selected);
                assert_that!(calendar.value.get_untracked()).is_equal_to(Some(selected));
            }

            let fifteenth = Callback::new(|date: Date| date.day() == 15);
            let calendar = state(UseCalendarStateInput {
                is_date_unavailable: Some(fifteenth),
                default_focused_value: Some(date(2026, 4, 20)),
                ..UseCalendarStateInput::default()
            });
            calendar.select_date(selected);
            assert_that!(calendar.value.get_untracked()).is_equal_to(Some(date(2026, 4, 14)));

            let first_half = Callback::new(|date: Date| date.day() <= 15);
            let calendar = state(UseCalendarStateInput {
                is_date_unavailable: Some(first_half),
                default_focused_value: Some(date(2026, 4, 20)),
                ..UseCalendarStateInput::default()
            });
            calendar.select_date(selected);
            assert_that!(calendar.value.get_untracked()).is_none();
        });
    }
}
