//! The `DialogTrigger` alone, for its performance test
//! (`leptonic/tests/perf_tests/atoms/dialog_trigger.rs`): its `Button` (`#perf-trigger`) opening a
//! dismissable modal (`ModalBackdrop`, `ModalContent`) with a `Dialog` ("Settings"), between two
//! plain buttons (`#perf-before`, `#perf-after`) to tab from and to, mounted at first, closed. The
//! open state is the app's (`is_open` + `set_open`). Its controls ([`Control`]) mount and unmount
//! it, and open and close it (the app's state). The backdrop covers the viewport.

use leptonic::atoms::{
    button::Button,
    dialog::{Dialog, DialogTrigger},
    modal::{ModalBackdrop, ModalContent},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let open = RwSignal::new(false);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("open", move || open.set(true)),
        Control::action("close", move || open.set(false)),
    ];
    view! {
        <h1>"DialogTrigger performance"</h1>
        <style>
            ".leptonic-ModalBackdrop { position: fixed; inset: 0; background: rgb(0 0 0 / 0.3); }
            .leptonic-ModalContent { margin: 100px auto; width: 300px; background: white; }"
        </style>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <DialogTrigger is_open=open set_open=open>
                <Button id="perf-trigger">"Settings"</Button>
                <ModalBackdrop is_dismissable=true>
                    <ModalContent>
                        <Dialog aria_label="Settings">
                            <button>"Apply"</button>
                        </Dialog>
                    </ModalContent>
                </ModalBackdrop>
            </DialogTrigger>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
