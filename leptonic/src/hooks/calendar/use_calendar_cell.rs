use leptos::{
    attr,
    attr::{
        custom::{custom_attribute, CustomAttr},
        Attr,
    },
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use web_sys::{FocusEvent, KeyboardEvent};

use crate::{
    hooks::{
        focus::use_focus_ring::{use_focus_ring, UseFocusRingInput, UseFocusRingReturn},
        IntoAttrs,
    },
    utils::{
        aria::{AriaDisabled, AriaSelected},
        time::Day,
        EventHandler,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/calendar/src/useCalendarCell.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

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
    /// Props for the cell element. Call `.into_attrs()` for view spreading.
    pub cell_props: UseCalendarCellProps,

    /// Props for the button inside the cell. Call `.into_attrs()` for view spreading.
    pub button_props: UseCalendarCellButtonProps,

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

/// Props from `use_calendar_cell` for the cell element (td).
#[derive(Debug)]
pub struct UseCalendarCellProps {
    pub role: &'static str,
    pub aria_disabled: Option<AriaDisabled>,
    pub aria_selected: Signal<Option<AriaSelected>>,
}

impl IntoAttrs for UseCalendarCellProps {
    type Attrs = UseCalendarCellAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaDisabled, self.aria_disabled),
            Attr(attr::AriaSelected, self.aria_selected),
        )
    }
}

/// Attributes for the calendar cell element (td).
pub type UseCalendarCellAttrs = (
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaDisabled, Option<AriaDisabled>>,
    Attr<attr::AriaSelected, Signal<Option<AriaSelected>>>,
);

/// Props from `use_calendar_cell` for the button inside the cell.
#[derive(Debug)]
pub struct UseCalendarCellButtonProps {
    pub role: &'static str,
    pub tabindex: Signal<&'static str>,
    pub aria_label: String,
    pub aria_disabled: Option<AriaDisabled>,
    pub on_click: EventHandler<web_sys::MouseEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
    pub data_focus_visible: Signal<Option<&'static str>>,
}

impl IntoAttrs for UseCalendarCellButtonProps {
    type Attrs = UseCalendarCellButtonAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaDisabled, self.aria_disabled),
            self.on_click.into_on(ev::click),
            self.on_keydown.into_on(ev::keydown),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
            custom_attribute("data-focus-visible", self.data_focus_visible),
        )
    }
}

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
    CustomAttr<&'static str, Signal<Option<&'static str>>>,
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
///     <td {..cell.cell_props.into_attrs()}>
///         <button {..cell.button_props.into_attrs()}>
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
        is_text_input: false,
        on_focus: Some(Callback::new(move |_e| {
            if let Some(on_focus) = on_focus {
                on_focus.run(day);
            }
        })),
        on_blur: None,
        on_focus_change: None,
    });

    UseCalendarCellReturn {
        cell_props: UseCalendarCellProps {
            role: "gridcell",
            aria_disabled: cell_disabled,
            aria_selected,
        },
        button_props: UseCalendarCellButtonProps {
            role: "button",
            tabindex,
            aria_label,
            aria_disabled: cell_disabled,
            on_click: EventHandler::new(handle_click),
            on_keydown: EventHandler::new(handle_keydown),
            on_focus: focus_ring_props.on_focus,
            on_blur: focus_ring_props.on_blur,
            on_focusin: focus_ring_props.on_focusin,
            on_focusout: focus_ring_props.on_focusout,
            data_focus_visible: focus_ring_props.data_focus_visible,
        },
        is_disabled,
        is_selected,
        is_focused,
        is_today,
        is_outside_month,
        formatted_date,
        is_focus_visible,
    }
}
