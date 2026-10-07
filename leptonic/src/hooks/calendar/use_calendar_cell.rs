// Upstream: react-aria/src/calendar/useCalendarCell.ts @ 99e6102368
use std::time::Duration;

use jiff::civil::Date;
use leptos::{
    attr::{self, Attr},
    ev::{self, On, SharedEventCallback},
    prelude::*,
};
use web_sys::{FocusEvent, MouseEvent, PointerEvent};

use super::states::{CalendarData, full_date_formatter, strings};
use crate::{
    hooks::{
        IntoAttrs, PressEvent, PropsWithStyles, UsePressAttrs, UsePressInput, UsePressProps,
        use_press,
    },
    utils::{
        CapturedElement, ElementCaptureAttr, EventAccessors, EventHandler,
        aria::AriaRole,
        date::use_today,
        date_time_formatter::{DateTimeFormatOptions, DateTimeFormatter, NumericFormat},
        i18n::use_locale,
        pointer_type::PointerType,
        use_description::use_description,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Takes the calendar's `CalendarData` (react-aria: the state, with the calendar's data in a
//   `WeakMap`) and captures the button element (`element`).
// - `date` is a signal: the atoms keep a cell's elements when the calendar pages (as
//   react-aria-components keys cells by their position), so that the focus stays in the grid.
//
// ## OMITTED FEATURES
// - Localized strings: "Today, {date}", "{date} selected", "First available date", the range
//   selection prompts are English.
//
// =============================================================================

/// Input of [`use_calendar_cell`].
#[derive(Clone)]
pub struct UseCalendarCellInput {
    pub data: CalendarData,
    /// The cell's date. It may change: a calendar keeps its cells (and the focus) when it
    /// pages.
    pub date: Signal<Date>,
    /// Disables the cell regardless of the calendar.
    pub is_disabled: Signal<bool>,
    /// Whether the date belongs to another month than the grid's (shown, but not selectable).
    pub is_outside_month: Signal<bool>,
    /// The cell's button.
    pub element: CapturedElement,
}

/// Return value of [`use_calendar_cell`].
pub struct UseCalendarCellReturn {
    /// For the cell (a `td`).
    pub cell_props: UseCalendarCellProps,
    /// For the button inside the cell (it has the focus and the date's label).
    pub button_props: PropsWithStyles<UseCalendarCellButtonProps>,
    pub is_pressed: Signal<bool>,
    /// Whether the cell has the focus (the calendar's focused date).
    pub is_focused: Signal<bool>,
    pub is_selected: Signal<bool>,
    /// Whether the date can't be focused or selected (outside the visible range or min/max,
    /// another month, a disabled calendar).
    pub is_disabled: Signal<bool>,
    pub is_unavailable: Signal<bool>,
    pub is_outside_visible_range: Signal<bool>,
    /// Whether the date is part of an invalid selection.
    pub is_invalid: Signal<bool>,
    /// Whether the date is today (in the browser; never on the server).
    pub is_today: Signal<bool>,
    /// The day number, formatted for the locale.
    pub formatted_date: Signal<String>,
}

/// Props of the cell.
#[derive(Debug, Clone)]
pub struct UseCalendarCellProps {
    pub role: AriaRole,
    pub aria_disabled: Signal<Option<&'static str>>,
    pub aria_selected: Signal<Option<&'static str>>,
    pub aria_invalid: Signal<Option<&'static str>>,
}

pub type UseCalendarCellAttrs = (
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaDisabled, Signal<Option<&'static str>>>,
    Attr<attr::AriaSelected, Signal<Option<&'static str>>>,
    Attr<attr::AriaInvalid, Signal<Option<&'static str>>>,
);

impl IntoAttrs for UseCalendarCellProps {
    type Attrs = UseCalendarCellAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaDisabled, self.aria_disabled),
            Attr(attr::AriaSelected, self.aria_selected),
            Attr(attr::AriaInvalid, self.aria_invalid),
        )
    }
}

/// Props of the cell's button.
#[derive(Debug)]
pub struct UseCalendarCellButtonProps {
    pub press: UsePressProps,
    pub role: AriaRole,
    pub tabindex: Signal<Option<i32>>,
    pub aria_label: Signal<String>,
    pub aria_disabled: Signal<Option<&'static str>>,
    pub aria_invalid: Signal<Option<&'static str>>,
    pub aria_describedby: Signal<Option<String>>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_pointerenter: EventHandler<PointerEvent>,
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub on_contextmenu: EventHandler<MouseEvent>,
    pub element_capture: ElementCaptureAttr,
}

pub type UseCalendarCellButtonAttrs = (
    UsePressAttrs,
    (
        Attr<attr::Role, AriaRole>,
        Attr<attr::Tabindex, Signal<Option<i32>>>,
        Attr<attr::AriaLabel, Signal<String>>,
        Attr<attr::AriaDisabled, Signal<Option<&'static str>>>,
        Attr<attr::AriaInvalid, Signal<Option<&'static str>>>,
        Attr<attr::AriaDescribedby, Signal<Option<String>>>,
    ),
    (
        On<ev::focus, SharedEventCallback<FocusEvent>>,
        On<ev::pointerenter, SharedEventCallback<PointerEvent>>,
        On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
        On<ev::contextmenu, SharedEventCallback<MouseEvent>>,
        ElementCaptureAttr,
    ),
);

impl IntoAttrs for UseCalendarCellButtonProps {
    type Attrs = UseCalendarCellButtonAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.press.into_attrs(),
            (
                Attr(attr::Role, self.role),
                Attr(attr::Tabindex, self.tabindex),
                Attr(attr::AriaLabel, self.aria_label),
                Attr(attr::AriaDisabled, self.aria_disabled),
                Attr(attr::AriaInvalid, self.aria_invalid),
                Attr(attr::AriaDescribedby, self.aria_describedby),
            ),
            (
                self.on_focus.into_on(ev::focus),
                self.on_pointerenter.into_on(ev::pointerenter),
                self.on_pointerdown.into_on(ev::pointerdown),
                self.on_contextmenu.into_on(ev::contextmenu),
                self.element_capture,
            ),
        )
    }
}

fn flag(signal: Signal<bool>) -> Signal<Option<&'static str>> {
    Signal::derive(move || signal.get().then_some("true"))
}

/// Focuses a cell's button, scrolling it into view unless a pointer moved the focus.
#[cfg(not(feature = "ssr"))]
fn focus_cell(button: &web_sys::Element) {
    use crate::{
        hooks::{Modality, get_modality},
        utils::{
            focus::focus_element,
            scroll::{ScrollIntoViewportOpts, get_scroll_parent, scroll_into_viewport},
            shadow_dom::get_active_element,
        },
    };
    focus_element(button, true);
    let is_active = button
        .owner_document()
        .as_ref()
        .and_then(get_active_element)
        .is_some_and(|active| &active == button);
    if get_modality() != Modality::Pointer && is_active {
        scroll_into_viewport(
            Some(button),
            &ScrollIntoViewportOpts {
                containing_element: Some(get_scroll_parent(button, false)),
            },
        );
    }
}

/// Behavior and accessibility of a date in a calendar's grid: selecting it by press (a range by
/// two presses or by dragging), its label ("Today, Monday, May 20, 2024 selected"), and moving
/// the browser's focus to it when it becomes the focused date.
#[allow(clippy::too_many_lines)]
pub fn use_calendar_cell(input: UseCalendarCellInput) -> UseCalendarCellReturn {
    let UseCalendarCellInput {
        data,
        date,
        is_disabled: is_disabled_prop,
        is_outside_month,
        element,
    } = input;
    let state = data.state;
    let calendar = state.calendar();
    let range = state.range();
    let locale = use_locale();

    let is_focused =
        Signal::derive(move || calendar.is_cell_focused(date.get()) && !is_outside_month.get());
    let is_disabled = Signal::derive(move || {
        is_disabled_prop.get() || state.is_cell_disabled(date.get()) || is_outside_month.get()
    });
    let is_unavailable = Signal::derive(move || calendar.is_cell_unavailable(date.get()));
    let is_selectable = Signal::derive(move || !is_disabled.get() && !is_unavailable.get());
    let is_invalid = Signal::derive(move || {
        if !state.is_value_invalid().get() {
            return false;
        }
        match range {
            Some(range) => {
                range.anchor_date.get().is_none()
                    && range
                        .highlighted_range
                        .get()
                        .is_some_and(|highlighted| highlighted.contains(date.get()))
            }
            None => calendar.value.get() == Some(date.get()),
        }
    });
    // Invalid selected dates show as selected.
    let is_selected = Signal::derive(move || {
        (state.is_selected(date.get()) && is_selectable.get())
            || (is_invalid.get() && !is_disabled.get())
    });

    let today = use_today();
    let is_today = Signal::derive(move || today.get() == Some(date.get()));
    let selected_date_description = data.selected_date_description;
    let label = Signal::derive(move || {
        let mut label = String::new();
        // The first and last dates of a selected range name the whole range.
        if let Some(range) = range
            && range.anchor_date.get().is_none()
            && range
                .value
                .get()
                .is_some_and(|value| value.start == date.get() || value.end == date.get())
        {
            label = format!("{}, ", selected_date_description.get());
        }
        label.push_str(&full_date_formatter(&locale.get()).format_date(date.get()));
        let mut label = if is_today.get() {
            if is_selected.get() {
                strings::today_selected(&label)
            } else {
                strings::today(&label)
            }
        } else if is_selected.get() {
            strings::selected(&label)
        } else {
            label
        };
        if calendar.min_value.get() == Some(date.get()) {
            label.push_str(", ");
            label.push_str(strings::MINIMUM_DATE);
        } else if calendar.max_value.get() == Some(date.get()) {
            label.push_str(", ");
            label.push_str(strings::MAXIMUM_DATE);
        }
        label
    });

    // In a range calendar, the focused cell says how to select.
    let prompt = Signal::derive(move || {
        let range = range?;
        (is_focused.get() && !calendar.is_read_only.get() && is_selectable.get()).then(|| {
            if range.anchor_date.get().is_some() {
                strings::FINISH_RANGE_SELECTION.to_owned()
            } else {
                strings::START_RANGE_SELECTION.to_owned()
            }
        })
    });
    let prompt_id = use_description(prompt);

    let is_anchor_pressed = StoredValue::new(false);
    let is_range_boundary_pressed = StoredValue::new(false);
    let touch_drag_timer = StoredValue::new(None::<TimeoutHandle>);
    // A calendar closed within the delay (e.g. in a popover) doesn't select.
    on_cleanup(move || {
        if let Some(Some(timer)) = touch_drag_timer.try_get_value() {
            timer.clear();
        }
    });
    let focus_and_select = move || {
        state.select_date(date.get_untracked());
        calendar.set_focused_date(date.get_untracked());
        calendar.set_focused(true);
    };

    let on_press_start = Callback::new(move |e: PressEvent| {
        if calendar.is_read_only.get_untracked() {
            calendar.set_focused_date(date.get_untracked());
            calendar.set_focused(true);
            return;
        }
        let Some(range) = range else {
            return;
        };
        if range.anchor_date.get_untracked().is_some()
            || !matches!(e.pointer_type, PointerType::Mouse | PointerType::Touch)
        {
            return;
        }
        // Dragging an end of the selected range changes it, instead of starting a new range
        // (not while invalid: the range would jump to available dates).
        if let Some(highlighted) = range.highlighted_range.get_untracked()
            && !is_invalid.get_untracked()
        {
            let other_end = if date.get_untracked() == highlighted.start {
                Some(highlighted.end)
            } else if date.get_untracked() == highlighted.end {
                Some(highlighted.start)
            } else {
                None
            };
            if let Some(other_end) = other_end {
                range.set_anchor_date(Some(other_end));
                calendar.set_focused_date(date.get_untracked());
                calendar.set_focused(true);
                range.set_dragging(true);
                is_range_boundary_pressed.set_value(true);
                return;
            }
        }
        // Mouse and touch start the range on press, so that users can drag; touch only after a
        // moment, as the user may be scrolling.
        let start_dragging = move || {
            range.set_dragging(true);
            touch_drag_timer.set_value(None);
            focus_and_select();
            is_anchor_pressed.set_value(true);
        };
        if e.pointer_type == PointerType::Touch {
            touch_drag_timer.set_value(
                set_timeout_with_handle(start_dragging, Duration::from_millis(200)).ok(),
            );
        } else {
            start_dragging();
        }
    });
    let on_press_end = Callback::new(move |_: PressEvent| {
        is_range_boundary_pressed.set_value(false);
        is_anchor_pressed.set_value(false);
        if let Some(timer) = touch_drag_timer.get_value() {
            timer.clear();
        }
        touch_drag_timer.set_value(None);
    });
    // A single date is selected on press (up).
    let on_press = Callback::new(move |_: PressEvent| {
        if range.is_none() && !calendar.is_read_only.get_untracked() {
            focus_and_select();
        }
    });
    let on_press_up = Callback::new(move |e: PressEvent| {
        if calendar.is_read_only.get_untracked() {
            return;
        }
        let Some(range) = range else {
            return;
        };
        // A quick tap: the timer is still running, the date not selected yet.
        if touch_drag_timer.get_value().is_some() {
            focus_and_select();
        }
        if is_range_boundary_pressed.get_value() {
            // Pressing an end of the selected range starts a new range there on release.
            range.set_anchor_date(Some(date.get_untracked()));
        } else if range.anchor_date.get_untracked().is_some() && !is_anchor_pressed.get_value() {
            // Releasing a drag, or pressing the other end: select it.
            focus_and_select();
        } else if e.pointer_type == PointerType::Keyboard
            && range.anchor_date.get_untracked().is_none()
        {
            // Keyboard selection moves on by a day, to show that a range is being selected.
            state.select_date(date.get_untracked());
            range.focus_nearest_available_date(date.get_untracked());
        } else if e.pointer_type == PointerType::Virtual {
            focus_and_select();
        }
    });

    let press = use_press(UsePressInput {
        // Dragging back over the anchor shouldn't start a new press.
        should_cancel_on_pointer_exit: Signal::derive(move || {
            range.is_some_and(|range| range.anchor_date.get().is_some())
        }),
        prevent_focus_on_press: Signal::stored(true),
        is_disabled: Signal::derive(move || !is_selectable.get() || calendar.is_read_only.get()),
        on_press_start: Some(on_press_start),
        on_press_end: Some(on_press_end),
        on_press: Some(on_press),
        on_press_up: Some(on_press_up),
        ..UsePressInput::default()
    });
    let (press_props, press_styles) = press.props.into_inner();

    let tabindex = Signal::derive(move || {
        (!is_disabled.get()).then(|| {
            if calendar.focused_date.get() == date.get() {
                0
            } else {
                -1
            }
        })
    });

    // The browser's focus follows the focused date. Keyboard navigation scrolls it into view (a
    // pointer shouldn't move the view under it).
    #[cfg(not(feature = "ssr"))]
    Effect::new(move |_| {
        if !is_focused.get() {
            return;
        }
        let Some(button) = element.get_untracked() else {
            return;
        };
        let button = (*button).clone();
        if crate::utils::focusability::is_focusable(&button) {
            focus_cell(&button);
        } else {
            // Paging reused a cell that was disabled (another month's date): this effect may run
            // before its `tabindex` is rendered.
            let button = send_wrapper::SendWrapper::new(button);
            request_animation_frame(move || {
                if is_focused.get_untracked() {
                    focus_cell(&button);
                }
            });
        }
    });

    let error_message_id = data.error_message_id;
    let aria_describedby = Signal::derive(move || {
        let ids: Vec<String> = [
            is_invalid.get().then(|| error_message_id.get()).flatten(),
            prompt_id.get(),
        ]
        .into_iter()
        .flatten()
        .collect();
        (!ids.is_empty()).then(|| ids.join(" "))
    });

    let formatted_date = Signal::derive(move || {
        DateTimeFormatter::new(
            &locale.get(),
            DateTimeFormatOptions {
                day: Some(NumericFormat::Numeric),
                ..DateTimeFormatOptions::default()
            },
        )
        .format_date(date.get())
    });

    let not_selectable = Signal::derive(move || !is_selectable.get());
    UseCalendarCellReturn {
        cell_props: UseCalendarCellProps {
            role: AriaRole::Gridcell,
            aria_disabled: flag(not_selectable),
            aria_selected: flag(is_selected),
            aria_invalid: flag(is_invalid),
        },
        button_props: PropsWithStyles::new(
            UseCalendarCellButtonProps {
                press: press_props,
                role: AriaRole::Button,
                tabindex,
                aria_label: label,
                aria_disabled: flag(not_selectable),
                aria_invalid: flag(is_invalid),
                aria_describedby,
                on_focus: EventHandler::new(move |_: FocusEvent| {
                    if !is_disabled.get_untracked() {
                        calendar.set_focused_date(date.get_untracked());
                        calendar.set_focused(true);
                    }
                }),
                // Hovering (or dragging over) a date while selecting a range highlights it.
                on_pointerenter: EventHandler::new(move |e: PointerEvent| {
                    if let Some(range) = range
                        && (PointerType::from(e.pointer_type()) != PointerType::Touch
                            || range.is_dragging.get_untracked())
                        && is_selectable.get_untracked()
                    {
                        range.highlight_date(date.get_untracked());
                    }
                }),
                // Touch drags may leave the pressed cell.
                on_pointerdown: EventHandler::new(|e: PointerEvent| {
                    if let Some(target) =
                        wasm_bindgen::JsCast::dyn_ref::<web_sys::Element>(&e.expect_target())
                        && target.has_pointer_capture(e.pointer_id())
                    {
                        let _ = target.release_pointer_capture(e.pointer_id());
                    }
                }),
                // No context menu on a long press.
                on_contextmenu: EventHandler::new(|e: MouseEvent| e.prevent_default()),
                element_capture: element.attr(),
            },
            press_styles,
        ),
        is_pressed: press.is_pressed,
        is_focused,
        is_selected,
        is_disabled,
        is_unavailable,
        is_outside_visible_range: Signal::derive(move || {
            !calendar.visible_range.get().contains(date.get())
        }),
        is_invalid,
        is_today,
        formatted_date,
    }
}
