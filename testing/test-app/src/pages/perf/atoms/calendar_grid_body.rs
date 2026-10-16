//! A `CalendarGridBody` in a `Calendar` (`#perf-calendar`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/calendar_grid_body.rs`): June 2024 (six weeks; June 12
//! focused) with its heading and next button, mounted at first. Its controls ([`Control`])
//! mount and unmount the body (the calendar and its grid stay) and fix the number of week rows
//! at six or let the month decide again.
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
    let weeks_in_month = RwSignal::new(None::<u8>);
    Effect::new(|| announce("", Assertiveness::Polite));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("six-weeks", move || weeks_in_month.set(Some(6))),
        Control::action("weeks-of-month", move || weeks_in_month.set(None)),
    ];
    view! {
        <h1>"CalendarGridBody performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Calendar
            id="perf-calendar"
            aria_label="Event date"
            default_focused_value=date(2024, 6, 12)
            weeks_in_month=weeks_in_month
        >
            <CalendarHeading />
            <CalendarNextButton>">"</CalendarNextButton>
            <CalendarGrid>
                <CalendarGridHeader>
                    <CalendarHeaderRow children=|day| {
                        view! { <CalendarHeaderCell>{day}</CalendarHeaderCell> }
                    } />
                </CalendarGridHeader>
                <Show when=move || mounted.get()>
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
                </Show>
            </CalendarGrid>
        </Calendar>
        <button id="perf-after">"After"</button>
    }
}
