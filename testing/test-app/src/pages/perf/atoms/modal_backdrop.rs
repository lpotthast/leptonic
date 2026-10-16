//! The `ModalBackdrop` alone (`.leptonic-ModalBackdrop`, covering the viewport), for its
//! performance test (`leptonic/tests/perf_tests/atoms/modal_backdrop.rs`): a modal (its
//! `ModalContent`, 300px wide, 100px from the top) with a `Dialog` ("Settings"), next to a plain
//! button (`#perf-opener`) to focus before opening it, mounted at first, closed, dismissable. The
//! open state is the app's (`is_open` + `set_open`). Its controls ([`Control`]) mount and unmount
//! it, open and close it (the app's state), make it dismissable or not, and disable and enable
//! dismissing it with Escape.

use leptonic::atoms::{
    dialog::Dialog,
    modal::{ModalBackdrop, ModalContent},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let open = RwSignal::new(false);
    let dismissable = RwSignal::new(true);
    let keyboard_dismiss_disabled = RwSignal::new(false);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("open", move || open.set(true)),
        Control::action("close", move || open.set(false)),
        Control::action("dismissable", move || dismissable.set(true)),
        Control::action("not-dismissable", move || dismissable.set(false)),
        Control::action("disable-keyboard-dismiss", move || {
            keyboard_dismiss_disabled.set(true);
        }),
        Control::action("enable-keyboard-dismiss", move || {
            keyboard_dismiss_disabled.set(false);
        }),
    ];
    view! {
        <h1>"ModalBackdrop performance"</h1>
        <style>
            ".leptonic-ModalBackdrop { position: fixed; inset: 0; background: rgb(0 0 0 / 0.3); }
            .leptonic-ModalContent { margin: 100px auto; width: 300px; background: white; }"
        </style>
        <PerfControls controls />
        <button id="perf-opener">"Opener"</button>
        <Show when=move || mounted.get()>
            <ModalBackdrop
                is_open=open
                set_open=open
                is_dismissable=dismissable
                is_keyboard_dismiss_disabled=keyboard_dismiss_disabled
            >
                <ModalContent>
                    <Dialog aria_label="Settings">
                        <button>"Apply"</button>
                    </Dialog>
                </ModalContent>
            </ModalBackdrop>
        </Show>
    }
}
