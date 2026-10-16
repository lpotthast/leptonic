//! `use_date_field_state` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_date_field_state.rs`): a date field's state (empty, its
//! segments starting from June 12, 2024 when edited: never today), rendered as plain text: its
//! segments' texts (`#perf-segments`, "mm/dd/yyyy") and value (`#perf-value`), mounted at first.
//! Its controls ([`Control`]) mount and unmount it, call the state's methods (stepping, setting
//! and clearing segments, confirming the shown date, setting the value) and change its reactive
//! inputs (disabled, limits, leading zeros, placeholder).

use leptonic::{
    hooks::datepicker::{
        DateFieldState, DateSegmentType, UseDateFieldStateInput, use_date_field_state,
    },
    jiff::civil::{Date, date},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The state's inputs, app state the controls change.
#[derive(Clone, Copy)]
struct Inputs {
    placeholder: RwSignal<Option<Date>>,
    min_value: RwSignal<Option<Date>>,
    disabled: RwSignal<bool>,
    leading_zeros: RwSignal<bool>,
}

/// Creates the state (handing it to the page's controls) and renders it.
#[component]
fn State(inputs: Inputs, state: StoredValue<Option<DateFieldState<Date>>>) -> impl IntoView {
    let field = use_date_field_state(UseDateFieldStateInput::<Date> {
        placeholder_value: inputs.placeholder.into(),
        min_value: inputs.min_value.into(),
        is_disabled: inputs.disabled.into(),
        should_force_leading_zeros: inputs.leading_zeros.into(),
        ..UseDateFieldStateInput::default()
    });
    state.set_value(Some(field));
    on_cleanup(move || state.set_value(None));
    view! {
        <p>
            "Segments: "
            <span id="perf-segments">
                {move || {
                    field
                        .segments
                        .with(|segments| {
                            segments.iter().map(|segment| segment.text.as_str()).collect::<String>()
                        })
                }}
            </span>
        </p>
        <p>
            "Value: "
            <span id="perf-value">
                {move || field.value.get().map_or_else(|| "none".to_owned(), |date| date.to_string())}
            </span>
        </p>
        <p>"Invalid: " <span id="perf-invalid">{move || field.is_invalid.get().to_string()}</span></p>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let state = StoredValue::new(None::<DateFieldState<Date>>);
    let inputs = Inputs {
        placeholder: RwSignal::new(Some(date(2024, 6, 12))),
        min_value: RwSignal::new(None),
        disabled: RwSignal::new(false),
        leading_zeros: RwSignal::new(false),
    };
    // Calls `method` on the state while it is mounted.
    let call = move |method: fn(&DateFieldState<Date>)| {
        move || {
            if let Some(state) = state.get_value() {
                method(&state);
            }
        }
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action(
            "increment-month",
            call(|state| state.increment(DateSegmentType::Month)),
        ),
        Control::action(
            "decrement-month",
            call(|state| state.decrement(DateSegmentType::Month)),
        ),
        Control::action(
            "increment-day-page",
            call(|state| state.increment_page(DateSegmentType::Day)),
        ),
        Control::action(
            "decrement-day-page",
            call(|state| state.decrement_page(DateSegmentType::Day)),
        ),
        Control::action(
            "day-to-max",
            call(|state| state.increment_to_max(DateSegmentType::Day)),
        ),
        Control::action(
            "day-to-min",
            call(|state| state.decrement_to_min(DateSegmentType::Day)),
        ),
        Control::action(
            "set-year",
            call(|state| state.set_segment(DateSegmentType::Year, 2030)),
        ),
        Control::action(
            "clear-year",
            call(|state| state.clear_segment(DateSegmentType::Year)),
        ),
        Control::action("confirm", call(DateFieldState::confirm_placeholder)),
        Control::action(
            "set-value",
            call(|state| state.set_value(Some(date(2024, 6, 5)))),
        ),
        Control::action("clear-value", call(|state| state.set_value(None))),
        Control::action("disable", move || inputs.disabled.set(true)),
        Control::action("enable", move || inputs.disabled.set(false)),
        Control::action("set-min", move || {
            inputs.min_value.set(Some(date(2025, 1, 1)));
        }),
        Control::action("clear-min", move || inputs.min_value.set(None)),
        Control::action("force-leading-zeros", move || {
            inputs.leading_zeros.set(true);
        }),
        Control::action("no-leading-zeros", move || inputs.leading_zeros.set(false)),
        Control::action("move-placeholder", move || {
            inputs.placeholder.set(Some(date(2030, 1, 20)));
        }),
        Control::action("placeholder-back", move || {
            inputs.placeholder.set(Some(date(2024, 6, 12)));
        }),
    ];
    view! {
        <h1>"use_date_field_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <State inputs=inputs state=state />
        </Show>
    }
}
