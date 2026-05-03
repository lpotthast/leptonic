use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use uuid::Uuid;
use web_sys::KeyboardEvent;

use super::use_range_calendar_state::{
    DateRange, UseRangeCalendarStateInput, UseRangeCalendarStateReturn, use_range_calendar_state,
};
use crate::{
    hooks::IntoAttrs,
    utils::{
        EventHandler,
        aria::{AriaDisabled, AriaRole},
    },
};

// This is based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/calendar/src/useRangeCalendar.ts

//
// 1. No blur-to-finalize: react-aria finalizes range selection (calls
//    `selectFocusedDate`) when focus leaves the calendar. Not implemented
//    because the grid's focus/blur events don't reliably track subtree focus.
//    The `from_range_calendar_state` method on `UseCalendarGridInput` does
//    include a best-effort blur handler.
//
// 2. No VoiceOver virtual click workaround: react-aria filters virtual
//    pointer events from VoiceOver to prevent conflicts with `usePress`.
//    Not needed since we don't use `usePress` in calendar cells.
//
// 3. No touch scroll prevention: react-aria prevents touch scrolling via
//    `touchmove` during drag interaction. Not needed since we don't support
//    drag-to-select.
//

// Re-export for convenience.
pub use super::use_range_calendar_state::DateRange as RangeDateRange;

/// Input parameters for the `use_range_calendar` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseRangeCalendarInput {
    /// The initial value for the range.
    pub default_value: Option<DateRange>,

    /// The minimum allowed date.
    pub min: Option<time::OffsetDateTime>,

    /// The maximum allowed date.
    pub max: Option<time::OffsetDateTime>,

    /// Whether the calendar is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the calendar is read-only.
    pub is_read_only: Signal<bool>,

    /// Callback to check if a specific date is unavailable.
    pub is_date_unavailable: Option<Callback<time::OffsetDateTime, bool>>,

    /// Whether to allow ranges spanning unavailable dates.
    pub allows_non_contiguous_ranges: bool,

    /// Callback when the range changes.
    pub on_change: Option<Callback<DateRange>>,

    /// Callback when the focused date changes.
    pub on_focus_change: Option<Callback<time::OffsetDateTime>>,

    /// The initial focused date.
    pub default_focused_value: Option<time::OffsetDateTime>,

    /// External validity signal.
    pub is_invalid: Option<Signal<bool>>,

    /// The first day of the week. Defaults to Monday.
    pub first_day_of_week: time::Weekday,
}

impl Default for UseRangeCalendarInput {
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

/// The return value of the `use_range_calendar` hook.
pub struct UseRangeCalendarReturn {
    /// Props for the calendar container. Call `.into_attrs()` for view spreading.
    pub calendar_props: UseRangeCalendarProps,

    /// The range calendar state. Access navigation/display via `state.calendar`
    /// and range-specific features directly on `state`.
    pub state: UseRangeCalendarStateReturn,

    /// The ID of the calendar container.
    pub calendar_id: String,
}

/// Props from `use_range_calendar` for the calendar container.
#[derive(Debug)]
pub struct UseRangeCalendarProps {
    pub id: String,
    pub role: AriaRole,
    pub aria_label: &'static str,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub on_keydown: EventHandler<KeyboardEvent>,
}

impl IntoAttrs for UseRangeCalendarProps {
    type Attrs = UseRangeCalendarAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaDisabled, self.aria_disabled),
            self.on_keydown.into_on(ev::keydown),
        )
    }
}

/// Attributes for the range calendar container.
pub type UseRangeCalendarAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaLabel, &'static str>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
);

/// Provides the behavior and accessibility for a range calendar.
///
/// A range calendar allows selecting a date range (start and end dates).
/// This hook creates the range calendar state internally and provides
/// ARIA props for the container element.
///
/// For the grid and cells, use
/// [`UseCalendarGridInput::from_range_calendar_state`](super::use_calendar_grid::UseCalendarGridInput::from_range_calendar_state)
/// and [`use_calendar_cell`](super::use_calendar_cell::use_calendar_cell).
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
/// let state = range_calendar.state;
///
/// view! {
///     <div {..range_calendar.calendar_props.into_attrs()}>
///         <div class="calendar-header">
///             <button on:click=move |_| state.calendar.focus_previous_page.run(())>"<"</button>
///             <span>{move || state.calendar.focused_month_name.get()}</span>
///             <span>{move || state.calendar.focused_year.get()}</span>
///             <button on:click=move |_| state.calendar.focus_next_page.run(())>">"</button>
///         </div>
///         // Render grid with weeks from state.calendar.weeks ...
///     </div>
/// }
/// ```
pub fn use_range_calendar(input: UseRangeCalendarInput) -> UseRangeCalendarReturn {
    let is_disabled = input.is_disabled;

    let state = use_range_calendar_state(UseRangeCalendarStateInput {
        default_value: input.default_value,
        min: input.min,
        max: input.max,
        is_disabled: input.is_disabled,
        is_read_only: input.is_read_only,
        is_date_unavailable: input.is_date_unavailable,
        allows_non_contiguous_ranges: input.allows_non_contiguous_ranges,
        on_change: input.on_change,
        on_focus_change: input.on_focus_change,
        default_focused_value: input.default_focused_value,
        is_invalid: input.is_invalid,
        first_day_of_week: input.first_day_of_week,
    });

    let calendar_id = format!("range-calendar-{}", Uuid::new_v4());

    let aria_disabled = Signal::derive(move || is_disabled.get().then_some(AriaDisabled::True));

    // Escape cancels the current range selection.
    let anchor_date = state.anchor_date;
    let set_anchor_date = state.set_anchor_date;
    let handle_keydown = move |e: KeyboardEvent| {
        if is_disabled.get_untracked() {
            return;
        }
        if e.key().as_str() == "Escape" && anchor_date.get_untracked().is_some() {
            e.prevent_default();
            set_anchor_date.run(None);
        }
    };

    UseRangeCalendarReturn {
        calendar_props: UseRangeCalendarProps {
            id: calendar_id.clone(),
            role: AriaRole::Application,
            aria_label: "Date range picker",
            aria_disabled,
            on_keydown: EventHandler::new(handle_keydown),
        },
        state,
        calendar_id,
    }
}
