use std::{collections::HashSet, sync::Arc};

use leptonic::hooks::FocusMode;
use leptonic::hooks::KeyboardNavigationBehavior;
use leptonic::hooks::collections::CollectionOptions;
use leptonic::{
    atoms::checkbox::Checkbox,
    hooks::{
        DragItem, DraggableCollectionState, DropPosition, DropTarget, DroppableCollectionData,
        DroppableCollectionOptions, DroppableCollectionReorderEvent, GridListData, IntoAttrs,
        ListDropTargetDelegate, Orientation, SelectionMode, UseDraggableCollectionInput, UseDraggableCollectionStateInput,
        UseDraggableItemInput, UseDraggableItemReturn, UseDropIndicatorInput,
        UseDropIndicatorReturn, UseDroppableCollectionInput, UseDroppableCollectionReturn,
        UseDroppableCollectionStateInput, UseDroppableItemInput, UseDroppableItemReturn,
        UseGridListInput, UseGridListItemInput, UseGridListItemReturn, UseGridListReturn,
        collections::{
            Key, ListLayout, SelectionOptions, UseListStateInput, use_list_collection,
            UseListKeyboardDelegateInput, use_list_keyboard_delegate, use_list_state,
        },
        use_draggable_collection, use_draggable_collection_state, use_draggable_item,
        use_drop_indicator, use_droppable_collection, use_droppable_collection_state,
        use_droppable_item, use_grid_list, use_grid_list_item,
    },
    utils::CapturedElement,
};
use leptos::prelude::*;

const TASKS: [&str; 5] = ["Plan", "Design", "Build", "Test", "Release"];

/// Moves the items with `keys` before or after the target item, keeping their order.
fn reorder(tasks: &mut Vec<&'static str>, e: &DroppableCollectionReorderEvent) {
    let Some(target) = tasks
        .iter()
        .position(|task| Key::from(*task) == e.target.key)
    else {
        return;
    };
    let index = if e.target.drop_position == DropPosition::After {
        target + 1
    } else {
        target
    };
    let is_moved = |task: &&str| e.keys.contains(&Key::from(*task));
    // The insertion index once the moved items are taken out.
    let index = index - tasks[..index].iter().filter(|task| is_moved(task)).count();
    let (moved, mut rest): (Vec<_>, Vec<_>) = tasks.drain(..).partition(is_moved);
    rest.splice(index..index, moved);
    *tasks = rest;
}

fn sorted(keys: impl IntoIterator<Item = Key>, tasks: &[&str]) -> String {
    let keys: HashSet<Key> = keys.into_iter().collect();
    let names: Vec<&str> = tasks
        .iter()
        .copied()
        .filter(|task| keys.contains(&Key::from(*task)))
        .collect();
    if names.is_empty() {
        "none".to_owned()
    } else {
        names.join(", ")
    }
}

/// A grid list whose rows are reordered by dragging them with the mouse, or with the keyboard: Enter starts a
/// drag, arrow keys move between the drop positions, Enter drops.
#[component]
pub fn ReorderDemo() -> impl IntoView {
    let tasks = RwSignal::new(TASKS.to_vec());
    let disabled = RwSignal::new(false);
    let last_drop = RwSignal::new(String::from("none yet"));

    // The rows and their selection: dragging a selected row drags all selected rows.
    let collection = use_list_collection(
        tasks.into(),
        |task| Key::from(*task),
        |task| (*task).to_owned(),
    );
    let list = use_list_state(UseListStateInput {
        collection,
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            ..SelectionOptions::default()
        },
    });
    let element = CapturedElement::new();
    let UseGridListReturn { props, data } = use_grid_list(UseGridListInput {
        aria_label: "Tasks".into(),
        // Select when the press ends, so that dragging a row doesn't select it.
        should_select_on_press_up: true,
        state: list,
        element,
        id: None,
        aria_labelledby: Signal::stored(None),
        layout: ListLayout::Stack,
        keyboard_delegate: None,
        options: CollectionOptions::default(),
        keyboard_navigation_behavior: KeyboardNavigationBehavior::default(),
        on_action: None,
        tree: None,
    });

    // Dragging: the dragged rows' data.
    let drag_state = use_draggable_collection_state(UseDraggableCollectionStateInput {
        is_disabled: disabled.into(),
        list,
        get_items: Callback::new(|keys: HashSet<Key>| {
            keys.iter()
                .map(|key| DragItem::text(key.to_string()))
                .collect()
        }),
        preview: None,
        allowed_drop_operations: None,
        on_drag_start: None,
        on_drag_move: None,
        on_drag_end: None,
    });
    use_draggable_collection(UseDraggableCollectionInput {
        state: drag_state,
        element,
    });

    // Dropping: only reorders (drops between rows) are valid.
    let drop_state = use_droppable_collection_state(UseDroppableCollectionStateInput {
        list,
        options: DroppableCollectionOptions {
            on_reorder: Some(Callback::new(move |e: DroppableCollectionReorderEvent| {
                let moved = tasks.with(|tasks| sorted(e.keys.iter().cloned(), tasks));
                let position = if e.target.drop_position == DropPosition::After {
                    "after"
                } else {
                    "before"
                };
                last_drop.set(format!(
                    "{moved} {position} {} ({:?})",
                    e.target.key, e.drop_operation
                ));
                tasks.update(|tasks| reorder(tasks, &e));
            })),
            ..DroppableCollectionOptions::default()
        },
        is_disabled: disabled.into(),
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
            element: element,
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

    let last = move || tasks.with(|tasks| tasks.last().copied());
    view! {
        <div {..props.into_attrs()} {..collection_props.into_attrs()} class="demo-dnd-list">
            <For
                each=move || tasks.get()
                key=|task| *task
                children={
                    let (data, drop) = (data.clone(), drop.clone());
                    move |task| view! { <TaskRow task list=data.clone() drag_state drop=drop.clone()/> }
                }
            />
            // The position after the last row.
            {move || last().map(|task| view! { <DropIndicator task position=DropPosition::After drop=drop.clone()/> })}
        </div>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Disabled"
            </Checkbox>
        </div>

        <p class="demo-status">
            "Order: "{move || tasks.get().join(", ")}<br/>
            "Selected: "{move || tasks.with(|tasks| sorted(list.selection.selected_keys(), tasks))}<br/>
            "Dragging: "{move || tasks.with(|tasks| sorted(drag_state.dragging_keys.get(), tasks))}<br/>
            "Last drop: "{last_drop}
        </p>
    }
}

/// A row with the drop indicator before it.
#[component]
fn TaskRow(
    task: &'static str,
    list: GridListData,
    drag_state: DraggableCollectionState,
    drop: DroppableCollectionData,
) -> impl IntoView {
    let key = Key::from(task);
    let UseGridListItemReturn {
        row_props,
        grid_cell_props,
        is_selected,
        is_focus_visible,
        ..
    } = use_grid_list_item(UseGridListItemInput {
        list,
        key: key.clone(),
        focus_mode: FocusMode::Row,
        allows_arrow_navigation: false,
        on_context_menu: None,
    });
    let UseDraggableItemReturn { mut drag_props, .. } = use_draggable_item(UseDraggableItemInput {
        state: drag_state,
        key: key.clone(),
        has_drag_button: false,
        has_action: false,
    });
    // All dragged rows, not only the one the drag started from.
    let is_dragging = {
        let key = key.clone();
        Signal::derive(move || drag_state.is_dragging(&key))
    };
    // The row is a drop target of keyboard drags too (rows that can't take the drop are hidden from them).
    let element = CapturedElement::new();
    let UseDroppableItemReturn { drop_props, .. } = use_droppable_item(UseDroppableItemInput {
        collection: drop.clone(),
        target: DropTarget::item(key, DropPosition::On).into(),
        element,
        activate_button: None,
    });
    // Both hooks describe the row: join their descriptions.
    let (drag_description, drop_description) =
        (drag_props.aria_describedby, drop_props.aria_describedby);
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
        <DropIndicator task position=DropPosition::Before drop/>
        <div
            {..row_attrs}
            {..drag_props.into_attrs()}
            {..element.attr()}
            class="demo-dnd-row"
            style=row_styles
            aria-hidden=move || aria_hidden.get()
            data-dragging=move || is_dragging.get().then_some("")
            data-focus-visible=move || is_focus_visible.get().then_some("")
        >
            <div {..grid_cell_props.into_attrs()} class="demo-dnd-cell">
                <span class="demo-dnd-check" aria-hidden="true">{move || if is_selected.get() { "\u{2713}" } else { "" }}</span>
                <span class="demo-dnd-grip" aria-hidden="true">"\u{2630}"</span>
                <span>{task}</span>
            </div>
        </div>
    }
}

/// A drop position between rows: a line, shown while a drag is over it.
#[component]
fn DropIndicator(
    task: &'static str,
    position: DropPosition,
    drop: DroppableCollectionData,
) -> impl IntoView {
    let UseDropIndicatorReturn {
        drop_indicator_props,
        is_drop_target,
        is_hidden,
    } = use_drop_indicator(UseDropIndicatorInput {
        collection: drop,
        target: DropTarget::item(task, position).into(),
        activate_button: None,
    });
    view! {
        <div role="row" class="demo-dnd-indicator" data-drop-target=move || is_drop_target.get().then_some("") data-hidden=move || is_hidden.get().then_some("")>
            <div role="gridcell" {..drop_indicator_props.into_attrs()}></div>
        </div>
    }
}
