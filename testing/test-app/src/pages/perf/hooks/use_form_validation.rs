//! `use_form_validation` (native validation) on an `<input>` (`#perf-input`) in a `<form>`
//! (`#perf-form`), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_form_validation.rs`): mounted at first, its value "Ada"
//! at first, validated by a `use_form_validation_state` of the page (letters only). Between two
//! plain buttons (`#perf-before`, `#perf-after`). Its controls ([`Control`]) mount and unmount
//! it, set an invalid value (`set-invalid`: "Ada1"), make the input `required` and read whether
//! the field shows itself invalid (`invalid`: 1).

use std::sync::Arc;

use leptonic::{
    CapturedElement,
    hooks::form::{
        FormValidationState, UseFormValidationInput, UseFormValidationStateInput,
        ValidationBehavior, use_form_validation, use_form_validation_state,
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let value = RwSignal::new("Ada".to_owned());
    let required = RwSignal::new(false);
    let state = use_form_validation_state(UseFormValidationStateInput {
        is_invalid: Signal::stored(false),
        value: value.into(),
        validate: Some(Arc::new(|value: &String| {
            if value.chars().any(|c| c.is_ascii_digit()) {
                Err(vec!["Letters only.".to_owned()])
            } else {
                Ok(())
            }
        })),
        builtin_validation: Signal::default(),
        validation_behavior: ValidationBehavior::Native,
        names: Vec::new(),
    });
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("set-invalid", move || value.set("Ada1".to_owned())),
        Control::action("require", move || required.set(true)),
        Control::action("unrequire", move || required.set(false)),
        Control::read("invalid", move || {
            f64::from(u8::from(state.is_invalid.get_untracked()))
        }),
    ];
    view! {
        <h1>"use_form_validation performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <form id="perf-form">
            <Show when=move || mounted.get()>
                <ValidatedInput value required state />
            </Show>
        </form>
        <button id="perf-after">"After"</button>
    }
}

/// The validated input.
#[component]
fn ValidatedInput(
    value: RwSignal<String>,
    required: RwSignal<bool>,
    state: FormValidationState,
) -> impl IntoView {
    let element = CapturedElement::new();
    use_form_validation(UseFormValidationInput {
        element,
        state,
        validation_behavior: ValidationBehavior::Native,
        focus: None,
    });
    view! {
        <input
            id="perf-input"
            aria-label="Name"
            {..element.attr()}
            required=required
            prop:value=value
            on:input=move |e| value.set(event_target_value(&e))
        />
    }
}
