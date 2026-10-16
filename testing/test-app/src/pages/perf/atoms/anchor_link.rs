//! The `AnchorLink` alone (`#perf-anchor-link`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/anchor_link.rs`): linking to `#perf-anchor-target` far below
//! (scrolling instantly), between two plain buttons (`#perf-before`, `#perf-after`) to tab from
//! and to, mounted at first. Its controls ([`Control`]) mount and unmount it, change its label,
//! and read how often it was pressed (`presses`).

use leptonic::{ScrollBehavior, atoms::link::AnchorLink};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let label = RwSignal::new(None::<String>);
    let press_count = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("label", move || label.set(Some("To the target".to_owned()))),
        Control::action("unlabel", move || label.set(None)),
        Control::read("presses", move || f64::from(press_count.get_value())),
    ];
    view! {
        <h1>"AnchorLink performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <AnchorLink
                href="#perf-anchor-target"
                attr:id="perf-anchor-link"
                scroll_behavior=ScrollBehavior::Instant
                aria_label=label
                on_press=move |_| press_count.update_value(|count| *count += 1)
            >
                "To the target"
            </AnchorLink>
        </Show>
        <button id="perf-after">"After"</button>
        <div style="height: 3000px"></div>
        <div id="perf-anchor-target">"Target"</div>
    }
}
