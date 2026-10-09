// Upstream: react-stately/src/calendar/useRangeCalendarState.ts @ 99e6102368
// Upstream: react-aria-components/test/RangeCalendar.test.tsx @ 99e6102368
// Upstream: @adobe/react-spectrum/test/calendar/RangeCalendar.test.js @ 99e6102368
use jiff::civil::{Date, Weekday};
use leptos::prelude::*;

use super::{
    use_calendar_state::{
        CalendarState, PageBehavior, SelectionAlignment, UseCalendarStateInput, use_calendar_state,
    },
    utils::{align_center, constrain_value, is_invalid, previous_available_date},
};
use crate::{
    ValueBinding,
    utils::date::{DateDuration, DateExt, DateRange, max_date, min_date},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Dates are `jiff::civil::Date`, the value a `DateRange` (react-aria: `RangeValue<DateValue>`,
//   keeping a time part).
// - Hook-owned value (C4): `default_value` + `on_change`, or a binding to app state.
// - The calendar state is the `calendar` field (react-aria: spread into the range state).
// - `is_date_unavailable` takes a `DateAvailabilityQuery`: the date and the anchor date.
// - The layout props are signals (C11), as in `use_calendar_state`.
//
// =============================================================================

/// What `is_date_unavailable` of a range calendar is asked: whether `date` can't be selected,
/// given the anchor of a range being selected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateAvailabilityQuery {
    pub date: Date,
    /// The first selected end of a range being selected.
    pub anchor_date: Option<Date>,
}

/// Input of [`use_range_calendar_state`].
#[derive(Clone)]
pub struct UseRangeCalendarStateInput {
    /// The initially selected range.
    pub default_value: Option<DateRange>,
    /// The selected range as app state, replacing `default_value`.
    pub value: Option<ValueBinding<Option<DateRange>>>,
    /// Called with the newly selected range.
    pub on_change: Option<Callback<Option<DateRange>>>,
    pub min_value: Signal<Option<Date>>,
    pub max_value: Signal<Option<Date>>,
    /// Whether a date can't be selected, given the anchor date of a selection in progress.
    pub is_date_unavailable: Option<Callback<DateAvailabilityQuery, bool>>,
    /// Whether a range may span unavailable dates.
    pub allows_non_contiguous_ranges: bool,
    pub is_disabled: Signal<bool>,
    pub is_read_only: Signal<bool>,
    pub is_invalid: Signal<bool>,
    pub auto_focus: bool,
    pub default_focused_value: Option<Date>,
    pub focused_value: Option<ValueBinding<Date>>,
    pub on_focus_change: Option<Callback<Date>>,
    /// How much is visible at once. Default: one month.
    pub visible_duration: Signal<DateDuration>,
    pub page_behavior: Signal<PageBehavior>,
    /// `None`: centered, or the start if the range doesn't fit then.
    pub selection_alignment: Signal<Option<SelectionAlignment>>,
    /// The first day of the week. `None`: the locale's.
    pub first_day_of_week: Signal<Option<Weekday>>,
    pub weeks_in_month: Signal<Option<u8>>,
}

impl Default for UseRangeCalendarStateInput {
    fn default() -> Self {
        Self {
            default_value: None,
            value: None,
            on_change: None,
            min_value: Signal::stored(None),
            max_value: Signal::stored(None),
            is_date_unavailable: None,
            allows_non_contiguous_ranges: false,
            is_disabled: Signal::stored(false),
            is_read_only: Signal::stored(false),
            is_invalid: Signal::stored(false),
            auto_focus: false,
            default_focused_value: None,
            focused_value: None,
            on_focus_change: None,
            visible_duration: Signal::stored(DateDuration::months(1)),
            page_behavior: Signal::stored(PageBehavior::Visible),
            selection_alignment: Signal::stored(None),
            first_day_of_week: Signal::stored(None),
            weeks_in_month: Signal::stored(None),
        }
    }
}

/// The state of a range calendar: a calendar whose selection is a range, chosen with two
/// selections (an anchor, then the other end).
#[derive(Clone, Copy)]
pub struct RangeCalendarState {
    /// The calendar: focused date, visible range, navigation. Its min and max narrow to the
    /// available range around the anchor while a range is being selected.
    pub calendar: CalendarState,
    /// The selected range.
    pub value: Signal<Option<DateRange>>,
    /// The first selected end of a range being selected.
    pub anchor_date: Signal<Option<Date>>,
    /// The range to show as selected: from the anchor to the focused date while selecting, else
    /// the value.
    pub highlighted_range: Signal<Option<DateRange>>,
    /// Whether the user is dragging to select a range.
    pub is_dragging: Signal<bool>,
    /// Whether the value is invalid: out of range, unavailable, or marked invalid.
    pub is_value_invalid: Signal<bool>,
    binding: ValueBinding<Option<DateRange>>,
    anchor: RwSignal<Option<Date>>,
    dragging: RwSignal<bool>,
    available_range: Memo<Option<(Option<Date>, Option<Date>)>>,
    available_range_for: StoredValue<AvailableRange>,
}

/// The available range around an anchor (react-aria's `getAvailableRange`).
#[derive(Clone, Copy)]
struct AvailableRange {
    is_date_unavailable: Option<Callback<DateAvailabilityQuery, bool>>,
    visible_duration: Signal<DateDuration>,
    allows_non_contiguous_ranges: bool,
}

impl AvailableRange {
    /// The dates around `anchor` up to the nearest unavailable ones, if ranges must be
    /// contiguous.
    fn around(self, anchor: Option<Date>) -> Option<(Option<Date>, Option<Date>)> {
        let anchor = anchor?;
        let is_unavailable = self.is_date_unavailable?;
        if self.allows_non_contiguous_ranges {
            return None;
        }
        let unavailable = |date: Date| {
            is_unavailable.run(DateAvailabilityQuery {
                date,
                anchor_date: Some(anchor),
            })
        };
        let visible_duration = self.visible_duration.get();
        Some((
            next_unavailable_date(anchor, &unavailable, visible_duration, -1),
            next_unavailable_date(anchor, &unavailable, visible_duration, 1),
        ))
    }
}

/// The last available date before the first unavailable one in `direction` from `anchor`,
/// within a visible duration.
fn next_unavailable_date(
    anchor: Date,
    is_unavailable: &dyn Fn(Date) -> bool,
    visible_duration: DateDuration,
    direction: i32,
) -> Option<Date> {
    let step = DateDuration::days(direction);
    let mut next = anchor.add(step);
    let min = anchor.subtract(visible_duration);
    let max = anchor.add(visible_duration);
    while (if direction < 0 {
        next >= min
    } else {
        next <= max
    }) && !is_unavailable(next)
    {
        let after = next.add(step);
        if after == next {
            break;
        }
        next = after;
    }
    is_unavailable(next).then(|| next.subtract(step))
}

impl RangeCalendarState {
    /// Sets the selected range (no checks; `None` clears it).
    pub fn set_value(&self, value: Option<DateRange>) {
        self.binding.set(value);
    }

    /// Sets (or clears) the anchor of a range being selected.
    pub fn set_anchor_date(&self, date: Option<Date>) {
        self.anchor.set(date);
    }

    /// Selects `date` as one end: the anchor first, then the other end, which sets the value.
    pub fn select_date(&self, date: Date) {
        let calendar = &self.calendar;
        if calendar.is_read_only.get_untracked() {
            return;
        }
        let min = calendar.min_value.get_untracked();
        let max = calendar.max_value.get_untracked();
        let constrained = constrain_value(date, min, max);
        let anchor = self.anchor.get_untracked();
        let is_unavailable = |date: Date| untrack(|| calendar.is_cell_unavailable(date));
        let Some(date) = previous_available_date(
            constrained,
            calendar.visible_range.get_untracked().start,
            Some(&is_unavailable),
        ) else {
            return;
        };
        match anchor {
            None => self.anchor.set(Some(date)),
            Some(anchor) => {
                self.binding.set(Some(DateRange::between(anchor, date)));
                self.anchor.set(None);
            }
        }
    }

    /// Selects the focused date, unless it is unavailable.
    pub fn select_focused_date(&self) {
        let focused = self.calendar.focused_date.get_untracked();
        if !untrack(|| self.calendar.is_cell_unavailable(focused)) {
            self.select_date(focused);
        }
    }

    /// Finishes a range being selected at the focused date.
    pub fn commit_selection(&self) {
        self.select_date(self.calendar.focused_date.get_untracked());
    }

    /// Moves the focused date to `date` while a range is being selected (hovering, dragging).
    pub fn highlight_date(&self, date: Date) {
        if self.anchor.get_untracked().is_some() {
            self.calendar.set_focused_date(date);
        }
    }

    /// Whether `date` lies in the highlighted range (and is selectable).
    pub fn is_selected(&self, date: Date) -> bool {
        self.highlighted_range
            .get()
            .is_some_and(|range| range.contains(date))
            && !self.calendar.is_cell_disabled(date)
            && !self.calendar.is_cell_unavailable(date)
    }

    /// Whether `date` lies outside min and max, or outside the available range around the anchor.
    pub fn is_invalid(&self, date: Date) -> bool {
        self.calendar.is_invalid(date)
            || self
                .available_range
                .get()
                .is_some_and(|(start, end)| is_invalid(date, start, end))
    }

    pub fn set_dragging(&self, dragging: bool) {
        self.dragging.set(dragging);
    }

    /// Clears the value and any range being selected.
    pub fn clear_selection(&self) {
        self.anchor.set(None);
        self.binding.set(None);
    }

    /// Focuses the date after `anchor` (else the one before) if it can be part of a range.
    pub fn focus_nearest_available_date(&self, anchor: Date) {
        let available = self.available_range_for.get_value().around(Some(anchor));
        let is_date_invalid = |date: Date| {
            untrack(|| self.is_invalid(date))
                || available.is_some_and(|(start, end)| is_invalid(date, start, end))
        };
        let mut next = anchor.add(DateDuration::days(1));
        if is_date_invalid(next) {
            next = anchor.subtract(DateDuration::days(1));
        }
        if !is_date_invalid(next) {
            self.calendar.set_focused_date(next);
            self.calendar.set_focused(true);
        }
    }
}

/// Creates the state of a range calendar.
pub fn use_range_calendar_state(input: UseRangeCalendarStateInput) -> RangeCalendarState {
    let UseRangeCalendarStateInput {
        default_value,
        value,
        on_change,
        min_value,
        max_value,
        is_date_unavailable,
        allows_non_contiguous_ranges,
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
        first_day_of_week,
        weeks_in_month,
    } = input;

    let owned = value.unwrap_or_else(|| ValueBinding::from(RwSignal::new(default_value)));
    let binding = ValueBinding::new(
        owned.value,
        Callback::new(move |range: Option<DateRange>| {
            owned.set(range);
            if let Some(on_change) = on_change {
                on_change.run(range);
            }
        }),
    );
    let anchor = RwSignal::new(None::<Date>);
    let dragging = RwSignal::new(false);

    // Start-aligned when a centered range wouldn't show the whole value.
    let locale = crate::utils::i18n::use_locale();
    let alignment = Signal::derive(move || {
        selection_alignment.get().unwrap_or_else(|| {
            let Some(range) = binding.value.get_untracked() else {
                return SelectionAlignment::Center;
            };
            let visible_duration = visible_duration.get();
            let start = align_center(
                range.start,
                visible_duration,
                first_day_of_week
                    .get()
                    .unwrap_or_else(|| crate::utils::date::first_day_of_week(&locale.get())),
                min_value.get_untracked(),
                max_value.get_untracked(),
            );
            let end = start.add(visible_duration).subtract(DateDuration::days(1));
            if range.end > end {
                SelectionAlignment::Start
            } else {
                SelectionAlignment::Center
            }
        })
    });

    let available = AvailableRange {
        is_date_unavailable,
        visible_duration,
        allows_non_contiguous_ranges,
    };
    let available_range = Memo::new(move |_| available.around(anchor.get()));
    let min = Signal::derive(move || {
        let start = available_range.get().and_then(|(start, _)| start);
        match (min_value.get(), start) {
            (Some(min), Some(start)) => Some(max_date(min, start)),
            (min, start) => min.or(start),
        }
    });
    let max = Signal::derive(move || {
        let end = available_range.get().and_then(|(_, end)| end);
        match (max_value.get(), end) {
            (Some(max), Some(end)) => Some(min_date(max, end)),
            (max, end) => max.or(end),
        }
    });

    let calendar = use_calendar_state(UseCalendarStateInput {
        // The calendar shows the range's start; selection goes through this state.
        value: Some(ValueBinding::new(
            Signal::derive(move || binding.value.get().map(|range| range.start)),
            Callback::new(|_: Option<Date>| {}),
        )),
        min_value: min,
        max_value: max,
        is_date_unavailable: is_date_unavailable.map(|is_unavailable| {
            Callback::new(move |date: Date| {
                is_unavailable.run(DateAvailabilityQuery {
                    date,
                    anchor_date: anchor.get(),
                })
            })
        }),
        is_disabled,
        is_read_only,
        auto_focus,
        default_focused_value,
        focused_value,
        on_focus_change,
        visible_duration,
        page_behavior,
        selection_alignment: alignment,
        first_day_of_week,
        weeks_in_month,
        ..UseCalendarStateInput::default()
    });

    let highlighted_range = Signal::derive(move || match anchor.get() {
        Some(anchor) => Some(DateRange::between(anchor, calendar.focused_date.get())),
        None => binding.value.get(),
    });
    let is_value_invalid = Signal::derive(move || {
        if is_marked_invalid.get() {
            return true;
        }
        let Some(range) = binding.value.get() else {
            return false;
        };
        if anchor.get().is_some() {
            return false;
        }
        let unavailable = |date: Date| {
            is_date_unavailable.is_some_and(|is_unavailable| {
                is_unavailable.run(DateAvailabilityQuery {
                    date,
                    anchor_date: None,
                })
            })
        };
        unavailable(range.start)
            || unavailable(range.end)
            || is_invalid(range.start, min_value.get(), max_value.get())
            || is_invalid(range.end, min_value.get(), max_value.get())
    });

    RangeCalendarState {
        calendar,
        value: binding.value,
        anchor_date: anchor.into(),
        highlighted_range,
        is_dragging: dragging.into(),
        is_value_invalid,
        binding,
        anchor,
        dragging,
        available_range,
        available_range_for: StoredValue::new(available),
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;
    use jiff::civil::date;

    use super::*;
    use crate::testing::with_owner;

    fn state(input: UseRangeCalendarStateInput) -> RangeCalendarState {
        use_range_calendar_state(UseRangeCalendarStateInput {
            first_day_of_week: Signal::stored(Some(Weekday::Sunday)),
            ..input
        })
    }

    #[test]
    fn selects_a_range_with_two_selections() {
        with_owner(|| {
            let range = state(UseRangeCalendarStateInput {
                default_focused_value: Some(date(2024, 5, 15)),
                ..UseRangeCalendarStateInput::default()
            });
            range.select_date(date(2024, 5, 20));
            assert_that!(range.anchor_date.get_untracked()).is_equal_to(Some(date(2024, 5, 20)));
            range.calendar.set_focused_date(date(2024, 5, 10));
            // While selecting, the highlighted range runs from the anchor to the focused date.
            assert_that!(range.highlighted_range.get_untracked()).is_equal_to(Some(DateRange {
                start: date(2024, 5, 10),
                end: date(2024, 5, 20),
            }));
            range.commit_selection();
            assert_that!(range.value.get_untracked()).is_equal_to(Some(DateRange {
                start: date(2024, 5, 10),
                end: date(2024, 5, 20),
            }));
            assert_that!(range.anchor_date.get_untracked()).is_none();
        });
    }

    #[test]
    fn contiguous_ranges_stop_at_unavailable_dates() {
        with_owner(|| {
            let booked = Callback::new(|query: DateAvailabilityQuery| query.date.day() == 25);
            let range = state(UseRangeCalendarStateInput {
                default_focused_value: Some(date(2024, 5, 15)),
                is_date_unavailable: Some(booked),
                ..UseRangeCalendarStateInput::default()
            });
            range.select_date(date(2024, 5, 20));
            // The 25th is booked: the anchor's available range ends on the 24th.
            assert_that!(range.calendar.max_value.get_untracked())
                .is_equal_to(Some(date(2024, 5, 24)));
            assert_that!(range.is_invalid(date(2024, 5, 26))).is_true();
            range.calendar.set_focused_date(date(2024, 5, 28));
            assert_that!(range.calendar.focused_date.get_untracked())
                .is_equal_to(date(2024, 5, 24));
        });
    }
}
