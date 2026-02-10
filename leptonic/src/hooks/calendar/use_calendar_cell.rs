use leptos::attr;
use leptos::attr::{Attr, Attribute};
use leptos::ev;
use leptos::ev::{on, On, SharedEventCallback};
use leptos::prelude::*;
use web_sys::{FocusEvent, KeyboardEvent};

use crate::hooks::focus::use_focus_ring::{use_focus_ring, UseFocusRingInput, UseFocusRingReturn};
use crate::utils::aria::{AriaDisabled, AriaSelected};
use crate::utils::time::Day;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/calendar/src/useCalendarCell.ts

/// Input parameters for the `use_calendar_cell` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseCalendarCellInput {
    /// The day this cell represents.
    pub day: Day,

    /// Whether the cell is currently focused.
    pub is_focused: Signal<bool>,

    /// Whether the cell is selected.
    pub is_selected: Signal<bool>,

    /// Whether the calendar is disabled.
    pub is_disabled: Signal<bool>,

    /// Callback when the cell is selected.
    pub on_select: Option<Callback<Day>>,

    /// Callback when focus moves to this cell.
    pub on_focus: Option<Callback<Day>>,

    /// Callback to navigate to the previous month.
    pub on_previous_month: Option<Callback<()>>,

    /// Callback to navigate to the next month.
    pub on_next_month: Option<Callback<()>>,

    /// Callback to navigate to the previous week.
    pub on_previous_week: Option<Callback<()>>,

    /// Callback to navigate to the next week.
    pub on_next_week: Option<Callback<()>>,

    /// Callback to navigate to the previous day.
    pub on_previous_day: Option<Callback<()>>,

    /// Callback to navigate to the next day.
    pub on_next_day: Option<Callback<()>>,
}

/// The return value of the `use_calendar_cell` hook.
pub struct UseCalendarCellReturn {
    /// Props for the cell element.
    pub cell_props: UseCalendarCellAttrs,

    /// Props for the button inside the cell.
    pub button_props: UseCalendarCellButtonAttrs,

    /// Whether the cell is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the cell is selected.
    pub is_selected: Signal<bool>,

    /// Whether the cell is focused.
    pub is_focused: Signal<bool>,

    /// Whether this is today's date.
    pub is_today: bool,

    /// Whether the day is outside the current month.
    pub is_outside_month: bool,

    /// The formatted date string for the cell.
    pub formatted_date: String,

    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,
}

/// Attributes for the calendar cell element (td).
pub type UseCalendarCellAttrs = (
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaDisabled, Option<AriaDisabled>>,
    Attr<attr::AriaSelected, Signal<Option<AriaSelected>>>,
);

/// Attributes for the button inside the calendar cell.
pub type UseCalendarCellButtonAttrs = (
    Attr<attr::Role, &'static str>,
    Attr<attr::Tabindex, Signal<&'static str>>,
    Attr<attr::AriaLabel, String>,
    Attr<attr::AriaDisabled, Option<AriaDisabled>>,
    On<ev::click, SharedEventCallback<web_sys::MouseEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
    attr::custom::CustomAttr<&'static str, Signal<Option<&'static str>>>,
);

/// Provides the behavior and accessibility for a calendar cell.
///
/// A calendar cell represents a single day in the calendar grid.
///
/// # Example
///
/// ```ignore
/// let cell = use_calendar_cell(UseCalendarCellInput {
///     day,
///     is_focused: is_cell_focused.into(),
///     is_selected: is_cell_selected.into(),
///     is_disabled: Signal::derive(|| false),
///     on_select: Some(Callback::new(|day| { /* select day */ })),
///     ..Default::default()
/// });
///
/// view! {
///     <td {..cell.cell_props}>
///         <button {..cell.button_props}>
///             {cell.formatted_date}
///         </button>
///     </td>
/// }
/// ```
#[allow(clippy::too_many_lines)]
pub fn use_calendar_cell(input: UseCalendarCellInput) -> UseCalendarCellReturn {
    let UseCalendarCellInput {
        day,
        is_focused,
        is_selected,
        is_disabled: disabled,
        on_select,
        on_focus,
        on_previous_month,
        on_next_month,
        on_previous_week,
        on_next_week,
        on_previous_day,
        on_next_day,
    } = input;

    // Check if the day is disabled (either from input or from day.disabled)
    let is_disabled = Signal::derive(move || disabled.get() || day.disabled);

    let is_today = day.is_now;
    let is_outside_month = day.in_month != crate::utils::time::InMonth::Current;

    // Format the date for display and aria-label
    let formatted_date = format!("{}", day.index);
    let aria_label = format!(
        "{} {} {}",
        day.date_time.day(),
        day.date_time.month(),
        day.date_time.year()
    );

    // Compute aria-selected
    let aria_selected = Signal::derive(move || Some(AriaSelected::from(is_selected.get())));

    // Compute tabindex
    let tabindex = Signal::derive(move || if is_focused.get() { "0" } else { "-1" });

    let cell_disabled = day.disabled.then_some(AriaDisabled::True);

    // Handle click
    let handle_click = move |_e: web_sys::MouseEvent| {
        if day.disabled {
            return;
        }
        if let Some(on_select) = on_select {
            on_select.run(day);
        }
    };

    // Handle keyboard navigation
    let handle_keydown = move |e: KeyboardEvent| {
        if day.disabled {
            return;
        }

        let key = e.key();
        match key.as_str() {
            "Enter" | " " => {
                e.prevent_default();
                if let Some(on_select) = on_select {
                    on_select.run(day);
                }
            }
            "ArrowUp" => {
                e.prevent_default();
                if let Some(cb) = on_previous_week {
                    cb.run(());
                }
            }
            "ArrowDown" => {
                e.prevent_default();
                if let Some(cb) = on_next_week {
                    cb.run(());
                }
            }
            "ArrowLeft" => {
                e.prevent_default();
                if let Some(cb) = on_previous_day {
                    cb.run(());
                }
            }
            "ArrowRight" => {
                e.prevent_default();
                if let Some(cb) = on_next_day {
                    cb.run(());
                }
            }
            "PageUp" => {
                e.prevent_default();
                if e.shift_key() {
                    // Navigate to previous year (not implemented here)
                } else if let Some(cb) = on_previous_month {
                    cb.run(());
                }
            }
            "PageDown" => {
                e.prevent_default();
                if e.shift_key() {
                    // Navigate to next year (not implemented here)
                } else if let Some(cb) = on_next_month {
                    cb.run(());
                }
            }
            _ => {}
        }
    };

    let UseFocusRingReturn {
        props: focus_ring_props,
        is_focus_visible,
        is_focused: _,
    } = use_focus_ring(UseFocusRingInput {
        disabled: is_disabled,
        within: false,
        auto_focus: false,
        on_focus: Some(Callback::new(move |_e| {
            if let Some(on_focus) = on_focus {
                on_focus.run(day);
            }
        })),
        on_blur: None,
        on_focus_change: None,
    });
    let (on_focus, on_blur, on_focusin, on_focusout, data_focus_visible) =
        focus_ring_props.into_attrs();

    UseCalendarCellReturn {
        cell_props: (
            Attr(attr::Role, "gridcell"),
            Attr(attr::AriaDisabled, cell_disabled),
            Attr(attr::AriaSelected, aria_selected),
        ),
        button_props: (
            Attr(attr::Role, "button"),
            Attr(attr::Tabindex, tabindex),
            Attr(attr::AriaLabel, aria_label),
            Attr(attr::AriaDisabled, cell_disabled),
            on(ev::click, handle_click).into_cloneable(),
            on(ev::keydown, handle_keydown).into_cloneable(),
            on_focus,
            on_blur,
            on_focusin,
            on_focusout,
            data_focus_visible,
        ),
        is_disabled,
        is_selected,
        is_focused,
        is_today,
        is_outside_month,
        formatted_date,
        is_focus_visible,
    }
}
