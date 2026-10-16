//! The `Popover` alone (`.leptonic-Popover`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/popover.rs`): the popover of a `DialogTrigger` (its `Button`
//! `#perf-trigger`, 150px from the top), with two buttons (`#perf-first`, `#perf-last`; no dialog
//! inside, so the popover is the dialog), in a scrolling container (`#perf-scroller`), between two
//! plain buttons (`#perf-before`, `#perf-after`), mounted at first, closed, modal, below the
//! trigger. The open state is the app's (`is_open` + `set_open` of the trigger). Its controls
//! ([`Control`]) mount it (modal, or non-modal: `mount-non-modal`) and unmount it (the trigger
//! stays), open and close it (the app's state), move it above the trigger and back, and change its
//! offset from the trigger.

use leptonic::{
    atoms::{button::Button, dialog::DialogTrigger, popover::Popover},
    hooks::overlay::{Placement, PopoverModality},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let modality = RwSignal::new(PopoverModality::Modal);
    let open = RwSignal::new(false);
    let placement = RwSignal::new(Placement::Bottom);
    let offset = RwSignal::new(8.0);
    let controls = vec![
        Control::action("mount", move || {
            modality.set(PopoverModality::Modal);
            mounted.set(true);
        }),
        Control::action("mount-non-modal", move || {
            modality.set(PopoverModality::NonModal);
            mounted.set(true);
        }),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("open", move || open.set(true)),
        Control::action("close", move || open.set(false)),
        Control::action("place-above", move || placement.set(Placement::Top)),
        Control::action("place-below", move || placement.set(Placement::Bottom)),
        Control::action("offset-far", move || offset.set(24.0)),
        Control::action("offset-near", move || offset.set(8.0)),
    ];
    view! {
        <h1>"Popover performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <div id="perf-scroller" style="overflow: auto; height: 400px;">
            <div style="height: 800px; padding: 150px 2em;">
                <DialogTrigger is_open=open set_open=open>
                    <Button id="perf-trigger">"Settings"</Button>
                    <Show when=move || mounted.get()>
                        <Popover
                            modality=modality.get_untracked()
                            placement=placement
                            offset=offset
                            aria_label="Settings"
                        >
                            <button id="perf-first">"First"</button>
                            <button id="perf-last">"Last"</button>
                        </Popover>
                    </Show>
                </DialogTrigger>
            </div>
        </div>
        <button id="perf-after">"After"</button>
    }
}
