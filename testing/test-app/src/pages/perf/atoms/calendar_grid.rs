//! A `CalendarGrid` in a `Calendar` (`#perf-calendar`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/calendar_grid.rs`): June 2024 (June 12 focused) with its
//! heading and next button, the grid between two plain buttons (`#perf-before`, `#perf-after`)
//! to tab from and to, mounted at first. Its controls ([`Control`]) mount and unmount the grid
//! (the calendar stays) and read the focused date (`focused`: `yyyymmdd`).
//!
//! The page creates the live announcer on load (`announce("")`): the calendar announces its
//! visible range, and the first announcement of a page waits 100 ms for the live regions.

use leptonic::{
    Assertiveness, announce,
    atoms::calendar::{
        Calendar, CalendarCell, CalendarCellButton, CalendarGrid, CalendarGridBody,
        CalendarGridHeader, CalendarHeaderCell, CalendarHeaderRow, CalendarHeading,
        CalendarNextButton, CalendarWeek,
    },
    jiff::civil::{Date, date},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// `date` as `yyyymmdd`, for the read controls.
fn number(date: Date) -> f64 {
    f64::from(date.year()) * 10_000.0 + f64::from(date.month()) * 100.0 + f64::from(date.day())
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let focused = RwSignal::new(date(2024, 6, 12));
    Effect::new(|| announce("", Assertiveness::Polite));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::read("focused", move || number(focused.get_untracked())),
    ];
    view! {
        <h1>"CalendarGrid performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Calendar
            id="perf-calendar"
            aria_label="Event date"
            focused_value=focused
            set_focused_value=focused
        >
            <CalendarHeading />
            <CalendarNextButton>">"</CalendarNextButton>
            <Show when=move || mounted.get()>
                <CalendarGrid>
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
            </Show>
        </Calendar>
        <button id="perf-after">"After"</button>
    }
}
