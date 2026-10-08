use std::{collections::HashSet, sync::Arc};

use leptonic::hooks::FocusMode;
use leptonic::hooks::KeyboardNavigationBehavior;
use leptonic::hooks::collections::CollectionOptions;
use leptonic::{
    hooks::{
        DragEndEvent, DragItem, DropEnterEvent, DropEvent, DropExitEvent, DropItem, DropPosition,
        DropTarget, DroppableCollectionData, DroppableCollectionOptions,
        DroppableCollectionReorderEvent, GridListData, IntoAttrs, ListDropTargetDelegate,
        Orientation, UseDragInput, UseDragReturn, UseDraggableCollectionInput,
        UseDraggableCollectionStateInput, UseDraggableItemInput, UseDraggableItemReturn,
        UseDropIndicatorInput, UseDropIndicatorReturn, UseDropInput, UseDropReturn,
        UseDroppableCollectionInput, UseDroppableCollectionReturn,
        UseDroppableCollectionStateInput, UseDroppableItemInput, UseDroppableItemReturn,
        UseGridListInput, UseGridListItemInput, UseGridListItemReturn, UseGridListReturn,
        collections::{
            Key, ListLayout, ListState, SelectionOptions, UseListKeyboardDelegateInput,
            UseListStateInput, use_list_collection, use_list_keyboard_delegate, use_list_state,
        },
        use_drag, use_draggable_collection, use_draggable_collection_state, use_draggable_item,
        use_drop, use_drop_indicator, use_droppable_collection, use_droppable_collection_state,
        use_droppable_item, use_grid_list, use_grid_list_item,
    },
    utils::CapturedElement,
};
use leptos::{context::Provider, prelude::*};

/// Drag and drop: a draggable element and two drop targets (react-aria's `dnd.test.js` setup),
/// and a list whose items can be reordered.
#[component]
pub fn PageHookDnd() -> impl IntoView {
    let log = RwSignal::new(Vec::<String>::new());
    let push = move |entry: String| log.update(|l| l.push(entry));
    view! {
        <div id="test-page-hook-dnd">
            <h1>"Drag and drop"</h1>
            <button id="test-dnd-before">"Before"</button>
            <Draggable log=push />
            <Droppable label="Drop here" index=1 log=push />
            <Droppable label="Drop here 2" index=2 log=push />
            <ol id="test-dnd-log">
                {move || log.get().into_iter().map(|e| view! { <li>{e}</li> }).collect_view()}
            </ol>
            <button id="test-dnd-before-list">"Before"</button>
            <ReorderableList />
        </div>
    }
}

#[component]
fn Draggable(log: impl Fn(String) + Copy + Send + Sync + 'static) -> impl IntoView {
    let UseDragReturn {
        drag_props,
        is_dragging,
        ..
    } = use_drag(UseDragInput {
        on_drag_start: Some(Callback::new(move |_| log("dragstart".to_owned()))),
        on_drag_end: Some(Callback::new(move |e: DragEndEvent| {
            log(format!("dragend {:?}", e.drop_operation));
        })),
        items: Signal::stored(vec![DragItem::text("hello world")]),
        allowed_drop_operations: None,
        preview: None,
        on_drag_move: None,
        has_drag_button: false,
        is_disabled: Signal::stored(false),
    });
    view! {
        <div
            id="test-dnd-draggable"
            tabindex="0"
            {..drag_props.into_attrs()}
            data-dragging=move || is_dragging.get().to_string()
        >
            "Drag me"
        </div>
    }
}

#[component]
fn Droppable(
    label: &'static str,
    index: usize,
    log: impl Fn(String) + Copy + Send + Sync + 'static,
) -> impl IntoView {
    let element = CapturedElement::new();
    let UseDropReturn {
        drop_props,
        is_drop_target,
        ..
    } = use_drop(UseDropInput {
        on_drop_enter: Some(Callback::new(move |_: DropEnterEvent| {
            log(format!("dropenter {index}"))
        })),
        on_drop_exit: Some(Callback::new(move |_: DropExitEvent| {
            log(format!("dropexit {index}"))
        })),
        on_drop: Some(Callback::new(move |e: DropEvent| {
            let text = e
                .items
                .iter()
                .find_map(|item| match item {
                    DropItem::Text(text) => text.get_text("text/plain").map(ToOwned::to_owned),
                    _ => None,
                })
                .unwrap_or_default();
            log(format!("drop {index} {text} {:?}", e.drop_operation));
        })),
        element,
        get_drop_operation: None,
        get_drop_operation_for_point: None,
        on_drop_move: None,
        on_drop_activate: None,
        has_drop_button: false,
        is_disabled: Signal::stored(false),
    });
    view! {
        <div
            role="button"
            tabindex="0"
            class="test-dnd-droppable"
            {..drop_props.into_attrs()}
            data-droptarget=move || is_drop_target.get().to_string()
        >
            {label}
        </div>
    }
}

const LETTERS: [&str; 4] = ["A", "B", "C", "D"];

/// What the rows and indicators of the reorderable list need.
#[derive(Clone)]
struct ListContext {
    list: GridListData,
    drag_state: leptonic::hooks::DraggableCollectionState,
    drop: DroppableCollectionData,
}

/// A grid list whose rows can be reordered by dragging (with drop indicators between rows).
#[component]
fn ReorderableList() -> impl IntoView {
    let letters = RwSignal::new(LETTERS.to_vec());
    let collection = use_list_collection(
        letters.into(),
        |letter| Key::from(*letter),
        |letter| (*letter).to_owned(),
    );
    let list: ListState = use_list_state(UseListStateInput {
        collection,
        selection: SelectionOptions::default(),
    });
    let element = CapturedElement::new();
    let UseGridListReturn { props, data } = use_grid_list(UseGridListInput {
        aria_label: "Letters".into(),
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

    let drag_state = use_draggable_collection_state(UseDraggableCollectionStateInput {
        list,
        get_items: Callback::new(|keys: HashSet<Key>| {
            keys.iter().map(|k| DragItem::text(k.to_string())).collect()
        }),
        preview: None,
        allowed_drop_operations: None,
        on_drag_start: None,
        on_drag_move: None,
        on_drag_end: None,
        is_disabled: Signal::stored(false),
    });
    use_draggable_collection(UseDraggableCollectionInput {
        state: drag_state,
        element,
    });

    let on_reorder = Callback::new(move |e: DroppableCollectionReorderEvent| {
        letters.update(|letters| {
            let moved: Vec<&str> = letters
                .iter()
                .copied()
                .filter(|l| e.keys.contains(&Key::from(*l)))
                .collect();
            letters.retain(|l| !e.keys.contains(&Key::from(*l)));
            let target = e.target.key.to_string();
            let index = letters
                .iter()
                .position(|l| *l == target)
                .unwrap_or(letters.len());
            let index = if e.target.drop_position == DropPosition::After {
                index + 1
            } else {
                index
            };
            for (offset, letter) in moved.into_iter().enumerate() {
                letters.insert(index + offset, letter);
            }
        });
    });
    let drop_state = use_droppable_collection_state(UseDroppableCollectionStateInput {
        list,
        options: DroppableCollectionOptions {
            on_reorder: Some(on_reorder),
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

    let context = ListContext {
        list: data,
        drag_state,
        drop,
    };
    let last = move || letters.with(|l| l.last().copied());
    view! {
        <Provider value=context>
            <div {..props.into_attrs()} {..collection_props.into_attrs()}>
                <For
                    each=move || letters.get()
                    key=|letter| *letter
                    children=move |letter: &'static str| view! { <Row letter /> }
                />
                {move || last().map(|letter| view! { <DropIndicator letter position=DropPosition::After /> })}
            </div>
        </Provider>
        <div>"Order: " <span id="test-dnd-order">{move || letters.get().join("")}</span></div>
    }
}

#[component]
fn Row(letter: &'static str) -> impl IntoView {
    let ListContext {
        list,
        drag_state,
        drop,
    } = expect_context::<ListContext>();
    let key = Key::from(letter);
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
    let UseDraggableItemReturn {
        mut drag_props,
        is_dragging,
        ..
    } = use_draggable_item(UseDraggableItemInput {
        state: drag_state,
        key: key.clone(),
        has_drag_button: false,
        has_action: false,
    });
    let element = CapturedElement::new();
    let UseDroppableItemReturn { drop_props, .. } = use_droppable_item(UseDroppableItemInput {
        collection: drop,
        target: DropTarget::item(key, DropPosition::On).into(),
        element,
        activate_button: None,
    });
    // Both describe the row: join the ids (react-aria merges them).
    let drag_description = drag_props.aria_describedby;
    let drop_description = drop_props.aria_describedby;
    drag_props.aria_describedby = Signal::derive(move || {
        let ids: Vec<String> = [drag_description.get(), drop_description.get()]
            .into_iter()
            .flatten()
            .collect();
        (!ids.is_empty()).then(|| ids.join(" "))
    });
    let aria_hidden = drop_props.aria_hidden;
    let (row_attrs, row_styles) = row_props.into_parts();

    view! {
        <DropIndicator letter position=DropPosition::Before />
        <div
            {..row_attrs}
            {..drag_props.into_attrs()}
            {..element.attr()}
            style=row_styles
            aria-hidden=move || aria_hidden.get()
            data-dragging=move || is_dragging.get().then_some("")
        >
            <div {..grid_cell_props.into_attrs()}>{letter}</div>
        </div>
    }
}

#[component]
fn DropIndicator(letter: &'static str, position: DropPosition) -> impl IntoView {
    let ListContext { drop, .. } = expect_context::<ListContext>();
    let UseDropIndicatorReturn {
        drop_indicator_props,
        is_drop_target,
        is_hidden,
    } = use_drop_indicator(UseDropIndicatorInput {
        collection: drop,
        target: DropTarget::item(letter, position).into(),
        activate_button: None,
    });
    view! {
        <div role="row" class="test-dnd-indicator">
            <div
                role="gridcell"
                {..drop_indicator_props.into_attrs()}
                data-drop-target=move || is_drop_target.get().then_some("")
                data-hidden=move || is_hidden.get().then_some("")
            ></div>
        </div>
    }
}
