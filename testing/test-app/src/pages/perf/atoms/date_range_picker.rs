//! The `DateRangePicker` alone (labelled "Trip", its range and open state app state), for its
//! performance test (`leptonic/tests/perf_tests/atoms/date_range_picker.rs`): its `Label`, a
//! `DatePickerGroup` with the `DateInput`s of its start and end and its `DatePickerButton`, and a
//! `Popover` with a `Dialog` and a `RangeCalendar`, between two plain buttons (`#perf-before`,
//! `#perf-after`) to tab from and to, mounted at first, empty and closed. It opens on June 2024
//! (its placeholder, June 12, 2024: never today). Its controls ([`Control`]) mount and unmount
//! it, open and close it, change its reactive props, and read its range (`start`, `end`:
//! `yyyymmdd`, 0 without) and open state (`is-open`: 1 or 0).
//!
//! The page creates the live announcer on load (`announce("")`): the calendar and the segments
//! announce changes, and the first announcement of a page waits 100 ms for the live regions.

use leptonic::{
    Assertiveness, announce,
    atoms::{
        calendar::{
            CalendarCell, CalendarCellButton, CalendarGrid, CalendarGridBody, CalendarGridHeader,
            CalendarHeaderCell, CalendarHeaderRow, CalendarHeading, CalendarNextButton,
            CalendarPreviousButton, CalendarWeek, RangeCalendar,
        },
        datepicker::{DateInput, DatePickerButton, DatePickerGroup, DateRangePicker, DateSegment},
        dialog::Dialog,
        field::Label,
        popover::Popover,
    },
    hooks::datepicker::{RangePart, RangeValue},
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
    let value = RwSignal::new(None::<RangeValue<Date>>);
    let open = RwSignal::new(false);
    let min_value = RwSignal::new(None::<Date>);
    let disabled = RwSignal::new(false);
    let read_only = RwSignal::new(false);
    let invalid = RwSignal::new(false);
    Effect::new(|| announce("", Assertiveness::Polite));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("open", move || open.set(true)),
        Control::action("close", move || open.set(false)),
        Control::action("set-value", move || {
            value.set(Some(RangeValue {
                start: date(2024, 6, 5),
                end: date(2024, 6, 9),
            }));
        }),
        Control::action("clear-value", move || value.set(None)),
        Control::action("set-min", move || min_value.set(Some(date(2025, 1, 1)))),
        Control::action("clear-min", move || min_value.set(None)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("make-read-only", move || read_only.set(true)),
        Control::action("make-editable", move || read_only.set(false)),
        Control::action("invalidate", move || invalid.set(true)),
        Control::action("validate", move || invalid.set(false)),
        Control::read("start", move || {
            number(value.get_untracked().map(|range| range.start))
        }),
        Control::read("end", move || {
            number(value.get_untracked().map(|range| range.end))
        }),
        Control::read("is-open", move || f64::from(u8::from(open.get_untracked()))),
    ];
    view! {
        <h1>"DateRangePicker performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <DateRangePicker
                value=value
                set_value=value
                is_open=open
                set_open=open
                placeholder_value=date(2024, 6, 12)
                min_value=min_value
                is_disabled=disabled
                is_read_only=read_only
                is_invalid=invalid
            >
                <Label>"Trip"</Label>
                <DatePickerGroup>
                    <DateInput
                        part=RangePart::Start
                        children=|segment| view! { <DateSegment segment=segment /> }
                    />
                    <span aria-hidden="true">" – "</span>
                    <DateInput
                        part=RangePart::End
                        children=|segment| view! { <DateSegment segment=segment /> }
                    />
                    <DatePickerButton>"▼"</DatePickerButton>
                </DatePickerGroup>
                <Popover>
                    <Dialog>
                        <RangeCalendar>
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
                        </RangeCalendar>
                    </Dialog>
                </Popover>
            </DateRangePicker>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
