//! `use_color_area_state` alone (red on x and green on y of an RGB color, `#8080FF` at first),
//! for its performance test (`leptonic/tests/perf_tests/hooks/use_color_area_state.rs`): its
//! value shown in `#perf-value` (`RRGGBB`) and whether it is dragged in `#perf-dragging`, mounted
//! at first. Its controls ([`Control`]) mount and unmount it, and call its methods: set the
//! value, a channel's value, the color from a point, step the channels, start and end dragging.
//! `change-ends` reads how often it called `on_change_end`.

use leptonic::{
    Point, RGB8,
    hooks::color::{ColorAreaState, UseColorAreaStateInput, use_color_area_state},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

const START: RGB8 = RGB8 {
    r: 128,
    g: 128,
    b: 255,
};
const BLUE: RGB8 = RGB8 { r: 0, g: 0, b: 255 };

/// The state, its value shown. Hands the state to the page's controls.
#[component]
fn State(
    state: StoredValue<Option<ColorAreaState<RGB8>>>,
    end_count: StoredValue<u32>,
) -> impl IntoView {
    let area = use_color_area_state(UseColorAreaStateInput {
        default_value: START,
        value: None,
        x_channel: None,
        y_channel: None,
        x_channel_step: None,
        y_channel_step: None,
        on_change: None,
        on_change_end: Some(Callback::new(move |_: RGB8| {
            end_count.update_value(|count| *count += 1);
        })),
    });
    state.set_value(Some(area));
    on_cleanup(move || state.set_value(None));
    view! {
        <p>"Value: " <span id="perf-value">{move || format!("{:X}", area.value.get())}</span></p>
        <p>"Dragging: " <span id="perf-dragging">{move || area.is_dragging.get().to_string()}</span></p>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let state = StoredValue::new(None::<ColorAreaState<RGB8>>);
    let end_count = StoredValue::new(0_u32);
    let with_state = move |call: fn(&ColorAreaState<RGB8>)| {
        if let Some(state) = state.get_value() {
            call(&state);
        }
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("set-blue", move || {
            with_state(|state| state.set_value(BLUE));
        }),
        Control::action("set-start", move || {
            with_state(|state| state.set_value(START));
        }),
        Control::action("set-x-64", move || {
            with_state(|state| state.set_x_value(64.0));
        }),
        Control::action("set-y-64", move || {
            with_state(|state| state.set_y_value(64.0));
        }),
        Control::action("set-from-point", move || {
            with_state(|state| state.set_color_from_point(Point::new(0.25, 0.75)));
        }),
        Control::action("increment-x", move || {
            with_state(|state| state.increment_x(1.0));
        }),
        Control::action("decrement-x", move || {
            with_state(|state| state.decrement_x(1.0));
        }),
        Control::action("increment-y", move || {
            with_state(|state| state.increment_y(1.0));
        }),
        Control::action("decrement-y", move || {
            with_state(|state| state.decrement_y(1.0));
        }),
        Control::action("start-dragging", move || {
            with_state(|state| state.set_dragging(true));
        }),
        Control::action("end-dragging", move || {
            with_state(|state| state.set_dragging(false));
        }),
        Control::read("change-ends", move || f64::from(end_count.get_value())),
    ];
    view! {
        <h1>"use_color_area_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <State state end_count />
        </Show>
    }
}
