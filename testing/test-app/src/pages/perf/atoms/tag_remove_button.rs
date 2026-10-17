//! `TagRemoveButton`s in the tags of a `TagGroup` (`.leptonic-TagGroup`, labelled "Tags"), for
//! the performance test of `TagRemoveButton` (`leptonic/tests/perf_tests/atoms/tag_remove_button.rs`):
//! a `TagList` of the tags "Tag 0", "Tag 1", ... (a tag's text is a span with `data-perf-tag`
//! holding the number, followed by its remove button), [`SMALL`] at first, [`LARGE`] after the
//! control `large`; removing a tag removes it from the data. Before a plain button
//! (`#perf-after`) to hover, mounted at first.
//!
//! Its controls ([`Control`]) mount and unmount the remove buttons (the tags stay mounted), change
//! the number of tags, and prepare the live announcer (`prepare-announcer`: its first announcement
//! waits for its live regions).

use std::collections::HashSet;

use leptonic::{
    Assertiveness, announce,
    atoms::tag_group::{Tag, TagGroup, TagList, TagRemoveButton},
    hooks::collections::{Key, use_collection},
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
    let on_remove = move |keys: HashSet<Key>| {
        items.update(|items| items.retain(|i| !keys.contains(&Key::from(*i))));
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("large", move || items.set((0..LARGE).collect())),
        Control::action("prepare-announcer", || announce("", Assertiveness::Polite)),
    ];
    view! {
        <h1>"TagRemoveButton performance"</h1>
        <PerfControls controls />
        <TagGroup collection=collection on_remove=on_remove aria_label="Tags">
            <TagList empty_state=|| "No tags">
                <For each=move || items.get() key=|i| *i let:i>
                    <Tag key=i>
                        <span data-perf-tag=i>{format!("Tag {i}")}</span>
                        <Show when=move || mounted.get()>
                            <TagRemoveButton>"×"</TagRemoveButton>
                        </Show>
                    </Tag>
                </For>
            </TagList>
        </TagGroup>
        <button id="perf-after">"After"</button>
    }
}
