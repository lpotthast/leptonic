//! `use_form_validation_state` (native validation) of a value of the page ("Ada" at first; at
//! least three characters, named `name`), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_form_validation_state.rs`): mounted at first, in a
//! [`FormValidationContext`] with the server's errors. The state renders nothing; its controls
//! ([`Control`]) mount and unmount it, change its value and props, call its methods
//! (`commit`, `reset`, `report-native-error`: `update_validation`) and read it (`exists`,
//! `invalid`, `errors`: the number of displayed errors).

use std::{collections::HashMap, sync::Arc};

use leptonic::hooks::form::{
    CUSTOM_VALIDITY_STATE, FormValidationContext, FormValidationState, UseFormValidationStateInput,
    ValidationBehavior, ValidationResult, use_form_validation_state,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let value = RwSignal::new("Ada".to_owned());
    let is_invalid = RwSignal::new(false);
    let server_errors = RwSignal::new(HashMap::<String, Vec<String>>::new());
    provide_context(FormValidationContext {
        errors: server_errors.into(),
    });
    // The mounted state, for the controls.
    let state = StoredValue::new(None::<FormValidationState>);
    let read = move |read: fn(&FormValidationState) -> f64| {
        move || state.get_value().as_ref().map_or(0.0, read)
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("set-short", move || value.set("A".to_owned())),
        Control::action("invalidate", move || is_invalid.set(true)),
        Control::action("uninvalidate", move || is_invalid.set(false)),
        Control::action("set-server-errors", move || {
            server_errors.set(HashMap::from([(
                "name".to_owned(),
                vec!["Name taken.".to_owned()],
            )]));
        }),
        Control::action("commit", move || {
            if let Some(state) = state.get_value() {
                state.commit_validation();
            }
        }),
        Control::action("reset", move || {
            if let Some(state) = state.get_value() {
                state.reset_validation();
            }
        }),
        Control::action("report-native-error", move || {
            if let Some(state) = state.get_value() {
                state.update_validation(ValidationResult {
                    is_invalid: true,
                    validation_errors: vec!["Native error.".to_owned()],
                    validation_details: CUSTOM_VALIDITY_STATE,
                });
            }
        }),
        Control::read("exists", read(|_| 1.0)),
        Control::read(
            "invalid",
            read(|state| f64::from(u8::from(state.is_invalid.get_untracked()))),
        ),
        Control::read(
            "errors",
            read(|state| {
                state
                    .validation_errors
                    .with_untracked(|errors| f64::from(u32::try_from(errors.len()).unwrap_or(0)))
            }),
        ),
    ];
    view! {
        <h1>"use_form_validation_state performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <ValidationState value is_invalid state />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// Creates the validation state and hands it to the page's controls while mounted.
#[component]
fn ValidationState(
    value: RwSignal<String>,
    is_invalid: RwSignal<bool>,
    state: StoredValue<Option<FormValidationState>>,
) -> impl IntoView {
    let validation = use_form_validation_state(UseFormValidationStateInput {
        is_invalid: is_invalid.into(),
        value: value.into(),
        validate: Some(Arc::new(|value: &String| {
            if value.chars().count() < 3 {
                Err(vec!["At least 3 characters.".to_owned()])
            } else {
                Ok(())
            }
        })),
        builtin_validation: Signal::default(),
        validation_behavior: ValidationBehavior::Native,
        names: vec!["name".to_owned()],
    });
    state.set_value(Some(validation));
    on_cleanup(move || {
        state.try_update_value(|state| *state = None);
    });
}
