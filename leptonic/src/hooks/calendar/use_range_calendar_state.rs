use leptos::prelude::*;
use time::OffsetDateTime;

use super::use_calendar_state::{UseCalendarStateInput, UseCalendarStateReturn};
use crate::utils::time::is_in_range;

// This is based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-stately/calendar/src/useRangeCalendarState.ts

//
// 1. No drag-to-select: react-aria supports `isDragging` state and drag
//    interaction for touch/mouse range selection. Not implemented.
//
// 2. No controlled state: react-aria uses `useControlledState` to support
//    both controlled and uncontrolled modes. We always create and own the
//    state internally (hook-owned state convention). Callers get read-only
//    signals and use mutation callbacks.
//
// 3. No time preservation: react-aria's `convertValue()` preserves
//    hour/minute/second from the original value. We use `OffsetDateTime`
//    directly without time preservation.
//
// 4. No calendar system conversion: react-aria converts dates to match the
//    original value's calendar system. We use `time::OffsetDateTime` only.
//
// 5. No multi-month visible duration or selection alignment.
//
// 6. No locale / i18n support.
//
// 7. Available range does not reactively narrow min/max on the inner
//    calendar state: during range selection, keyboard navigation can move
//    focus outside the available range. The actual selection is correctly
//    constrained when finalized.
//

/// A date range with optional start and end dates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateRange {
    /// The start of the range.
    pub start: Option<OffsetDateTime>,
    /// The end of the range.
    pub end: Option<OffsetDateTime>,
}

impl DateRange {
    /// Creates a new empty range.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            start: None,
            end: None,
        }
    }

    /// Creates a new range with start and end.
    #[must_use]
    pub fn new(start: OffsetDateTime, end: OffsetDateTime) -> Self {
        Self {
            start: Some(start),
            end: Some(end),
        }
    }

    /// Checks if a date is within this range.
    #[must_use]
    pub fn contains(&self, date: &OffsetDateTime) -> bool {
        match (self.start, self.end) {
            (Some(start), Some(end)) => date >= &start && date <= &end,
            (Some(start), None) => date >= &start,
            (None, Some(end)) => date <= &end,
            (None, None) => false,
        }
    }

    /// Checks if this range is complete (has both start and end).
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.start.is_some() && self.end.is_some()
    }
}

impl Default for DateRange {
    fn default() -> Self {
        Self::empty()
    }
}

/// Input parameters for the `use_range_calendar_state` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseRangeCalendarStateInput {
    /// The initial value for the range.
    pub default_value: Option<DateRange>,

    /// The minimum allowed date.
    pub min: Option<OffsetDateTime>,

    /// The maximum allowed date.
    pub max: Option<OffsetDateTime>,

    /// Whether the calendar is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the calendar is read-only.
    pub is_read_only: Signal<bool>,

    /// Callback to check if a specific date is unavailable.
    /// Unavailable dates are focusable but not selectable, unlike disabled dates
    /// which are out of the min/max range.
    pub is_date_unavailable: Option<Callback<OffsetDateTime, bool>>,

    /// Whether to allow ranges that span unavailable dates.
    /// When `false` (default), selecting an anchor date computes a contiguous
    /// available range around it, and the end date is constrained to that range.
    pub allows_non_contiguous_ranges: bool,

    /// Called when the selected range changes.
    pub on_change: Option<Callback<DateRange>>,

    /// Called when the focused date changes.
    pub on_focus_change: Option<Callback<OffsetDateTime>>,

    /// The initial focused date. Defaults to `default_value.start` or now.
    pub default_focused_value: Option<OffsetDateTime>,

    /// External validity signal (e.g. from form validation).
    pub is_invalid: Option<Signal<bool>>,

    /// The first day of the week. Defaults to Monday.
    pub first_day_of_week: time::Weekday,
}

impl Default for UseRangeCalendarStateInput {
    fn default() -> Self {
        Self {
            default_value: None,
            min: None,
            max: None,
            is_disabled: Signal::derive(|| false),
            is_read_only: Signal::derive(|| false),
            is_date_unavailable: None,
            allows_non_contiguous_ranges: false,
            on_change: None,
            on_focus_change: None,
            default_focused_value: None,
            is_invalid: None,
            first_day_of_week: time::Weekday::Monday,
        }
    }
}

/// The return value of the `use_range_calendar_state` hook.
#[derive(Clone, Copy)]
pub struct UseRangeCalendarStateReturn {
    /// The underlying calendar state for navigation and display.
    /// Use this for `focused_date`, `weeks`, `months`, `years`, navigation
    /// callbacks, and other single-calendar features.
    pub calendar: UseCalendarStateReturn,

    /// The currently selected date range.
    pub value: Signal<DateRange>,

    /// The anchor date (first click in range selection), if selection is in progress.
    pub anchor_date: Signal<Option<OffsetDateTime>>,

    /// The currently highlighted range.
    /// During selection: computed from `anchor_date` and `calendar.focused_date`.
    /// When not selecting: equals the saved `value`.
    pub highlighted_range: Signal<DateRange>,

    /// Whether the current value is invalid.
    pub is_value_invalid: Signal<bool>,

    /// Set the selected range directly.
    pub set_value: Callback<DateRange>,

    /// Select a date (handles both first click/anchor and second click/finalize).
    /// The date is constrained to min/max and the available range, and checked
    /// for unavailability before selection.
    pub select_date: Callback<OffsetDateTime>,

    /// Select the currently focused date (for keyboard Enter/Space).
    pub select_focused_date: Callback<()>,

    /// Highlight a date during range selection.
    /// Moves the focused date when an anchor is set, causing the highlighted
    /// range to update. No-op when no anchor is set.
    pub highlight_date: Callback<OffsetDateTime>,

    /// Set or clear the anchor date.
    /// Setting to `None` cancels the current range selection.
    pub set_anchor_date: Callback<Option<OffsetDateTime>>,

    /// Clear the selection (value and anchor).
    pub clear: Callback<()>,

    /// Check if a date is within the highlighted range and not disabled/unavailable.
    pub is_selected: Callback<OffsetDateTime, bool>,
}

/// The contiguous available range around the anchor date.
#[derive(Debug, Clone, Copy)]
struct AvailableRange {
    /// The earliest available date (inclusive), or `None` for no lower bound.
    start: Option<OffsetDateTime>,
    /// The latest available date (inclusive), or `None` for no upper bound.
    end: Option<OffsetDateTime>,
}

/// Creates a `DateRange` from two dates, auto-swapping if needed so start <= end.
fn make_range(a: OffsetDateTime, b: OffsetDateTime) -> DateRange {
    if a <= b {
        DateRange::new(a, b)
    } else {
        DateRange::new(b, a)
    }
}

/// Walks from `anchor` in `dir` (±1 day steps) until finding an unavailable date.
/// Returns the last available date before the unavailable one, or `None` if no
/// unavailable date is found within bounds.
fn next_unavailable_date(
    anchor: OffsetDateTime,
    min: Option<OffsetDateTime>,
    max: Option<OffsetDateTime>,
    is_unavailable: impl Fn(OffsetDateTime) -> bool,
    dir: i64,
) -> Option<OffsetDateTime> {
    // Use min/max as search bounds, defaulting to ±1 year from anchor.
    let bound = if dir < 0 {
        min.unwrap_or(anchor - time::Duration::days(366))
    } else {
        max.unwrap_or(anchor + time::Duration::days(366))
    };

    let mut next = anchor + time::Duration::days(dir);
    while (if dir < 0 {
        next >= bound
    } else {
        next <= bound
    }) && !is_unavailable(next)
    {
        next += time::Duration::days(dir);
    }

    if is_unavailable(next) {
        // Found an unavailable date; the boundary is one step back.
        Some(next - time::Duration::days(dir))
    } else {
        // No unavailable date found within bounds — no constraint.
        None
    }
}

/// Walks backward from `date` until finding a date that is not unavailable.
/// Returns `None` if no available date is found before `min_bound`.
fn previous_available_date(
    date: OffsetDateTime,
    min_bound: OffsetDateTime,
    is_unavailable: impl Fn(OffsetDateTime) -> bool,
) -> Option<OffsetDateTime> {
    let mut d = date;
    while d >= min_bound && is_unavailable(d) {
        d -= time::Duration::days(1);
    }
    if d >= min_bound { Some(d) } else { None }
}

/// Clamp a date to the intersection of [min, max] and the available range.
fn constrain_to_range(
    date: OffsetDateTime,
    min: Option<OffsetDateTime>,
    max: Option<OffsetDateTime>,
    available: Option<AvailableRange>,
) -> OffsetDateTime {
    let mut d = date;

    // Apply available range constraint (narrower).
    if let Some(avail) = available {
        if let Some(start) = avail.start {
            d = d.max(start);
        }
        if let Some(end) = avail.end {
            d = d.min(end);
        }
    }

    // Apply min/max constraint.
    if let Some(m) = min {
        d = d.max(m);
    }
    if let Some(m) = max {
        d = d.min(m);
    }
    d
}

/// Creates state for a range calendar.
///
/// Wraps [`use_calendar_state`](super::use_calendar_state::use_calendar_state)
/// and adds range-specific behavior: anchor tracking, highlighted range
/// computation, unavailable date enforcement, and validation.
///
/// # Example
///
/// ```ignore
/// let state = use_range_calendar_state(UseRangeCalendarStateInput {
///     on_change: Some(Callback::new(|range| {
///         tracing::debug!("Selected range: {:?}", range);
///     })),
///     ..Default::default()
/// });
///
/// // Access navigation/display via state.calendar
/// let weeks = state.calendar.weeks;
/// let focused_year = state.calendar.focused_year;
///
/// // Access range-specific state
/// let highlighted = state.highlighted_range;
/// let is_selecting = Signal::derive(move || state.anchor_date.get().is_some());
/// ```
#[allow(clippy::too_many_lines)]
pub fn use_range_calendar_state(input: UseRangeCalendarStateInput) -> UseRangeCalendarStateReturn {
    let UseRangeCalendarStateInput {
        default_value,
        min,
        max,
        is_disabled,
        is_read_only,
        is_date_unavailable,
        allows_non_contiguous_ranges,
        on_change,
        on_focus_change,
        default_focused_value,
        is_invalid,
        first_day_of_week,
    } = input;

    // Create the underlying calendar state for navigation/display.
    // The start of the range is passed as the calendar's selected value.
    // We don't wire on_change — the range state manages change notification.
    let calendar = super::use_calendar_state::use_calendar_state(UseCalendarStateInput {
        default_value: default_value.and_then(|r| r.start),
        min,
        max,
        is_disabled,
        is_read_only,
        is_date_unavailable,
        on_change: None,
        on_focus_change,
        default_focused_value: default_focused_value.or(default_value.and_then(|r| r.start)),
        is_invalid: None,
        first_day_of_week,
    });

    // --- Range-specific state ---

    let (value, set_value_signal) = signal(default_value.unwrap_or_default());
    let (anchor_date, set_anchor_date_signal) = signal::<Option<OffsetDateTime>>(None);
    let (available_range, set_available_range) = signal::<Option<AvailableRange>>(None);

    // --- Highlighted range ---
    // Key architectural choice: the highlighted range is computed from
    // anchor + focused_date (matching react-aria), not from a separate signal.
    let focused_date = calendar.focused_date;
    let highlighted_range = Signal::derive(move || {
        if let Some(anchor) = anchor_date.get() {
            make_range(anchor, focused_date.get())
        } else {
            value.get()
        }
    });

    // --- Available range computation ---
    // When `is_date_unavailable` is set and `allows_non_contiguous_ranges` is false,
    // compute the contiguous available range around the anchor.
    let update_available_range = move |date: Option<OffsetDateTime>| {
        if let (Some(date), Some(is_unavailable)) = (date, is_date_unavailable) {
            if !allows_non_contiguous_ranges {
                let start_bound =
                    next_unavailable_date(date, min, max, |d| is_unavailable.run(d), -1);
                let end_bound = next_unavailable_date(date, min, max, |d| is_unavailable.run(d), 1);
                set_available_range.set(Some(AvailableRange {
                    start: start_bound,
                    end: end_bound,
                }));
                return;
            }
        }
        set_available_range.set(None);
    };

    // --- Selection logic ---

    let do_select = move |date: OffsetDateTime| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }

        let constrained = constrain_to_range(date, min, max, available_range.get_untracked());

        let available_date = if let Some(is_unavailable) = is_date_unavailable {
            let min_bound = min.unwrap_or(constrained - time::Duration::days(366));
            match previous_available_date(constrained, min_bound, |d| is_unavailable.run(d)) {
                Some(d) => d,
                None => return,
            }
        } else {
            constrained
        };

        let current_anchor = anchor_date.get_untracked();
        if let Some(anchor) = current_anchor {
            // Second click — complete the range.
            let range = make_range(anchor, available_date);
            set_value_signal.set(range);
            set_anchor_date_signal.set(None);
            set_available_range.set(None);

            if let Some(on_change) = on_change {
                on_change.run(range);
            }
        } else {
            // First click — set anchor.
            set_anchor_date_signal.set(Some(available_date));
            update_available_range(Some(available_date));
        }
    };

    // --- Callbacks ---

    let select_date_cb = Callback::new(move |date: OffsetDateTime| {
        do_select(date);
    });

    let select_focused_date_cb = Callback::new(move |()| {
        let focused = calendar.focused_date.get_untracked();
        do_select(focused);
    });

    let highlight_date_cb = Callback::new(move |date: OffsetDateTime| {
        if anchor_date.get_untracked().is_some() {
            calendar.set_focused_date.run(date);
        }
    });

    let set_anchor_date_cb = Callback::new(move |date: Option<OffsetDateTime>| {
        set_anchor_date_signal.set(date);
        update_available_range(date);
    });

    let set_value_cb = Callback::new(move |range: DateRange| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }
        set_value_signal.set(range);
        if let Some(on_change) = on_change {
            on_change.run(range);
        }
    });

    let clear_cb = Callback::new(move |()| {
        set_value_signal.set(DateRange::empty());
        set_anchor_date_signal.set(None);
        set_available_range.set(None);
    });

    // --- Validation ---

    let is_value_invalid = Signal::derive(move || {
        // External invalid signal.
        if let Some(is_invalid) = is_invalid {
            if is_invalid.get() {
                return true;
            }
        }

        // Don't validate during active selection.
        if anchor_date.get().is_some() {
            return false;
        }

        let val = value.get();

        // Check start endpoint.
        if let Some(start) = val.start {
            if !is_in_range(&start, min.as_ref(), max.as_ref()) {
                return true;
            }
            if let Some(is_unavailable) = is_date_unavailable {
                if is_unavailable.run(start) {
                    return true;
                }
            }
        }

        // Check end endpoint.
        if let Some(end) = val.end {
            if !is_in_range(&end, min.as_ref(), max.as_ref()) {
                return true;
            }
            if let Some(is_unavailable) = is_date_unavailable {
                if is_unavailable.run(end) {
                    return true;
                }
            }
        }

        false
    });

    // --- Query ---

    let is_cell_disabled = calendar.is_cell_disabled;
    let is_cell_unavailable = calendar.is_cell_unavailable;
    let is_selected_cb = Callback::new(move |date: OffsetDateTime| -> bool {
        let range = highlighted_range.get();
        range.contains(&date) && !is_cell_disabled.run(date) && !is_cell_unavailable.run(date)
    });

    UseRangeCalendarStateReturn {
        calendar,
        value: value.into(),
        anchor_date: anchor_date.into(),
        highlighted_range,
        is_value_invalid,
        set_value: set_value_cb,
        select_date: select_date_cb,
        select_focused_date: select_focused_date_cb,
        highlight_date: highlight_date_cb,
        set_anchor_date: set_anchor_date_cb,
        clear: clear_cb,
        is_selected: is_selected_cb,
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;
    use time::macros::datetime;

    use super::*;

    #[test]
    fn make_range_auto_swaps_when_end_before_start() {
        let a = datetime!(2026-03-20 0:00 UTC);
        let b = datetime!(2026-03-10 0:00 UTC);
        let range = make_range(a, b);
        assert_that(range.start).is_equal_to(Some(b));
        assert_that(range.end).is_equal_to(Some(a));
    }

    #[test]
    fn make_range_preserves_order_when_start_before_end() {
        let a = datetime!(2026-03-10 0:00 UTC);
        let b = datetime!(2026-03-20 0:00 UTC);
        let range = make_range(a, b);
        assert_that(range.start).is_equal_to(Some(a));
        assert_that(range.end).is_equal_to(Some(b));
    }

    #[test]
    fn make_range_handles_same_date() {
        let a = datetime!(2026-03-15 0:00 UTC);
        let range = make_range(a, a);
        assert_that(range.start).is_equal_to(Some(a));
        assert_that(range.end).is_equal_to(Some(a));
    }

    #[test]
    fn next_unavailable_date_finds_boundary_forward() {
        let anchor = datetime!(2026-03-10 0:00 UTC);
        // March 15 is unavailable.
        let result = next_unavailable_date(
            anchor,
            None,
            Some(datetime!(2026-03-31 0:00 UTC)),
            |d| d.day() == 15 && d.month() == time::Month::March,
            1,
        );
        // Should return March 14 (last available date before March 15).
        assert_that(result).is_equal_to(Some(datetime!(2026-03-14 0:00 UTC)));
    }

    #[test]
    fn next_unavailable_date_finds_boundary_backward() {
        let anchor = datetime!(2026-03-20 0:00 UTC);
        // March 15 is unavailable.
        let result = next_unavailable_date(
            anchor,
            Some(datetime!(2026-03-01 0:00 UTC)),
            None,
            |d| d.day() == 15 && d.month() == time::Month::March,
            -1,
        );
        // Should return March 16 (last available date after March 15, walking backward).
        assert_that(result).is_equal_to(Some(datetime!(2026-03-16 0:00 UTC)));
    }

    #[test]
    fn next_unavailable_date_returns_none_when_no_unavailable() {
        let anchor = datetime!(2026-03-10 0:00 UTC);
        let result = next_unavailable_date(
            anchor,
            None,
            Some(datetime!(2026-03-20 0:00 UTC)),
            |_| false,
            1,
        );
        assert_that(result).is_equal_to(None);
    }

    #[test]
    fn previous_available_date_finds_nearest() {
        let date = datetime!(2026-03-15 0:00 UTC);
        let min_bound = datetime!(2026-03-01 0:00 UTC);
        // March 15 and 14 are unavailable.
        let result = previous_available_date(date, min_bound, |d| d.day() == 15 || d.day() == 14);
        assert_that(result).is_equal_to(Some(datetime!(2026-03-13 0:00 UTC)));
    }

    #[test]
    fn previous_available_date_returns_none_when_all_unavailable() {
        let date = datetime!(2026-03-03 0:00 UTC);
        let min_bound = datetime!(2026-03-01 0:00 UTC);
        let result = previous_available_date(date, min_bound, |_| true);
        assert_that(result).is_equal_to(None);
    }

    #[test]
    fn previous_available_date_returns_date_when_available() {
        let date = datetime!(2026-03-15 0:00 UTC);
        let min_bound = datetime!(2026-03-01 0:00 UTC);
        let result = previous_available_date(date, min_bound, |_| false);
        assert_that(result).is_equal_to(Some(date));
    }

    #[test]
    fn constrain_to_range_clamps_to_min_max() {
        let date = datetime!(2026-03-25 0:00 UTC);
        let min = Some(datetime!(2026-03-10 0:00 UTC));
        let max = Some(datetime!(2026-03-20 0:00 UTC));
        let result = constrain_to_range(date, min, max, None);
        assert_that(result).is_equal_to(datetime!(2026-03-20 0:00 UTC));
    }

    #[test]
    fn constrain_to_range_clamps_to_available_range() {
        let date = datetime!(2026-03-25 0:00 UTC);
        let avail = Some(AvailableRange {
            start: Some(datetime!(2026-03-12 0:00 UTC)),
            end: Some(datetime!(2026-03-18 0:00 UTC)),
        });
        let result = constrain_to_range(date, None, None, avail);
        assert_that(result).is_equal_to(datetime!(2026-03-18 0:00 UTC));
    }

    #[test]
    fn constrain_to_range_applies_both_constraints() {
        let date = datetime!(2026-03-05 0:00 UTC);
        let min = Some(datetime!(2026-03-01 0:00 UTC));
        let max = Some(datetime!(2026-03-31 0:00 UTC));
        let avail = Some(AvailableRange {
            start: Some(datetime!(2026-03-10 0:00 UTC)),
            end: Some(datetime!(2026-03-20 0:00 UTC)),
        });
        // date < avail.start, so clamped up to March 10.
        let result = constrain_to_range(date, min, max, avail);
        assert_that(result).is_equal_to(datetime!(2026-03-10 0:00 UTC));
    }

    #[test]
    fn date_range_contains_inclusive_bounds() {
        let range = DateRange::new(
            datetime!(2026-03-10 0:00 UTC),
            datetime!(2026-03-20 0:00 UTC),
        );
        assert_that(range.contains(&datetime!(2026-03-15 0:00 UTC))).is_true();
        assert_that(range.contains(&datetime!(2026-03-10 0:00 UTC))).is_true();
        assert_that(range.contains(&datetime!(2026-03-20 0:00 UTC))).is_true();
        assert_that(range.contains(&datetime!(2026-03-09 0:00 UTC))).is_false();
        assert_that(range.contains(&datetime!(2026-03-21 0:00 UTC))).is_false();
    }

    #[test]
    fn date_range_empty_contains_nothing() {
        let range = DateRange::empty();
        assert_that(range.contains(&datetime!(2026-03-15 0:00 UTC))).is_false();
    }
}
