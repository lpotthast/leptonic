//! The `ModalContent` alone (`.leptonic-ModalContent`, 300px wide, 100px from the top), for its
//! performance test (`leptonic/tests/perf_tests/atoms/modal_content.rs`): the content of an open,
//! dismissable `ModalBackdrop` (covering the viewport), with a `Dialog` ("Settings") holding two
//! buttons (`#perf-first`, `#perf-last`), mounted at first. Its controls ([`Control`]) mount and
//! unmount the content (the backdrop stays open), and stop and start containing the focus.

use leptonic::atoms::{
    dialog::Dialog,
    modal::{ModalBackdrop, ModalContent},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let open = RwSignal::new(true);
    let contain = RwSignal::new(true);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("contain", move || contain.set(true)),
        Control::action("uncontain", move || contain.set(false)),
    ];
    view! {
        <h1>"ModalContent performance"</h1>
        <style>
            ".leptonic-ModalBackdrop { position: fixed; inset: 0; background: rgb(0 0 0 / 0.3); }
            .leptonic-ModalContent { margin: 100px auto; width: 300px; background: white; }"
        </style>
        <PerfControls controls />
        <ModalBackdrop is_open=open set_open=open is_dismissable=true>
            <Show when=move || mounted.get()>
                <ModalContent contain_focus=contain>
                    <Dialog aria_label="Settings">
                        <button id="perf-first">"First"</button>
                        <button id="perf-last">"Last"</button>
                    </Dialog>
                </ModalContent>
            </Show>
        </ModalBackdrop>
    }
}
