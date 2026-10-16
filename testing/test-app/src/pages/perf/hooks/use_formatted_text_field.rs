//! `use_formatted_text_field` on an `<input>` (`#perf-input`) showing the text of a
//! `use_number_field_state` (of `i32`, 5 at first), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_formatted_text_field.rs`): mounted at first, between two
//! plain buttons (`#perf-before`, `#perf-after`). Its controls ([`Control`]) mount and unmount
//! it.

use leptonic::{
    CapturedElement,
    hooks::form::{
        FormattedTextFieldHandlers, UseFormattedTextFieldInput, UseNumberFieldStateInput,
        use_formatted_text_field, use_number_field_state,
    },
};
use leptos::{ev, prelude::*};

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
    ];
    view! {
        <h1>"use_formatted_text_field performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <FormattedInput />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// The input whose text stays a valid number.
#[component]
fn FormattedInput() -> impl IntoView {
    let state = use_number_field_state(UseNumberFieldStateInput {
        default_value: Some(5_i32),
        ..UseNumberFieldStateInput::default()
    });
    let element = CapturedElement::new();
    let FormattedTextFieldHandlers {
        on_beforeinput,
        on_compositionstart,
        on_compositionend,
    } = use_formatted_text_field(UseFormattedTextFieldInput { element, state });
    view! {
        <input
            id="perf-input"
            aria-label="Quantity"
            {..element.attr()}
            {..on_beforeinput.into_on(ev::beforeinput)}
            {..on_compositionstart.into_on(ev::compositionstart)}
            {..on_compositionend.into_on(ev::compositionend)}
            prop:value=state.input_value
            on:input=move |e| state.set_input_value(event_target_value(&e))
        />
    }
}
