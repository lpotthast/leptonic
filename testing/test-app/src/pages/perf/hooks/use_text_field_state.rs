//! `use_text_field_state` ("Ada" at first), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_text_field_state.rs`): mounted at first, its value shown
//! in `#perf-value`. Its controls ([`Control`]) mount and unmount it, set its value
//! (`set-value`: "Grace", `set-same-value`: "Ada", `set-long-value`: 10,000 characters) and read
//! the length of its value (`length`) and how often it reported a change (`changes`).

use leptonic::hooks::form::{TextFieldState, UseTextFieldStateInput, use_text_field_state};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The length of the long value (`set-long-value`).
const LONG_VALUE_LENGTH: usize = 10_000;

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let changes = StoredValue::new(0_u32);
    // The mounted state, for the controls.
    let state = StoredValue::new(None::<TextFieldState>);
    let set = move |value: fn() -> String| {
        move || {
            if let Some(state) = state.get_value() {
                state.set_value(value());
            }
        }
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("set-value", set(|| "Grace".to_owned())),
        Control::action("set-same-value", set(|| "Ada".to_owned())),
        Control::action("set-long-value", set(|| "a".repeat(LONG_VALUE_LENGTH))),
        Control::read("length", move || {
            state.get_value().map_or(0.0, |state| {
                state.value.with_untracked(|value| {
                    f64::from(u32::try_from(value.len()).unwrap_or(u32::MAX))
                })
            })
        }),
        Control::read("changes", move || f64::from(changes.get_value())),
    ];
    view! {
        <h1>"use_text_field_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <ShownState state changes />
        </Show>
    }
}

/// Creates the state, shows its value and hands the state to the page's controls while mounted.
#[component]
fn ShownState(
    state: StoredValue<Option<TextFieldState>>,
    changes: StoredValue<u32>,
) -> impl IntoView {
    let text_field = use_text_field_state(UseTextFieldStateInput {
        default_value: "Ada".to_owned(),
        value: None,
        on_change: Some(Callback::new(move |_: String| {
            changes.update_value(|changes| *changes += 1);
        })),
    });
    state.set_value(Some(text_field));
    on_cleanup(move || {
        state.try_update_value(|state| *state = None);
    });
    view! { <p id="perf-value">{move || text_field.value.get()}</p> }
}
