use leptos::attr;
use leptos::attr::{Attr, Attribute};
use leptos::ev;
use leptos::ev::{on, On, SharedEventCallback};
use leptos::prelude::*;
use uuid::Uuid;
use web_sys::KeyboardEvent;

use super::use_calendar::create_weeks;
use crate::utils::time::{start_of_next_month, start_of_previous_month, Day, Week};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/calendar/src/useRangeCalendar.ts

/// A date range with start and end dates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateRange {
    /// The start of the range.
    pub start: Option<time::OffsetDateTime>,
    /// The end of the range.
    pub end: Option<time::OffsetDateTime>,
}

impl DateRange {
    /// Creates a new empty range.
    pub fn empty() -> Self {
        Self {
            start: None,
            end: None,
        }
    }

    /// Creates a new range with start and end.
    pub fn new(start: time::OffsetDateTime, end: time::OffsetDateTime) -> Self {
        Self {
            start: Some(start),
            end: Some(end),
        }
    }

    /// Checks if a date is within this range.
    pub fn contains(&self, date: &time::OffsetDateTime) -> bool {
        match (self.start, self.end) {
            (Some(start), Some(end)) => date >= &start && date <= &end,
            (Some(start), None) => date >= &start,
            (None, Some(end)) => date <= &end,
            (None, None) => false,
        }
    }

    /// Checks if this range is complete (has both start and end).
    pub fn is_complete(&self) -> bool {
        self.start.is_some() && self.end.is_some()
    }
}

impl Default for DateRange {
    fn default() -> Self {
        Self::empty()
    }
}

/// Input parameters for the `use_range_calendar` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseRangeCalendarInput {
    /// The initial value for the range (optional).
    pub value: Option<DateRange>,

    /// The minimum allowed date.
    pub min: Option<time::OffsetDateTime>,

    /// The maximum allowed date.
    pub max: Option<time::OffsetDateTime>,

    /// Whether the calendar is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the calendar is read-only.
    pub is_read_only: Signal<bool>,

    /// Callback when the range changes.
    pub on_change: Option<Callback<DateRange>>,
}

impl Default for UseRangeCalendarInput {
    fn default() -> Self {
        Self {
            value: None,
            min: None,
            max: None,
            is_disabled: Signal::derive(|| false),
            is_read_only: Signal::derive(|| false),
            on_change: None,
        }
    }
}

/// The return value of the `use_range_calendar` hook.
#[derive(Debug, Clone)]
pub struct UseRangeCalendarReturn {
    /// Props for the calendar container.
    pub calendar_props: UseRangeCalendarAttrs,

    /// The current focused/staging date.
    pub staging: ReadSignal<time::OffsetDateTime>,

    /// The year of the staging date.
    pub staging_year: Memo<i32>,

    /// The month name of the staging date.
    pub staging_month_name: Memo<String>,

    /// The current selected range.
    pub value: Signal<DateRange>,

    /// The currently highlighted range (during selection).
    pub highlighted_range: Signal<DateRange>,

    /// The weeks to display.
    pub weeks: Signal<Vec<Week>>,

    /// The ID of the calendar.
    pub calendar_id: String,

    /// Whether we're currently selecting the start or end of the range.
    pub anchor_date: Signal<Option<time::OffsetDateTime>>,

    /// Navigate to the previous month.
    pub previous_month: Callback<()>,

    /// Navigate to the next month.
    pub next_month: Callback<()>,

    /// Select a date (for range selection).
    pub select_date: Callback<Day>,

    /// Set the highlighted date (for hover effects).
    pub set_highlighted: Callback<Option<time::OffsetDateTime>>,
}

/// Attributes for the range calendar container.
pub type UseRangeCalendarAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaLabel, &'static str>,
    Attr<attr::AriaDisabled, Signal<&'static str>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
);

/// Provides the behavior and accessibility for a range calendar.
///
/// A range calendar allows selecting a date range (start and end dates).
///
/// # Example
///
/// ```ignore
/// let range_calendar = use_range_calendar(UseRangeCalendarInput {
///     on_change: Some(Callback::new(|range| {
///         tracing::debug!("Selected range: {:?}", range);
///     })),
///     ..Default::default()
/// });
///
/// view! {
///     <div {..range_calendar.calendar_props}>
///         <div class="calendar-header">
///             <button on:click=move |_| range_calendar.previous_month.run(())>"<"</button>
///             <span>{move || range_calendar.staging_month_name.get()}</span>
///             <span>{move || range_calendar.staging_year.get()}</span>
///             <button on:click=move |_| range_calendar.next_month.run(())>">"</button>
///         </div>
///         // Render grid with weeks...
///     </div>
/// }
/// ```
///
/// # Panics
///
/// Panics if the `on` event handler cannot be converted to a cloneable callback.
#[allow(clippy::too_many_lines)]
pub fn use_range_calendar(input: UseRangeCalendarInput) -> UseRangeCalendarReturn {
    let calendar_id = format!("range-calendar-{}", Uuid::new_v4());
    let is_disabled = input.is_disabled;
    let is_read_only = input.is_read_only;
    let on_change = input.on_change;
    let min = input.min;
    let max = input.max;

    // Initialize staging date
    let initial_staging = input
        .value
        .and_then(|r| r.start)
        .unwrap_or_else(time::OffsetDateTime::now_utc);
    let (staging, set_staging) = signal(initial_staging);

    // The selected range
    let (value, set_value) = signal(input.value.unwrap_or_default());

    // The anchor date (first click in range selection)
    let (anchor_date, set_anchor_date) = signal::<Option<time::OffsetDateTime>>(None);

    // The currently highlighted date (for hover effects)
    let (highlighted_date, set_highlighted_date) = signal::<Option<time::OffsetDateTime>>(None);

    // Compute the highlighted range based on anchor and highlighted dates
    let highlighted_range =
        Signal::derive(move || match (anchor_date.get(), highlighted_date.get()) {
            (Some(anchor), Some(highlighted)) => {
                if anchor <= highlighted {
                    DateRange::new(anchor, highlighted)
                } else {
                    DateRange::new(highlighted, anchor)
                }
            }
            (Some(anchor), None) => DateRange {
                start: Some(anchor),
                end: None,
            },
            _ => DateRange::empty(),
        });

    // Derived signals
    let staging_year = Memo::new(move |_| staging.get().year());
    let staging_month_name = Memo::new(move |_| staging.get().month().to_string());

    let weeks = Signal::derive(move || create_weeks(&staging.get(), min.as_ref(), max.as_ref()));

    // Compute aria-disabled
    let aria_disabled = Signal::derive(move || if is_disabled.get() { "true" } else { "false" });

    // Navigation callbacks
    let previous_month = Callback::new(move |_| {
        set_staging.update(|s| *s = start_of_previous_month(*s));
    });

    let next_month = Callback::new(move |_| {
        set_staging.update(|s| *s = start_of_next_month(*s));
    });

    // Select a date for range
    let select_date = Callback::new(move |day: Day| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() || day.disabled {
            return;
        }

        let current_anchor = anchor_date.get_untracked();

        if let Some(anchor) = current_anchor {
            // Second click - complete the range
            let (start, end) = if anchor <= day.date_time {
                (anchor, day.date_time)
            } else {
                (day.date_time, anchor)
            };

            let new_range = DateRange::new(start, end);
            set_value.set(new_range);
            set_anchor_date.set(None);
            set_highlighted_date.set(None);

            if let Some(on_change) = on_change {
                on_change.run(new_range);
            }
        } else {
            // First click - set anchor
            set_anchor_date.set(Some(day.date_time));
        }
    });

    // Set highlighted date (for hover)
    let set_highlighted = Callback::new(move |date: Option<time::OffsetDateTime>| {
        set_highlighted_date.set(date);
    });

    // Handle keyboard navigation
    let handle_keydown = move |e: KeyboardEvent| {
        if is_disabled.get_untracked() {
            return;
        }

        let key = e.key();
        if key.as_str() == "Escape" {
            // Cancel current selection
            if anchor_date.get_untracked().is_some() {
                e.prevent_default();
                set_anchor_date.set(None);
                set_highlighted_date.set(None);
            }
        }
    };

    UseRangeCalendarReturn {
        calendar_props: (
            Attr(attr::Id, calendar_id.clone()),
            Attr(attr::Role, "application"),
            Attr(attr::AriaLabel, "Date range picker"),
            Attr(attr::AriaDisabled, aria_disabled),
            on(ev::keydown, handle_keydown).into_cloneable(),
        ),
        staging,
        staging_year,
        staging_month_name,
        value: value.into(),
        highlighted_range,
        weeks,
        calendar_id,
        anchor_date: anchor_date.into(),
        previous_month,
        next_month,
        select_date,
        set_highlighted,
    }
}

/// State for managing range calendar selection.
#[derive(Clone, Copy)]
pub struct UseRangeCalendarStateReturn {
    /// The current selected range.
    pub value: Signal<DateRange>,

    /// Set the selected range.
    pub set_value: Callback<DateRange>,

    /// Clear the selection.
    pub clear: Callback<()>,
}

/// Creates internal state for a range calendar.
pub fn use_range_calendar_state(default_value: Option<DateRange>) -> UseRangeCalendarStateReturn {
    let (value, set_value) = signal(default_value.unwrap_or_default());

    UseRangeCalendarStateReturn {
        value: value.into(),
        set_value: Callback::new(move |range| set_value.set(range)),
        clear: Callback::new(move |_| set_value.set(DateRange::empty())),
    }
}
