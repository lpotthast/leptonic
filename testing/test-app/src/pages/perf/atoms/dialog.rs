//! The `Dialog` alone (`role="dialog"`, not in a modal), for its performance test
//! (`leptonic/tests/perf_tests/atoms/dialog.rs`): named by its title ("Settings") unless the app
//! gives it an `aria_label`, with a button (`#perf-dialog-button`), between two plain buttons
//! (`#perf-before`, `#perf-after`) and next to a description outside it
//! (`#perf-external-description`), mounted at first (it focuses itself). Its controls
//! ([`Control`]) mount and unmount it, set and unset its `aria_label`, and point its
//! `aria_describedby` at the description outside and unset it.

use leptonic::atoms::dialog::{Dialog, DialogTitle};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let label = RwSignal::new(None::<String>);
    let described_by = RwSignal::new(None::<String>);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("label", move || {
            label.set(Some("Settings dialog".to_owned()));
        }),
        Control::action("unlabel", move || label.set(None)),
        Control::action("describe", move || {
            described_by.set(Some("perf-external-description".to_owned()));
        }),
        Control::action("undescribe", move || described_by.set(None)),
    ];
    view! {
        <h1>"Dialog performance"</h1>
        <PerfControls controls />
        <p id="perf-external-description">"Changes apply at once."</p>
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Dialog aria_label=label aria_describedby=described_by>
                <DialogTitle>"Settings"</DialogTitle>
                <button id="perf-dialog-button">"Apply"</button>
            </Dialog>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
