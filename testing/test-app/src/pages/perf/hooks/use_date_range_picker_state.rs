//! `use_date_range_picker_state` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_date_range_picker_state.rs`): a date range picker's state
//! of dates and times (empty, closed; its placeholder June 12, 2024, midnight: never today),
//! rendered as plain text: its start (`#perf-start`), end (`#perf-end`), whether it is open
//! (`#perf-open`) and its validity (`#perf-invalid`), mounted at first. Its controls
//! ([`Control`]) mount and unmount it, call the state's methods (selecting a range and times,
//! setting an end, opening and closing, setting the range) and change a minimum.

use leptonic::{
    DateRange,
    hooks::datepicker::{
        DateRangePickerState, RangePart, UseDateRangePickerStateInput, use_date_range_picker_state,
    },
    jiff::civil::{DateTime, date, time},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// `value` as text ("none" without).
fn show(value: Option<DateTime>) -> String {
    value.map_or_else(|| "none".to_owned(), |value| value.to_string())
}

/// Creates the state (handing it to the page's controls) and renders it.
#[component]
fn State(
    min_value: RwSignal<Option<DateTime>>,
    state: StoredValue<Option<DateRangePickerState<DateTime>>>,
) -> impl IntoView {
    let picker = use_date_range_picker_state(UseDateRangePickerStateInput::<DateTime> {
        placeholder_value: Signal::stored(Some(date(2024, 6, 12).at(0, 0, 0, 0))),
        min_value: min_value.into(),
        ..UseDateRangePickerStateInput::default()
    });
    state.set_value(Some(picker));
    on_cleanup(move || state.set_value(None));
    view! {
        <p>"Start: " <span id="perf-start">{move || show(picker.start.get())}</span></p>
        <p>"End: " <span id="perf-end">{move || show(picker.end.get())}</span></p>
        <p>"Open: " <span id="perf-open">{move || picker.overlay.is_open.get().to_string()}</span></p>
        <p>"Invalid: " <span id="perf-invalid">{move || picker.is_invalid.get().to_string()}</span></p>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let state = StoredValue::new(None::<DateRangePickerState<DateTime>>);
    let min_value = RwSignal::new(None::<DateTime>);
    // Calls `method` on the state while it is mounted.
    let call = move |method: fn(&DateRangePickerState<DateTime>)| {
        move || {
            if let Some(state) = state.get_value() {
                method(&state);
            }
        }
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("open", call(|state| state.set_open(true))),
        Control::action("close", call(|state| state.set_open(false))),
        Control::action(
            "select-june-10-to-14",
            call(|state| {
                state.select_range(DateRange {
                    start: date(2024, 6, 10),
                    end: date(2024, 6, 14),
                });
            }),
        ),
        Control::action(
            "select-start-9-30",
            call(|state| state.select_time(RangePart::Start, time(9, 30, 0, 0))),
        ),
        Control::action(
            "set-end",
            call(|state| {
                state.set_date_time(RangePart::End, Some(date(2024, 6, 20).at(18, 0, 0, 0)));
            }),
        ),
        Control::action(
            "set-value",
            call(|state| {
                state.set_value(
                    Some(date(2024, 6, 5).at(8, 0, 0, 0)),
                    Some(date(2024, 6, 9).at(8, 0, 0, 0)),
                );
            }),
        ),
        Control::action("clear-value", call(|state| state.set_value(None, None))),
        Control::action("set-min", move || {
            min_value.set(Some(date(2025, 1, 1).at(0, 0, 0, 0)));
        }),
        Control::action("clear-min", move || min_value.set(None)),
    ];
    view! {
        <h1>"use_date_range_picker_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <State min_value=min_value state=state />
        </Show>
    }
}
