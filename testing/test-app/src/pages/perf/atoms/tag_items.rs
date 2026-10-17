//! The `TagItems` of the `TagList` of a `TagGroup` (`.leptonic-TagGroup`, labelled "Tags"), for its
//! performance test (`leptonic/tests/perf_tests/atoms/tag_items.rs`): a tag for each item "Tag 0",
//! "Tag 1", ... of the collection (its text, from the collection, in a span with `data-perf-tag`
//! holding the number), [`SMALL`] at first, [`LARGE`] after the control `large`. Mounted at first.
//!
//! Its controls ([`Control`]) mount and unmount the `TagItems` (the group and its list stay
//! mounted), change the tags (`large`, `append`, `remove-last`) and the first one's text
//! (`rename-first`: "Renamed 0", `restore-names`), and prepare the live announcer
//! (`prepare-announcer`: its first announcement waits for its live regions).

use leptonic::{
    Assertiveness, announce,
    atoms::tag_group::{TagGroup, TagItems, TagList},
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
    // The tag shown as "Renamed <i>".
    let renamed = RwSignal::new(None::<u32>);
    let collection = use_collection(move |b| {
        let renamed = renamed.get();
        items.with(|items| {
            for &i in items {
                let text = if renamed == Some(i) {
                    format!("Renamed {i}")
                } else {
                    format!("Tag {i}")
                };
                b.item(i, text);
            }
        });
    });
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("large", move || items.set((0..LARGE).collect())),
        Control::action("append", move || {
            items.update(|items| {
                let next = items.iter().max().map_or(0, |last| last + 1);
                items.push(next);
            });
        }),
        Control::action("remove-last", move || {
            items.update(|items| {
                items.pop();
            });
        }),
        Control::action("rename-first", move || renamed.set(Some(0))),
        Control::action("restore-names", move || renamed.set(None)),
        Control::action("prepare-announcer", || announce("", Assertiveness::Polite)),
    ];
    view! {
        <h1>"TagItems performance"</h1>
        <PerfControls controls />
        <TagGroup collection=collection aria_label="Tags">
            <TagList empty_state=|| "No tags">
                <Show when=move || mounted.get()>
                    <TagItems let:node>
                        <span data-perf-tag=node.key.to_string()>
                            {node.text_value.to_string()}
                        </span>
                    </TagItems>
                </Show>
            </TagList>
        </TagGroup>
    }
}
