//! `use_calendar_heading` in a `Calendar` (`#perf-calendar`), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_calendar_heading.rs`): a heading (`h2`) of the hook's
//! text, of the calendar's state (`use_calendar_states`), in a calendar of June 2024 (June 12
//! focused) with its previous and next buttons and a grid, mounted at first. Its controls
//! ([`Control`]) mount and unmount the heading (the calendar stays) and move the focused date to
//! another year (October 2030) and back.
//!
//! The page creates the live announcer on load (`announce("")`): the calendar announces its
//! visible range, and the first announcement of a page waits 100 ms for the live regions.

use leptonic::{
    Assertiveness, DateDuration, announce,
    atoms::calendar::{
        Calendar, CalendarCell, CalendarCellButton, CalendarGrid, CalendarGridBody,
        CalendarGridHeader, CalendarHeaderCell, CalendarHeaderRow, CalendarNextButton,
        CalendarPreviousButton, CalendarWeek, use_calendar_states,
    },
    hooks::calendar::{CalendarHeadingFormat, UseCalendarHeadingInput, use_calendar_heading},
    jiff::civil::{Date, date},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The date focused at first.
const FOCUSED: Date = date(2024, 6, 12);

/// A heading of the calendar around it: `use_calendar_heading`'s text.
#[component]
fn Heading() -> impl IntoView {
    let state = use_calendar_states().expect("inside a calendar");
    let heading = use_calendar_heading(UseCalendarHeadingInput {
        state,
        offset: DateDuration::default(),
        format: CalendarHeadingFormat::default(),
    });
    view! { <h2 aria-hidden="true">{heading}</h2> }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let focused = RwSignal::new(FOCUSED);
    Effect::new(|| announce("", Assertiveness::Polite));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("focus-2030", move || focused.set(date(2030, 10, 15))),
        Control::action("focus-june", move || focused.set(FOCUSED)),
    ];
    view! {
        <h1>"use_calendar_heading performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Calendar
            id="perf-calendar"
            aria_label="Event date"
            focused_value=focused
            set_focused_value=focused
        >
            <header>
                <CalendarPreviousButton>"<"</CalendarPreviousButton>
                <Show when=move || mounted.get()>
                    <Heading />
                </Show>
                <CalendarNextButton>">"</CalendarNextButton>
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
