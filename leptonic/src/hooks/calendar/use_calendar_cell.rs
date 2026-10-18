// Upstream: react-aria/src/calendar/useCalendarCell.ts @ 99e6102368
// Upstream: react-aria/test/calendar/useCalendar.test.js @ 99e6102368
// Upstream: react-aria-components/test/Calendar.test.js @ 99e6102368
// Upstream: react-aria-components/test/RangeCalendar.test.tsx @ 99e6102368
// Upstream: @adobe/react-spectrum/test/calendar/CalendarBase.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/calendar/Calendar.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/calendar/RangeCalendar.test.js @ 99e6102368
use std::time::Duration;

use jiff::civil::Date;
use leptos::{
    ev::{self},
    prelude::*,
};
use web_sys::{FocusEvent, MouseEvent, PointerEvent};

use super::{states::CalendarData, use_calendar_state::CalendarState};
use crate::{
    CapturedElement, ElementCaptureAttr, EventHandler, Id, IdRefs, IntoAttrs, OnEvent,
    PropsWithStyles,
    hooks::interactions::{PressEvent, UsePressAttrs, UsePressInput, UsePressProps, use_press},
    utils::{
        aria::{AriaDisabled, AriaInvalid, AriaRole, AriaSelected},
        dom_ext::EventAccessors,
        part::{Part, Remount},
        pointer_type::PointerType,
        signal_attr::{FixedAttr, SignalAttr, fixed_attr, signal_attr},
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
// ## DIFFERENT BEHAVIOR
// - The cells share the calendar's formatters (`CalendarData`); a cell's states are two memos
//   (its focus, and the rest), so that moving the focus or the highlighted range only updates
//   the cells that change. The browser's focus follows the focused date through one effect per
//   calendar, which focuses the registered cell of the focused date (react-aria: an effect per
//   cell).
//
// ## ADDITIONS
// - Returns `is_selection_start` and `is_selection_end` (react-aria-components' `CalendarCell`
//   compares the date with `state.highlightedRange` itself): from the cell's memo, which the
//   range's ends notify, instead of every cell reading the highlighted range.
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

/// The return value of [`use_calendar_cell`].
#[derive(Debug)]
pub struct UseCalendarCellReturn {
    /// The cell (`<td>`) element.
    pub cell: Part<UseCalendarCellProps>,
    /// The cell's button element: `let (attrs, styles) = button.attrs();`.
    pub button: Part<PropsWithStyles<UseCalendarCellButtonProps>>,
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
    /// Whether the date is the first of the highlighted range (range calendars: the selected
    /// range, or the range being selected).
    pub is_selection_start: Signal<bool>,
    /// Whether the date is the last of the highlighted range.
    pub is_selection_end: Signal<bool>,
    /// The day number, formatted for the locale.
    pub formatted_date: Signal<String>,
}

/// Props of the cell.
#[derive(Debug)]
pub struct UseCalendarCellProps {
    pub role: AriaRole,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub aria_selected: Signal<Option<AriaSelected>>,
    pub aria_invalid: Signal<Option<AriaInvalid>>,
}

impl Remount for UseCalendarCellProps {
    fn remount(&self) -> Self {
        Self {
            role: self.role,
            aria_disabled: self.aria_disabled,
            aria_selected: self.aria_selected,
            aria_invalid: self.aria_invalid,
        }
    }
}

pub type UseCalendarCellAttrs = (
    FixedAttr,
    SignalAttr<Option<AriaDisabled>>,
    SignalAttr<Option<AriaSelected>>,
    SignalAttr<Option<AriaInvalid>>,
);

impl IntoAttrs for UseCalendarCellProps {
    type Attrs = UseCalendarCellAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            fixed_attr("role", self.role),
            signal_attr("aria-disabled", self.aria_disabled),
            signal_attr("aria-selected", self.aria_selected),
            signal_attr("aria-invalid", self.aria_invalid),
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
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub aria_invalid: Signal<Option<AriaInvalid>>,
    pub aria_describedby: Signal<Option<Id>>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_pointerenter: EventHandler<PointerEvent>,
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub on_contextmenu: EventHandler<MouseEvent>,
    pub element_capture: ElementCaptureAttr,
}

impl Remount for UseCalendarCellButtonProps {
    fn remount(&self) -> Self {
        Self {
            press: self.press.remount(),
            role: self.role,
            tabindex: self.tabindex,
            aria_label: self.aria_label,
            aria_disabled: self.aria_disabled,
            aria_invalid: self.aria_invalid,
            aria_describedby: self.aria_describedby,
            on_focus: self.on_focus.clone(),
            on_pointerenter: self.on_pointerenter.clone(),
            on_pointerdown: self.on_pointerdown.clone(),
            on_contextmenu: self.on_contextmenu.clone(),
            element_capture: self.element_capture.clone(),
        }
    }
}

pub type UseCalendarCellButtonAttrs = (
    UsePressAttrs,
    (
        FixedAttr,
        SignalAttr<Option<i32>>,
        SignalAttr<String>,
        SignalAttr<Option<AriaDisabled>>,
        SignalAttr<Option<AriaInvalid>>,
        SignalAttr<Option<Id>>,
    ),
    (
        OnEvent<ev::focus>,
        OnEvent<ev::pointerenter>,
        OnEvent<ev::pointerdown>,
        OnEvent<ev::contextmenu>,
        ElementCaptureAttr,
    ),
);

impl IntoAttrs for UseCalendarCellButtonProps {
    type Attrs = UseCalendarCellButtonAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.press.into_attrs(),
            (
                fixed_attr("role", self.role),
                signal_attr("tabindex", self.tabindex),
                signal_attr("aria-label", self.aria_label),
                signal_attr("aria-disabled", self.aria_disabled),
                signal_attr("aria-invalid", self.aria_invalid),
                signal_attr("aria-describedby", self.aria_describedby),
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

/// What a cell shows of its date, the selection and the calendar's limits: one memo per cell
/// (`performance.md` R5), as paging changes every cell's date but rarely these, so that their
/// readers stay put.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools)] // Independent flags, each read by its own attribute.
struct CellFlags {
    disabled: bool,
    unavailable: bool,
    invalid: bool,
    selected: bool,
    today: bool,
    outside_visible_range: bool,
    /// The first or last date of the selected range (its label names the range).
    range_boundary: bool,
    /// The first and the last date of the highlighted range (range calendars).
    selection_start: bool,
    selection_end: bool,
    /// The calendar's minimum or maximum date (its label says so).
    is_min: bool,
    is_max: bool,
}

impl CellFlags {
    fn selectable(&self) -> bool {
        !self.disabled && !self.unavailable
    }
}

/// A cell's focus: a memo of its own, as a focus move changes it in two cells only, and its
/// readers differ from those of [`CellFlags`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CellFocus {
    /// The calendar's focused date (the grid's tab stop), with or without the focus.
    is_focused_date: bool,
    /// The focused date while the grid has the focus.
    is_focused: bool,
}

/// One flag of a cell (one closure type for all of them: `performance.md` R13).
fn flag(flags: Memo<CellFlags>, field: fn(&CellFlags) -> bool) -> Signal<bool> {
    Signal::derive(move || flags.with(field))
}

/// An ARIA value that is `True` while a flag of the cell is set, else absent.
fn aria_flag<T: From<bool> + Send + Sync + 'static>(
    flags: Memo<CellFlags>,
    field: fn(&CellFlags) -> bool,
) -> Signal<Option<T>> {
    Signal::derive(move || flags.with(field).then(|| T::from(true)))
}

/// What a range calendar's cell remembers between the events of a press.
#[derive(Default)]
struct RangePress {
    is_anchor_pressed: bool,
    is_range_boundary_pressed: bool,
    touch_drag_timer: Option<TimeoutHandle>,
}

/// Focuses a cell's button, scrolling it into view unless a pointer moved the focus.
#[cfg(not(feature = "ssr"))]
fn focus_cell(button: &web_sys::Element) {
    use crate::{
        hooks::focus::{Modality, get_modality},
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
    if get_modality() != Some(Modality::Pointer) && is_active {
        scroll_into_viewport(
            Some(button),
            &ScrollIntoViewportOpts {
                containing_element: Some(get_scroll_parent(button, false)),
            },
        );
    }
}

/// A calendar's cells, for moving the browser's focus to the cell of the focused date: one
/// effect per calendar instead of one per cell (`performance.md`, R6). Cells register in
/// [`use_calendar_cell`] (react-aria: an effect per cell).
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "ssr", allow(dead_code))]
pub(crate) struct CalendarCells {
    registry: StoredValue<CellRegistry>,
    /// Notified once per batch of registrations (e.g. the cells of a grid mounted for another
    /// visible duration), so that the focus moves into them.
    registered: Trigger,
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "ssr", allow(dead_code))]
struct CellRegistry {
    /// The registered cells; `None` where one unregistered (reused by the next).
    cells: Vec<Option<CellTarget>>,
    free: Vec<usize>,
    next_id: u64,
    /// Whether registrations since the effect last ran have been notified already.
    notified: bool,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "ssr", allow(dead_code))]
struct CellTarget {
    /// Unique per registration (a slot is reused): tells whether the focus moved to another cell.
    id: u64,
    date: Signal<Date>,
    is_outside_month: Signal<bool>,
    element: CapturedElement,
}

impl CalendarCells {
    /// No cells yet, and the effect moving the browser's focus to the cell of `calendar`'s focused
    /// date while its grid has the focus. Keyboard navigation scrolls it into view (a pointer
    /// shouldn't move the view under it).
    pub(crate) fn new(calendar: &CalendarState) -> Self {
        let calendar = *calendar;
        let cells = Self {
            registry: StoredValue::new(CellRegistry::default()),
            registered: Trigger::new(),
        };
        #[cfg(not(feature = "ssr"))]
        Effect::new(move |previous: Option<Option<u64>>| {
            // The calendar's inputs first ("Effect Read Order", leptos-and-dom.md).
            let focused_date = calendar.focused_date.get();
            let has_focus = calendar.is_focused.get();
            // Paging changes the cells' dates, a remounted grid its cells.
            calendar.visible_range.track();
            cells.registered.track();
            cells
                .registry
                .update_value(|registry| registry.notified = false);
            if !has_focus {
                return None;
            }
            let target = cells.target(focused_date)?;
            // The focused cell has the focus already (react-aria focuses a cell when it becomes
            // the focused one).
            if previous.flatten() == Some(target.id) {
                return Some(target.id);
            }
            let button = (*target.element.get_untracked()?).clone();
            if crate::utils::focusability::is_focusable(&button) {
                focus_cell(&button);
            } else {
                // Paging reused a cell that was disabled (another month's date): this effect may
                // run before its `tabindex` is rendered.
                let button = send_wrapper::SendWrapper::new(button);
                request_animation_frame(move || {
                    let still_focused = calendar.is_focused.try_get_untracked().unwrap_or(false)
                        && calendar.focused_date.try_get_untracked()
                            == target.date.try_get_untracked()
                        && target.is_outside_month.try_get_untracked() == Some(false);
                    if still_focused {
                        focus_cell(&button);
                    }
                });
            }
            Some(target.id)
        });
        #[cfg(feature = "ssr")]
        let _ = calendar;
        cells
    }

    /// Registers a cell until the current owner is cleaned up.
    fn register(
        self,
        date: Signal<Date>,
        is_outside_month: Signal<bool>,
        element: CapturedElement,
    ) {
        #[cfg(not(feature = "ssr"))]
        {
            let registered = self.registry.try_update_value(|registry| {
                let target = CellTarget {
                    id: registry.next_id,
                    date,
                    is_outside_month,
                    element,
                };
                registry.next_id += 1;
                let index = if let Some(index) = registry.free.pop() {
                    registry.cells[index] = Some(target);
                    index
                } else {
                    registry.cells.push(Some(target));
                    registry.cells.len() - 1
                };
                (index, !std::mem::replace(&mut registry.notified, true))
            });
            let Some((index, notify)) = registered else {
                return;
            };
            if notify {
                self.registered.notify();
            }
            let registry = self.registry;
            on_cleanup(move || {
                registry.try_update_value(|registry| {
                    if let Some(slot) = registry.cells.get_mut(index) {
                        *slot = None;
                        registry.free.push(index);
                    }
                });
            });
        }
        #[cfg(feature = "ssr")]
        let _ = (self, date, is_outside_month, element);
    }

    /// The registered cell showing `date` in its month.
    #[cfg(not(feature = "ssr"))]
    fn target(self, date: Date) -> Option<CellTarget> {
        self.registry.with_value(|registry| {
            registry.cells.iter().flatten().copied().find(|cell| {
                cell.date.try_get_untracked() == Some(date)
                    && cell.is_outside_month.try_get_untracked() == Some(false)
            })
        })
    }
}

/// Behavior and accessibility of a date in a calendar's grid: selecting it by press (a range by
/// two presses or by dragging), its label ("Today, Monday, May 20, 2024 selected"), and moving
/// the browser's focus to it when it becomes the focused date.
pub fn use_calendar_cell(input: UseCalendarCellInput) -> UseCalendarCellReturn {
    crate::metrics::subject!();
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
    let formatters = data.formatters;
    let today = data.today;
    let never = data.never;

    let flags = Memo::new(move |_| {
        let date = date.get();
        let disabled =
            is_disabled_prop.get() || state.is_cell_disabled(date) || is_outside_month.get();
        let unavailable = calendar.is_cell_unavailable(date);
        // `is_selected` subscribes to the selection (or highlight) changes of this date only (a
        // change notifies the dates entering or leaving it), which the untracked reads below
        // need too: only the selected cells read whether the value is invalid.
        let is_selected = state.is_selected(date);
        let invalid = match range {
            Some(range) => {
                range.highlighted_range.with_untracked(|highlighted| {
                    highlighted.is_some_and(|highlighted| highlighted.contains(date))
                }) && range.anchor_date.get().is_none()
            }
            None => calendar.value.with_untracked(|value| *value == Some(date)),
        } && state.is_value_invalid().get();
        // Invalid selected dates show as selected.
        let selected = (is_selected && !disabled && !unavailable) || (invalid && !disabled);
        // Tracked by `is_selected` above: a change of the highlighted range notifies its old and
        // new ends too.
        let (selection_start, selection_end) = range.map_or((false, false), |range| {
            range.highlighted_range.with_untracked(|highlighted| {
                highlighted.map_or((false, false), |highlighted| {
                    (highlighted.start == date, highlighted.end == date)
                })
            })
        });
        // Tracked by `is_selected` above too: the selected range's old and new ends are notified
        // when they start or stop naming the range (`performance.md` R1).
        let range_boundary = range.is_some_and(|range| {
            range.anchor_date.with_untracked(Option::is_none)
                && range.value.with_untracked(|value| {
                    value.is_some_and(|value| value.start == date || value.end == date)
                })
        });
        CellFlags {
            disabled,
            unavailable,
            invalid,
            selected,
            today: today.get() == Some(date),
            outside_visible_range: !calendar.visible_range.get().contains(date),
            range_boundary,
            selection_start,
            selection_end,
            is_min: calendar.min_value.get() == Some(date),
            is_max: calendar.max_value.get() == Some(date),
        }
    });
    let focus = Memo::new(move |_| {
        let date = date.get();
        // `is_cell_focused` subscribes to the focus changes of this date only (a focus move
        // notifies the old and the new date), which the comparison below needs too.
        let is_focused = calendar.is_cell_focused(date) && !is_outside_month.get();
        CellFocus {
            is_focused_date: calendar.focused_date.get_untracked() == date,
            is_focused,
        }
    });
    let is_focused = Signal::derive(move || focus.with(|focus| focus.is_focused));

    let selected_date_description = data.selected_date_description;
    let strings = data.strings;
    let label = Signal::derive(move || {
        // The date first: it is upstream of the flags ("Effect Read Order", leptos-and-dom.md).
        let date = date.get();
        let flags = flags.get();
        let strings = strings.read();
        let mut label = String::new();
        // The first and last dates of a selected range name the whole range.
        if flags.range_boundary {
            selected_date_description.with(|description| label.push_str(description));
            label.push_str(", ");
        }
        label.push_str(&formatters.full_date(date));
        let mut label = match (flags.today, flags.selected) {
            (true, true) => strings.today_date_selected(&label),
            (true, false) => strings.today_date(&label),
            (false, true) => strings.date_selected(&label),
            (false, false) => label,
        };
        if flags.is_min {
            label.push_str(", ");
            label.push_str(&strings.minimum_date());
        } else if flags.is_max {
            label.push_str(", ");
            label.push_str(&strings.maximum_date());
        }
        label
    });

    // In a range calendar, the focused cell says how to select.
    let prompt_id = range.map(|range| {
        use_description(Signal::derive(move || {
            (focus.with(|focus| focus.is_focused)
                && !calendar.is_read_only.get()
                && flags.with(CellFlags::selectable))
            .then(|| {
                if range.anchor_date.get().is_some() {
                    strings.read().finish_range_selection_prompt()
                } else {
                    strings.read().start_range_selection_prompt()
                }
            })
        }))
    });

    // A range calendar's press state (a single date needs none).
    let range_press = range.map(|range| (range, StoredValue::new(RangePress::default())));
    if let Some((_, press)) = range_press {
        // A calendar closed within the delay (e.g. in a popover) doesn't select.
        on_cleanup(move || {
            if let Some(Some(timer)) = press.try_with_value(|press| press.touch_drag_timer) {
                timer.clear();
            }
        });
    }
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
        let Some((range, press)) = range_press else {
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
            && !flags.with_untracked(|flags| flags.invalid)
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
                press.update_value(|press| press.is_range_boundary_pressed = true);
                return;
            }
        }
        // Mouse and touch start the range on press, so that users can drag; touch only after a
        // moment, as the user may be scrolling.
        let start_dragging = move || {
            range.set_dragging(true);
            press.update_value(|press| press.touch_drag_timer = None);
            focus_and_select();
            press.update_value(|press| press.is_anchor_pressed = true);
        };
        if e.pointer_type == PointerType::Touch {
            let timer = set_timeout_with_handle(start_dragging, Duration::from_millis(200)).ok();
            press.update_value(|press| press.touch_drag_timer = timer);
        } else {
            start_dragging();
        }
    });
    let on_press_end = range_press.map(|(_, press)| {
        Callback::new(move |_: PressEvent| {
            let timer = press.try_update_value(|press| {
                press.is_range_boundary_pressed = false;
                press.is_anchor_pressed = false;
                press.touch_drag_timer.take()
            });
            if let Some(Some(timer)) = timer {
                timer.clear();
            }
        })
    });
    // A single date is selected on press (up).
    let on_press = range.is_none().then(|| {
        Callback::new(move |_: PressEvent| {
            if !calendar.is_read_only.get_untracked() {
                focus_and_select();
            }
        })
    });
    let on_press_up = range_press.map(|(range, press)| {
        Callback::new(move |e: PressEvent| {
            if calendar.is_read_only.get_untracked() {
                return;
            }
            // The anchor as the press found it (react-aria reads the one of its render): a quick
            // tap below sets it, which mustn't count as the range's other end.
            let anchor = range.anchor_date.get_untracked();
            // A quick tap: the timer is still running, the date not selected yet.
            if press.with_value(|press| press.touch_drag_timer.is_some()) {
                focus_and_select();
            }
            let (is_range_boundary_pressed, is_anchor_pressed) = press
                .with_value(|press| (press.is_range_boundary_pressed, press.is_anchor_pressed));
            if is_range_boundary_pressed {
                // Pressing an end of the selected range starts a new range there on release.
                range.set_anchor_date(Some(date.get_untracked()));
            } else if anchor.is_some() && !is_anchor_pressed {
                // Releasing a drag, or pressing the other end: select it.
                focus_and_select();
            } else if e.pointer_type == PointerType::Keyboard && anchor.is_none() {
                // Keyboard selection moves on by a day, to show that a range is being selected.
                state.select_date(date.get_untracked());
                range.focus_nearest_available_date(date.get_untracked());
            } else if e.pointer_type == PointerType::Virtual {
                focus_and_select();
            }
        })
    });

    let press = use_press(UsePressInput {
        // Dragging back over the anchor shouldn't start a new press.
        should_cancel_on_pointer_exit: match range {
            Some(range) => Signal::derive(move || range.anchor_date.get().is_some()),
            None => never,
        },
        prevent_focus_on_press: Signal::stored(true),
        is_disabled: Signal::derive(move || {
            !flags.with(CellFlags::selectable) || calendar.is_read_only.get()
        }),
        on_press_start: Some(on_press_start),
        on_press_end,
        on_press,
        on_press_up,
        propagation: crate::hooks::interactions::PressPropagation::Stop,
        allow_text_selection_on_press: never,
        force_is_pressed: never,
        on_press_change: None,
        on_double_press: None,
        long_press: None,
    });
    let (press_props, press_styles) = press.target.into_props().into_inner();

    let tabindex = Signal::derive(move || {
        (!flags.with(|flags| flags.disabled)).then(|| {
            if focus.with(|focus| focus.is_focused_date) {
                0
            } else {
                -1
            }
        })
    });

    // The browser's focus follows the focused date (one effect per calendar).
    data.cells.register(date, is_outside_month, element);

    let error_message_id = data.error_message_id;
    // Without a range, the error message is the only contribution: passed on as it is.
    let aria_describedby = IdRefs::derive(
        [Signal::derive(move || {
            flags
                .with(|flags| flags.invalid)
                .then(|| error_message_id.get())
                .flatten()
        })]
        .into_iter()
        .chain(prompt_id),
    );

    // The cell and its button share them.
    let aria_disabled = aria_flag(flags, |flags| !flags.selectable());
    let aria_invalid = aria_flag(flags, |flags| flags.invalid);
    UseCalendarCellReturn {
        cell: Part::stored(UseCalendarCellProps {
            role: AriaRole::Gridcell,
            aria_disabled,
            aria_selected: aria_flag(flags, |flags| flags.selected),
            aria_invalid,
        }),
        button: Part::stored(PropsWithStyles::new(
            UseCalendarCellButtonProps {
                press: press_props,
                role: AriaRole::Button,
                tabindex,
                aria_label: label,
                aria_disabled,
                aria_invalid,
                aria_describedby,
                on_focus: EventHandler::new(move |_: FocusEvent| {
                    if !flags.with_untracked(|flags| flags.disabled) {
                        calendar.set_focused_date(date.get_untracked());
                        calendar.set_focused(true);
                    }
                }),
                // Hovering (or dragging over) a date while selecting a range highlights it.
                on_pointerenter: EventHandler::new(move |e: PointerEvent| {
                    if let Some(range) = range
                        && (PointerType::of(&e) != PointerType::Touch
                            || range.is_dragging.get_untracked())
                        && flags.with_untracked(CellFlags::selectable)
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
        )),
        is_pressed: press.is_pressed,
        is_focused,
        is_selected: flag(flags, |flags| flags.selected),
        is_disabled: flag(flags, |flags| flags.disabled),
        is_unavailable: flag(flags, |flags| flags.unavailable),
        is_outside_visible_range: flag(flags, |flags| flags.outside_visible_range),
        is_invalid: flag(flags, |flags| flags.invalid),
        is_today: flag(flags, |flags| flags.today),
        is_selection_start: range.map_or(never, |_| flag(flags, |flags| flags.selection_start)),
        is_selection_end: range.map_or(never, |_| flag(flags, |flags| flags.selection_end)),
        formatted_date: Signal::derive(move || formatters.day(date.get())),
    }
}
