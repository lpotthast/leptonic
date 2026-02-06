use leptos::attr;
use leptos::attr::{Attr, Attribute};
use leptos::ev;
use leptos::ev::{on, On, SharedEventCallback};
use leptos::prelude::*;
use uuid::Uuid;
use web_sys::KeyboardEvent;

use crate::utils::time::Week;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/calendar/src/useCalendarGrid.ts

/// Input parameters for the `use_calendar_grid` hook.
#[derive(Debug, Clone)]
pub struct UseCalendarGridInput {
    /// The weeks to display in the grid.
    pub weeks: Signal<Vec<Week>>,

    /// Whether the calendar is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the calendar is read-only.
    pub is_read_only: Signal<bool>,

    /// The start day of the week (0 = Monday, 6 = Sunday).
    pub start_of_week: u8,

    /// Labels for the days of the week.
    pub weekday_labels: Vec<String>,
}

impl Default for UseCalendarGridInput {
    fn default() -> Self {
        Self {
            weeks: Signal::derive(Vec::new),
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
        }
    }
}

/// The return value of the `use_calendar_grid` hook.
pub struct UseCalendarGridReturn {
    /// Props for the grid (table) element.
    pub grid_props: UseCalendarGridAttrs,

    /// Props for the header row element.
    pub header_props: UseCalendarGridHeaderProps,

    /// The weekday labels for column headers.
    pub weekday_labels: Vec<String>,

    /// The ID of the grid.
    pub grid_id: String,
}

/// Attributes for the calendar grid element.
pub type UseCalendarGridAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaDisabled, Signal<&'static str>>,
    Attr<attr::AriaReadonly, Signal<&'static str>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
);

/// Props for the calendar grid header row.
#[derive(Debug, Clone)]
pub struct UseCalendarGridHeaderProps {
    /// The role for the header row.
    pub role: &'static str,
}

/// Provides the behavior and accessibility for a calendar grid.
///
/// A calendar grid displays a month of dates in a table format, with each week
/// as a row and each day as a cell.
///
/// # Example
///
/// ```ignore
/// let calendar = use_calendar(time::OffsetDateTime::now_utc(), None, None);
///
/// let grid = use_calendar_grid(UseCalendarGridInput {
///     weeks: calendar.weeks,
///     ..Default::default()
/// });
///
/// view! {
///     <table {..grid.grid_props}>
///         <thead>
///             <tr {..grid.header_props}>
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
pub fn use_calendar_grid(input: UseCalendarGridInput) -> UseCalendarGridReturn {
    let grid_id = format!("calendar-grid-{}", Uuid::new_v4());
    let is_disabled = input.is_disabled;
    let is_read_only = input.is_read_only;

    // Reorder weekday labels based on start_of_week
    let mut weekday_labels = input.weekday_labels;
    if input.start_of_week > 0 {
        let start = input.start_of_week as usize % 7;
        weekday_labels.rotate_left(start);
    }

    // Compute aria-disabled
    let aria_disabled = Signal::derive(move || if is_disabled.get() { "true" } else { "false" });

    // Compute aria-readonly
    let aria_readonly = Signal::derive(move || if is_read_only.get() { "true" } else { "false" });

    // Handle keyboard navigation within the grid
    let handle_keydown = move |e: KeyboardEvent| {
        if is_disabled.get_untracked() {
            return;
        }

        let key = e.key();
        let handled = matches!(
            key.as_str(),
            "ArrowUp"
                | "ArrowDown"
                | "ArrowLeft"
                | "ArrowRight"
                | "Home"
                | "End"
                | "PageUp"
                | "PageDown"
        );

        if handled {
            // Navigation is handled at the cell level
            // Grid just needs to allow these keys to propagate
        }
    };

    UseCalendarGridReturn {
        grid_props: (
            Attr(attr::Id, grid_id.clone()),
            Attr(attr::Role, "grid"),
            Attr(attr::AriaDisabled, aria_disabled),
            Attr(attr::AriaReadonly, aria_readonly),
            on(ev::keydown, handle_keydown).into_cloneable(),
        ),
        header_props: UseCalendarGridHeaderProps { role: "row" },
        weekday_labels,
        grid_id,
    }
}
