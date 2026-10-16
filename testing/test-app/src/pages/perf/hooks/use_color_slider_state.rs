//! `use_color_slider_state` alone (the red channel of an RGB color, `#8080FF` at first), for its
//! performance test (`leptonic/tests/perf_tests/hooks/use_color_slider_state.rs`): its value shown
//! in `#perf-value` (`RRGGBB`), its formatted value in `#perf-formatted`, mounted at first. Its
//! controls ([`Control`]) mount and unmount it, call its methods (set the value, the channel's
//! value, step it, start and end dragging), and change its inputs (disabled, orientation).
//! `change-ends` reads how often it called `on_change_end`.

use leptonic::{
    Orientation, RGB8, RgbChannel,
    hooks::color::{ColorSliderState, UseColorSliderStateInput, use_color_slider_state},
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
    state: StoredValue<Option<ColorSliderState<RGB8>>>,
    disabled: RwSignal<bool>,
    orientation: RwSignal<Orientation>,
    end_count: StoredValue<u32>,
) -> impl IntoView {
    let slider = use_color_slider_state(UseColorSliderStateInput {
        default_value: START,
        value: None,
        channel: RgbChannel::Red,
        is_disabled: disabled.into(),
        orientation: orientation.into(),
        on_change: None,
        on_change_end: Some(Callback::new(move |_: RGB8| {
            end_count.update_value(|count| *count += 1);
        })),
    });
    state.set_value(Some(slider));
    on_cleanup(move || state.set_value(None));
    let formatted = slider.formatted_value();
    view! {
        <p>"Value: " <span id="perf-value">{move || format!("{:X}", slider.value.get())}</span></p>
        <p>"Formatted: " <span id="perf-formatted">{formatted}</span></p>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let orientation = RwSignal::new(Orientation::Horizontal);
    let state = StoredValue::new(None::<ColorSliderState<RGB8>>);
    let end_count = StoredValue::new(0_u32);
    let with_state = move |call: fn(&ColorSliderState<RGB8>)| {
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
        Control::action("set-channel-64", move || {
            with_state(|state| state.slider.set_thumb_value(0, 64.0));
        }),
        Control::action("increment", move || {
            with_state(|state| state.slider.increment_thumb(0, None));
        }),
        Control::action("decrement", move || {
            with_state(|state| state.slider.decrement_thumb(0, None));
        }),
        Control::action("start-dragging", move || {
            with_state(|state| state.slider.set_thumb_dragging(0, true));
        }),
        Control::action("end-dragging", move || {
            with_state(|state| state.slider.set_thumb_dragging(0, false));
        }),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("make-vertical", move || {
            orientation.set(Orientation::Vertical);
        }),
        Control::action("make-horizontal", move || {
            orientation.set(Orientation::Horizontal);
        }),
        Control::read("change-ends", move || f64::from(end_count.get_value())),
    ];
    view! {
        <h1>"use_color_slider_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <State state disabled orientation end_count />
        </Show>
    }
}
