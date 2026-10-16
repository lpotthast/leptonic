//! `CalendarHeaderCell`s in a `Calendar` (`#perf-calendar`), for their performance test
//! (`leptonic/tests/perf_tests/atoms/calendar_header_cell.rs`): June 2024 with a header cell per
//! weekday, mounted at first. Its controls ([`Control`]) mount and unmount the header cells (the
//! calendar, its grid and the header row stay) and start the weeks on Monday or on the locale's
//! first day (Sunday): the row renders the cells of the new weekday order.

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
        <h1>"CalendarHeaderCell performance"</h1>
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
                    <CalendarHeaderRow children=move |day| {
                        let day = StoredValue::new(day);
                        view! {
                            <Show when=move || mounted.get()>
                                <CalendarHeaderCell>{day.get_value()}</CalendarHeaderCell>
                            </Show>
                        }
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
        </Calendar>
        <button id="perf-after">"After"</button>
    }
}
