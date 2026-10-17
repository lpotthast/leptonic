//! Hook sections (`use_grid_list_section` on plain elements) in a hook grid list, for the
//! performance test of `use_grid_list_section`
//! (`leptonic/tests/perf_tests/hooks/use_grid_list_section.rs`): [`SMALL`] sections "Section <s>"
//! of [`ITEMS_PER_SECTION`] items "Item <s>-<i>", each a row group with a header row showing the
//! heading the hook reads from the collection, its items hook items ([`HookGridListItem`]:
//! `use_grid_list_item` on a `div`), with multiple selection, between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls ([`Control`])
//! mount and unmount the list, set its size while unmounted (`small`, `large`: [`LARGE`] sections),
//! add and remove a section and rename the first section's header (`rename`, `restore-name`).

use std::sync::Arc;

use leptonic::{
    CapturedElement, IntoAttrs, Orientation, flag,
    hooks::{
        collections::{
            Collection, CollectionMemo, CollectionOptions, Key, ListLayout, ListState,
            SelectionMode, SelectionOptions, UseListStateInput, use_list_state,
        },
        gridlist::{
            FocusMode, GridListData, KeyboardNavigationBehavior, UseGridListInput,
            UseGridListItemInput, UseGridListItemReturn, UseGridListReturn,
            UseGridListSectionInput, UseGridListSectionReturn, use_grid_list, use_grid_list_item,
            use_grid_list_section,
        },
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The sections of the small list (10 items).
const SMALL: usize = 2;
/// The sections of the large list (1,000 items).
const LARGE: usize = 200;
/// The items of each section.
const ITEMS_PER_SECTION: usize = 5;

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
fn sections_collection(sections: RwSignal<usize>, renamed: RwSignal<bool>) -> CollectionMemo {
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

/// An item: `use_grid_list_item` on a `div` (its cell a `div` inside), its pressed state in
/// `data-pressed`.
#[component]
fn HookGridListItem(list: GridListData, key: Key, children: Children) -> impl IntoView {
    let UseGridListItemReturn {
        row: row_props,
        cell: grid_cell_props,
        is_pressed,
        ..
    } = use_grid_list_item(UseGridListItemInput {
        list,
        key,
        focus_mode: FocusMode::Row,
        allows_arrow_navigation: false,
        on_context_menu: None,
    });
    let (attrs, styles) = row_props.attrs();
    view! {
        <div {..attrs} style=styles data-pressed=flag(is_pressed)>
            <div {..grid_cell_props.attrs()}>{children()}</div>
        </div>
    }
}

/// Section `s`: `use_grid_list_section` on a `div`, its header row and its items.
#[component]
fn HookSection(list: GridListData, s: usize) -> impl IntoView {
    let UseGridListSectionReturn {
        header,
        row_group: row_group_props,
        heading,
    } = use_grid_list_section(UseGridListSectionInput {
        list: list.clone(),
        key: Key::from(format!("section-{s}")),
    });
    let (row_props, row_header_props) = header.props().expect("the first header takes its props");
    view! {
        <div {..row_group_props.attrs()}>
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
    let UseGridListReturn {
        grid_list: props,
        data,
    } = use_grid_list(UseGridListInput {
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
        <div {..props.attrs()}>
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
