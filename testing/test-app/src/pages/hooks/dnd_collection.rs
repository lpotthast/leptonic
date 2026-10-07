use std::sync::Arc;

use leptonic::{
    hooks::{
        CollectionDropOperationQuery, DragEndEvent, DragItem, DropItem, DropOperation,
        DropPosition, DropTarget, DroppableCollectionData, DroppableCollectionEnterEvent,
        DroppableCollectionExitEvent, DroppableCollectionInsertDropEvent,
        DroppableCollectionOnItemDropEvent, DroppableCollectionOptions,
        DroppableCollectionRootDropEvent, FocusMode, GridListData, IntoAttrs,
        KeyboardNavigationBehavior, ListDropTargetDelegate, Orientation, SelectionMode,
        UseDragInput, UseDragReturn, UseDropIndicatorInput, UseDropIndicatorReturn,
        UseDroppableCollectionInput, UseDroppableCollectionReturn,
        UseDroppableCollectionStateInput, UseGridListInput, UseGridListItemInput,
        UseGridListItemReturn, UseGridListReturn,
        collections::{
            CollectionOptions, Key, ListLayout, ListState, SelectionOptions,
            UseListKeyboardDelegateInput, UseListStateInput, use_list_collection,
            use_list_keyboard_delegate, use_list_state,
        },
        use_drag, use_drop_indicator, use_droppable_collection, use_droppable_collection_state,
        use_grid_list, use_grid_list_item,
    },
    utils::CapturedElement,
};
use leptos::{context::Provider, prelude::*};
use leptos_router::hooks::use_query_map;

/// An item of the droppable list.
#[derive(Debug, Clone, PartialEq)]
struct Item {
    id: String,
    text: String,
}

/// A draggable ("Drag me", dragging the text "hello world") and a grid list "List" taking drops
/// (react-aria's `DroppableGridExample` in `useDroppableCollection.test.js`): on the list (root),
/// between its rows (insert) and on rows (except "Two"; an operation of `Copy`). Rows 50px, the
/// list 150px high (scrolls). Multiple selection. `#test-dnd-collection-log` logs the drop
/// target changes ("enter root", "exit 3 after") and drops ("insert 2 before hello world Move",
/// "root ...", "on 1 hello world Copy").
///
/// Query parameters: `items=<n>`: rows "Item 0".."Item n-1" (keys "0"..) instead of One, Two,
/// Three (keys "1".."3"); `only-on`: only drops on rows (with `Move`); `cancel=<keys>`: the rows
/// not taking drops on them (comma-separated; default: "2").
#[component]
pub fn PageHookDndCollection() -> impl IntoView {
    let query = use_query_map();
    let (count, only_on, canceled) = query.with_untracked(|q| {
        (
            q.get("items").and_then(|n| n.parse::<usize>().ok()),
            q.get("only-on").is_some(),
            q.get("cancel").map_or_else(
                || vec![Key::from("2")],
                |keys| {
                    keys.split(',')
                        .filter(|k| !k.is_empty())
                        .map(|k| Key::from(k.to_owned()))
                        .collect()
                },
            ),
        )
    });
    let initial = match count {
        Some(count) => (0..count)
            .map(|i| Item {
                id: i.to_string(),
                text: format!("Item {i}"),
            })
            .collect(),
        None => ["One", "Two", "Three"]
            .iter()
            .enumerate()
            .map(|(i, text)| Item {
                id: (i + 1).to_string(),
                text: (*text).to_owned(),
            })
            .collect::<Vec<_>>(),
    };
    let next_id = StoredValue::new(initial.len() + 1);
    let items = RwSignal::new(initial);
    let log = RwSignal::new(Vec::<String>::new());
    let push = move |entry: String| log.update(|l| l.push(entry));

    let UseDragReturn { drag_props, .. } = use_drag(UseDragInput {
        items: Signal::stored(vec![DragItem::text("hello world")]),
        allowed_drop_operations: None,
        preview: None,
        on_drag_start: None,
        on_drag_move: None,
        on_drag_end: Some(Callback::new(move |e: DragEndEvent| {
            push(format!("dragend {:?}", e.drop_operation));
        })),
        has_drag_button: false,
        is_disabled: Signal::stored(false),
    });

    // Inserts the dropped text items at `index`.
    let insert = move |index: usize, dropped: &[DropItem]| {
        let new: Vec<Item> = dropped
            .iter()
            .filter_map(|item| match item {
                DropItem::Text(text) => text.get_text("text/plain").map(ToOwned::to_owned),
                _ => None,
            })
            .map(|text| {
                let id = next_id.get_value();
                next_id.set_value(id + 1);
                Item {
                    id: id.to_string(),
                    text,
                }
            })
            .collect();
        items.update(|items| {
            for (offset, item) in new.into_iter().enumerate() {
                items.insert(index + offset, item);
            }
        });
    };
    let dropped_text = |dropped: &[DropItem]| {
        dropped
            .iter()
            .find_map(|item| match item {
                DropItem::Text(text) => text.get_text("text/plain").map(ToOwned::to_owned),
                _ => None,
            })
            .unwrap_or_default()
    };

    let collection = use_list_collection(
        items.into(),
        |item: &Item| Key::from(item.id.clone()),
        |item: &Item| item.text.clone(),
    );
    let list: ListState = use_list_state(UseListStateInput {
        collection,
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            ..SelectionOptions::default()
        },
    });
    let element = CapturedElement::new();
    let UseGridListReturn { props, data } = use_grid_list(UseGridListInput {
        aria_label: "List".into(),
        state: list,
        element,
        id: None,
        aria_labelledby: Signal::stored(None),
        layout: ListLayout::Stack,
        keyboard_delegate: None,
        options: CollectionOptions::default(),
        keyboard_navigation_behavior: KeyboardNavigationBehavior::default(),
        should_select_on_press_up: false,
        on_action: None,
        tree: None,
    });

    let drop_state = use_droppable_collection_state(UseDroppableCollectionStateInput {
        list,
        options: DroppableCollectionOptions {
            on_drop_enter: Some(Callback::new(move |e: DroppableCollectionEnterEvent| {
                push(format!("enter {}", describe(&e.target)));
            })),
            on_drop_exit: Some(Callback::new(move |e: DroppableCollectionExitEvent| {
                push(format!("exit {}", describe(&e.target)));
            })),
            on_insert: Some(Callback::new(move |e: DroppableCollectionInsertDropEvent| {
                let target = DropTarget::Item(e.target.clone());
                push(format!(
                    "insert {} {} {:?}",
                    describe(&target),
                    dropped_text(&e.items),
                    e.drop_operation
                ));
                let key = e.target.key.to_string();
                let index = items
                    .with_untracked(|items| items.iter().position(|item| item.id == key))
                    .unwrap_or(0);
                let index = if e.target.drop_position == DropPosition::After {
                    index + 1
                } else {
                    index
                };
                insert(index, &e.items);
            })),
            on_root_drop: Some(Callback::new(move |e: DroppableCollectionRootDropEvent| {
                push(format!(
                    "root {} {:?}",
                    dropped_text(&e.items),
                    e.drop_operation
                ));
                insert(0, &e.items);
            })),
            on_item_drop: Some(Callback::new(move |e: DroppableCollectionOnItemDropEvent| {
                push(format!(
                    "on {} {} {:?}",
                    e.target.key,
                    dropped_text(&e.items),
                    e.drop_operation
                ));
            })),
            get_drop_operation: Some(Callback::new(move |q: CollectionDropOperationQuery| {
                match &q.target {
                    DropTarget::Root if only_on => DropOperation::Cancel,
                    DropTarget::Root => DropOperation::Move,
                    DropTarget::Item(t) if t.drop_position != DropPosition::On => {
                        if only_on {
                            DropOperation::Cancel
                        } else {
                            q.allowed_operations
                                .first()
                                .copied()
                                .unwrap_or(DropOperation::Cancel)
                        }
                    }
                    DropTarget::Item(t) if canceled.contains(&t.key) => DropOperation::Cancel,
                    DropTarget::Item(_) if only_on => DropOperation::Move,
                    DropTarget::Item(_) => DropOperation::Copy,
                }
            })),
            ..DroppableCollectionOptions::default()
        },
        is_disabled: Signal::stored(false),
    });
    let UseDroppableCollectionReturn {
        collection_props,
        data: drop,
    } = use_droppable_collection(UseDroppableCollectionInput {
        state: drop_state,
        element,
        collection_id: props.id.clone(),
        keyboard_delegate: use_list_keyboard_delegate(UseListKeyboardDelegateInput {
            state: list,
            element,
            orientation: Orientation::Vertical,
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

    let context = ListContext { list: data, drop };
    let last = move || items.with(|items| items.last().map(|item| item.id.clone()));
    view! {
        <div id="test-page-hook-dnd-collection">
            <h1>"Droppable collection"</h1>
            <button>"Before"</button>
            <div role="button" tabindex="0" {..drag_props.into_attrs()}>
                "Drag me"
            </div>
            <Provider value=context>
                <div
                    {..props.into_attrs()}
                    {..collection_props.into_attrs()}
                    style="height: 150px; overflow: auto; position: relative"
                >
                    <DropIndicator target=DropTarget::Root />
                    <For
                        each=move || items.get()
                        key=|item| item.id.clone()
                        children=move |item: Item| view! { <Row item /> }
                    />
                    {move || {
                        last()
                            .map(|id| {
                                view! {
                                    <DropIndicator target=DropTarget::item(id, DropPosition::After) />
                                }
                            })
                    }}
                </div>
            </Provider>
            <ol id="test-dnd-collection-log">
                {move || log.get().into_iter().map(|e| view! { <li>{e}</li> }).collect_view()}
            </ol>
        </div>
    }
}

/// "root", "2 before", "3 on".
fn describe(target: &DropTarget) -> String {
    match target {
        DropTarget::Root => "root".to_owned(),
        DropTarget::Item(t) => format!(
            "{} {}",
            t.key,
            match t.drop_position {
                DropPosition::Before => "before",
                DropPosition::After => "after",
                DropPosition::On => "on",
            }
        ),
    }
}

#[derive(Clone)]
struct ListContext {
    list: GridListData,
    drop: DroppableCollectionData,
}

#[component]
fn Row(item: Item) -> impl IntoView {
    let ListContext { list, .. } = expect_context::<ListContext>();
    let key = Key::from(item.id.clone());
    let UseGridListItemReturn {
        row_props,
        grid_cell_props,
        ..
    } = use_grid_list_item(UseGridListItemInput {
        list,
        key: key.clone(),
        focus_mode: FocusMode::Row,
        allows_arrow_navigation: false,
        on_context_menu: None,
    });
    let (row_attrs, row_styles) = row_props.into_parts();
    view! {
        <DropIndicator target=DropTarget::item(key.clone(), DropPosition::Before) />
        <div {..row_attrs} style=row_styles>
            <div {..grid_cell_props.into_attrs()} style="height: 50px">
                {item.text}
                <DropIndicator target=DropTarget::item(key, DropPosition::On) inline=true />
            </div>
        </div>
    }
}

/// A drop target: a row of its own between rows (or for the root), or a button in a row's cell
/// (`inline`, for drops on the row).
#[component]
fn DropIndicator(target: DropTarget, #[prop(optional)] inline: bool) -> impl IntoView {
    let ListContext { drop, .. } = expect_context::<ListContext>();
    let UseDropIndicatorReturn {
        drop_indicator_props,
        is_drop_target,
        is_hidden,
    } = use_drop_indicator(UseDropIndicatorInput {
        collection: drop,
        target: target.into(),
        activate_button: None,
    });
    let indicator = view! {
        <div
            role="button"
            class="test-dnd-indicator"
            {..drop_indicator_props.into_attrs()}
            data-drop-target=move || is_drop_target.get().then_some("")
            data-hidden=move || is_hidden.get().then_some("")
        ></div>
    };
    if inline {
        indicator.into_any()
    } else {
        view! {
            <div role="row">
                <div role="gridcell">{indicator}</div>
            </div>
        }
        .into_any()
    }
}
