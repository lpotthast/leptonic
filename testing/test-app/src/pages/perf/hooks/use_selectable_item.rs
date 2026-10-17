//! `use_selectable_item` on plain `<div>`s (`data-perf-option` holding the number) in a plain
//! list (`#perf-list`, `role="listbox"`, `use_selectable_list`), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_selectable_item.rs`): a multiple selection
//! (`use_list_state`) of the items "Option 0", "Option 1", ..., [`SMALL`] at first, [`LARGE`] after
//! the control `large`, in a 200px high scroll view. Between two plain buttons (`#perf-before`,
//! `#perf-after`) to tab from and to, mounted at first.
//!
//! Its controls ([`Control`]) mount and unmount the items (the list stays mounted), change them,
//! call the state's methods (select all, clear) and disable the first item and enable it again.

use std::collections::HashSet;

use leptonic::{
    CapturedElement, Orientation,
    hooks::collections::{
        CollectionOptions, Key, LinkBehavior, ListLayout, ListState, SelectionMode,
        SelectionOptions, UseListCollectionInput, UseListStateInput, UseSelectableItemInput,
        UseSelectableItemReturn, UseSelectableListInput, use_list_collection, use_list_state,
        use_selectable_item, use_selectable_list,
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The items at first.
const SMALL: u32 = 10;
/// The items after the control `large`.
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
            text_value: |i| format!("Option {i}"),
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
    ];
    view! {
        <h1>"use_selectable_item performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <PlainList state items items_mounted=mounted />
        <button id="perf-after">"After"</button>
    }
}

/// The list element (`use_selectable_list`) with its items, while `items_mounted`.
#[component]
fn PlainList(
    state: ListState,
    items: RwSignal<Vec<u32>>,
    items_mounted: RwSignal<bool>,
) -> impl IntoView {
    let collection_return = use_selectable_list(UseSelectableListInput {
        state,
        element: CapturedElement::new(),
        orientation: Signal::stored(Orientation::Vertical),
        layout: ListLayout::Stack,
        keyboard_delegate: None,
        layout_delegate: None,
        options: CollectionOptions::default(),
    });
    let props = collection_return.collection;
    let collection_id = StoredValue::new(collection_return.collection_id.clone());
    view! {
        <div
            {..props.attrs()}
            id="perf-list"
            role="listbox"
            aria-label="Options"
            aria-orientation="vertical"
            style="max-height: 200px; overflow: auto"
        >
            <Show when=move || items_mounted.get()>
                <For each=move || items.get() key=|i| *i let:i>
                    <PlainItem state collection_id=collection_id.get_value() i />
                </For>
            </Show>
        </div>
    }
}

/// An item (`use_selectable_item`).
#[component]
fn PlainItem(state: ListState, collection_id: leptonic::Id, i: u32) -> impl IntoView {
    let UseSelectableItemReturn {
        item: props,
        is_selected,
        is_focused,
        is_disabled,
        is_pressed,
        ..
    } = use_selectable_item(UseSelectableItemInput {
        selection: state.selection,
        item_elements: state.item_elements,
        key: Key::from(i),
        element: CapturedElement::new(),
        id: None,
        collection_id,
        is_disabled: Signal::stored(false),
        should_select_on_press_up: false,
        allows_different_press_origin: false,
        on_action: Signal::stored(None),
        link_behavior: Signal::stored(LinkBehavior::default()),
        focus: None,
        should_use_virtual_focus: false,
        on_context_menu: None,
    });
    let (attrs, styles) = props.attrs();
    view! {
        <div
            {..attrs}
            style=styles
            role="option"
            data-perf-option=i
            aria-selected=move || is_selected.get().to_string()
            aria-disabled=move || is_disabled.get().then_some("true")
            data-focused=move || is_focused.get().then_some("true")
            data-pressed=move || is_pressed.get().then_some("true")
        >
            {format!("Option {i}")}
        </div>
    }
}
