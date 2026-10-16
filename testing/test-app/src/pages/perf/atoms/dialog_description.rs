//! The `DialogDescription` alone (`.leptonic-DialogDescription`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/dialog_description.rs`): the description of an alert dialog
//! (`role="alertdialog"`, not in a modal; its content describes it), mounted at first. Its
//! controls ([`Control`]) mount and unmount the description (the dialog stays).

use leptonic::{
    atoms::dialog::{Dialog, DialogDescription, DialogTitle},
    hooks::dialog::DialogRole,
};
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
        <h1>"DialogDescription performance"</h1>
        <PerfControls controls />
        <Dialog role=DialogRole::AlertDialog>
            <DialogTitle>"Leave"</DialogTitle>
            <Show when=move || mounted.get()>
                <DialogDescription>"Leave this page?"</DialogDescription>
            </Show>
            <button>"Leave"</button>
        </Dialog>
    }
}
