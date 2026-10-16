//! The `OverlayArrow` alone (`.leptonic-OverlayArrow`, a 12px square), for its performance test
//! (`leptonic/tests/perf_tests/atoms/overlay_arrow.rs`): the arrow of an open, non-modal `Popover`
//! (below its `DialogTrigger`'s `Button`, `#perf-trigger`, 150px from the top), mounted at first.
//! Its controls ([`Control`]) mount and unmount the arrow (the popover stays open), and move the
//! popover above the trigger and back.

use leptonic::{
    atoms::{button::Button, dialog::DialogTrigger, overlay_arrow::OverlayArrow, popover::Popover},
    hooks::overlay::{Placement, PopoverModality},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let placement = RwSignal::new(Placement::Bottom);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("place-above", move || placement.set(Placement::Top)),
        Control::action("place-below", move || placement.set(Placement::Bottom)),
    ];
    view! {
        <h1>"OverlayArrow performance"</h1>
        <PerfControls controls />
        <div style="padding: 150px 2em;">
            <DialogTrigger default_open=true>
                <Button id="perf-trigger">"Settings"</Button>
                <Popover modality=PopoverModality::NonModal placement=placement offset=12.0>
                    <Show when=move || mounted.get()>
                        <OverlayArrow>
                            <div style="width: 12px; height: 12px; background: black;" />
                        </OverlayArrow>
                    </Show>
                    <p>"Popover content"</p>
                </Popover>
            </DialogTrigger>
        </div>
    }
}
