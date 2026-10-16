//! The `DisclosurePanel` alone, for its performance test
//! (`leptonic/tests/perf_tests/atoms/disclosure_panel.rs`): in a `Disclosure` (`#perf-disclosure`,
//! collapsed, its trigger `#perf-trigger`), with a button inside (`#perf-panel-button`), between
//! two plain buttons (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. The
//! expanded state is the app's (`is_expanded` + `set_expanded`). Its controls ([`Control`]) mount
//! and unmount the panel (the disclosure stays), and expand and collapse the disclosure.

use leptonic::atoms::{
    button::Button,
    disclosure::{Disclosure, DisclosurePanel, DisclosureTrigger},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let expanded = RwSignal::new(false);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("expand", move || expanded.set(true)),
        Control::action("collapse", move || expanded.set(false)),
    ];
    view! {
        <h1>"DisclosurePanel performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Disclosure attr:id="perf-disclosure" is_expanded=expanded set_expanded=expanded>
            <DisclosureTrigger>
                <Button id="perf-trigger">"Details"</Button>
            </DisclosureTrigger>
            <Show when=move || mounted.get()>
                <DisclosurePanel>
                    <button id="perf-panel-button">"Content"</button>
                </DisclosurePanel>
            </Show>
        </Disclosure>
        <button id="perf-after">"After"</button>
    }
}
