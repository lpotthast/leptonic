//! A `Breadcrumb` alone in its `Breadcrumbs` (in `#perf-nav`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/breadcrumb.rs`): an item with a `Link` to this page with a
//! query (`?crumb=1`: pressing it navigates without leaving the page; the router marks such links
//! `aria-current="page"`, as their path is the current one), between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls ([`Control`])
//! mount and unmount it, make it the current item and not, disable and enable the breadcrumbs
//! around it, and read how often it was pressed (`actions`, the breadcrumbs' `on_action` with its
//! key).

use leptonic::{
    atoms::{
        breadcrumbs::{Breadcrumb, Breadcrumbs},
        link::Link,
    },
    hooks::collections::Key,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let current = RwSignal::new(false);
    let disabled = RwSignal::new(false);
    let action_count = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("make-current", move || current.set(true)),
        Control::action("make-not-current", move || current.set(false)),
        Control::action("disable-breadcrumbs", move || disabled.set(true)),
        Control::action("enable-breadcrumbs", move || disabled.set(false)),
        Control::read("actions", move || f64::from(action_count.get_value())),
    ];
    let on_action = move |key: Key| {
        if key == Key::from("item") {
            action_count.update_value(|count| *count += 1);
        }
    };
    view! {
        <h1>"Breadcrumb performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <nav id="perf-nav">
            <Breadcrumbs is_disabled=disabled on_action=on_action>
                <Show when=move || mounted.get()>
                    <Breadcrumb key="item" is_current=current>
                        <Link href="/perf/atoms/breadcrumb?crumb=1">"Item"</Link>
                    </Breadcrumb>
                </Show>
            </Breadcrumbs>
        </nav>
        <button id="perf-after">"After"</button>
    }
}
