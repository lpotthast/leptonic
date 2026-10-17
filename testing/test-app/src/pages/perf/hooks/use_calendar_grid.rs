//! `use_calendar_grid` in a calendar of hooks, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_calendar_grid.rs`): a calendar (`#perf-calendar`) of
//! June 2024 (`use_calendar_state`, June 12 focused; fixed dates, never today) with
//! `use_calendar`, its next button (`use_button`), and the grid (`use_calendar_grid`) of the
//! visible month with a cell per date (`use_calendar_cell`, keyed by date), between two plain
//! buttons (`#perf-before`, `#perf-after`), mounted at first. Its controls ([`Control`]) mount
//! and unmount the grid (the calendar stays) and read the focused date (`focused`: `yyyymmdd`).
//!
//! The page creates the live announcer on load (`announce("")`): the calendar announces its
//! visible range, and the first announcement of a page waits 100 ms for the live regions.

use leptonic::{
    Assertiveness, CapturedElement, DateExt, DateTimeFormat, announce, flag,
    hooks::{
        button::use_button,
        calendar::{
            CalendarData, UseCalendarCellInput, UseCalendarCellReturn, UseCalendarGridInput,
            UseCalendarInput, UseCalendarReturn, UseCalendarStateInput, use_calendar,
            use_calendar_cell, use_calendar_grid, use_calendar_state,
        },
    },
    jiff::civil::{Date, date},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// `date` as `yyyymmdd`, for the read controls.
fn number(date: Date) -> f64 {
    f64::from(date.year()) * 10_000.0 + f64::from(date.month()) * 100.0 + f64::from(date.day())
}

/// The visible month's grid: a row per week, a cell per date.
#[component]
fn MonthGrid(data: CalendarData) -> impl IntoView {
    let calendar = data.state.calendar();
    let grid = use_calendar_grid(UseCalendarGridInput {
        data: data.clone(),
        start_date: None,
        end_date: None,
        weekday_style: DateTimeFormat::Narrow,
    });
    let month = grid.start_date;
    let data = StoredValue::new(data);
    view! {
        <table {..grid.grid.attrs()}>
            <tbody>
                <For each=move || 0..grid.weeks_in_month.get() key=|week| *week let(week)>
                    <tr>
                        <For
                            each=move || {
                                calendar.dates_in_week(week, Some(month.get())).into_iter().flatten()
                            }
                            key=|date| *date
                            let(date)
                        >
                            <DayCell
                                data=data.get_value()
                                date=date
                                is_outside_month=Signal::derive(move || {
                                    !date.is_same_month(month.get())
                                })
                            />
                        </For>
                    </tr>
                </For>
            </tbody>
        </table>
    }
}

/// A date: its cell and button.
#[component]
fn DayCell(data: CalendarData, date: Date, is_outside_month: Signal<bool>) -> impl IntoView {
    let UseCalendarCellReturn {
        cell: cell_props,
        button: button_props,
        is_focused,
        formatted_date,
        ..
    } = use_calendar_cell(UseCalendarCellInput {
        data,
        date: date.into(),
        is_disabled: Signal::stored(false),
        is_outside_month,
        element: CapturedElement::new(),
    });
    let (attrs, styles) = button_props.attrs();
    view! {
        <td {..cell_props.attrs()}>
            <div
                {..attrs}
                style=styles
                data-focused=flag(is_focused)
                data-outside-month=flag(is_outside_month)
            >
                {formatted_date}
            </div>
        </td>
    }
}

/// The calendar: its state, `use_calendar`, its next button and (while `grid_mounted`) its grid.
#[component]
fn HookCalendar(focused: RwSignal<Date>, grid_mounted: RwSignal<bool>) -> impl IntoView {
    let state = use_calendar_state(UseCalendarStateInput {
        focused_value: Some(focused.into()),
        ..UseCalendarStateInput::default()
    });
    let UseCalendarReturn {
        calendar: calendar_props,
        next_button,
        data,
        ..
    } = use_calendar(UseCalendarInput {
        state,
        id: Some("perf-calendar".into()),
        aria_label: "Event date".into(),
        aria_labelledby: Signal::default(),
        aria_describedby: Signal::default(),
        aria_details: Signal::default(),
    });
    let (next_attrs, next_styles) = use_button(next_button).button.attrs();
    let data = StoredValue::new(data);
    view! {
        <div {..calendar_props.attrs()}>
            <button {..next_attrs} style=next_styles>
                ">"
            </button>
            <Show when=move || grid_mounted.get()>
                <MonthGrid data=data.get_value() />
            </Show>
        </div>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let grid_mounted = RwSignal::new(true);
    let focused = RwSignal::new(date(2024, 6, 12));
    Effect::new(|| announce("", Assertiveness::Polite));
    let controls = vec![
        Control::action("mount", move || grid_mounted.set(true)),
        Control::action("unmount", move || grid_mounted.set(false)),
        Control::read("focused", move || number(focused.get_untracked())),
    ];
    view! {
        <h1>"use_calendar_grid performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <HookCalendar focused=focused grid_mounted=grid_mounted />
        <button id="perf-after">"After"</button>
    }
}
