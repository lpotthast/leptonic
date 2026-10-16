//! `use_checkbox_group_state` (nothing checked at first), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_checkbox_group_state.rs`): mounted at first, the number
//! of checked values shown in `#perf-count`. Its controls ([`Control`]) mount and unmount it,
//! call its methods (`add-a`, `remove-a`, `toggle-a`: values "a"; `check-1000`: `set_value` of
//! the values 0 to 999; `invalidate-a`: `set_invalid` for "a"), make it read-only, and read
//! whether it shows itself invalid (`invalid`).

use std::collections::HashSet;

use leptonic::hooks::{
    collections::Key,
    form::{
        CUSTOM_VALIDITY_STATE, CheckboxGroupState, UseCheckboxGroupStateInput, ValidationBehavior,
        ValidationResult, use_checkbox_group_state,
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The number of values `check-1000` checks.
const MANY: i32 = 1_000;

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let read_only = RwSignal::new(false);
    // The mounted state, for the controls.
    let state = StoredValue::new(None::<CheckboxGroupState>);
    let call = move |method: fn(&CheckboxGroupState)| {
        move || {
            if let Some(state) = state.get_value() {
                method(&state);
            }
        }
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("add-a", call(|state| state.add_value(Key::from("a")))),
        Control::action(
            "remove-a",
            call(|state| state.remove_value(&Key::from("a"))),
        ),
        Control::action("toggle-a", call(|state| state.toggle_value(Key::from("a")))),
        Control::action(
            "check-1000",
            call(|state| state.set_value((0..MANY).map(Key::from).collect())),
        ),
        Control::action(
            "invalidate-a",
            call(|state| {
                state.set_invalid(
                    Key::from("a"),
                    ValidationResult {
                        is_invalid: true,
                        validation_errors: vec!["Check a.".to_owned()],
                        validation_details: CUSTOM_VALIDITY_STATE,
                    },
                );
            }),
        ),
        Control::action("set-read-only", move || read_only.set(true)),
        Control::read("invalid", move || {
            state.get_value().map_or(0.0, |state| {
                f64::from(u8::from(state.is_invalid.get_untracked()))
            })
        }),
    ];
    view! {
        <h1>"use_checkbox_group_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <ShownState read_only state />
        </Show>
    }
}

/// Creates the state, shows how many values are checked and hands the state to the page's
/// controls while mounted.
#[component]
fn ShownState(
    read_only: RwSignal<bool>,
    state: StoredValue<Option<CheckboxGroupState>>,
) -> impl IntoView {
    let group = use_checkbox_group_state(UseCheckboxGroupStateInput {
        default_value: HashSet::new(),
        is_read_only: read_only.into(),
        validation_behavior: ValidationBehavior::Aria,
        name: Some("letters".to_owned()),
        ..UseCheckboxGroupStateInput::default()
    });
    state.set_value(Some(group));
    on_cleanup(move || {
        state.try_update_value(|state| *state = None);
    });
    let value = group.value;
    view! { <p id="perf-count">{move || value.with(HashSet::len)}</p> }
}
