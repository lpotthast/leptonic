//! `use_field` on a `<label>`, an `<input>` (`#perf-input`), a description (in
//! `#perf-description-slot`) and an error message (in `#perf-error-slot`), for its performance
//! test (`leptonic/tests/perf_tests/hooks/use_field.rs`): mounted at first, with its description
//! shown and its error message hidden, beside a hint (`#perf-hint`), between two plain buttons
//! (`#perf-before`, `#perf-after`). Its controls ([`Control`]) mount and unmount it, show and
//! hide the description and the error message, and describe the input with the hint too.

use leptonic::hooks::form::{UseFieldInput, use_field};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let description = RwSignal::new(true);
    let error = RwSignal::new(false);
    let described_by = RwSignal::new(None::<leptonic::Id>);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("show-description", move || description.set(true)),
        Control::action("hide-description", move || description.set(false)),
        Control::action("show-error", move || error.set(true)),
        Control::action("hide-error", move || error.set(false)),
        Control::action("add-hint", move || {
            described_by.set(Some("perf-hint".into()));
        }),
        Control::action("remove-hint", move || described_by.set(None)),
    ];
    view! {
        <h1>"use_field performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <p id="perf-hint">"Letters only."</p>
        <Show when=move || mounted.get()>
            <Field description error described_by />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// The field: label, input, description and error message.
#[component]
fn Field(
    description: RwSignal<bool>,
    error: RwSignal<bool>,
    described_by: RwSignal<Option<leptonic::Id>>,
) -> impl IntoView {
    let field = use_field(UseFieldInput {
        id: Some("perf-input".into()),
        has_label: true.into(),
        aria_describedby: described_by.into(),
        ..UseFieldInput::default()
    });
    let description_props = field.description;
    let error_message_props = field.error_message;
    view! {
        <label {..field.label.attrs()}>"Name"</label>
        <input {..field.field.attrs()} />
        <div id="perf-description-slot">
            <Show when=move || description.get()>
                <p {..description_props.attrs()}>"Shown to others."</p>
            </Show>
        </div>
        <div id="perf-error-slot">
            <Show when=move || error.get()>
                <p {..error_message_props.attrs()}>"Name taken."</p>
            </Show>
        </div>
    }
}
