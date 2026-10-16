//! The `RangeCalendar` alone (`#perf-calendar`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/range_calendar.rs`): June 2024 (June 12 focused, no range
//! selected) with its heading, previous and next buttons, a grid per
//! visible month and its error message, between two plain buttons (`#perf-before`,
//! `#perf-after`) to tab from and to, mounted at first. Its range and focused date are app state
//! (controlled). Its controls ([`Control`]) mount and unmount it, show one month, twelve or a
//! week, change its reactive props, and read its range and focused date (`start`, `end`,
//! `focused`: `yyyymmdd`, 0 without).
//!
//! The page creates the live announcer on load (`announce("")`): the calendar announces its
//! visible range and selection, and the first announcement of a page waits 100 ms for the live
//! regions, after a measurement would have ended.

use leptonic::{
    Assertiveness, DateDuration, DateRange, announce,
    atoms::calendar::{
        CalendarCell, CalendarCellButton, CalendarErrorMessage, CalendarGrid, CalendarGridBody,
        CalendarGridHeader, CalendarHeaderCell, CalendarHeaderRow, CalendarHeading,
        CalendarNextButton, CalendarPreviousButton, CalendarWeek, RangeCalendar,
    },
    hooks::calendar::{PageBehavior, SelectionAlignment},
    jiff::civil::{Date, Weekday, date},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The date focused at first.
const FOCUSED: Date = date(2024, 6, 12);

/// `date` as `yyyymmdd`, for the read controls (0: none).
fn number(date: Option<Date>) -> f64 {
    date.map_or(0.0, |date| {
        f64::from(date.year()) * 10_000.0 + f64::from(date.month()) * 100.0 + f64::from(date.day())
    })
}

/// A month's grid: the weekday names and a row per week of cells with their buttons.
#[component]
fn Grid(offset: DateDuration) -> impl IntoView {
    view! {
        <CalendarGrid offset=offset>
            <CalendarGridHeader>
                <CalendarHeaderRow children=|day| {
                    view! { <CalendarHeaderCell>{day}</CalendarHeaderCell> }
                } />
            </CalendarGridHeader>
            <CalendarGridBody children=|week| {
                view! {
                    <CalendarWeek
                        week=week
                        children=|date| {
                            view! {
                                <CalendarCell date=date>
                                    <CalendarCellButton />
                                </CalendarCell>
                            }
                        }
                    />
                }
            } />
        </CalendarGrid>
    }
}

#[component]
#[allow(clippy::too_many_lines)]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let months = RwSignal::new(1_i32);
    let week_view = RwSignal::new(false);
    let value = RwSignal::new(None::<DateRange>);
    let focused = RwSignal::new(FOCUSED);
    let min_value = RwSignal::new(None::<Date>);
    let max_value = RwSignal::new(None::<Date>);
    let disabled = RwSignal::new(false);
    let read_only = RwSignal::new(false);
    let invalid = RwSignal::new(false);
    let page_behavior = RwSignal::new(PageBehavior::Visible);
    let first_day = RwSignal::new(None::<Weekday>);
    let alignment = RwSignal::new(None::<SelectionAlignment>);
    let weeks_in_month = RwSignal::new(None::<u8>);
    let label = RwSignal::new("Trip".to_owned());
    let visible_duration = Signal::derive(move || {
        if week_view.get() {
            DateDuration::weeks(1)
        } else {
            DateDuration::months(months.get())
        }
    });
    Effect::new(|| announce("", Assertiveness::Polite));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("show-twelve-months", move || months.set(12)),
        Control::action("show-one-month", move || months.set(1)),
        Control::action("show-a-week", move || week_view.set(true)),
        Control::action("show-months", move || week_view.set(false)),
        Control::action("set-value", move || {
            value.set(Some(DateRange {
                start: date(2024, 6, 17),
                end: date(2024, 6, 19),
            }));
        }),
        Control::action("clear-value", move || value.set(None)),
        Control::action("focus-october", move || focused.set(date(2024, 10, 15))),
        Control::action("focus-june", move || focused.set(FOCUSED)),
        Control::action("set-min", move || min_value.set(Some(date(2024, 6, 10)))),
        Control::action("set-max", move || max_value.set(Some(date(2024, 6, 25)))),
        Control::action("clear-limits", move || {
            min_value.set(None);
            max_value.set(None);
        }),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("make-read-only", move || read_only.set(true)),
        Control::action("make-editable", move || read_only.set(false)),
        Control::action("invalidate", move || invalid.set(true)),
        Control::action("validate", move || invalid.set(false)),
        Control::action("page-single", move || {
            page_behavior.set(PageBehavior::Single);
        }),
        Control::action("page-visible", move || {
            page_behavior.set(PageBehavior::Visible);
        }),
        Control::action("start-on-monday", move || {
            first_day.set(Some(Weekday::Monday));
        }),
        Control::action("start-on-locale-day", move || first_day.set(None)),
        Control::action("align-start", move || {
            alignment.set(Some(SelectionAlignment::Start));
        }),
        Control::action("align-default", move || alignment.set(None)),
        Control::action("six-weeks", move || weeks_in_month.set(Some(6))),
        Control::action("weeks-of-month", move || weeks_in_month.set(None)),
        Control::action("relabel", move || label.set("Holiday".to_owned())),
        Control::action("label-back", move || label.set("Trip".to_owned())),
        Control::read("start", move || {
            number(value.get_untracked().map(|range| range.start))
        }),
        Control::read("end", move || {
            number(value.get_untracked().map(|range| range.end))
        }),
        Control::read("focused", move || number(Some(focused.get_untracked()))),
    ];
    view! {
        <h1>"RangeCalendar performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <RangeCalendar
                id="perf-calendar"
                aria_label=label
                value=value
                set_value=value
                focused_value=focused
                set_focused_value=focused
                min_value=min_value
                max_value=max_value
                is_disabled=disabled
                is_read_only=read_only
                is_invalid=invalid
                visible_duration=visible_duration
                page_behavior=page_behavior
                first_day_of_week=first_day
                selection_alignment=alignment
                weeks_in_month=weeks_in_month
            >
                <header>
                    <CalendarPreviousButton>"<"</CalendarPreviousButton>
                    <CalendarHeading />
                    <CalendarNextButton>">"</CalendarNextButton>
                </header>
                {move || {
                    (0..months.get())
                        .map(|month| view! { <Grid offset=DateDuration::months(month) /> })
                        .collect_view()
                }}
                <CalendarErrorMessage>"Pick available dates."</CalendarErrorMessage>
            </RangeCalendar>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
