//! `CalendarCellButton`s in a `Calendar` (`#perf-calendar`), for their performance test
//! (`leptonic/tests/perf_tests/atoms/calendar_cell_button.rs`): June 2024 (June 12 focused, no
//! date selected), a cell with its button per date of the grid, between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. A cell's button can't
//! mount again in the same cell, so its controls ([`Control`]) mount and unmount the cells with
//! their buttons (the calendar stays); `value` reads the calendar's value (`yyyymmdd`, 0
//! without).
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

/// `date` as `yyyymmdd`, for the read controls (0: none).
fn number(date: Option<Date>) -> f64 {
    date.map_or(0.0, |date| {
        f64::from(date.year()) * 10_000.0 + f64::from(date.month()) * 100.0 + f64::from(date.day())
    })
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let value = RwSignal::new(None::<Date>);
    Effect::new(|| announce("", Assertiveness::Polite));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::read("value", move || number(value.get_untracked())),
    ];
    view! {
        <h1>"CalendarCellButton performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Calendar
            id="perf-calendar"
            aria_label="Event date"
            value=value
            set_value=value
            default_focused_value=date(2024, 6, 12)
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
                                        <CalendarCell date=date>
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
