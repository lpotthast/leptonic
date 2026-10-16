//! The `DisclosureTrigger` alone, for its performance test
//! (`leptonic/tests/perf_tests/atoms/disclosure_trigger.rs`): in a `Disclosure`
//! (`#perf-disclosure`, collapsed) with its panel, handing its `Button` (`#perf-trigger`) the
//! toggling, between two plain buttons (`#perf-before`, `#perf-after`) to tab from and to, mounted
//! at first. Its controls ([`Control`]) mount and unmount the trigger (the disclosure stays),
//! disable and enable the disclosure, and read how often it reported a change (`changes`).

use leptonic::atoms::{
    button::Button,
    disclosure::{Disclosure, DisclosurePanel, DisclosureTrigger},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let changes = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("changes", move || f64::from(changes.get_value())),
    ];
    view! {
        <h1>"DisclosureTrigger performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Disclosure
            attr:id="perf-disclosure"
            is_disabled=disabled
            on_expanded_change=move |_| changes.update_value(|count| *count += 1)
        >
            <Show when=move || mounted.get()>
                <DisclosureTrigger>
                    <Button id="perf-trigger">"Details"</Button>
                </DisclosureTrigger>
            </Show>
            <DisclosurePanel>"Content"</DisclosurePanel>
        </Disclosure>
        <button id="perf-after">"After"</button>
    }
}
