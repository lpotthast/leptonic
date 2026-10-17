//! The `TagList` of a `TagGroup` (`.leptonic-TagGroup`, labelled "Tags"), for its performance test
//! (`leptonic/tests/perf_tests/atoms/tag_list.rs`): the tags "Tag 0", "Tag 1", ... (`Tag`s; a
//! tag's text is a span with `data-perf-tag` holding the number), [`SMALL`] at first, [`LARGE`]
//! after the control `large`, and its empty state ("No tags"). After a plain button
//! (`#perf-before`) to tab from, mounted at first.
//!
//! Its controls ([`Control`]) mount and unmount the list (the group stays mounted), change the tags
//! (`large`, `remove-all`, `restore`: [`SMALL`] again), and prepare the live announcer
//! (`prepare-announcer`: its first announcement waits for its live regions).

use leptonic::{
    Assertiveness, announce,
    atoms::tag_group::{Tag, TagGroup, TagList},
    hooks::collections::use_collection,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The tags at first.
const SMALL: u32 = 10;
/// The tags after the control `large`.
const LARGE: u32 = 1_000;

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let items = RwSignal::new((0..SMALL).collect::<Vec<u32>>());
    let collection = use_collection(move |b| {
        items.with(|items| {
            for &i in items {
                b.item(i, format!("Tag {i}"));
            }
        });
    });
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("large", move || items.set((0..LARGE).collect())),
        Control::action("remove-all", move || items.set(Vec::new())),
        Control::action("restore", move || items.set((0..SMALL).collect())),
        Control::action("prepare-announcer", || announce("", Assertiveness::Polite)),
    ];
    view! {
        <h1>"TagList performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <TagGroup collection=collection aria_label="Tags">
            <Show when=move || mounted.get()>
                <TagList empty_state=|| "No tags">
                    <For each=move || items.get() key=|i| *i let:i>
                        <Tag key=i>
                            <span data-perf-tag=i>{format!("Tag {i}")}</span>
                        </Tag>
                    </For>
                </TagList>
            </Show>
        </TagGroup>
    }
}
