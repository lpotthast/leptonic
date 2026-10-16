//! `use_selectable_list` on a plain `<div>` (`#perf-list`, `role="listbox"`), for its performance
//! test (`leptonic/tests/perf_tests/hooks/use_selectable_list.rs`): a multiple selection
//! (`use_list_state`) of items "Option 0", "Option 1", ... (plain `<div>`s with
//! `use_selectable_item`, `data-perf-option` holding the number), [`SMALL`] at first, [`LARGE`]
//! after the control `large`, in a 200px high scroll view. Between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first.
//!
//! [`SelectableListFixture`] renders it for the performance tests of the hooks underneath too:
//! with `use_selectable_collection` and a list keyboard delegate built by the fixture
//! ([`Collection::SelectableCollection`]) or by `use_list_keyboard_delegate`
//! ([`Collection::ListKeyboardDelegate`]); and the controls `mount` and `unmount` mount the list
//! or only its items (`use_selectable_item`). The other controls ([`Control`]) change the items,
//! the state (its methods: select all, clear) and the list's reactive input (orientation, disabled
//! items).

use std::{collections::HashSet, sync::Arc};

use leptonic::{
    CapturedElement, IntoAttrs, Orientation,
    hooks::collections::{
        CollectionOptions, DomLayoutDelegate, Key, KeyboardDelegate, LinkBehavior,
        ListKeyboardDelegate, ListLayout, ListState, SelectionMode, SelectionOptions,
        UseListCollectionInput, UseListKeyboardDelegateInput, UseListStateInput,
        UseSelectableCollectionInput, UseSelectableCollectionProps, UseSelectableItemInput,
        UseSelectableItemReturn, UseSelectableListInput, keyboard_delegate_memo,
        use_list_collection, use_list_keyboard_delegate, use_list_state, use_selectable_collection,
        use_selectable_item, use_selectable_list,
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The items at first.
pub const SMALL: u32 = 10;
/// The items after the control `large`.
pub const LARGE: u32 = 1_000;

/// Which hooks make the list a collection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Collection {
    /// `use_selectable_list`.
    SelectableList,
    /// `use_selectable_collection` with a `ListKeyboardDelegate` the fixture builds (without
    /// type-ahead: it has no collator).
    SelectableCollection,
    /// `use_selectable_collection` with the delegate of `use_list_keyboard_delegate`.
    ListKeyboardDelegate,
}

/// What the controls `mount` and `unmount` mount.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Part {
    /// The list with its items.
    List,
    /// The items (in the mounted list).
    Items,
}

#[component]
pub fn PagePerf() -> impl IntoView {
    view! {
        <SelectableListFixture
            title="use_selectable_list performance"
            collection=Collection::SelectableList
            part=Part::List
        />
    }
}

/// The list (see the module documentation), made a collection by `collection`'s hooks, `part`
/// mounted by the controls.
#[component]
pub fn SelectableListFixture(
    title: &'static str,
    collection: Collection,
    part: Part,
) -> impl IntoView {
    let mounted = RwSignal::new(true);
    let is_mounted = move |of: Part| Signal::derive(move || of != part || mounted.get());
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
    let orientation = RwSignal::new(Orientation::Vertical);
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
        Control::action("horizontal", move || {
            orientation.set(Orientation::Horizontal);
        }),
        Control::action("vertical", move || orientation.set(Orientation::Vertical)),
    ];
    let items_mounted = is_mounted(Part::Items);
    view! {
        <h1>{title}</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || is_mounted(Part::List).get()>
            <PlainList collection state items orientation items_mounted />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// The props of the list element, from `collection`'s hooks.
fn collection_props(
    collection: Collection,
    state: &ListState,
    element: CapturedElement,
    orientation: RwSignal<Orientation>,
) -> UseSelectableCollectionProps {
    let state = *state;
    let delegate = |delegate: Signal<Arc<dyn KeyboardDelegate>>| {
        use_selectable_collection(UseSelectableCollectionInput {
            selection: state.selection,
            item_elements: state.item_elements,
            delegate,
            element,
            options: CollectionOptions::default(),
        })
        .props
    };
    match collection {
        Collection::SelectableList => {
            use_selectable_list(UseSelectableListInput {
                state,
                element,
                orientation: orientation.into(),
                layout: ListLayout::Stack,
                keyboard_delegate: None,
                layout_delegate: None,
                options: CollectionOptions::default(),
            })
            .props
        }
        Collection::SelectableCollection => {
            let layout = Arc::new(DomLayoutDelegate::new(element, state.item_elements));
            delegate(keyboard_delegate_memo(move || {
                Arc::new(
                    ListKeyboardDelegate::new(state.collection, state.selection, layout.clone())
                        .with_orientation(orientation.get()),
                ) as Arc<dyn KeyboardDelegate>
            }))
        }
        Collection::ListKeyboardDelegate => {
            delegate(use_list_keyboard_delegate(UseListKeyboardDelegateInput {
                state,
                element,
                orientation: orientation.into(),
                layout: ListLayout::Stack,
                layout_delegate: None,
            }))
        }
    }
}

/// The list element with its items, while `items_mounted`.
#[component]
fn PlainList(
    collection: Collection,
    state: ListState,
    items: RwSignal<Vec<u32>>,
    orientation: RwSignal<Orientation>,
    items_mounted: Signal<bool>,
) -> impl IntoView {
    let props = collection_props(collection, &state, CapturedElement::new(), orientation);
    let collection_id = StoredValue::new(props.collection_id.clone());
    view! {
        <div
            {..props.into_attrs()}
            id="perf-list"
            role="listbox"
            aria-label="Options"
            aria-orientation=move || match orientation.get() {
                Orientation::Horizontal => "horizontal",
                Orientation::Vertical => "vertical",
            }
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
fn PlainItem(state: ListState, collection_id: String, i: u32) -> impl IntoView {
    let UseSelectableItemReturn {
        props,
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
    let (attrs, styles) = props.into_parts();
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
