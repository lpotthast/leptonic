//! The `TagGroup` alone (`.leptonic-TagGroup`, labelled "Tags"), for its performance test
//! (`leptonic/tests/perf_tests/atoms/tag_group.rs`): a `TagList` of removable tags "Tag 0", "Tag
//! 1", ... (`Tag`s, or `TagItems` for its own test; a tag's text is a span with `data-perf-tag`
//! holding the number, followed by its `TagRemoveButton`), [`SMALL`] at first, [`LARGE`] after the
//! control `large`. A multiple selection (controlled); removing a tag removes it from the data.
//! Between two plain buttons (`#perf-before`, `#perf-after`) to tab from and to, mounted at first.
//!
//! [`TagGroupFixture`] renders it for the performance tests of its parts too: the controls `mount`
//! and `unmount` mount the part (the rest stays mounted). The other controls ([`Control`]) change
//! the tags and the group's reactive props, set the selection, and prepare the live announcer
//! (`prepare-announcer`: its first announcement waits for its live regions).

use std::collections::HashSet;

use leptonic::{
    Assertiveness, announce,
    atoms::tag_group::{Tag, TagGroup, TagItems, TagList, TagRemoveButton},
    hooks::collections::{Key, Selection, SelectionMode, use_collection},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The tags at first.
pub const SMALL: u32 = 10;
/// The tags after the control `large`.
pub const LARGE: u32 = 1_000;

/// The part of the tag group that the controls `mount` and `unmount` mount.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Part {
    /// The whole tag group.
    TagGroup,
    TagList,
    /// Every `Tag`.
    Tags,
    /// The `TagItems` (rendering the tags instead of the fixture's `Tag`s).
    TagItems,
    /// Every `TagRemoveButton`.
    RemoveButtons,
}

#[component]
pub fn PagePerf() -> impl IntoView {
    view! { <TagGroupFixture title="TagGroup performance" part=Part::TagGroup /> }
}

/// The tag group (see the module documentation), `part` mounted by the controls.
#[component]
pub fn TagGroupFixture(title: &'static str, part: Part) -> impl IntoView {
    let mounted = RwSignal::new(true);
    let is_mounted = move |of: Part| move || of != part || mounted.get();
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
    let selection = RwSignal::new(Selection::default());
    let selection_mode = RwSignal::new(SelectionMode::Multiple);
    let disabled_keys = RwSignal::new(HashSet::<Key>::new());
    let on_remove = move |keys: HashSet<Key>| {
        items.update(|items| items.retain(|i| !keys.contains(&Key::from(*i))));
    };
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
        Control::action("remove-all", move || items.set(Vec::new())),
        Control::action("restore", move || items.set((0..SMALL).collect())),
        Control::action("rename-first", move || renamed.set(Some(0))),
        Control::action("restore-names", move || renamed.set(None)),
        Control::action("select-first", move || {
            selection.set(Selection::keys([Key::from(0_u32)]));
        }),
        Control::action("select-all", move || selection.set(Selection::All)),
        Control::action("clear-selection", move || {
            selection.set(Selection::default());
        }),
        Control::action("single", move || selection_mode.set(SelectionMode::Single)),
        Control::action("multiple", move || {
            selection_mode.set(SelectionMode::Multiple);
        }),
        Control::action("disable-first", move || {
            disabled_keys.set(HashSet::from([Key::from(0_u32)]));
        }),
        Control::action("enable-all", move || disabled_keys.set(HashSet::new())),
        Control::action("prepare-announcer", || announce("", Assertiveness::Polite)),
    ];
    let content = move |i: u32, text: String| {
        view! {
            <span data-perf-tag=i>{text}</span>
            <Show when=is_mounted(Part::RemoveButtons)>
                <TagRemoveButton>"×"</TagRemoveButton>
            </Show>
        }
    };
    view! {
        <h1>{title}</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=is_mounted(Part::TagGroup)>
            <TagGroup
                collection=collection
                selection=selection
                set_selection=selection
                selection_mode=selection_mode
                disabled_keys=disabled_keys
                on_remove=on_remove
                aria_label="Tags"
            >
                <Show when=is_mounted(Part::TagList)>
                    <TagList empty_state=|| "No tags">
                        {(part != Part::TagItems)
                            .then(|| {
                                view! {
                                    <Show when=is_mounted(Part::Tags)>
                                        <For each=move || items.get() key=|i| *i let:i>
                                            <Tag key=i>{content(i, format!("Tag {i}"))}</Tag>
                                        </For>
                                    </Show>
                                }
                            })}
                        {(part == Part::TagItems)
                            .then(|| {
                                view! {
                                    <Show when=is_mounted(Part::TagItems)>
                                        <TagItems let:node>
                                            {
                                                let i = node
                                                    .key
                                                    .as_i64()
                                                    .and_then(|i| u32::try_from(i).ok())
                                                    .unwrap_or_default();
                                                content(i, node.text_value.to_string())
                                            }
                                        </TagItems>
                                    </Show>
                                }
                            })}
                    </TagList>
                </Show>
            </TagGroup>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
