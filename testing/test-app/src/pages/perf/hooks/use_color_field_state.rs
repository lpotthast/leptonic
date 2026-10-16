//! `use_color_field_state` alone (an RGB color, `#808080` at first), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_color_field_state.rs`): its text shown in `#perf-text`
//! and its committed color in `#perf-color` (`RRGGBB`, `none`), mounted at first. Its controls
//! ([`Control`]) mount and unmount it, and call its methods: set the color, set the text and
//! commit it, validate a text, step the color, and set it to white and black.

use leptonic::{
    RGB8,
    hooks::color::{ColorFieldState, UseColorFieldStateInput, use_color_field_state},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

const GRAY: RGB8 = RGB8 {
    r: 128,
    g: 128,
    b: 128,
};
const RED: RGB8 = RGB8 { r: 255, g: 0, b: 0 };

/// The state, its text and color shown. Hands the state to the page's controls.
#[component]
fn State(state: StoredValue<Option<ColorFieldState<RGB8>>>) -> impl IntoView {
    let field = use_color_field_state(UseColorFieldStateInput {
        default_value: Some(GRAY),
        ..UseColorFieldStateInput::default()
    });
    state.set_value(Some(field));
    on_cleanup(move || state.set_value(None));
    view! {
        <p>"Text: " <span id="perf-text">{move || field.input_value.get()}</span></p>
        <p>
            "Color: "
            <span id="perf-color">
                {move || {
                    field
                        .color_value
                        .get()
                        .map_or_else(|| "none".to_owned(), |color| format!("{color:X}"))
                }}
            </span>
        </p>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let state = StoredValue::new(None::<ColorFieldState<RGB8>>);
    let valid = StoredValue::new(false);
    let with_state = move |call: fn(&ColorFieldState<RGB8>)| {
        if let Some(state) = state.get_value() {
            call(&state);
        }
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("set-red", move || {
            with_state(|state| state.set_color_value(Some(RED)));
        }),
        Control::action("set-gray", move || {
            with_state(|state| state.set_color_value(Some(GRAY)));
        }),
        Control::action("set-text", move || {
            with_state(|state| state.set_input_value("123".to_owned()));
        }),
        Control::action("commit", move || with_state(ColorFieldState::commit)),
        Control::action("validate", move || {
            if let Some(state) = state.get_value() {
                valid.set_value(state.validate("#12ab"));
            }
        }),
        Control::action("increment", move || with_state(ColorFieldState::increment)),
        Control::action("decrement", move || with_state(ColorFieldState::decrement)),
        Control::action("to-max", move || {
            with_state(ColorFieldState::increment_to_max);
        }),
        Control::action("to-min", move || {
            with_state(ColorFieldState::decrement_to_min);
        }),
        Control::read("valid", move || if valid.get_value() { 1.0 } else { 0.0 }),
    ];
    view! {
        <h1>"use_color_field_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <State state />
        </Show>
    }
}
