//! A `CalendarYearPicker` in a `Calendar` (`#perf-calendar`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/calendar_year_picker.rs`): June 2024 (June 12 focused) with
//! its heading, next button and a grid; the picker renders its 20 years around the focused
//! date's as a group of buttons (`#perf-years`, the focused date's year pressed), mounted at
//! first. Its controls ([`Control`]) mount and unmount the picker (the calendar stays) and move
//! the focused date to 2040 (other years) and back.
//!
//! The page creates the live announcer on load (`announce("")`): the calendar announces its
//! visible range, and the first announcement of a page waits 100 ms for the live regions.

use leptonic::{
    Assertiveness, announce,
    atoms::calendar::{
        Calendar, CalendarCell, CalendarCellButton, CalendarGrid, CalendarGridBody,
        CalendarGridHeader, CalendarHeaderCell, CalendarHeaderRow, CalendarHeading,
        CalendarNextButton, CalendarWeek, CalendarYearPicker,
    },
    hooks::calendar::{CalendarPickerItem, UseCalendarPickerReturn},
    jiff::civil::{Date, date},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The picker's items as buttons, the picked one pressed.
#[component]
fn PickerButtons(id: &'static str, picker: UseCalendarPickerReturn) -> impl IntoView {
    let UseCalendarPickerReturn {
        aria_label,
        value,
        items,
        on_change,
    } = picker;
    view! {
        <div id=id role="group" aria-label=aria_label>
            <For
                each=move || items.get()
                key=|item: &CalendarPickerItem| item.id
                children=move |item: CalendarPickerItem| {
                    let id = item.id;
                    view! {
                        <button
                            aria-pressed=move || (value.get() == id).to_string()
                            on:click=move |_| on_change.run(id)
                        >
                            {item.formatted}
                        </button>
                    }
                }
            />
        </div>
    }
}

/// The date focused at first.
const FOCUSED: Date = date(2024, 6, 12);

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let focused = RwSignal::new(FOCUSED);
    Effect::new(|| announce("", Assertiveness::Polite));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("focus-2040", move || focused.set(date(2040, 6, 12))),
        Control::action("focus-2024", move || focused.set(FOCUSED)),
    ];
    view! {
        <h1>"CalendarYearPicker performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Calendar
            id="perf-calendar"
            aria_label="Event date"
            focused_value=focused
            set_focused_value=focused
        >
            <header>
                <CalendarHeading />
                <CalendarNextButton>">"</CalendarNextButton>
                <Show when=move || mounted.get()>
                    <CalendarYearPicker children=|picker| {
                        view! { <PickerButtons id="perf-years" picker=picker /> }
                    } />
                </Show>
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
