//! `use_calendar_cell` in a calendar of hooks, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_calendar_cell.rs`): a calendar (`#perf-calendar`) of
//! June 2024 (`use_calendar_state` or `use_range_calendar_state`, June 12 focused; fixed dates,
//! never today) with `use_calendar`, a grid (`use_calendar_grid`) and a cell per date
//! (`use_calendar_cell`, keyed by date), between two plain buttons (`#perf-before`,
//! `#perf-after`), mounted at first with a single date. Its controls ([`Control`]) mount and
//! unmount the calendar, switch it to a range calendar while unmounted (`use-range`), disable and
//! enable the cells (their `is_disabled`), and read the value (`value`, `start`, `end`:
//! `yyyymmdd`, 0 without).
//!
//! The page creates the live announcer on load (`announce("")`): the calendar announces its
//! selection, and the first announcement of a page waits 100 ms for the live regions.

use leptonic::{
    Assertiveness, CapturedElement, DateExt, DateRange, DateTimeFormat, announce, flag,
    hooks::calendar::{
        CalendarData, CommitBehavior, UseCalendarCellInput, UseCalendarCellReturn,
        UseCalendarGridInput, UseCalendarInput, UseCalendarReturn, UseCalendarStateInput,
        UseRangeCalendarInput, UseRangeCalendarStateInput, use_calendar, use_calendar_cell,
        use_calendar_grid, use_calendar_state, use_range_calendar, use_range_calendar_state,
    },
    jiff::civil::{Date, date},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// `date` as `yyyymmdd`, for the read controls (0: none).
fn number(date: Option<Date>) -> f64 {
    date.map_or(0.0, |date| {
        f64::from(date.year()) * 10_000.0 + f64::from(date.month()) * 100.0 + f64::from(date.day())
    })
}

/// The visible month's grid: a row per week, a cell per date.
#[component]
fn MonthGrid(data: CalendarData, cells_disabled: RwSignal<bool>) -> impl IntoView {
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
                                is_disabled=cells_disabled.into()
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
fn DayCell(
    data: CalendarData,
    date: Date,
    is_outside_month: Signal<bool>,
    is_disabled: Signal<bool>,
) -> impl IntoView {
    let UseCalendarCellReturn {
        cell: cell_props,
        button: button_props,
        is_selected,
        is_pressed,
        formatted_date,
        ..
    } = use_calendar_cell(UseCalendarCellInput {
        data,
        date: date.into(),
        is_disabled,
        is_outside_month,
        element: CapturedElement::new(),
    });
    let (attrs, styles) = button_props.attrs();
    view! {
        <td {..cell_props.attrs()}>
            <div
                {..attrs}
                style=styles
                data-selected=flag(is_selected)
                data-pressed=flag(is_pressed)
                data-outside-month=flag(is_outside_month)
            >
                {formatted_date}
            </div>
        </td>
    }
}

/// A calendar of one date.
#[component]
fn SingleCalendar(value: RwSignal<Option<Date>>, cells_disabled: RwSignal<bool>) -> impl IntoView {
    let state = use_calendar_state(UseCalendarStateInput {
        value: Some(value.into()),
        default_focused_value: Some(date(2024, 6, 12)),
        ..UseCalendarStateInput::default()
    });
    let UseCalendarReturn {
        calendar: calendar_props,
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
    view! {
        <div {..calendar_props.attrs()}>
            <MonthGrid data=data cells_disabled=cells_disabled />
        </div>
    }
}

/// A calendar of a range.
#[component]
fn RangeCalendar(
    value: RwSignal<Option<DateRange>>,
    cells_disabled: RwSignal<bool>,
) -> impl IntoView {
    let state = use_range_calendar_state(UseRangeCalendarStateInput {
        value: Some(value.into()),
        default_focused_value: Some(date(2024, 6, 12)),
        ..UseRangeCalendarStateInput::default()
    });
    let UseCalendarReturn {
        calendar: calendar_props,
        data,
        ..
    } = use_range_calendar(UseRangeCalendarInput {
        state,
        commit_behavior: CommitBehavior::Select,
        id: Some("perf-calendar".into()),
        aria_label: "Trip".into(),
        aria_labelledby: Signal::default(),
        aria_describedby: Signal::default(),
        aria_details: Signal::default(),
    });
    view! {
        <div {..calendar_props.attrs()}>
            <MonthGrid data=data cells_disabled=cells_disabled />
        </div>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let range = RwSignal::new(false);
    let value = RwSignal::new(None::<Date>);
    let range_value = RwSignal::new(None::<DateRange>);
    let cells_disabled = RwSignal::new(false);
    Effect::new(|| announce("", Assertiveness::Polite));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("use-range", move || range.set(true)),
        Control::action("disable", move || cells_disabled.set(true)),
        Control::action("enable", move || cells_disabled.set(false)),
        Control::read("value", move || number(value.get_untracked())),
        Control::read("start", move || {
            number(range_value.get_untracked().map(|range| range.start))
        }),
        Control::read("end", move || {
            number(range_value.get_untracked().map(|range| range.end))
        }),
    ];
    view! {
        <h1>"use_calendar_cell performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            {move || {
                if range.get() {
                    view! { <RangeCalendar value=range_value cells_disabled=cells_disabled /> }
                        .into_any()
                } else {
                    view! { <SingleCalendar value=value cells_disabled=cells_disabled /> }
                        .into_any()
                }
            }}
        </Show>
        <button id="perf-after">"After"</button>
    }
}
