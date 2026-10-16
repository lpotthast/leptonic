//! A `CalendarGridHeader` in a `Calendar` (`#perf-calendar`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/calendar_grid_header.rs`): June 2024 with the weekday names
//! in the header, mounted at first. Its controls ([`Control`]) mount and unmount the header (the
//! calendar and its grid stay).

use leptonic::{
    atoms::calendar::{
        Calendar, CalendarCell, CalendarCellButton, CalendarGrid, CalendarGridBody,
        CalendarGridHeader, CalendarHeaderCell, CalendarHeaderRow, CalendarHeading, CalendarWeek,
    },
    jiff::civil::date,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
    ];
    view! {
        <h1>"CalendarGridHeader performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Calendar
            id="perf-calendar"
            aria_label="Event date"
            default_focused_value=date(2024, 6, 12)
        >
            <CalendarHeading />
            <CalendarGrid>
                <Show when=move || mounted.get()>
                    <CalendarGridHeader>
                        <CalendarHeaderRow children=|day| {
                            view! { <CalendarHeaderCell>{day}</CalendarHeaderCell> }
                        } />
                    </CalendarGridHeader>
                </Show>
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
        </Calendar>
        <button id="perf-after">"After"</button>
    }
}
