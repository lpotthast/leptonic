//! `Tag`s in the `TagList` of a `TagGroup` (`.leptonic-TagGroup`, labelled "Tags"), for the
//! performance test of `Tag` (`leptonic/tests/perf_tests/atoms/tag.rs`): the tags "Tag 0", "Tag 1",
//! ... (a tag's text is a span with `data-perf-tag` holding the number), [`SMALL`] at first,
//! [`LARGE`] after the control `large`. A multiple selection (controlled); removing a tag (Delete,
//! Backspace) removes it from the data. Between two plain buttons (`#perf-before`, `#perf-after`)
//! to tab from and to, mounted at first.
//!
//! Its controls ([`Control`]) mount and unmount the tags (the group and its list stay mounted),
//! change their number, set the selection, disable the first tag and enable it again, and prepare
//! the live announcer (`prepare-announcer`: its first announcement waits for its live regions).

use std::collections::HashSet;

use leptonic::{
    Assertiveness, announce,
    atoms::tag_group::{Tag, TagGroup, TagList},
    hooks::collections::{Key, Selection, SelectionMode, use_collection},
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
    let selection = RwSignal::new(Selection::default());
    let disabled_keys = RwSignal::new(HashSet::<Key>::new());
    let on_remove = move |keys: HashSet<Key>| {
        items.update(|items| items.retain(|i| !keys.contains(&Key::from(*i))));
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("large", move || items.set((0..LARGE).collect())),
        Control::action("select-first", move || {
            selection.set(Selection::keys([Key::from(0_u32)]));
        }),
        Control::action("select-all", move || selection.set(Selection::All)),
        Control::action("clear-selection", move || {
            selection.set(Selection::default());
        }),
        Control::action("disable-first", move || {
            disabled_keys.set(HashSet::from([Key::from(0_u32)]));
        }),
        Control::action("enable-all", move || disabled_keys.set(HashSet::new())),
        Control::action("prepare-announcer", || announce("", Assertiveness::Polite)),
    ];
    view! {
        <h1>"Tag performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <TagGroup
            collection=collection
            selection=selection
            set_selection=selection
            selection_mode=SelectionMode::Multiple
            disabled_keys=disabled_keys
            on_remove=on_remove
            aria_label="Tags"
        >
            <TagList empty_state=|| "No tags">
                <Show when=move || mounted.get()>
                    <For each=move || items.get() key=|i| *i let:i>
                        <Tag key=i>
                            <span data-perf-tag=i>{format!("Tag {i}")}</span>
                        </Tag>
                    </For>
                </Show>
            </TagList>
        </TagGroup>
        <button id="perf-after">"After"</button>
    }
}
