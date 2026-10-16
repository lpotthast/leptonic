//! `use_date_picker_state` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_date_picker_state.rs`): a date picker's state of a date
//! and time (empty, closed; its placeholder June 12, 2024, midnight: never today), rendered as
//! plain text: its value (`#perf-value`), whether it is open (`#perf-open`) and its validity
//! (`#perf-invalid`), mounted at first. Its controls ([`Control`]) mount and unmount it, call the
//! state's methods (selecting a date and a time, opening and closing, setting the value) and
//! change its reactive inputs (closing on selection, a minimum).

use leptonic::{
    hooks::datepicker::{DatePickerState, UseDatePickerStateInput, use_date_picker_state},
    jiff::civil::{DateTime, date, time},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The state's inputs, app state the controls change.
#[derive(Clone, Copy)]
struct Inputs {
    should_close_on_select: RwSignal<bool>,
    min_value: RwSignal<Option<DateTime>>,
}

/// Creates the state (handing it to the page's controls) and renders it.
#[component]
fn State(inputs: Inputs, state: StoredValue<Option<DatePickerState<DateTime>>>) -> impl IntoView {
    let picker = use_date_picker_state(UseDatePickerStateInput::<DateTime> {
        placeholder_value: Signal::stored(Some(date(2024, 6, 12).at(0, 0, 0, 0))),
        should_close_on_select: inputs.should_close_on_select.into(),
        min_value: inputs.min_value.into(),
        ..UseDatePickerStateInput::default()
    });
    state.set_value(Some(picker));
    on_cleanup(move || state.set_value(None));
    view! {
        <p>
            "Value: "
            <span id="perf-value">
                {move || picker.value.get().map_or_else(|| "none".to_owned(), |value| value.to_string())}
            </span>
        </p>
        <p>"Open: " <span id="perf-open">{move || picker.overlay.is_open.get().to_string()}</span></p>
        <p>"Invalid: " <span id="perf-invalid">{move || picker.is_invalid.get().to_string()}</span></p>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let state = StoredValue::new(None::<DatePickerState<DateTime>>);
    let inputs = Inputs {
        should_close_on_select: RwSignal::new(true),
        min_value: RwSignal::new(None),
    };
    // Calls `method` on the state while it is mounted.
    let call = move |method: fn(&DatePickerState<DateTime>)| {
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
            "select-june-14",
            call(|state| state.select_date(date(2024, 6, 14))),
        ),
        Control::action(
            "select-9-30",
            call(|state| state.select_time(time(9, 30, 0, 0))),
        ),
        Control::action(
            "set-value",
            call(|state| state.set_value(Some(date(2024, 6, 5).at(8, 0, 0, 0)))),
        ),
        Control::action("clear-value", call(|state| state.set_value(None))),
        Control::action("stay-open-on-select", move || {
            inputs.should_close_on_select.set(false);
        }),
        Control::action("close-on-select", move || {
            inputs.should_close_on_select.set(true);
        }),
        Control::action("set-min", move || {
            inputs.min_value.set(Some(date(2025, 1, 1).at(0, 0, 0, 0)));
        }),
        Control::action("clear-min", move || inputs.min_value.set(None)),
    ];
    view! {
        <h1>"use_date_picker_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <State inputs=inputs state=state />
        </Show>
    }
}
