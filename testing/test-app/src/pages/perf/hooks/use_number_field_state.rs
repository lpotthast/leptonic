//! `use_number_field_state` (of `i32`, 0 to 10, 5 at first), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_number_field_state.rs`): mounted at first, its text
//! shown in `#perf-text`. Its controls ([`Control`]) mount and unmount it, call its methods
//! (`increment`, `decrement`, `increment-to-max`, `decrement-to-min`, `type-12`:
//! `set_input_value`, `commit`, `set-42`: `set_number_value`), format it as US dollars
//! (`format-as-dollars`), and read its committed value (`value`, -1: empty).

use leptonic::{
    NumberFormatOptions, NumberStyle,
    hooks::form::{NumberFieldState, UseNumberFieldStateInput, use_number_field_state},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let format_options = RwSignal::new(NumberFormatOptions::default());
    // The mounted state, for the controls.
    let state = StoredValue::new(None::<NumberFieldState<i32>>);
    let call = move |method: fn(&NumberFieldState<i32>)| {
        move || {
            if let Some(state) = state.get_value() {
                method(&state);
            }
        }
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("increment", call(NumberFieldState::increment)),
        Control::action("decrement", call(NumberFieldState::decrement)),
        Control::action("increment-to-max", call(NumberFieldState::increment_to_max)),
        Control::action("decrement-to-min", call(NumberFieldState::decrement_to_min)),
        Control::action(
            "type-12",
            call(|state| state.set_input_value("12".to_owned())),
        ),
        Control::action("commit", call(|state| state.commit(None))),
        Control::action("set-42", call(|state| state.set_number_value(Some(42)))),
        Control::action("format-as-dollars", move || {
            format_options.set(NumberFormatOptions {
                style: NumberStyle::Currency,
                currency: Some("USD".to_owned()),
                ..NumberFormatOptions::default()
            });
        }),
        Control::read("value", move || {
            state
                .get_value()
                .and_then(|state| state.value().get_untracked())
                .map_or(-1.0, f64::from)
        }),
    ];
    view! {
        <h1>"use_number_field_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <ShownState format_options state />
        </Show>
    }
}

/// Creates the state, shows its text and hands the state to the page's controls while mounted.
#[component]
fn ShownState(
    format_options: RwSignal<NumberFormatOptions>,
    state: StoredValue<Option<NumberFieldState<i32>>>,
) -> impl IntoView {
    let number = use_number_field_state(UseNumberFieldStateInput {
        default_value: Some(5_i32),
        min_value: Signal::stored(Some(0)),
        max_value: Signal::stored(Some(10)),
        format_options: format_options.into(),
        ..UseNumberFieldStateInput::default()
    });
    state.set_value(Some(number));
    on_cleanup(move || {
        state.try_update_value(|state| *state = None);
    });
    view! { <p id="perf-text">{move || number.input_value.get()}</p> }
}
