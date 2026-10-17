//! `use_list_keyboard_delegate`, the keyboard delegate of `use_selectable_collection` on a plain
//! `<div>` (`#perf-list`, `role="listbox"`), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_list_keyboard_delegate.rs`): a multiple selection
//! (`use_list_state`) of items "Option 0", "Option 1", ... (plain `<div>`s with
//! `use_selectable_item`, `data-perf-option` holding the number), [`SMALL`] at first, [`LARGE`]
//! after the control `large`, in a 200px high scroll view. Between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first.
//!
//! Its controls ([`Control`]) mount and unmount the list (creating the delegate), change the items,
//! and change the list's orientation.

use leptonic::{
    CapturedElement, Orientation,
    hooks::collections::{
        CollectionOptions, Key, LinkBehavior, ListLayout, ListState, SelectionMode,
        SelectionOptions, UseListCollectionInput, UseListKeyboardDelegateInput, UseListStateInput,
        UseSelectableCollectionInput, UseSelectableItemInput, UseSelectableItemReturn,
        use_list_collection, use_list_keyboard_delegate, use_list_state, use_selectable_collection,
        use_selectable_item,
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
    let state = use_list_state(UseListStateInput {
        collection: use_list_collection(UseListCollectionInput {
            items: items.into(),
            key: |i| Key::from(*i),
            text_value: |i| format!("Option {i}"),
        }),
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            ..SelectionOptions::default()
        },
    });
    let orientation = RwSignal::new(Orientation::Vertical);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("large", move || items.set((0..LARGE).collect())),
        Control::action("horizontal", move || {
            orientation.set(Orientation::Horizontal);
        }),
        Control::action("vertical", move || orientation.set(Orientation::Vertical)),
    ];
    view! {
        <h1>"use_list_keyboard_delegate performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <PlainList state items orientation />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// The list element with its items, made a collection by `use_selectable_collection` with the delegate of
/// `use_list_keyboard_delegate`.
#[component]
fn PlainList(
    state: ListState,
    items: RwSignal<Vec<u32>>,
    orientation: RwSignal<Orientation>,
) -> impl IntoView {
    let element = CapturedElement::new();
    let delegate = use_list_keyboard_delegate(UseListKeyboardDelegateInput {
        state,
        element,
        orientation: orientation.into(),
        layout: ListLayout::Stack,
        layout_delegate: None,
    });
    let collection_return = use_selectable_collection(UseSelectableCollectionInput {
        selection: state.selection,
        item_elements: state.item_elements,
        delegate,
        element,
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
            aria-orientation=move || match orientation.get() {
                Orientation::Horizontal => "horizontal",
                Orientation::Vertical => "vertical",
            }
            style="max-height: 200px; overflow: auto"
        >
            <For each=move || items.get() key=|i| *i let:i>
                <PlainItem state collection_id=collection_id.get_value() i />
            </For>
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
