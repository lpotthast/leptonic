//! The `CheckboxField` (`#perf-checkbox-field`) with its `CheckboxButton` (`#perf-checkbox`), for
//! its performance test (`leptonic/tests/perf_tests/atoms/checkbox_field.rs`): its selection
//! controlled by app state (`is_selected` + `set_selected`, unchecked at first), between two plain
//! buttons (`#perf-before`, `#perf-after`), mounted at first. Its controls ([`Control`]) mount and
//! unmount it, change the app's selection and the field's reactive props, and read the selection
//! (`selected`).

use leptonic::atoms::checkbox::{CheckboxButton, CheckboxField};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let selected = RwSignal::new(false);
    let indeterminate = RwSignal::new(false);
    let disabled = RwSignal::new(false);
    let read_only = RwSignal::new(false);
    let required = RwSignal::new(false);
    let invalid = RwSignal::new(false);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("select", move || selected.set(true)),
        Control::action("deselect", move || selected.set(false)),
        Control::action("set-indeterminate", move || indeterminate.set(true)),
        Control::action("unset-indeterminate", move || indeterminate.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("make-read-only", move || read_only.set(true)),
        Control::action("make-editable", move || read_only.set(false)),
        Control::action("require", move || required.set(true)),
        Control::action("make-optional", move || required.set(false)),
        Control::action("invalidate", move || invalid.set(true)),
        Control::action("validate", move || invalid.set(false)),
        Control::read("selected", move || {
            f64::from(u8::from(selected.get_untracked()))
        }),
    ];
    view! {
        <h1>"CheckboxField performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <CheckboxField
                attr:id="perf-checkbox-field"
                is_selected=selected
                set_selected=selected
                is_indeterminate=indeterminate
                is_disabled=disabled
                is_read_only=read_only
                is_required=required
                is_invalid=invalid
            >
                <CheckboxButton attr:id="perf-checkbox">"Check me"</CheckboxButton>
            </CheckboxField>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
