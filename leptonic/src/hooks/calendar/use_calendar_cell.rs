use leptos::{
    attr,
    attr::{
        Attr,
        custom::{CustomAttr, custom_attribute},
    },
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use web_sys::FocusEvent;

use crate::{
    hooks::{
        IntoAttrs,
        focus::use_focus_ring::{UseFocusRingInput, UseFocusRingReturn, use_focus_ring},
    },
    utils::{
        EventHandler,
        aria::{AriaDisabled, AriaRole, AriaSelected},
        time::Day,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/calendar/src/useCalendarCell.ts

//
// 1. No keyboard handling in cell: react-aria's cell uses `usePress` which
//    handles Enter/Space via the press interaction. In our architecture, all
//    keyboard navigation (including Enter/Space selection) is centralized in
//    `use_calendar_grid`. The cell only handles mouse clicks.
//
// 2. No drag-to-select for range calendars: react-aria supports dragging
//    across cells to select a range via pointer events. Not implemented.
//    Hover highlighting for range selection should be wired by consumers via
//    `on:pointerenter` calling `state.highlight_date`.
//
// 3. No `isInvalid` / `aria-invalid` support on individual cells.
//    Validation is handled at the state level (`is_value_invalid`).
//
// 4. No `isPressed` return value (would require `usePress` integration).
//
// 5. Simplified aria-label: includes "today" and "selected" markers but not
//    locale-aware formatting, range descriptions, or min/max markers.
//

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

    /// Callback when the cell is selected (via click).
    pub on_select: Option<Callback<Day>>,

    /// Callback when focus moves to this cell.
    pub on_focus: Option<Callback<Day>>,
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
    pub role: AriaRole,
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
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaDisabled, Option<AriaDisabled>>,
    Attr<attr::AriaSelected, Signal<Option<AriaSelected>>>,
);

/// Props from `use_calendar_cell` for the button inside the cell.
///
/// Note: No `on_keydown` handler — all keyboard navigation is centralized
/// in `use_calendar_grid`. Keyboard events on the button bubble up to the
/// grid's keydown handler.
#[derive(Debug)]
pub struct UseCalendarCellButtonProps {
    pub role: AriaRole,
    pub tabindex: Signal<&'static str>,
    pub aria_label: Signal<String>,
    pub aria_disabled: Option<AriaDisabled>,
    pub on_click: EventHandler<web_sys::MouseEvent>,
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
    Attr<attr::Role, AriaRole>,
    Attr<attr::Tabindex, Signal<&'static str>>,
    Attr<attr::AriaLabel, Signal<String>>,
    Attr<attr::AriaDisabled, Option<AriaDisabled>>,
    On<ev::click, SharedEventCallback<web_sys::MouseEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
    CustomAttr<&'static str, Signal<Option<&'static str>>>,
);

/// Provides the behavior and accessibility for a calendar cell.
///
/// A calendar cell represents a single day in the calendar grid.
/// It handles click selection and focus tracking. Keyboard navigation
/// is handled at the grid level — keyboard events on the cell button
/// bubble up to `use_calendar_grid`'s keydown handler.
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
///     on_focus: None,
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
pub fn use_calendar_cell(input: UseCalendarCellInput) -> UseCalendarCellReturn {
    let UseCalendarCellInput {
        day,
        is_focused,
        is_selected,
        is_disabled: disabled,
        on_select,
        on_focus,
    } = input;

    // Check if the day is disabled (either from input or from day.disabled)
    let is_disabled = Signal::derive(move || disabled.get() || day.disabled);

    let is_today = day.is_now;
    let is_outside_month = day.in_month != crate::utils::time::InMonth::Current;

    // Format the date for display and aria-label
    let formatted_date = format!("{}", day.index);
    // Enriched aria-label includes "today" and "selected" markers for screen readers.
    let aria_label = Signal::derive(move || {
        let mut label = format!(
            "{} {} {}",
            day.date_time.day(),
            day.date_time.month(),
            day.date_time.year()
        );
        if day.is_now && is_selected.get() {
            label.push_str(", today, selected");
        } else if day.is_now {
            label.push_str(", today");
        } else if is_selected.get() {
            label.push_str(", selected");
        }
        label
    });

    // Compute aria-selected
    let aria_selected = Signal::derive(move || Some(AriaSelected::from(is_selected.get())));

    // Compute tabindex
    let tabindex = Signal::derive(move || if is_focused.get() { "0" } else { "-1" });

    let cell_disabled = day.disabled.then_some(AriaDisabled::True);

    // Handle click selection
    let handle_click = move |_e: web_sys::MouseEvent| {
        if day.disabled {
            return;
        }
        if let Some(on_select) = on_select {
            on_select.run(day);
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
            role: AriaRole::Gridcell,
            aria_disabled: cell_disabled,
            aria_selected,
        },
        button_props: UseCalendarCellButtonProps {
            role: AriaRole::Button,
            tabindex,
            aria_label,
            aria_disabled: cell_disabled,
            on_click: EventHandler::new(handle_click),
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
