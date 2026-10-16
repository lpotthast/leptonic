//! The `FieldError` of a `TextField`, for its performance test
//! (`leptonic/tests/perf_tests/atoms/field_error.rs`): mounted at first, in a text field
//! labelled "Name" (input `#perf-input`) that validates as the user edits: a value needs at
//! least three characters ("2 more characters."). The value is the fixture's ("Ada" at first,
//! valid), so its controls ([`Control`]) change it (`set-valid`, `set-short`: "A", `set-empty`),
//! and mount and unmount the error. Between two plain buttons (`#perf-before`, `#perf-after`).

use std::sync::Arc;

use leptonic::{
    atoms::{
        field::{FieldError, Label},
        input::Input,
        text_field::TextField,
    },
    hooks::form::{ValidateFn, ValidationBehavior},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let value = RwSignal::new("Ada".to_owned());
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("set-valid", move || value.set("Ada".to_owned())),
        Control::action("set-short", move || value.set("A".to_owned())),
        Control::action("set-empty", move || value.set(String::new())),
    ];
    let validate: ValidateFn<String> = Arc::new(|value: &String| {
        let missing = 3_usize.saturating_sub(value.chars().count());
        if missing == 0 {
            Ok(())
        } else {
            Err(vec![format!("{missing} more characters.")])
        }
    });
    view! {
        <h1>"FieldError performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <TextField
            id="perf-input"
            value=value
            set_value=value
            validate
            validation_behavior=ValidationBehavior::Aria
        >
            <Label>"Name"</Label>
            <Input />
            <Show when=move || mounted.get()>
                <FieldError />
            </Show>
        </TextField>
        <button id="perf-after">"After"</button>
    }
}
