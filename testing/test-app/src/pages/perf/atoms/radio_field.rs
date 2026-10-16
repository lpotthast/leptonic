//! The `RadioField` (`#perf-radio-field`, valued 0, with its `RadioButton`) in its `RadioGroup`,
//! beside a second radio (valued 1), for its performance test
//! (`leptonic/tests/perf_tests/atoms/radio_field.rs`): the group's value controlled by app state
//! (`value` + `set_value`, nothing selected at first), between two plain buttons (`#perf-before`,
//! `#perf-after`), the field mounted at first. Its controls ([`Control`]) mount and unmount the
//! field (the group stays), select it, the other radio or none, and disable and enable it.

use leptonic::atoms::radio::{RadioButton, RadioField, RadioGroup};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let value = RwSignal::new(None::<usize>);
    let disabled = RwSignal::new(false);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("select", move || value.set(Some(0))),
        Control::action("select-other", move || value.set(Some(1))),
        Control::action("select-none", move || value.set(None)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
    ];
    view! {
        <h1>"RadioField performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <RadioGroup aria_label="Items" value=value set_value=value>
            <Show when=move || mounted.get()>
                <RadioField attr:id="perf-radio-field" value=0_usize is_disabled=disabled>
                    <RadioButton>"Item"</RadioButton>
                </RadioField>
            </Show>
            <RadioField attr:id="perf-radio-field-other" value=1_usize>
                <RadioButton>"Other"</RadioButton>
            </RadioField>
        </RadioGroup>
        <button id="perf-after">"After"</button>
    }
}
