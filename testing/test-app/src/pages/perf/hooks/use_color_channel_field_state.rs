//! `use_color_channel_field_state` alone (the hue of an HSV color, 180 at first), for its
//! performance test (`leptonic/tests/perf_tests/hooks/use_color_channel_field_state.rs`): its
//! text shown in `#perf-text` and its color's hue in `#perf-hue` (`none` without a color),
//! mounted at first. Its controls ([`Control`]) mount and unmount it, call its methods (set the
//! color; its number field's: set the text and commit it, step, to the ends), and disable and
//! enable it.

use leptonic::{
    HSV, HsvChannel,
    hooks::{
        color::{
            ColorChannelFieldState, UseColorChannelFieldStateInput, use_color_channel_field_state,
        },
        form::ValidationBehavior,
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

fn hsv(hue: f64) -> HSV {
    HSV {
        hue,
        saturation: 0.5,
        brightness: 1.0,
    }
}

/// The state, its text and hue shown. Hands the state to the page's controls.
#[component]
fn State(
    state: StoredValue<Option<ColorChannelFieldState<HSV>>>,
    disabled: RwSignal<bool>,
) -> impl IntoView {
    let field = use_color_channel_field_state(UseColorChannelFieldStateInput {
        default_value: Some(hsv(180.0)),
        value: None,
        channel: HsvChannel::Hue,
        is_disabled: disabled.into(),
        is_read_only: Signal::stored(false),
        is_invalid: Signal::stored(false),
        validate: None,
        validation_behavior: ValidationBehavior::default(),
        name: None,
        on_change: None,
    });
    state.set_value(Some(field));
    on_cleanup(move || state.set_value(None));
    let text = field.number.input_value;
    view! {
        <p>"Text: " <span id="perf-text">{move || text.get()}</span></p>
        <p>
            "Hue: "
            <span id="perf-hue">
                {move || {
                    field
                        .color_value
                        .get()
                        .map_or_else(|| "none".to_owned(), |color| color.hue.to_string())
                }}
            </span>
        </p>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let state = StoredValue::new(None::<ColorChannelFieldState<HSV>>);
    let with_state = move |call: fn(&ColorChannelFieldState<HSV>)| {
        if let Some(state) = state.get_value() {
            call(&state);
        }
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("set-hue-90", move || {
            with_state(|state| state.set_color_value(Some(hsv(90.0))));
        }),
        Control::action("set-hue-180", move || {
            with_state(|state| state.set_color_value(Some(hsv(180.0))));
        }),
        Control::action("set-text", move || {
            with_state(|state| state.number.set_input_value("45".to_owned()));
        }),
        Control::action("commit", move || {
            with_state(|state| state.number.commit(None));
        }),
        Control::action("increment", move || {
            with_state(|state| state.number.increment());
        }),
        Control::action("decrement", move || {
            with_state(|state| state.number.decrement());
        }),
        Control::action("to-max", move || {
            with_state(|state| state.number.increment_to_max());
        }),
        Control::action("to-min", move || {
            with_state(|state| state.number.decrement_to_min());
        }),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
    ];
    view! {
        <h1>"use_color_channel_field_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <State state disabled />
        </Show>
    }
}
