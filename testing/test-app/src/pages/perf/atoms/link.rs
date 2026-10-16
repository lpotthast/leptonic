//! The `Link` alone (`.perf-link`: a class, which stays when it switches between `<a>` and
//! `<span>`; to this page with a query), for its performance test
//! (`leptonic/tests/perf_tests/atoms/link.rs`): between two plain buttons (`#perf-before`,
//! `#perf-after`) to tab from and to, mounted at first. A surrounding element prevents the
//! default of its clicks, so that a press measures the link and not a navigation, until the
//! control `allow-navigation`. Its controls ([`Control`]) mount and unmount it, change its
//! reactive props, and read how often it was pressed (`presses`).

use leptonic::{atoms::link::Link, hooks::link::LinkTarget};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let target = RwSignal::new(LinkTarget::Same);
    let prevent_navigation = StoredValue::new(true);
    let press_count = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("open-in-new-tab", move || target.set(LinkTarget::Blank)),
        Control::action("open-here", move || target.set(LinkTarget::Same)),
        Control::action("allow-navigation", move || {
            prevent_navigation.set_value(false);
        }),
        Control::read("presses", move || f64::from(press_count.get_value())),
    ];
    view! {
        <h1>"Link performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <div on:click=move |e| {
            if prevent_navigation.get_value() {
                e.prevent_default();
            }
        }>
            <Show when=move || mounted.get()>
                <Link
                    href="/perf/atoms/link?followed"
                    classes="perf-link"
                    is_disabled=disabled
                    target=target
                    on_press=move |_| press_count.update_value(|count| *count += 1)
                >
                    "Follow me"
                </Link>
            </Show>
        </div>
        <button id="perf-after">"After"</button>
    }
}
