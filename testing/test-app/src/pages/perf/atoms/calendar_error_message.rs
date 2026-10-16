//! A `CalendarErrorMessage` in a `Calendar` (`#perf-calendar`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/calendar_error_message.rs`): June 2024 with June 18
//! selected, mounted at first. Its controls ([`Control`]) mount and unmount the
//! error message (the calendar stays) and mark the calendar invalid and valid (the selected
//! date's cell then refers to the message).
//!
//! The page creates the live announcer on load (`announce("")`): the first announcement of a
//! page waits 100 ms for the live regions, after a measurement would have ended.

use leptonic::{
    Assertiveness, announce,
    atoms::calendar::{
        Calendar, CalendarCell, CalendarCellButton, CalendarErrorMessage, CalendarGrid,
        CalendarGridBody, CalendarGridHeader, CalendarHeaderCell, CalendarHeaderRow,
        CalendarHeading, CalendarWeek,
    },
    jiff::civil::date,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let invalid = RwSignal::new(false);
    Effect::new(|| announce("", Assertiveness::Polite));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("invalidate", move || invalid.set(true)),
        Control::action("validate", move || invalid.set(false)),
    ];
    view! {
        <h1>"CalendarErrorMessage performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Calendar
            id="perf-calendar"
            aria_label="Event date"
            default_value=date(2024, 6, 18)
            is_invalid=invalid
        >
            <CalendarHeading />
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
            <Show when=move || mounted.get()>
                <CalendarErrorMessage>
                    "Pick another date."
                </CalendarErrorMessage>
            </Show>
        </Calendar>
        <button id="perf-after">"After"</button>
    }
}
