//! A `CalendarHeaderRow` in a `Calendar` (`#perf-calendar`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/calendar_header_row.rs`): June 2024 with the row of weekday
//! names in the grid's header, mounted at first. Its controls ([`Control`]) mount and unmount
//! the row (the calendar and its grid stay) and start the weeks on Monday or on the locale's
//! first day (Sunday).

use leptonic::{
    atoms::calendar::{
        Calendar, CalendarCell, CalendarCellButton, CalendarGrid, CalendarGridBody,
        CalendarGridHeader, CalendarHeaderCell, CalendarHeaderRow, CalendarHeading, CalendarWeek,
    },
    jiff::civil::{Weekday, date},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let first_day = RwSignal::new(None::<Weekday>);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("start-on-monday", move || {
            first_day.set(Some(Weekday::Monday));
        }),
        Control::action("start-on-locale-day", move || first_day.set(None)),
    ];
    view! {
        <h1>"CalendarHeaderRow performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Calendar
            id="perf-calendar"
            aria_label="Event date"
            default_focused_value=date(2024, 6, 12)
            first_day_of_week=first_day
        >
            <CalendarHeading />
            <CalendarGrid>
                <CalendarGridHeader>
                    <Show when=move || mounted.get()>
                        <CalendarHeaderRow children=|day| {
                            view! { <CalendarHeaderCell>{day}</CalendarHeaderCell> }
                        } />
                    </Show>
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
        </Calendar>
        <button id="perf-after">"After"</button>
    }
}
