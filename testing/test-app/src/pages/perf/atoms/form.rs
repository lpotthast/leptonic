//! A `Form` (`#perf-form`, native validation) with one required `TextField` named `name`
//! (input `#perf-input`, its `FieldError`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/form.rs`): mounted at first, between two plain buttons
//! (`#perf-before`, `#perf-after`). Its controls ([`Control`]) mount and unmount it and set and
//! clear the server's validation errors (`set-server-errors`: "Name taken." for `name`).

use std::collections::HashMap;

use leptonic::atoms::{
    field::{FieldError, Label},
    form::Form,
    input::Input,
    text_field::TextField,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let server_errors = RwSignal::new(HashMap::<String, Vec<String>>::new());
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("set-server-errors", move || {
            server_errors.set(HashMap::from([(
                "name".to_owned(),
                vec!["Name taken.".to_owned()],
            )]));
        }),
        Control::action("clear-server-errors", move || {
            server_errors.set(HashMap::new());
        }),
    ];
    view! {
        <h1>"Form performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Form id="perf-form" validation_errors=server_errors>
                <TextField id="perf-input" name="name" is_required=true>
                    <Label>"Name"</Label>
                    <Input />
                    <FieldError />
                </TextField>
            </Form>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
