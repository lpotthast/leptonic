use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use uuid::Uuid;
use web_sys::{FocusEvent, KeyboardEvent};

use crate::{
    hooks::IntoAttrs,
    utils::{
        EventHandler,
        aria::{AriaDisabled, AriaHidden, AriaMultiselectable, AriaReadonly, AriaRole},
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/calendar/src/useCalendarGrid.ts

//
// 1. No RTL support: react-aria swaps ArrowLeft/ArrowRight based on
//    `useLocale().direction`. We always assume LTR because leptonic does not
//    yet have a locale/direction system.
//
// 2. Callback-based navigation instead of state object: react-aria receives a
//    `CalendarState | RangeCalendarState` object and calls methods like
//    `state.focusPreviousDay()`. We accept individual `Callback` fields so the
//    hook stays decoupled from any specific state implementation.
//
// 3. No locale-aware weekday formatting: react-aria uses `useDateFormatter`
//    with a `weekdayStyle` prop. We accept pre-formatted `weekday_labels`.
//
// 4. No `startDate`/`endDate` props for multi-grid calendars: react-aria
//    supports displaying multiple months by passing different date ranges to
//    each grid. We assume a single grid per calendar.
//
// 5. No `weeksInMonth` return: react-aria computes `getWeeksInMonth(...)` and
//    returns it. Our week data is computed in the state hook instead.
//
// 6. No `aria-label` / `aria-labelledby` with visible range description:
//    react-aria computes a label from the visible date range. We accept an
//    optional `aria_label` string directly.
//

/// Input parameters for the `use_calendar_grid` hook.
#[derive(Debug, Clone)]
pub struct UseCalendarGridInput {
    /// Whether the calendar is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the calendar is read-only.
    pub is_read_only: Signal<bool>,

    /// The start day of the week (0 = Monday, 6 = Sunday).
    pub start_of_week: u8,

    /// Labels for the days of the week.
    pub weekday_labels: Vec<String>,

    /// An accessible label for the grid.
    pub aria_label: Option<String>,

    /// Whether the grid supports multi-selection (for range calendars).
    pub is_range: bool,

    // --- Keyboard navigation callbacks (called from grid-level keydown) ---
    /// Called on `Enter`/`Space` to select the currently focused date.
    pub on_select_focused_date: Option<Callback<()>>,

    /// Called on `ArrowLeft` to move focus to the previous day.
    pub on_focus_previous_day: Option<Callback<()>>,

    /// Called on `ArrowRight` to move focus to the next day.
    pub on_focus_next_day: Option<Callback<()>>,

    /// Called on `ArrowUp` to move focus to the previous week (row).
    pub on_focus_previous_week: Option<Callback<()>>,

    /// Called on `ArrowDown` to move focus to the next week (row).
    pub on_focus_next_week: Option<Callback<()>>,

    /// Called on `PageUp` to move focus to the previous section.
    /// The bool argument is `true` when Shift is held (year-level navigation).
    pub on_focus_previous_section: Option<Callback<bool>>,

    /// Called on `PageDown` to move focus to the next section.
    /// The bool argument is `true` when Shift is held (year-level navigation).
    pub on_focus_next_section: Option<Callback<bool>>,

    /// Called on `Home` to move focus to the start of the current section.
    pub on_focus_section_start: Option<Callback<()>>,

    /// Called on `End` to move focus to the end of the current section.
    pub on_focus_section_end: Option<Callback<()>>,

    /// Called on `Escape` to cancel range selection (range calendars only).
    pub on_cancel_selection: Option<Callback<()>>,

    // --- Focus callbacks ---
    /// Called when the grid receives focus.
    pub on_focus: Option<Callback<()>>,

    /// Called when the grid loses focus.
    pub on_blur: Option<Callback<()>>,
}

impl UseCalendarGridInput {
    /// Create a `UseCalendarGridInput` wired to a `UseCalendarStateReturn`.
    ///
    /// This convenience method connects all navigation callbacks from the grid
    /// to the corresponding methods on the calendar state.
    #[must_use]
    pub fn from_calendar_state(state: super::use_calendar_state::UseCalendarStateReturn) -> Self {
        Self {
            is_disabled: state.is_disabled,
            is_read_only: state.is_read_only,
            on_select_focused_date: Some(state.select_focused_date),
            on_focus_previous_day: Some(state.focus_previous_day),
            on_focus_next_day: Some(state.focus_next_day),
            on_focus_previous_week: Some(state.focus_previous_row),
            on_focus_next_week: Some(state.focus_next_row),
            on_focus_previous_section: Some(state.focus_previous_section),
            on_focus_next_section: Some(state.focus_next_section),
            on_focus_section_start: Some(state.focus_section_start),
            on_focus_section_end: Some(state.focus_section_end),
            on_focus: Some(Callback::new(move |()| state.set_focused.run(true))),
            on_blur: Some(Callback::new(move |()| state.set_focused.run(false))),
            ..Default::default()
        }
    }

    /// Create a `UseCalendarGridInput` wired to a `UseRangeCalendarStateReturn`.
    ///
    /// This connects navigation callbacks from the underlying calendar state,
    /// and wires range-specific behaviors:
    /// - `select_focused_date` uses the range state's version (handles anchor/finalize)
    /// - `cancel_selection` clears the anchor date on Escape
    /// - `on_blur` finalizes the selection if an anchor is set
    #[must_use]
    pub fn from_range_calendar_state(
        state: super::use_range_calendar_state::UseRangeCalendarStateReturn,
    ) -> Self {
        let cal = state.calendar;
        let select_focused = state.select_focused_date;
        let set_anchor = state.set_anchor_date;
        let anchor_date = state.anchor_date;

        Self {
            is_disabled: cal.is_disabled,
            is_read_only: cal.is_read_only,
            is_range: true,
            on_select_focused_date: Some(select_focused),
            on_focus_previous_day: Some(cal.focus_previous_day),
            on_focus_next_day: Some(cal.focus_next_day),
            on_focus_previous_week: Some(cal.focus_previous_row),
            on_focus_next_week: Some(cal.focus_next_row),
            on_focus_previous_section: Some(cal.focus_previous_section),
            on_focus_next_section: Some(cal.focus_next_section),
            on_focus_section_start: Some(cal.focus_section_start),
            on_focus_section_end: Some(cal.focus_section_end),
            on_cancel_selection: Some(Callback::new(move |()| {
                set_anchor.run(None);
            })),
            on_focus: Some(Callback::new(move |()| cal.set_focused.run(true))),
            on_blur: Some(Callback::new(move |()| {
                // Finalize selection when focus leaves the grid.
                if anchor_date.get_untracked().is_some() {
                    select_focused.run(());
                }
                cal.set_focused.run(false);
            })),
            ..Default::default()
        }
    }
}

impl Default for UseCalendarGridInput {
    fn default() -> Self {
        Self {
            is_disabled: Signal::derive(|| false),
            is_read_only: Signal::derive(|| false),
            start_of_week: 0, // Monday
            weekday_labels: vec![
                "Mon".to_string(),
                "Tue".to_string(),
                "Wed".to_string(),
                "Thu".to_string(),
                "Fri".to_string(),
                "Sat".to_string(),
                "Sun".to_string(),
            ],
            aria_label: None,
            is_range: false,
            on_select_focused_date: None,
            on_focus_previous_day: None,
            on_focus_next_day: None,
            on_focus_previous_week: None,
            on_focus_next_week: None,
            on_focus_previous_section: None,
            on_focus_next_section: None,
            on_focus_section_start: None,
            on_focus_section_end: None,
            on_cancel_selection: None,
            on_focus: None,
            on_blur: None,
        }
    }
}

/// The return value of the `use_calendar_grid` hook.
pub struct UseCalendarGridReturn {
    /// Props for the grid (table) element. Call `.into_attrs()` for view spreading.
    pub grid_props: UseCalendarGridProps,

    /// Props for the header row element.
    pub header_props: UseCalendarGridHeaderProps,

    /// The weekday labels for column headers.
    pub weekday_labels: Vec<String>,

    /// The ID of the grid.
    pub grid_id: String,
}

/// Props from `use_calendar_grid` for the grid element.
#[derive(Debug)]
pub struct UseCalendarGridProps {
    pub id: String,
    pub role: AriaRole,
    pub aria_label: Option<String>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub aria_readonly: Signal<Option<AriaReadonly>>,
    pub aria_multiselectable: Option<AriaMultiselectable>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
}

impl IntoAttrs for UseCalendarGridProps {
    type Attrs = UseCalendarGridAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaDisabled, self.aria_disabled),
            Attr(attr::AriaReadonly, self.aria_readonly),
            Attr(attr::AriaMultiselectable, self.aria_multiselectable),
            self.on_keydown.into_on(ev::keydown),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
        )
    }
}

/// Attributes for the calendar grid element.
pub type UseCalendarGridAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaLabel, Option<String>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    Attr<attr::AriaReadonly, Signal<Option<AriaReadonly>>>,
    Attr<attr::AriaMultiselectable, Option<AriaMultiselectable>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
);

/// Props for the calendar grid header row.
#[derive(Debug)]
pub struct UseCalendarGridHeaderProps {
    /// The role for the header row.
    pub role: AriaRole,
    /// Column headers are hidden from screen readers. Day names are already
    /// included in each cell's aria-label, so announcing them again is redundant
    /// and makes touch screen reader navigation harder.
    pub aria_hidden: AriaHidden,
}

impl IntoAttrs for UseCalendarGridHeaderProps {
    type Attrs = UseCalendarGridHeaderAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaHidden, self.aria_hidden),
        )
    }
}

/// Attributes for the calendar grid header row.
pub type UseCalendarGridHeaderAttrs = (
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaHidden, AriaHidden>,
);

/// Provides the behavior and accessibility for a calendar grid.
///
/// A calendar grid displays a month of dates in a table format, with each week
/// as a row and each day as a cell. All keyboard navigation is centralized at
/// this grid level — individual cells do not handle keyboard events.
///
/// # Example
///
/// ```ignore
/// let calendar = use_calendar_state(UseCalendarStateInput {
///     initial_value: time::OffsetDateTime::now_utc(),
///     min: None,
///     max: None,
/// });
///
/// let grid = use_calendar_grid(UseCalendarGridInput {
///     on_select_focused_date: Some(Callback::new(move |_| {
///         // select the currently focused date
///     })),
///     on_focus_previous_day: Some(Callback::new(move |_| {
///         // move focus to previous day
///     })),
///     ..Default::default()
/// });
///
/// view! {
///     <table {..grid.grid_props.into_attrs()}>
///         <thead {..grid.header_props.into_attrs()}>
///             <tr>
///                 {grid.weekday_labels.iter().map(|label| {
///                     view! { <th>{label}</th> }
///                 }).collect_view()}
///             </tr>
///         </thead>
///         <tbody>
///             // Render weeks and days...
///         </tbody>
///     </table>
/// }
/// ```
#[allow(clippy::too_many_lines)]
pub fn use_calendar_grid(input: UseCalendarGridInput) -> UseCalendarGridReturn {
    let UseCalendarGridInput {
        is_disabled: disabled,
        is_read_only,
        start_of_week,
        weekday_labels,
        aria_label,
        is_range,
        on_select_focused_date,
        on_focus_previous_day,
        on_focus_next_day,
        on_focus_previous_week,
        on_focus_next_week,
        on_focus_previous_section,
        on_focus_next_section,
        on_focus_section_start,
        on_focus_section_end,
        on_cancel_selection,
        on_focus,
        on_blur,
    } = input;

    let grid_id = format!("calendar-grid-{}", Uuid::new_v4());

    // Reorder weekday labels based on start_of_week
    let mut weekday_labels = weekday_labels;
    if start_of_week > 0 {
        let start = start_of_week as usize % 7;
        weekday_labels.rotate_left(start);
    }

    // Compute aria-disabled
    let aria_disabled = Signal::derive(move || disabled.get().then_some(AriaDisabled::True));

    // Compute aria-readonly
    let aria_readonly = Signal::derive(move || is_read_only.get().then_some(AriaReadonly::True));

    // aria-multiselectable for range calendars
    let aria_multiselectable = is_range.then_some(AriaMultiselectable::True);

    // Handle keyboard navigation within the grid.
    // This centralizes all keyboard handling, matching react-aria's architecture.
    let handle_keydown = move |e: KeyboardEvent| {
        if disabled.get_untracked() {
            return;
        }

        let key = e.key();
        match key.as_str() {
            "Enter" | " " => {
                e.prevent_default();
                if let Some(cb) = on_select_focused_date {
                    cb.run(());
                }
            }
            "ArrowLeft" => {
                e.prevent_default();
                e.stop_propagation();
                if let Some(cb) = on_focus_previous_day {
                    cb.run(());
                }
            }
            "ArrowRight" => {
                e.prevent_default();
                e.stop_propagation();
                if let Some(cb) = on_focus_next_day {
                    cb.run(());
                }
            }
            "ArrowUp" => {
                e.prevent_default();
                e.stop_propagation();
                if let Some(cb) = on_focus_previous_week {
                    cb.run(());
                }
            }
            "ArrowDown" => {
                e.prevent_default();
                e.stop_propagation();
                if let Some(cb) = on_focus_next_week {
                    cb.run(());
                }
            }
            "PageUp" => {
                e.prevent_default();
                e.stop_propagation();
                if let Some(cb) = on_focus_previous_section {
                    cb.run(e.shift_key());
                }
            }
            "PageDown" => {
                e.prevent_default();
                e.stop_propagation();
                if let Some(cb) = on_focus_next_section {
                    cb.run(e.shift_key());
                }
            }
            "Home" => {
                e.prevent_default();
                e.stop_propagation();
                if let Some(cb) = on_focus_section_start {
                    cb.run(());
                }
            }
            "End" => {
                e.prevent_default();
                e.stop_propagation();
                if let Some(cb) = on_focus_section_end {
                    cb.run(());
                }
            }
            "Escape" => {
                if let Some(cb) = on_cancel_selection {
                    e.prevent_default();
                    cb.run(());
                }
            }
            _ => {}
        }
    };

    // Handle focus/blur to track whether the grid has focus.
    let handle_focus = move |_e: FocusEvent| {
        if let Some(cb) = on_focus {
            cb.run(());
        }
    };

    let handle_blur = move |_e: FocusEvent| {
        if let Some(cb) = on_blur {
            cb.run(());
        }
    };

    UseCalendarGridReturn {
        grid_props: UseCalendarGridProps {
            id: grid_id.clone(),
            role: AriaRole::Grid,
            aria_label,
            aria_disabled,
            aria_readonly,
            aria_multiselectable,
            on_keydown: EventHandler::new(handle_keydown),
            on_focus: EventHandler::new(handle_focus),
            on_blur: EventHandler::new(handle_blur),
        },
        header_props: UseCalendarGridHeaderProps {
            role: AriaRole::Row,
            aria_hidden: AriaHidden::True,
        },
        weekday_labels,
        grid_id,
    }
}
