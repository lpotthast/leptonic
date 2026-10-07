// Upstream: react-aria/src/calendar/useCalendarGrid.ts @ 99e6102368
use jiff::civil::Date;
use leptos::{
    attr::{self, Attr},
    ev::{self, On, SharedEventCallback},
    prelude::*,
};
use wasm_bindgen::JsCast;
use web_sys::{FocusEvent, KeyboardEvent};

use super::states::{CalendarData, CalendarStates, visible_range_description};
use crate::hooks::form::use_label::labels;
use crate::{
    hooks::{IntoAttrs, UseKeyboardInput, use_keyboard},
    utils::{
        EventAccessors, EventHandler,
        aria::AriaRole,
        date::{DateDuration, DateExt, DateRange, today},
        date_time_formatter::{DateTimeFormat, DateTimeFormatOptions, DateTimeFormatter},
        focusability::is_focusable,
        i18n::{use_direction, use_locale},
        id::use_id,
        keyboard_shortcut::{KeyboardShortcuts, Shortcut, ShortcutOutcome},
        locale::WritingDirection,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Takes the calendar's `CalendarData` (react-aria: the state, with the calendar's data in a
//   `WeakMap`).
// - `weekday_style` is a `DateTimeFormat` (react-aria: a string).
//
// ## LEPTOS-SPECIFIC ADAPTATIONS
// - A focused cell losing the focus because paging made it unfocusable doesn't unfocus the
//   calendar (see `is_focus_fixup`); disabling the calendar does.
//
// =============================================================================

/// Input of [`use_calendar_grid`].
#[derive(Clone)]
pub struct UseCalendarGridInput {
    pub data: CalendarData,
    /// The first date of the grid's month. Default: the start of the visible range (set it for
    /// the further months of a calendar showing several).
    pub start_date: Option<Signal<Date>>,
    /// The grid's last date. Default: the end of the visible range.
    pub end_date: Option<Signal<Date>>,
    /// How the weekday names in the header are formatted. Default: narrow ("M").
    pub weekday_style: DateTimeFormat,
}

/// Return value of [`use_calendar_grid`].
pub struct UseCalendarGridReturn {
    /// For the grid (a `table`).
    pub grid_props: UseCalendarGridProps,
    /// The grid's first date (its month's first day).
    pub start_date: Signal<Date>,
    /// The names of the weekdays, for the column headers (hidden from assistive technology: each
    /// cell's label names its weekday).
    pub week_days: Signal<Vec<String>>,
    /// The number of week rows.
    pub weeks_in_month: Signal<u8>,
}

/// Props of the calendar's grid.
#[derive(Debug, Clone)]
pub struct UseCalendarGridProps {
    /// Part of the grid's `aria-labelledby` when the calendar is labelled by other elements.
    pub id: String,
    pub role: AriaRole,
    pub aria_label: Signal<Option<String>>,
    pub aria_labelledby: Option<String>,
    pub aria_readonly: Signal<Option<&'static str>>,
    pub aria_disabled: Signal<Option<&'static str>>,
    pub aria_multiselectable: Option<&'static str>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_keyup: EventHandler<KeyboardEvent>,
}

pub type UseCalendarGridAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaLabel, Signal<Option<String>>>,
    Attr<attr::AriaLabelledby, Option<String>>,
    Attr<attr::AriaReadonly, Signal<Option<&'static str>>>,
    Attr<attr::AriaDisabled, Signal<Option<&'static str>>>,
    Attr<attr::AriaMultiselectable, Option<&'static str>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::keyup, SharedEventCallback<KeyboardEvent>>,
);

impl IntoAttrs for UseCalendarGridProps {
    type Attrs = UseCalendarGridAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaReadonly, self.aria_readonly),
            Attr(attr::AriaDisabled, self.aria_disabled),
            Attr(attr::AriaMultiselectable, self.aria_multiselectable),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
            self.on_keydown.into_on(ev::keydown),
            self.on_keyup.into_on(ev::keyup),
        )
    }
}

/// The keyboard shortcuts of a calendar grid: arrows move by a day or a week, Page Up/Down by a
/// month (with Shift: a year), Home/End to the section's ends, Enter/Space select, Escape
/// cancels a range being selected.
fn shortcuts(state: &CalendarStates, direction: Signal<WritingDirection>) -> KeyboardShortcuts {
    let state = *state;
    let calendar = state.calendar();
    // Home, End and Escape don't repeat (react-aria: a separate `useKeyboard` without repeats).
    let once = |action: fn(&CalendarStates)| {
        move |e: &KeyboardEvent| {
            if e.repeat() {
                ShortcutOutcome::Ignored
            } else {
                action(&state);
                ShortcutOutcome::Handled
            }
        }
    };
    KeyboardShortcuts::new()
        .on(
            Shortcut::key("End"),
            once(|state| state.calendar().focus_section_end()),
        )
        .on(
            Shortcut::key("Home"),
            once(|state| state.calendar().focus_section_start()),
        )
        .on(Shortcut::key("Escape"), move |e: &KeyboardEvent| {
            if e.repeat() {
                return ShortcutOutcome::Ignored;
            }
            // Cancels a range being selected; the Escape key goes on (e.g. to close a popover).
            if let Some(range) = state.range() {
                range.set_anchor_date(None);
            }
            ShortcutOutcome::Ignored
        })
        .on(Shortcut::key("Enter"), move |_| state.select_focused_date())
        .on(Shortcut::key(" "), move |_| state.select_focused_date())
        .on(Shortcut::key("PageUp"), move |_| {
            calendar.focus_previous_section(false);
        })
        .on(Shortcut::key("PageUp").shift(), move |_| {
            calendar.focus_previous_section(true);
        })
        .on(Shortcut::key("PageDown"), move |_| {
            calendar.focus_next_section(false);
        })
        .on(Shortcut::key("PageDown").shift(), move |_| {
            calendar.focus_next_section(true);
        })
        .on(Shortcut::key("ArrowLeft"), move |_| {
            if direction.get_untracked() == WritingDirection::Rtl {
                calendar.focus_next_day();
            } else {
                calendar.focus_previous_day();
            }
        })
        .on(Shortcut::key("ArrowRight"), move |_| {
            if direction.get_untracked() == WritingDirection::Rtl {
                calendar.focus_previous_day();
            } else {
                calendar.focus_next_day();
            }
        })
        .on(Shortcut::key("ArrowUp"), move |_| {
            calendar.focus_previous_row();
        })
        .on(Shortcut::key("ArrowDown"), move |_| {
            calendar.focus_next_row();
        })
}

/// Whether a cell lost the focus because it can't have it any more: paging (or new min/max)
/// moved it to another month (disabling it) or removed its row. The browser then blurs it (focus fixup) before the
/// newly focused cell takes the focus, which isn't the user leaving the grid. (react-aria
/// decides which cell is focused while rendering, before the browser blurs.)
fn is_focus_fixup(e: &FocusEvent) -> bool {
    if e.related_target().is_some() {
        return false;
    }
    let Ok(target) = e.expect_target().dyn_into::<web_sys::Element>() else {
        return false;
    };
    !target.is_connected() || !is_focusable(&target)
}

/// Behavior and accessibility of a calendar's grid of dates: keyboard navigation, its label, the
/// weekday names and the number of week rows.
pub fn use_calendar_grid(input: UseCalendarGridInput) -> UseCalendarGridReturn {
    let UseCalendarGridInput {
        data,
        start_date,
        end_date,
        weekday_style,
    } = input;
    let state = data.state;
    let calendar = state.calendar();
    let locale = use_locale();
    let direction = use_direction();

    let start_date =
        start_date.unwrap_or_else(|| Signal::derive(move || calendar.visible_range.get().start));
    let end_date =
        end_date.unwrap_or_else(|| Signal::derive(move || calendar.visible_range.get().end));
    let range = Signal::derive(move || DateRange {
        start: start_date.get(),
        end: end_date.get(),
    });

    let keyboard = use_keyboard(UseKeyboardInput {
        shortcuts: Some(shortcuts(&state, direction)),
        allow_repeats: true,
        ..UseKeyboardInput::default()
    });

    let id = use_id("calendar-grid");
    let aria_label = data.aria_label;
    let labelledby = data.aria_labelledby.clone();
    let own_id = id.clone();
    let label = Signal::derive(move || {
        let label = [
            aria_label.get(),
            Some(visible_range_description(range.get(), &locale.get())),
        ]
        .into_iter()
        .flatten()
        .filter(|label| !label.is_empty())
        .collect::<Vec<_>>()
        .join(", ");
        labels(&own_id, Some(label), labelledby.clone()).0
    });
    let (_, aria_labelledby) = labels(&id, Some(String::new()), data.aria_labelledby);

    let week_days = Signal::derive(move || {
        let formatter = DateTimeFormatter::new(
            &locale.get(),
            DateTimeFormatOptions {
                weekday: Some(weekday_style),
                ..DateTimeFormatOptions::default()
            },
        );
        let days = calendar.visible_duration.days;
        let (first, count) = if (1..7).contains(&days) {
            (start_date.get(), days)
        } else {
            (today().start_of_week(calendar.first_day_of_week.get()), 7)
        };
        (0..count)
            .map(|day| formatter.format_date(first.add(DateDuration::days(day))))
            .collect()
    });
    let weeks_in_month = Signal::derive(move || calendar.weeks_in_month(Some(start_date.get())));

    UseCalendarGridReturn {
        grid_props: UseCalendarGridProps {
            id,
            role: AriaRole::Grid,
            aria_label: label,
            aria_labelledby,
            aria_readonly: Signal::derive(move || calendar.is_read_only.get().then_some("true")),
            aria_disabled: Signal::derive(move || calendar.is_disabled.get().then_some("true")),
            aria_multiselectable: state.range().is_some().then_some("true"),
            on_focusin: EventHandler::new(move |_: FocusEvent| calendar.set_focused(true)),
            on_focusout: EventHandler::new(move |e: FocusEvent| {
                // After the calendar's disposal (a removed focused cell): nothing to update
                // ("Blur After Disposal").
                if !calendar.is_alive() {
                    return;
                }
                // Disabling the calendar takes the focus for real.
                if calendar.is_disabled.get_untracked() || !is_focus_fixup(&e) {
                    calendar.set_focused(false);
                }
            }),
            on_keydown: keyboard.props.on_keydown,
            on_keyup: keyboard.props.on_keyup,
        },
        start_date,
        week_days,
        weeks_in_month,
    }
}
