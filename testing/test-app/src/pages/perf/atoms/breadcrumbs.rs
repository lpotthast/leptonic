//! The `Breadcrumbs` alone (in `#perf-nav`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/breadcrumbs.rs`): a trail of [`FEW`] items (or [`MANY`],
//! control `use-many` while unmounted), each a `Breadcrumb` with a `Link` to this page with a query
//! (`?crumb=<n>`: pressing one navigates without leaving the page; the router marks such links
//! `aria-current="page"`, as their path is the current one), the last one current, between two
//! plain buttons (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls
//! ([`Control`]) mount and unmount it, change its reactive props (disabled, label, the current
//! item), and read how often an item was pressed (`actions`, its `on_action`).

use leptonic::{
    atoms::{
        breadcrumbs::{Breadcrumb, Breadcrumbs},
        link::Link,
    },
    hooks::collections::Key,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The items of the small trail.
const FEW: usize = 3;
/// The items of the large trail.
const MANY: usize = 100;

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let count = RwSignal::new(FEW);
    let disabled = RwSignal::new(false);
    let label = RwSignal::new(None::<String>);
    // The current item; `None`: the last.
    let current = RwSignal::new(None::<usize>);
    let action_count = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("use-many", move || count.set(MANY)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("set-label", move || label.set(Some("Trail".to_owned()))),
        Control::action("unset-label", move || label.set(None)),
        Control::action("make-first-current", move || current.set(Some(1))),
        Control::action("make-last-current", move || current.set(None)),
        Control::read("actions", move || f64::from(action_count.get_value())),
    ];
    let is_current = move |n: usize| {
        Signal::derive(move || current.get().unwrap_or_else(|| count.get_untracked()) == n)
    };
    view! {
        <h1>"Breadcrumbs performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <nav id="perf-nav">
            <Show when=move || mounted.get()>
                <Breadcrumbs
                    is_disabled=disabled
                    aria_label=label
                    on_action=move |_: Key| action_count.update_value(|count| *count += 1)
                >
                    {(1..=count.get_untracked())
                        .map(|n| {
                            view! {
                                <Breadcrumb is_current=is_current(n)>
                                    <Link href=format!(
                                        "/perf/atoms/breadcrumbs?crumb={n}",
                                    )>{format!("Item {n}")}</Link>
                                </Breadcrumb>
                            }
                        })
                        .collect_view()}
                </Breadcrumbs>
            </Show>
        </nav>
        <button id="perf-after">"After"</button>
    }
}
