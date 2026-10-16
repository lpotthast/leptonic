//! `use_time_field_state` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_time_field_state.rs`): a time field's state (a time of
//! day, empty, its value app state; 12-hour in the test-app's en-US), rendered as plain text: its
//! segments' texts (`#perf-segments`), value (`#perf-value`) and validity (`#perf-invalid`),
//! mounted at first. Its controls ([`Control`]) mount and unmount it, call its field's methods
//! (stepping the hour, the day period, setting a segment, setting the value) and change its
//! reactive inputs (granularity, hour cycle, a minimum, disabled).

use leptonic::{
    HourCycle,
    hooks::datepicker::{
        DateSegmentType, Granularity, TimeBound, TimeFieldState, UseTimeFieldStateInput,
        use_time_field_state,
    },
    jiff::civil::{Time, time},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The state's inputs, app state the controls change.
#[derive(Clone, Copy)]
struct Inputs {
    value: RwSignal<Option<Time>>,
    granularity: RwSignal<Option<Granularity>>,
    hour_cycle: RwSignal<Option<HourCycle>>,
    min_value: RwSignal<Option<TimeBound<Time>>>,
    disabled: RwSignal<bool>,
}

/// Creates the state (handing it to the page's controls) and renders it.
#[component]
fn State(inputs: Inputs, state: StoredValue<Option<TimeFieldState<Time>>>) -> impl IntoView {
    let time_field = use_time_field_state(UseTimeFieldStateInput::<Time> {
        value: Some(inputs.value.into()),
        granularity: inputs.granularity.into(),
        hour_cycle: inputs.hour_cycle.into(),
        min_value: inputs.min_value.into(),
        is_disabled: inputs.disabled.into(),
        ..UseTimeFieldStateInput::default()
    });
    state.set_value(Some(time_field));
    on_cleanup(move || state.set_value(None));
    let field = time_field.field;
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
                {move || time_field.value.get().map_or_else(|| "none".to_owned(), |time| time.to_string())}
            </span>
        </p>
        <p>"Invalid: " <span id="perf-invalid">{move || field.is_invalid.get().to_string()}</span></p>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let state = StoredValue::new(None::<TimeFieldState<Time>>);
    let inputs = Inputs {
        value: RwSignal::new(None),
        granularity: RwSignal::new(None),
        hour_cycle: RwSignal::new(None),
        min_value: RwSignal::new(None),
        disabled: RwSignal::new(false),
    };
    // Calls `method` on the state while it is mounted.
    let call = move |method: fn(&TimeFieldState<Time>)| {
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
            "increment-hour",
            call(|state| state.field.increment(DateSegmentType::Hour)),
        ),
        Control::action(
            "decrement-hour",
            call(|state| state.field.decrement(DateSegmentType::Hour)),
        ),
        Control::action(
            "set-pm",
            call(|state| state.field.set_segment(DateSegmentType::DayPeriod, 1)),
        ),
        Control::action(
            "set-minute",
            call(|state| state.field.set_segment(DateSegmentType::Minute, 45)),
        ),
        Control::action(
            "clear-minute",
            call(|state| state.field.clear_segment(DateSegmentType::Minute)),
        ),
        Control::action("set-value", move || {
            inputs.value.set(Some(time(8, 30, 0, 0)));
        }),
        Control::action("clear-value", move || inputs.value.set(None)),
        Control::action("show-seconds", move || {
            inputs.granularity.set(Some(Granularity::Second));
        }),
        Control::action("show-minutes", move || inputs.granularity.set(None)),
        Control::action("use-24-hours", move || {
            inputs.hour_cycle.set(Some(HourCycle::H24));
        }),
        Control::action("use-locale-hours", move || inputs.hour_cycle.set(None)),
        Control::action("set-min", move || {
            inputs
                .min_value
                .set(Some(TimeBound::TimeOfDay(time(9, 0, 0, 0))));
        }),
        Control::action("clear-min", move || inputs.min_value.set(None)),
        Control::action("disable", move || inputs.disabled.set(true)),
        Control::action("enable", move || inputs.disabled.set(false)),
    ];
    view! {
        <h1>"use_time_field_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <State inputs=inputs state=state />
        </Show>
    }
}
