//! `use_color_wheel_state` alone (an HSV color of hue 180 at first), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_color_wheel_state.rs`): its hue shown in `#perf-hue`,
//! mounted at first. Its controls ([`Control`]) mount and unmount it, call its methods (set the
//! value, the hue, the hue from a point, step it, start and end dragging), and disable and
//! enable it. `change-ends` reads how often it called `on_change_end`.

use leptonic::{
    HSV, Point,
    hooks::color::{ColorWheelState, UseColorWheelStateInput, use_color_wheel_state},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

fn hsv(hue: f64) -> HSV {
    HSV {
        hue,
        saturation: 1.0,
        brightness: 1.0,
    }
}

/// The state, its hue shown. Hands the state to the page's controls.
#[component]
fn State(
    state: StoredValue<Option<ColorWheelState<HSV>>>,
    disabled: RwSignal<bool>,
    end_count: StoredValue<u32>,
) -> impl IntoView {
    let wheel = use_color_wheel_state(UseColorWheelStateInput {
        default_value: hsv(180.0),
        value: None,
        is_disabled: disabled.into(),
        on_change: None,
        on_change_end: Some(Callback::new(move |_: HSV| {
            end_count.update_value(|count| *count += 1);
        })),
    });
    state.set_value(Some(wheel));
    on_cleanup(move || state.set_value(None));
    view! { <p>"Hue: " <span id="perf-hue">{move || wheel.hue.get().to_string()}</span></p> }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let state = StoredValue::new(None::<ColorWheelState<HSV>>);
    let end_count = StoredValue::new(0_u32);
    let with_state = move |call: fn(&ColorWheelState<HSV>)| {
        if let Some(state) = state.get_value() {
            call(&state);
        }
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("set-value-90", move || {
            with_state(|state| state.set_value(hsv(90.0)));
        }),
        Control::action("set-value-180", move || {
            with_state(|state| state.set_value(hsv(180.0)));
        }),
        Control::action("set-hue-45", move || {
            with_state(|state| state.set_hue(45.0));
        }),
        Control::action("set-from-point", move || {
            // Straight down from the center: 90°.
            with_state(|state| state.set_hue_from_point(Point::new(0.0, 87.0), 87.0));
        }),
        Control::action("increment", move || {
            with_state(|state| state.increment(1.0));
        }),
        Control::action("decrement", move || {
            with_state(|state| state.decrement(1.0));
        }),
        Control::action("start-dragging", move || {
            with_state(|state| state.set_dragging(true));
        }),
        Control::action("end-dragging", move || {
            with_state(|state| state.set_dragging(false));
        }),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("change-ends", move || f64::from(end_count.get_value())),
    ];
    view! {
        <h1>"use_color_wheel_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <State state disabled end_count />
        </Show>
    }
}
