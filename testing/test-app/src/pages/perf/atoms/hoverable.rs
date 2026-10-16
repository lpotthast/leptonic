//! The `Hoverable` alone, on a plain `<div>` (`#perf-hoverable`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/hoverable.rs`): between two plain buttons (`#perf-before`,
//! `#perf-after`) to move the pointer to, mounted at first. Its controls ([`Control`]) mount and
//! unmount it, disable and enable it, and read how often a hover started (`hovers`).

use leptonic::atoms::hoverable::Hoverable;
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let hover_count = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("hovers", move || f64::from(hover_count.get_value())),
    ];
    view! {
        <h1>"Hoverable performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Hoverable
                is_disabled=disabled
                on_hover_start=move |_| hover_count.update_value(|count| *count += 1)
            >
                <div id="perf-hoverable">"Hover me"</div>
            </Hoverable>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
