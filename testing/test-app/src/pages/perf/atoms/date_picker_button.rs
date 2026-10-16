//! A `DatePickerButton` in a `DatePicker` (labelled "Departure", its open state app state), for
//! its performance test (`leptonic/tests/perf_tests/atoms/date_picker_button.rs`): the picker's
//! `Label`, its `DatePickerGroup` with the `DateInput` of its segments and the button, and a
//! `Popover` with a `Dialog` and a `Calendar`, between two plain buttons (`#perf-before`,
//! `#perf-after`) to tab from and to, mounted at first, empty and closed (the button is the last
//! tab stop before `#perf-after`). It opens on June 2024 (its placeholder, June 12, 2024: never
//! today). Its controls ([`Control`]) mount and unmount the button (the picker stays), disable
//! and enable the picker, and read its open state (`is-open`: 1 or 0).
//!
//! The page creates the live announcer on load (`announce("")`): the calendar and the segments
//! announce changes, and the first announcement of a page waits 100 ms for the live regions.

use leptonic::{
    Assertiveness, announce,
    atoms::{
        calendar::{
            Calendar, CalendarCell, CalendarCellButton, CalendarGrid, CalendarGridBody,
            CalendarGridHeader, CalendarHeaderCell, CalendarHeaderRow, CalendarHeading,
            CalendarNextButton, CalendarPreviousButton, CalendarWeek,
        },
        datepicker::{DateInput, DatePicker, DatePickerButton, DatePickerGroup, DateSegment},
        dialog::Dialog,
        field::Label,
        popover::Popover,
    },
    jiff::civil::{Date, date},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let value = RwSignal::new(None::<Date>);
    let open = RwSignal::new(false);
    let disabled = RwSignal::new(false);
    Effect::new(|| announce("", Assertiveness::Polite));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("is-open", move || f64::from(u8::from(open.get_untracked()))),
    ];
    view! {
        <h1>"DatePickerButton performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <DatePicker
                value=value
                set_value=value
                is_open=open
                set_open=open
                placeholder_value=date(2024, 6, 12)
                is_disabled=disabled
            >
                <Label>"Departure"</Label>
                <DatePickerGroup>
                    <DateInput children=|segment| view! { <DateSegment segment=segment /> } />
                    <Show when=move || mounted.get()>
                        <DatePickerButton>"▼"</DatePickerButton>
                    </Show>
                </DatePickerGroup>
                <Popover>
                    <Dialog>
                        <Calendar>
                            <header>
                                <CalendarPreviousButton>"<"</CalendarPreviousButton>
                                <CalendarHeading />
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
                    </Dialog>
                </Popover>
            </DatePicker>
        <button id="perf-after">"After"</button>
    }
}
