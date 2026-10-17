//! `use_tag` on plain `<div>`s (`data-perf-tag` holding the number, each with a remove `<button>`
//! using `use_button`) in a `use_tag_group` group (`data-perf-group`, labelled "Tags"), for its
//! performance test (`leptonic/tests/perf_tests/hooks/use_tag.rs`): a multiple selection
//! (`use_list_state`) of the removable tags "Tag 0", "Tag 1", ..., [`SMALL`] at first, [`LARGE`]
//! after the control `large`; removing a tag removes it from the data. Between two plain buttons
//! (`#perf-before`, `#perf-after`), mounted at first.
//!
//! Its controls ([`Control`]) mount and unmount the tags (the group stays mounted), change them,
//! call the state's methods (select all, clear), disable the first tag and enable it again, and
//! prepare the live announcer (`prepare-announcer`: its first announcement waits for its live
//! regions).

use std::collections::HashSet;

use leptonic::{
    Assertiveness, CapturedElement, announce,
    hooks::{
        button::use_button,
        collections::{
            Key, ListState, SelectionMode, SelectionOptions, UseListCollectionInput,
            UseListStateInput, use_list_collection, use_list_state,
        },
        tag::{
            TagGroupData, UseTagGroupInput, UseTagGroupReturn, UseTagInput, UseTagReturn, use_tag,
            use_tag_group,
        },
    },
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
    let disabled_keys = RwSignal::new(HashSet::<Key>::new());
    let state = use_list_state(UseListStateInput {
        collection: use_list_collection(UseListCollectionInput {
            items: items.into(),
            key: |i| Key::from(*i),
            text_value: |i| format!("Tag {i}"),
        }),
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            disabled_keys: disabled_keys.into(),
            ..SelectionOptions::default()
        },
    });
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("large", move || items.set((0..LARGE).collect())),
        Control::action("select-all", move || state.selection.select_all()),
        Control::action("clear-selection", move || state.selection.clear_selection()),
        Control::action("disable-first", move || {
            disabled_keys.set(HashSet::from([Key::from(0_u32)]));
        }),
        Control::action("enable-all", move || disabled_keys.set(HashSet::new())),
        Control::action("prepare-announcer", || announce("", Assertiveness::Polite)),
    ];
    view! {
        <h1>"use_tag performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <PlainTagGroup state items tags_mounted=mounted />
        <button id="perf-after">"After"</button>
    }
}

/// The group element (`use_tag_group`) with its tags, while `tags_mounted`.
#[component]
fn PlainTagGroup(
    state: ListState,
    items: RwSignal<Vec<u32>>,
    tags_mounted: RwSignal<bool>,
) -> impl IntoView {
    let UseTagGroupReturn {
        grid: grid_props,
        data,
        ..
    } = use_tag_group(UseTagGroupInput {
        state,
        element: CapturedElement::new(),
        id: None,
        has_label: Signal::stored(false),
        aria_label: "Tags".into(),
        aria_labelledby: Signal::stored(None),
        aria_describedby: Signal::stored(None),
        keyboard_delegate: None,
        on_remove: Some(Callback::new(move |keys: HashSet<Key>| {
            items.update(|items| items.retain(|i| !keys.contains(&Key::from(*i))));
        })),
        on_action: None,
    });
    let data = StoredValue::new(data);
    view! {
        <div {..grid_props.attrs()} data-perf-group>
            <Show when=move || tags_mounted.get()>
                <For each=move || items.get() key=|i| *i let:i>
                    <PlainTag group=data.get_value() i />
                </For>
            </Show>
        </div>
    }
}

/// A tag (`use_tag`) with its remove button.
#[component]
fn PlainTag(group: TagGroupData, i: u32) -> impl IntoView {
    let UseTagReturn {
        row: row_props,
        cell: grid_cell_props,
        remove_button,
        is_selected,
        is_focused,
        is_disabled,
        is_pressed,
        ..
    } = use_tag(UseTagInput {
        group,
        key: Key::from(i),
    });
    let (attrs, styles) = row_props.attrs();
    let remove_button = remove_button.map(|input| {
        let (attrs, styles) = use_button(input).button.attrs();
        view! {
            <button {..attrs} style=styles data-perf-remove>
                "×"
            </button>
        }
    });
    view! {
        <div
            {..attrs}
            style=styles
            data-perf-tag=i
            data-selected=move || is_selected.get().then_some("true")
            data-focused=move || is_focused.get().then_some("true")
            data-disabled=move || is_disabled.get().then_some("true")
            data-pressed=move || is_pressed.get().then_some("true")
        >
            <div {..grid_cell_props.attrs()} style="display: contents">
                {format!("Tag {i}")}
                {remove_button}
            </div>
        </div>
    }
}
