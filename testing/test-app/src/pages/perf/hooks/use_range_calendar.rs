//! `use_range_calendar` on plain elements, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_range_calendar.rs`): a range calendar
//! (`#perf-calendar`) of June 2024 (`use_range_calendar_state`, June 12 focused; fixed dates,
//! never today, its range app state) with its title (`#perf-title`), previous and next buttons
//! (`use_button`) and the grid of the visible month (`use_calendar_grid`, a cell per date:
//! `use_calendar_cell`), between two plain buttons (`#perf-before`, `#perf-after`), mounted at
//! first. A range being selected is committed (finished at the focused date) when the focus leaves
//! the calendar or a pointer is released outside its dates. Its controls ([`Control`]) mount and
//! unmount it, change its inputs (label, disabled) and read its range (`start`, `end`:
//! `yyyymmdd`, 0 without).
//!
//! The page creates the live announcer on load (`announce("")`): the calendar announces its
//! visible range and selection, and the first announcement of a page waits 100 ms for the live
//! regions.

use leptonic::{
    Assertiveness, CapturedElement, DateExt, DateRange, DateTimeFormat, announce, flag,
    hooks::{
        button::use_button,
        calendar::{
            CalendarData, CommitBehavior, UseCalendarCellInput, UseCalendarCellReturn,
            UseCalendarGridInput, UseCalendarReturn, UseRangeCalendarInput,
            UseRangeCalendarStateInput, use_calendar_cell, use_calendar_grid, use_range_calendar,
            use_range_calendar_state,
        },
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
        is_selected,
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
                data-selected=flag(is_selected)
                data-outside-month=flag(is_outside_month)
            >
                {formatted_date}
            </div>
        </td>
    }
}

/// The range calendar: its state, `use_range_calendar` and the elements it returns props for.
#[component]
fn HookRangeCalendar(
    value: RwSignal<Option<DateRange>>,
    label: RwSignal<String>,
    disabled: RwSignal<bool>,
) -> impl IntoView {
    let state = use_range_calendar_state(UseRangeCalendarStateInput {
        value: Some(value.into()),
        default_focused_value: Some(date(2024, 6, 12)),
        is_disabled: disabled.into(),
        ..UseRangeCalendarStateInput::default()
    });
    let UseCalendarReturn {
        calendar: calendar_props,
        previous_button,
        next_button,
        title,
        data,
        ..
    } = use_range_calendar(UseRangeCalendarInput {
        state,
        commit_behavior: CommitBehavior::Select,
        id: Some("perf-calendar".into()),
        aria_label: label.into(),
        aria_labelledby: Signal::default(),
        aria_describedby: Signal::default(),
        aria_details: Signal::default(),
    });
    let (previous_attrs, previous_styles) = use_button(previous_button).button.attrs();
    let (next_attrs, next_styles) = use_button(next_button).button.attrs();
    view! {
        <div {..calendar_props.attrs()}>
            <button {..previous_attrs} style=previous_styles>
                "<"
            </button>
            <h2 id="perf-title">{title}</h2>
            <button {..next_attrs} style=next_styles>
                ">"
            </button>
            <MonthGrid data=data />
        </div>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let value = RwSignal::new(None::<DateRange>);
    let label = RwSignal::new("Trip".to_owned());
    let disabled = RwSignal::new(false);
    Effect::new(|| announce("", Assertiveness::Polite));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("relabel", move || label.set("Holiday".to_owned())),
        Control::action("label-back", move || label.set("Trip".to_owned())),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("start", move || {
            number(value.get_untracked().map(|range| range.start))
        }),
        Control::read("end", move || {
            number(value.get_untracked().map(|range| range.end))
        }),
    ];
    view! {
        <h1>"use_range_calendar performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <HookRangeCalendar value=value label=label disabled=disabled />
        </Show>
        <button id="perf-after">"After"</button>
    }
}
