//! A `CalendarPreviousButton` in a `Calendar` (`#perf-calendar`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/calendar_previous_button.rs`): June 2024 (June 12 focused)
//! with its heading and a grid, between two plain buttons (`#perf-before`, `#perf-after`) to tab
//! from and to (the button is the calendar's first tab stop), mounted at first. Its controls
//! ([`Control`]) mount and unmount the button (the calendar stays), set a minimum in June (no
//! previous page) and clear it, and disable and enable the calendar.
//!
//! The page creates the live announcer on load (`announce("")`): the calendar announces its visible
//! range, and the first announcement of a page waits 100 ms for the live regions.

use leptonic::{
    Assertiveness, announce,
    atoms::calendar::{
        Calendar, CalendarCell, CalendarCellButton, CalendarGrid, CalendarGridBody,
        CalendarGridHeader, CalendarHeaderCell, CalendarHeaderRow, CalendarHeading,
        CalendarPreviousButton, CalendarWeek,
    },
    jiff::civil::{Date, date},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let min_value = RwSignal::new(None::<Date>);
    let disabled = RwSignal::new(false);
    Effect::new(|| announce("", Assertiveness::Polite));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("set-min", move || min_value.set(Some(date(2024, 6, 10)))),
        Control::action("clear-min", move || min_value.set(None)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
    ];
    view! {
        <h1>"CalendarPreviousButton performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Calendar
            id="perf-calendar"
            aria_label="Event date"
            default_focused_value=date(2024, 6, 12)
            min_value=min_value
            is_disabled=disabled
        >
            <header>
                <Show when=move || mounted.get()>
                    <CalendarPreviousButton>"<"</CalendarPreviousButton>
                </Show>
                <CalendarHeading />
            </header>
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
        </Calendar>
        <button id="perf-after">"After"</button>
    }
}
