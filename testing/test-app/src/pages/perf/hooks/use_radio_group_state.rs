//! `use_radio_group_state` ("a" selected at first), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_radio_group_state.rs`): mounted at first, its selected
//! value shown in `#perf-selected`. Its controls ([`Control`]) mount and unmount it, call its
//! methods (`select-a`, `select-b`: `set_selected_value`; `focus-b`:
//! `set_last_focused_value`), make it read-only, and read whether "b" was focused last
//! (`b-focused-last`).

use leptonic::hooks::{
    collections::Key,
    form::{RadioGroupState, UseRadioGroupStateInput, use_radio_group_state},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let read_only = RwSignal::new(false);
    // The mounted state, for the controls.
    let state = StoredValue::new(None::<RadioGroupState>);
    let call = move |method: fn(&RadioGroupState)| {
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
            "select-a",
            call(|state| state.set_selected_value(Some(Key::from("a")))),
        ),
        Control::action(
            "select-b",
            call(|state| state.set_selected_value(Some(Key::from("b")))),
        ),
        Control::action(
            "focus-b",
            call(|state| state.set_last_focused_value(Some(Key::from("b")))),
        ),
        Control::action("set-read-only", move || read_only.set(true)),
        Control::read("b-focused-last", move || {
            state.get_value().map_or(0.0, |state| {
                f64::from(u8::from(
                    state
                        .last_focused_value
                        .with_untracked(|last| *last == Some(Key::from("b"))),
                ))
            })
        }),
    ];
    view! {
        <h1>"use_radio_group_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <ShownState read_only state />
        </Show>
    }
}

/// Creates the state, shows its selected value and hands the state to the page's controls
/// while mounted.
#[component]
fn ShownState(
    read_only: RwSignal<bool>,
    state: StoredValue<Option<RadioGroupState>>,
) -> impl IntoView {
    let group = use_radio_group_state(UseRadioGroupStateInput {
        default_value: Some(Key::from("a")),
        name: Some("letter".to_owned()),
        is_read_only: read_only.into(),
        ..UseRadioGroupStateInput::default()
    });
    state.set_value(Some(group));
    on_cleanup(move || {
        state.try_update_value(|state| *state = None);
    });
    let selected = group.selected_value;
    view! {
        <p id="perf-selected">
            {move || selected.get().map_or_else(String::new, |selected| selected.to_string())}
        </p>
    }
}
