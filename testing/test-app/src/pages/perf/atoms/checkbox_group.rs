//! The `CheckboxGroup` (`#perf-checkbox-group`) of 10 or 1,000 checkboxes (`CheckboxField`s with
//! their `CheckboxButton`s, valued 0, 1, ...), for its performance test
//! (`leptonic/tests/perf_tests/atoms/checkbox_group.rs`): its value controlled by app state
//! (`value` + `set_value`, nothing checked at first), between two plain buttons (`#perf-before`,
//! `#perf-after`), mounted with 10 checkboxes at first. Its controls ([`Control`]) mount it (with
//! 10 or 1,000 checkboxes) and unmount it, check all or none of them, change its reactive props,
//! and read how many are checked (`checked`).

use std::collections::HashSet;

use leptonic::atoms::checkbox::{CheckboxButton, CheckboxField, CheckboxGroup};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The checkboxes of the small group.
const SMALL: usize = 10;
/// The checkboxes of the large group.
const LARGE: usize = 1_000;

#[component]
pub fn PagePerf() -> impl IntoView {
    // The number of checkboxes of the mounted group.
    let size = RwSignal::new(Some(SMALL));
    let value = RwSignal::new(HashSet::<usize>::new());
    let disabled = RwSignal::new(false);
    let read_only = RwSignal::new(false);
    let required = RwSignal::new(false);
    let invalid = RwSignal::new(false);
    let controls = vec![
        Control::action("mount", move || size.set(Some(SMALL))),
        Control::action("mount-large", move || size.set(Some(LARGE))),
        Control::action("unmount", move || size.set(None)),
        Control::action("check-all", move || {
            let count = size.get_untracked().unwrap_or(0);
            value.set((0..count).collect());
        }),
        Control::action("check-none", move || value.set(HashSet::new())),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("make-read-only", move || read_only.set(true)),
        Control::action("make-editable", move || read_only.set(false)),
        Control::action("require", move || required.set(true)),
        Control::action("make-optional", move || required.set(false)),
        Control::action("invalidate", move || invalid.set(true)),
        Control::action("validate", move || invalid.set(false)),
        Control::read("checked", move || {
            f64::from(u32::try_from(value.with_untracked(HashSet::len)).unwrap_or(u32::MAX))
        }),
    ];
    view! {
        <h1>"CheckboxGroup performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        {move || {
            size.get()
                .map(|count| {
                    view! {
                        <CheckboxGroup
                            id="perf-checkbox-group"
                            aria_label="Items"
                            value=value
                            set_value=value
                            is_disabled=disabled
                            is_read_only=read_only
                            is_required=required
                            is_invalid=invalid
                        >
                            {(0..count)
                                .map(|item| {
                                    view! {
                                        <CheckboxField value=item>
                                            <CheckboxButton>"Item"</CheckboxButton>
                                        </CheckboxField>
                                    }
                                })
                                .collect_view()}
                        </CheckboxGroup>
                    }
                })
        }}
        <button id="perf-after">"After"</button>
    }
}
