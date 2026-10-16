//! The `Tooltip` alone (`role="tooltip"`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/tooltip.rs`): the tooltip of a `TooltipTrigger` without delays
//! (opening and closing at once, no timers) on a `Button` (`#perf-trigger`), in a scrolling
//! container (`#perf-scroller`), between two plain buttons (`#perf-before`, `#perf-after`) to tab
//! from and to, mounted at first, closed. The open state is the app's (`is_open` + `set_open`).
//! Its controls ([`Control`]) mount and unmount the tooltip (the trigger stays), and open and
//! close it (the app's state).

use std::time::Duration;

use leptonic::atoms::{
    button::Button,
    tooltip::{Tooltip, TooltipTrigger},
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
        <h1>"Tooltip performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <div id="perf-scroller" style="overflow: auto; height: 200px;">
            <div style="height: 400px; padding: 4em 2em;">
                <TooltipTrigger
                    delay=Duration::ZERO
                    close_delay=Duration::ZERO
                    is_open=open
                    set_open=open
                >
                    <Button id="perf-trigger">"Edit"</Button>
                    <Show when=move || mounted.get()>
                        <Tooltip>"Edit the entry"</Tooltip>
                    </Show>
                </TooltipTrigger>
            </div>
        </div>
        <button id="perf-after">"After"</button>
    }
}
