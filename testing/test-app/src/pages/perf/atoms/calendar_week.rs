//! `CalendarWeek`s in a `Calendar` (`#perf-calendar`), for their performance test
//! (`leptonic/tests/perf_tests/atoms/calendar_week.rs`): June 2024 (six weeks; June 12 focused)
//! with its heading and next button, a week row per week of the grid, mounted at first. Its
//! controls ([`Control`]) mount and unmount the weeks (the calendar, its grid and body stay).
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
    jiff::civil::date,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    Effect::new(|| announce("", Assertiveness::Polite));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
    ];
    view! {
        <h1>"CalendarWeek performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Calendar
            id="perf-calendar"
            aria_label="Event date"
            default_focused_value=date(2024, 6, 12)
        >
            <CalendarHeading />
            <CalendarNextButton>">"</CalendarNextButton>
            <CalendarGrid>
                <CalendarGridHeader>
                    <CalendarHeaderRow children=|day| {
                        view! { <CalendarHeaderCell>{day}</CalendarHeaderCell> }
                    } />
                </CalendarGridHeader>
                <CalendarGridBody children=move |week| {
                    view! {
                        <Show when=move || mounted.get()>
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
                        </Show>
                    }
                } />
            </CalendarGrid>
        </Calendar>
        <button id="perf-after">"After"</button>
    }
}
