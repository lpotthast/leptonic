//! The `TooltipTrigger` alone, for its performance test
//! (`leptonic/tests/perf_tests/atoms/tooltip_trigger.rs`): without delays (opening and closing at
//! once, no timers), on a `Button` (`#perf-trigger`), with its `Tooltip`, between two plain
//! buttons (`#perf-before`, `#perf-after`) to tab from and to, mounted at first, closed. The open
//! state is the app's (`is_open` + `set_open`). Its controls ([`Control`]) mount and unmount it,
//! open and close it (the app's state), and disable and enable it.

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
    let disabled = RwSignal::new(false);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("open", move || open.set(true)),
        Control::action("close", move || open.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
    ];
    view! {
        <h1>"TooltipTrigger performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <div style="padding: 4em 2em;">
            <Show when=move || mounted.get()>
                <TooltipTrigger
                    delay=Duration::ZERO
                    close_delay=Duration::ZERO
                    is_open=open
                    set_open=open
                    is_disabled=disabled
                >
                    <Button id="perf-trigger">"Edit"</Button>
                    <Tooltip>"Edit the entry"</Tooltip>
                </TooltipTrigger>
            </Show>
        </div>
        <button id="perf-after">"After"</button>
    }
}
