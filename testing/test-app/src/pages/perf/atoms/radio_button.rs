//! The `RadioButton` (`#perf-radio`: the `<label>` with its hidden input, valued 0) in its
//! `RadioField` and `RadioGroup`, beside a second radio (`#perf-radio-other`, valued 1), for its
//! performance test (`leptonic/tests/perf_tests/atoms/radio_button.rs`): the group uncontrolled,
//! nothing selected at first, between two plain buttons (`#perf-before`, `#perf-after`), the radio
//! mounted at first. Its controls ([`Control`]) mount and unmount the radio (its field; the group
//! stays), disable it and make the group read-only (for the presses it ignores), and read the
//! selected value (`selected`, `-1` for none) and how often it changed (`changes`).

use leptonic::atoms::radio::{RadioButton, RadioField, RadioGroup};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let read_only = RwSignal::new(false);
    let selected = StoredValue::new(None::<usize>);
    let changes = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("make-read-only", move || read_only.set(true)),
        Control::read("selected", move || {
            selected
                .get_value()
                .and_then(|value| u32::try_from(value).ok())
                .map_or(-1.0, f64::from)
        }),
        Control::read("changes", move || f64::from(changes.get_value())),
    ];
    let on_change = move |value: Option<usize>| {
        selected.set_value(value);
        changes.update_value(|count| *count += 1);
    };
    view! {
        <h1>"RadioButton performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <RadioGroup aria_label="Items" is_read_only=read_only on_change>
            <Show when=move || mounted.get()>
                <RadioField value=0_usize is_disabled=disabled>
                    <RadioButton attr:id="perf-radio">"Item"</RadioButton>
                </RadioField>
            </Show>
            <RadioField value=1_usize>
                <RadioButton attr:id="perf-radio-other">"Other"</RadioButton>
            </RadioField>
        </RadioGroup>
        <button id="perf-after">"After"</button>
    }
}
