//! The `DisclosureGroup` alone (`#perf-group`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/disclosure_group.rs`): an accordion of [`SMALL`] or [`LARGE`]
//! disclosures (`#perf-disclosure-<i>`, key `i`, triggers `#perf-trigger-<i>`), all collapsed,
//! between two plain buttons (`#perf-before`, `#perf-after`), mounted at first with [`SMALL`]. Its
//! expanded keys are the app's (`expanded_keys` + `set_expanded_keys`). Its controls ([`Control`])
//! mount it with either size and unmount it, switch between single and multiple expansion,
//! expand the first disclosure and collapse all (the app's state), and disable and enable it.

use std::collections::HashSet;

use leptonic::{
    atoms::{
        button::Button,
        disclosure::{Disclosure, DisclosureGroup, DisclosurePanel, DisclosureTrigger},
    },
    hooks::{collections::Key, disclosure::DisclosureGroupExpansion},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The disclosures of the small group.
pub const SMALL: i32 = 10;
/// The disclosures of the large group.
pub const LARGE: i32 = 1_000;

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let size = RwSignal::new(SMALL);
    let expansion = RwSignal::new(DisclosureGroupExpansion::Single);
    let disabled = RwSignal::new(false);
    let expanded_keys = RwSignal::new(HashSet::<Key>::new());
    let controls = vec![
        Control::action("mount-small", move || {
            size.set(SMALL);
            mounted.set(true);
        }),
        Control::action("mount-large", move || {
            size.set(LARGE);
            mounted.set(true);
        }),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("multiple", move || {
            expansion.set(DisclosureGroupExpansion::Multiple);
        }),
        Control::action("single", move || {
            expansion.set(DisclosureGroupExpansion::Single);
        }),
        Control::action("expand-first", move || {
            expanded_keys.set(HashSet::from([Key::from(0)]));
        }),
        Control::action("collapse-all", move || expanded_keys.set(HashSet::new())),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
    ];
    view! {
        <h1>"DisclosureGroup performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <DisclosureGroup
                attr:id="perf-group"
                expansion=expansion
                is_disabled=disabled
                expanded_keys=expanded_keys
                set_expanded_keys=expanded_keys
            >
                {(0..size.get_untracked())
                    .map(|i| {
                        view! {
                            <Disclosure attr:id=format!("perf-disclosure-{i}") key=Key::from(i)>
                                <DisclosureTrigger>
                                    <Button id=format!("perf-trigger-{i}")>
                                        {format!("Section {i}")}
                                    </Button>
                                </DisclosureTrigger>
                                <DisclosurePanel>{format!("Content {i}")}</DisclosurePanel>
                            </Disclosure>
                        }
                    })
                    .collect_view()}
            </DisclosureGroup>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
