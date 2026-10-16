//! A `TextField` (its input `#perf-input`) with a label "Name", a
//! description and an error, for its performance test
//! (`leptonic/tests/perf_tests/atoms/text_field.rs`): mounted at first, its value "Ada" (the
//! page's: `value` and `set_value`), between two plain buttons (`#perf-before`, `#perf-after`).
//! Its controls ([`Control`]) mount and unmount it, change its reactive props and set its value
//! (`set-value`: "Grace", `set-long-value`: 10,000 characters).

use leptonic::atoms::{
    field::{Description, FieldError, Label},
    input::Input,
    text_field::TextField,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The length of the long value (`set-long-value`).
const LONG_VALUE_LENGTH: usize = 10_000;

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let value = RwSignal::new("Ada".to_owned());
    let disabled = RwSignal::new(false);
    let read_only = RwSignal::new(false);
    let required = RwSignal::new(false);
    let invalid = RwSignal::new(false);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("set-read-only", move || read_only.set(true)),
        Control::action("unset-read-only", move || read_only.set(false)),
        Control::action("require", move || required.set(true)),
        Control::action("unrequire", move || required.set(false)),
        Control::action("invalidate", move || invalid.set(true)),
        Control::action("uninvalidate", move || invalid.set(false)),
        Control::action("set-value", move || value.set("Grace".to_owned())),
        Control::action("set-long-value", move || {
            value.set("a".repeat(LONG_VALUE_LENGTH));
        }),
    ];
    view! {
        <h1>"TextField performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <TextField
                id="perf-input"
                value=value
                set_value=value
                is_disabled=disabled
                is_read_only=read_only
                is_required=required
                is_invalid=invalid
            >
                <Label>"Name"</Label>
                <Input />
                <Description>"Shown to others."</Description>
                <FieldError>"Invalid name."</FieldError>
            </TextField>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
