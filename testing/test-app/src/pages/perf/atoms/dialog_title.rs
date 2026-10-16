//! The `DialogTitle` alone (`.leptonic-DialogTitle`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/dialog_title.rs`): the title ("Settings") of a `Dialog`
//! (`role="dialog"`, not in a modal), naming it, mounted at first. Its controls ([`Control`])
//! mount and unmount the title (the dialog stays).

use leptonic::atoms::dialog::{Dialog, DialogTitle};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
    ];
    view! {
        <h1>"DialogTitle performance"</h1>
        <PerfControls controls />
        <Dialog>
            <Show when=move || mounted.get()>
                <DialogTitle>"Settings"</DialogTitle>
            </Show>
            <button>"Apply"</button>
        </Dialog>
    }
}
