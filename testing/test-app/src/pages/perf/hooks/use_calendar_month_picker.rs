//! `use_calendar_month_picker` in a `Calendar` (`#perf-calendar`), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_calendar_month_picker.rs`): June 2024 (June 12 focused)
//! with its heading, next button and a grid; the picker (the hook, of the calendar's state:
//! `use_calendar_states`) renders its months as a group of buttons (`#perf-months`, the focused
//! date's month pressed), mounted at first. Its controls ([`Control`]) mount and unmount the picker
//! (the calendar stays).
//!
//! The page creates the live announcer on load (`announce("")`): the calendar announces its
//! visible range, and the first announcement of a page waits 100 ms for the live regions.

use leptonic::{
    Assertiveness, MonthFormat, announce,
    atoms::calendar::{
        Calendar, CalendarCell, CalendarCellButton, CalendarGrid, CalendarGridBody,
        CalendarGridHeader, CalendarHeaderCell, CalendarHeaderRow, CalendarHeading,
        CalendarNextButton, CalendarWeek, use_calendar_states,
    },
    hooks::calendar::{
        CalendarPickerItem, UseCalendarMonthPickerInput, UseCalendarPickerReturn,
        use_calendar_month_picker,
    },
    jiff::civil::date,
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

/// The picker of the calendar around it: `use_calendar_month_picker` rendered as buttons.
#[component]
fn Picker() -> impl IntoView {
    let state = use_calendar_states().expect("inside a calendar");
    let picker = use_calendar_month_picker(UseCalendarMonthPickerInput {
        state,
        format: MonthFormat::Short,
    });
    view! { <PickerButtons id="perf-months" picker=picker /> }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    Effect::new(|| announce("", Assertiveness::Polite));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
    ];
    view! {
        <h1>"use_calendar_month_picker performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Calendar
            id="perf-calendar"
            aria_label="Event date"
            default_focused_value=date(2024, 6, 12)
        >
            <header>
                <CalendarHeading />
                <CalendarNextButton>">"</CalendarNextButton>
                <Show when=move || mounted.get()>
                    <Picker />
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
