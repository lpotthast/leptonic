//! The hook `use_drop_indicator` with its collection, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_drop_indicator.rs`): the drop indicators
//! (`use_drop_indicator`) of a grid list (`#perf-list`, "Items", 150px high, scrolling) of [`SMALL`] or [`LARGE`] rows, 30px
//! high ("Item 0", ...; keys `item-0`, ...) taking drops on the list itself (a drop indicator for
//! the root first), between its rows (a drop indicator before each row and one after the last) and
//! on its rows (`use_droppable_item`). Drops between rows insert the dropped text as a row.
//!
//! Each drop indicator (`role=button`) has `data-target` (`root`, `before-item-<i>`,
//! `after-item-<i>`) and `data-drop-target` while it is the drop target; each row has `data-row`
//! (its number) and `data-drop-target` while it is. A `use_drag` source (`#perf-draggable`, "Drag me", dragging the
//! text "hello world") after a plain button (`#perf-before`) starts keyboard drags. The page
//! announces an empty message once mounted, so that the live announcer (which keyboard drags
//! announce through) exists before the test measures. Mounted at first, with [`SMALL`] rows.
//!
//! Its controls ([`Control`]) mount and unmount the list and switch its size (`small`, `large`;
//! while unmounted).

use std::sync::Arc;

use leptonic::{
    CapturedElement, Orientation, announce_polite, flag,
    hooks::{
        collections::{
            CollectionOptions, Key, ListLayout, SelectionMode, SelectionOptions,
            UseListCollectionInput, UseListKeyboardDelegateInput, UseListStateInput,
            use_list_collection, use_list_keyboard_delegate, use_list_state,
        },
        dnd::{
            DragItem, DropItem, DropPosition, DropTarget, DroppableCollectionData,
            DroppableCollectionInsertDropEvent, DroppableCollectionOptions, ListDropTargetDelegate,
            UseDragInput, UseDragReturn, UseDropIndicatorInput, UseDropIndicatorReturn,
            UseDroppableCollectionInput, UseDroppableCollectionReturn,
            UseDroppableCollectionStateInput, UseDroppableItemInput, UseDroppableItemReturn,
            use_drag, use_drop_indicator, use_droppable_collection, use_droppable_collection_state,
            use_droppable_item,
        },
        gridlist::{
            FocusMode, GridListData, KeyboardNavigationBehavior, UseGridListInput,
            UseGridListItemInput, UseGridListItemReturn, UseGridListReturn, use_grid_list,
            use_grid_list_item,
        },
    },
};
use leptos::{context::Provider, prelude::*};

use crate::pages::perf::{Control, PerfControls};

/// The rows of the small list.
const SMALL: u32 = 10;
/// The rows of the large list.
const LARGE: u32 = 1_000;

/// A row: its key's number and its text.
#[derive(Debug, Clone, PartialEq)]
struct Item {
    id: u32,
    text: String,
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let count = RwSignal::new(SMALL);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || count.set(SMALL)),
        Control::action("large", move || count.set(LARGE)),
    ];
    Effect::new(|| announce_polite(""));

    let UseDragReturn {
        draggable: drag_props,
        is_dragging,
        ..
    } = use_drag(UseDragInput {
        items: Signal::stored(vec![DragItem::text("hello world")]),
        allowed_drop_operations: None,
        preview: None,
        on_drag_start: None,
        on_drag_move: None,
        on_drag_end: None,
        has_drag_button: false,
        is_disabled: Signal::stored(false),
    });
    view! {
        <h1>"use_drop_indicator performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <div
            id="perf-draggable"
            role="button"
            tabindex="0"
            {..drag_props.attrs()}
            data-dragging=flag(is_dragging)
        >
            "Drag me"
        </div>
        <Show when=move || mounted.get()>
            <DroppableList count=count.get_untracked() />
        </Show>
    }
}

/// What the rows and drop indicators need.
#[derive(Clone)]
struct ListContext {
    list: GridListData,
    drop: DroppableCollectionData,
}

/// The text of the first dropped text item.
fn dropped_text(items: &[DropItem]) -> String {
    items
        .iter()
        .find_map(|item| match item {
            DropItem::Text(text) => text.get_text("text/plain").map(ToOwned::to_owned),
            _ => None,
        })
        .unwrap_or_default()
}

/// The grid list of `count` rows taking drops.
#[component]
fn DroppableList(count: u32) -> impl IntoView {
    let items = RwSignal::new(
        (0..count)
            .map(|id| Item {
                id,
                text: format!("Item {id}"),
            })
            .collect::<Vec<_>>(),
    );
    // Inserted rows get ids after the initial ones.
    let next_id = StoredValue::new(count);
    let collection = use_list_collection(UseListCollectionInput {
        items: items.into(),
        key: |item: &Item| Key::from(format!("item-{}", item.id)),
        text_value: |item: &Item| item.text.clone(),
    });
    let list = use_list_state(UseListStateInput {
        collection,
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            ..SelectionOptions::default()
        },
    });
    let element = CapturedElement::new();
    let UseGridListReturn {
        grid_list: props,
        data,
    } = use_grid_list(UseGridListInput {
        aria_label: "Items".into(),
        state: list,
        element,
        id: Some("perf-list".into()),
        aria_labelledby: Signal::stored(None),
        layout: ListLayout::Stack,
        orientation: Orientation::Vertical.into(),
        keyboard_delegate: None,
        options: CollectionOptions::default(),
        keyboard_navigation_behavior: KeyboardNavigationBehavior::default(),
        should_select_on_press_up: false,
        on_action: None,
        tree: None,
    });
    let on_insert = Callback::new(move |e: DroppableCollectionInsertDropEvent| {
        let target = e
            .target
            .key
            .to_string()
            .trim_start_matches("item-")
            .parse::<u32>()
            .ok();
        let id = next_id.get_value();
        next_id.set_value(id + 1);
        let text = dropped_text(&e.items);
        items.update(|items| {
            let index = items
                .iter()
                .position(|item| Some(item.id) == target)
                .unwrap_or(0);
            let index = if e.target.drop_position == DropPosition::After {
                index + 1
            } else {
                index
            };
            items.insert(index, Item { id, text });
        });
    });
    let state = use_droppable_collection_state(UseDroppableCollectionStateInput {
        list,
        options: DroppableCollectionOptions {
            on_insert: Some(on_insert),
            // Accepting drops on the list itself.
            on_root_drop: Some(Callback::new(|_| {})),
            // Accepting drops on the rows.
            on_item_drop: Some(Callback::new(|_| {})),
            ..DroppableCollectionOptions::default()
        },
        is_disabled: Signal::stored(false),
    });
    let UseDroppableCollectionReturn {
        collection: collection_props,
        data: drop,
    } = use_droppable_collection(UseDroppableCollectionInput {
        state,
        element,
        collection_id: data.id.clone(),
        keyboard_delegate: use_list_keyboard_delegate(UseListKeyboardDelegateInput {
            state: list,
            element,
            orientation: Orientation::Vertical.into(),
            layout: ListLayout::Stack,
            layout_delegate: None,
        }),
        drop_target_delegate: Arc::new(ListDropTargetDelegate::new(
            list.collection,
            list.item_elements,
            element,
        )),
        on_key_down: None,
    });
    let last = move || items.with(|items| items.last().map(|item| item.id));
    view! {
        <Provider value=ListContext { list: data, drop }>
            <div
                {..props.attrs()}
                {..collection_props.attrs()}
                style="height: 150px; overflow: auto; position: relative"
            >
                <DropIndicator target=DropTarget::Root name="root".to_owned() />
                <For
                    each=move || items.get()
                    key=|item| item.id
                    children=move |item: Item| view! { <Row item /> }
                />
                {move || {
                    last()
                        .map(|id| {
                            view! {
                                <DropIndicator
                                    target=DropTarget::item(format!("item-{id}"), DropPosition::After)
                                    name=format!("after-item-{id}")
                                />
                            }
                        })
                }}
            </div>
        </Provider>
    }
}

/// A row taking drops on it, after the drop indicator before it.
#[component]
fn Row(item: Item) -> impl IntoView {
    let ListContext { list, drop } = expect_context::<ListContext>();
    let key = Key::from(format!("item-{}", item.id));
    let UseGridListItemReturn {
        row: row_props,
        cell: grid_cell_props,
        ..
    } = use_grid_list_item(UseGridListItemInput {
        list,
        key: key.clone(),
        focus_mode: FocusMode::Row,
        allows_arrow_navigation: false,
        on_context_menu: None,
    });
    let element = CapturedElement::new();
    let UseDroppableItemReturn {
        item: drop_props,
        is_drop_target,
    } = use_droppable_item(UseDroppableItemInput {
        collection: drop,
        target: DropTarget::item(key.clone(), DropPosition::On).into(),
        element,
        activate_button: None,
    });
    let (row_attrs, row_styles) = row_props.attrs();
    view! {
        <DropIndicator
            target=DropTarget::item(key, DropPosition::Before)
            name=format!("before-item-{}", item.id)
        />
        <div
            {..row_attrs}
            {..drop_props.attrs()}
            {..element.attr()}
            style=row_styles
            data-row=item.id
            data-drop-target=flag(is_drop_target)
        >
            <div {..grid_cell_props.attrs()} style="height: 30px">
                {item.text}
            </div>
        </div>
    }
}

/// A drop indicator: a row of its own with a button, the drop target `target`.
#[component]
fn DropIndicator(target: DropTarget, name: String) -> impl IntoView {
    let ListContext { drop, .. } = expect_context::<ListContext>();
    let UseDropIndicatorReturn {
        indicator: drop_indicator_props,
        is_drop_target,
        is_hidden,
    } = use_drop_indicator(UseDropIndicatorInput {
        collection: drop,
        target: target.into(),
        activate_button: None,
    });
    view! {
        <div role="row">
            <div role="gridcell">
                <div
                    role="button"
                    {..drop_indicator_props.attrs()}
                    data-target=name
                    data-drop-target=flag(is_drop_target)
                    data-hidden=flag(is_hidden)
                ></div>
            </div>
        </div>
    }
}
