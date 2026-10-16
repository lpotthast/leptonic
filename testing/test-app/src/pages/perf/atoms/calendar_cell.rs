//! `CalendarCell`s in a `Calendar` (`#perf-calendar`), for their performance test
//! (`leptonic/tests/perf_tests/atoms/calendar_cell.rs`): June 2024 (June 12 focused, no date
//! selected), a cell per date of the grid, each with its `CalendarCellButton`, between two plain
//! buttons (`#perf-before`, `#perf-after`), mounted at first. Its controls ([`Control`]) mount and
//! unmount the cells (the calendar stays), disable and enable them (their `is_disabled`), move
//! the focused date to July (the cells' dates change) and back, and read the calendar's value
//! (`value`: `yyyymmdd`, 0 without).
//!
//! The page creates the live announcer on load (`announce("")`): the calendar announces its
//! selection, and the first announcement of a page waits 100 ms for the live regions.

use leptonic::{
    Assertiveness, announce,
    atoms::calendar::{
        Calendar, CalendarCell, CalendarCellButton, CalendarGrid, CalendarGridBody,
        CalendarGridHeader, CalendarHeaderCell, CalendarHeaderRow, CalendarHeading, CalendarWeek,
    },
    jiff::civil::{Date, date},
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

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let value = RwSignal::new(None::<Date>);
    let focused = RwSignal::new(FOCUSED);
    Effect::new(|| announce("", Assertiveness::Polite));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("focus-july", move || focused.set(date(2024, 7, 12))),
        Control::action("focus-june", move || focused.set(FOCUSED)),
        Control::read("value", move || number(value.get_untracked())),
    ];
    view! {
        <h1>"CalendarCell performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Calendar
            id="perf-calendar"
            aria_label="Event date"
            value=value
            set_value=value
            focused_value=focused
            set_focused_value=focused
        >
            <CalendarHeading />
            <CalendarGrid>
                <CalendarGridHeader>
                    <CalendarHeaderRow children=|day| {
                        view! { <CalendarHeaderCell>{day}</CalendarHeaderCell> }
                    } />
                </CalendarGridHeader>
                <CalendarGridBody children=move |week| {
                    view! {
                        <CalendarWeek
                            week=week
                            children=move |date| {
                                view! {
                                    <Show when=move || mounted.get()>
                                        <CalendarCell date=date is_disabled=disabled>
                                            <CalendarCellButton />
                                        </CalendarCell>
                                    </Show>
                                }
                            }
                        />
                    }
                } />
            </CalendarGrid>
        </Calendar>
        <button id="perf-after">"After"</button>
    }
}
