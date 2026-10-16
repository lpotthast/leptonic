//! `GridListSection`s with `GridListHeader`s in a `GridList` (`[role=grid]`), for the
//! performance tests of `GridListSection` and `GridListHeader`
//! (`leptonic/tests/perf_tests/atoms/grid_list_section.rs`, `.../grid_list_header.rs`, whose
//! fixture this is too): [`SMALL`] sections ("Section <s>", key `section-<s>`; its header's text
//! comes from the collection) of [`ITEMS_PER_SECTION`] items ("Item <s>-<i>", key `<s>-<i>`),
//! with multiple selection, between two plain buttons (`#perf-before`, `#perf-after`) to tab
//! from and to, mounted at first. Its controls ([`Control`]) mount and unmount the list, set its
//! size while unmounted (`small`, `large`: [`LARGE`] sections), add and remove a section and
//! rename the first section's header (`rename`, `restore-name`).

use std::sync::Arc;

use leptonic::{
    atoms::grid_list::{GridList, GridListHeader, GridListItem, GridListSection},
    hooks::collections::{Collection, CollectionMemo, SelectionMode},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The sections of the small list (10 items).
pub const SMALL: usize = 2;
/// The sections of the large list (1,000 items).
pub const LARGE: usize = 200;
/// The items of each section.
pub const ITEMS_PER_SECTION: usize = 5;

/// The header text of section `s`.
fn heading(s: usize, renamed: bool) -> String {
    if renamed && s == 0 {
        format!("Renamed {s}")
    } else {
        format!("Section {s}")
    }
}

/// `sections` sections of [`ITEMS_PER_SECTION`] items; the first one's header is renamed while
/// `renamed`.
pub fn sections_collection(sections: RwSignal<usize>, renamed: RwSignal<bool>) -> CollectionMemo {
    Memo::new(move |_| {
        let renamed = renamed.get();
        Arc::new(Collection::build(|b| {
            for s in 0..sections.get() {
                b.section(format!("section-{s}"), |section| {
                    section.header(format!("header-{s}"), heading(s, renamed));
                    for i in 0..ITEMS_PER_SECTION {
                        section.item(format!("{s}-{i}"), format!("Item {s}-{i}"));
                    }
                });
            }
        }))
    })
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let sections = RwSignal::new(SMALL);
    let renamed = RwSignal::new(false);
    let collection = sections_collection(sections, renamed);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || sections.set(SMALL)),
        Control::action("large", move || sections.set(LARGE)),
        Control::action("add-section", move || sections.update(|count| *count += 1)),
        Control::action("remove-section", move || {
            sections.update(|count| *count -= 1);
        }),
        Control::action("rename", move || renamed.set(true)),
        Control::action("restore-name", move || renamed.set(false)),
    ];
    view! {
        <h1>"GridListSection performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <GridList
                collection=collection
                selection_mode=SelectionMode::Multiple
                aria_label="Perf list"
            >
                <For each=move || 0..sections.get() key=|s| *s let:s>
                    <GridListSection key=format!("section-{s}")>
                        <GridListHeader />
                        {(0..ITEMS_PER_SECTION)
                            .map(|i| {
                                view! {
                                    <GridListItem key=format!("{s}-{i}")>
                                        {format!("Item {s}-{i}")}
                                    </GridListItem>
                                }
                            })
                            .collect_view()}
                    </GridListSection>
                </For>
            </GridList>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
