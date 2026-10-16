//! Hook sections (`use_grid_list_section` on plain elements) in a hook grid list, for the
//! performance test of `use_grid_list_section`
//! (`leptonic/tests/perf_tests/hooks/use_grid_list_section.rs`): the sections of the
//! `GridListSection` fixture ([`crate::pages::perf::atoms::grid_list_section`]: [`SMALL`]
//! sections "Section <s>" of [`ITEMS_PER_SECTION`] items "Item <s>-<i>"), each a row group with
//! a header row showing the heading the hook reads from the collection, its items hook items
//! ([`super::use_grid_list::HookGridListItem`]), with multiple selection, between two plain
//! buttons (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls
//! ([`Control`]) mount and unmount the list, set its size while unmounted (`small`, `large`:
//! [`LARGE`] sections), add and remove a section and rename the first section's header
//! (`rename`, `restore-name`).

use leptonic::{
    CapturedElement, IntoAttrs, Orientation,
    hooks::{
        collections::{
            CollectionOptions, Key, ListLayout, ListState, SelectionMode, SelectionOptions,
            UseListStateInput, use_list_state,
        },
        gridlist::{
            GridListData, KeyboardNavigationBehavior, UseGridListInput, UseGridListReturn,
            UseGridListSectionInput, UseGridListSectionReturn, use_grid_list,
            use_grid_list_section,
        },
    },
};
use leptos::prelude::*;

use super::use_grid_list::HookGridListItem;
use crate::pages::perf::{
    Control, PerfControls,
    atoms::grid_list_section::{ITEMS_PER_SECTION, LARGE, SMALL, sections_collection},
};

/// Section `s`: `use_grid_list_section` on a `div`, its header row and its items.
#[component]
fn HookSection(list: GridListData, s: usize) -> impl IntoView {
    let UseGridListSectionReturn {
        row_props,
        row_header_props,
        row_group_props,
        heading,
    } = use_grid_list_section(UseGridListSectionInput {
        list: list.clone(),
        key: Key::from(format!("section-{s}")),
    });
    view! {
        <div {..row_group_props.into_attrs()}>
            <div {..row_props.into_attrs()}>
                <div {..row_header_props.into_attrs()}>{move || heading.get()}</div>
            </div>
            {(0..ITEMS_PER_SECTION)
                .map(|i| {
                    view! {
                        <HookGridListItem list=list.clone() key=Key::from(format!("{s}-{i}"))>
                            {format!("Item {s}-{i}")}
                        </HookGridListItem>
                    }
                })
                .collect_view()}
        </div>
    }
}

/// The list: `use_grid_list` on a `div`, with its sections.
#[component]
fn HookList(state: ListState, sections: RwSignal<usize>) -> impl IntoView {
    let UseGridListReturn { props, data } = use_grid_list(UseGridListInput {
        state,
        element: CapturedElement::new(),
        id: None,
        aria_label: "Perf list".into(),
        aria_labelledby: Signal::stored(None),
        layout: ListLayout::Stack,
        orientation: Signal::stored(Orientation::Vertical),
        keyboard_delegate: None,
        options: CollectionOptions::default(),
        keyboard_navigation_behavior: KeyboardNavigationBehavior::Arrow,
        should_select_on_press_up: false,
        on_action: None,
        tree: None,
    });
    view! {
        <div {..props.into_attrs()}>
            <For each=move || 0..sections.get() key=|s| *s let:s>
                <HookSection list=data.clone() s=s />
            </For>
        </div>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let sections = RwSignal::new(SMALL);
    let renamed = RwSignal::new(false);
    let state = use_list_state(UseListStateInput {
        collection: sections_collection(sections, renamed),
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            ..SelectionOptions::default()
        },
    });
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
        <h1>"use_grid_list_section performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <HookList state sections />
        </Show>
        <button id="perf-after">"After"</button>
    }
}
