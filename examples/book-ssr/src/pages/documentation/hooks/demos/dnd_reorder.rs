use std::collections::HashSet;

use leptonic::{
    atoms::focus_ring::{FocusRing, FocusRingContext},
    hooks::*,
};
use leptos::{html, prelude::*};
use ringbuf::{
    HeapRb,
    traits::{Consumer, RingBuffer},
};

/// [`DropTargetDelegate`] that lazily obtains the collection element from a [`NodeRef`].
///
/// `ListDropTargetDelegate::new()` requires a `web_sys::Element` that only
/// exists after mount. This wrapper defers element access to each hit-test call.
struct NodeRefDropTargetDelegate {
    node_ref: NodeRef<html::Div>,
}

impl DropTargetDelegate for NodeRefDropTargetDelegate {
    fn get_drop_target_from_point(
        &self,
        x: f64,
        y: f64,
        is_valid_drop_target: &dyn Fn(&DropTarget) -> bool,
    ) -> Option<DropTarget> {
        let element: web_sys::HtmlDivElement = self.node_ref.get()?;
        ListDropTargetDelegate::new(element.into(), false).get_drop_target_from_point(
            x,
            y,
            is_valid_drop_target,
        )
    }
}

/// Reorders `items` by removing `dragged_keys` and re-inserting them at `target`.
fn reorder_items(
    items: &mut Vec<(String, String)>,
    dragged_keys: &[String],
    target: &ItemDropTarget,
) {
    let dragged: Vec<(String, String)> = items
        .iter()
        .filter(|(k, _)| dragged_keys.contains(k))
        .cloned()
        .collect();

    items.retain(|(k, _)| !dragged_keys.contains(k));

    let insert_idx = items
        .iter()
        .position(|(k, _)| k == &target.key)
        .map_or(items.len(), |idx| match target.position {
            DropPosition::Before => idx,
            DropPosition::After | DropPosition::On => idx + 1,
        });

    for (i, item) in dragged.into_iter().enumerate() {
        items.insert((insert_idx + i).min(items.len()), item);
    }
}

/// Renders a thin horizontal line that becomes visible when a drag hovers
/// over the corresponding drop position.
#[component]
fn DropIndicatorLine(
    target: DropTarget,
    state: DroppableCollectionState,
    keys: Signal<Vec<String>>,
) -> impl IntoView {
    let indicator = use_drop_indicator(UseDropIndicatorInput {
        target,
        state,
        collection_keys: keys,
        get_text_value: Callback::new(|key: String| key),
    });

    view! {
        <div
            {..indicator.drop_indicator_props.into_attrs()}
            style=move || {
                if indicator.is_drop_target.get() {
                    "height: 2px; background: var(--brand-color); margin: 0; transition: background 0.15s;"
                } else if indicator.is_hidden.get() {
                    "height: 0; margin: 0;"
                } else {
                    "height: 2px; background: transparent; margin: 0;"
                }
            }
        />
    }
}

/// Individual reorderable item using the collection drag-and-drop API.
#[component]
fn ReorderItem(
    key: String,
    label: String,
    drag_state: DraggableCollectionState,
    drop_state: DroppableCollectionState,
) -> impl IntoView {
    let drag = use_draggable_collection_item(UseDraggableCollectionItemInput {
        key: key.clone(),
        state: drag_state,
        is_disabled: Signal::derive(|| false),
        has_action: false,
    });

    let drop_item = use_collection_droppable_item(UseCollectionDroppableItemInput {
        target: DropTarget::Item {
            key: key.clone(),
            position: DropPosition::On,
        },
        state: drop_state,
        is_disabled: Signal::derive(|| false),
    });

    let is_dragging = drag.is_dragging;

    view! {
        <FocusRing>
            <div
                {..drag.drag_props.into_attrs()}
                {..drop_item.drop_item_props.into_attrs()}
                data-key=key
                style=move || {
                    let focus_visible = use_context::<FocusRingContext>()
                        .is_some_and(|ctx| ctx.is_focus_visible.get());
                    format!(
                        "padding: 0.75em 1em; border-radius: 8px; cursor: grab; user-select: none; \
                        display: flex; align-items: center; gap: 0.5em; \
                        background: white; border: 2px solid {}; \
                        transition: all 0.15s; {}",
                        if focus_visible { "var(--brand-color)" } else { "#ddd" },
                        if is_dragging.get() { "opacity: 0.4;" } else { "" }
                    )
                }
            >
                <span style="color: #999;">"⋮⋮"</span>
                { label }
            </div>
        </FocusRing>
    }
}

/// Demo: List reordering with the collection drag-and-drop API.
#[component]
pub fn ReorderDemo() -> impl IntoView {
    let (items, set_items) = signal(vec![
        ("item-1".to_string(), "First Item".to_string()),
        ("item-2".to_string(), "Second Item".to_string()),
        ("item-3".to_string(), "Third Item".to_string()),
        ("item-4".to_string(), "Fourth Item".to_string()),
        ("item-5".to_string(), "Fifth Item".to_string()),
    ]);

    let (events, set_events) = signal(HeapRb::<Oco<'static, str>>::new(10));

    let string = Memo::new(move |_| {
        events.with(|events| {
            let mut result = String::new();
            for e in events.iter().rev() {
                result.push_str(e.as_str());
                result.push('\n');
            }
            result
        })
    });

    let keys = Signal::derive(move || {
        items
            .get()
            .iter()
            .map(|(k, _)| k.clone())
            .collect::<Vec<_>>()
    });

    // Draggable collection state (dummy selection for simple reorder).
    let collection_ref = NodeRef::<html::Div>::new();
    let collection_element = Signal::derive(move || {
        collection_ref.get().map(|el| {
            let element: web_sys::Element = el.into();
            element
        })
    });

    let drag_state = use_draggable_collection_state(DraggableCollectionStateInput {
        collection_keys: keys,
        is_selected: Callback::new(|_: String| false),
        selected_keys: Signal::derive(HashSet::new),
        get_items: Callback::new(|keys: Vec<String>| {
            keys.into_iter().map(DragItem::text).collect()
        }),
        get_allowed_drop_operations: Some(Callback::new(|()| AllowedDropOperations::MOVE)),
        preview: None,
        collection_ref: collection_element,
        on_drag_start: Some(Callback::new(move |e: DraggableCollectionStartEvent| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!("DragStart: {:?}", e.keys)));
            });
        })),
        on_drag_move: None,
        on_drag_end: Some(Callback::new(move |e: DraggableCollectionEndEvent| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!("DragEnd: {:?}", e.drop_effect)));
            });
        })),
    });

    // Droppable collection state with reorder callback.
    let drop_state = use_droppable_collection_state(DroppableCollectionStateInput {
        collection_keys: keys,
        on_reorder: Some(Callback::new(move |e: CollectionReorderEvent| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!(
                    "Reorder: {:?} -> {} ({:?})",
                    e.keys, &e.target.key, e.target.position
                )));
            });
            set_items.update(|items| {
                reorder_items(items, &e.keys, &e.target);
            });
        })),
        ..Default::default()
    });

    // Collection container with hit-testing delegate and keyboard navigation.
    let collection = use_droppable_collection(UseDroppableCollectionInput {
        state: drop_state.clone(),
        keyboard_delegate: Box::new(ListKeyboardDelegate::new(
            keys,
            Signal::derive(HashSet::new),
            Orientation::Vertical,
            Signal::derive(|| leptonic::utils::locale::WritingDirection::Ltr),
        )),
        drop_target_delegate: Box::new(NodeRefDropTargetDelegate {
            node_ref: collection_ref,
        }),
        is_disabled: Signal::derive(|| false),
        accepted_types: vec!["text/plain".to_string()],
        collection_ref: collection_element,
    });

    view! {
        <div style="display: flex; gap: 2em; margin: 1em 0;">
            <div style="flex: 1;">
                <p style="margin: 0 0 0.5em 0; font-weight: bold;">"Drag items to reorder:"</p>
                <div
                    {..collection.collection_props.drop_props.into_attrs()}
                    node_ref=collection_ref
                    style="display: flex; flex-direction: column;"
                >
                    {move || {
                        let current_items = items.get();
                        let last_idx = current_items.len().saturating_sub(1);
                        current_items.into_iter().enumerate().map(|(idx, (key, label))| {
                            let is_last = idx == last_idx;
                            view! {
                                <DropIndicatorLine
                                    target=DropTarget::Item { key: key.clone(), position: DropPosition::Before }
                                    state=drop_state.clone()
                                    keys=keys
                                />
                                <ReorderItem
                                    key=key.clone()
                                    label=label
                                    drag_state=drag_state.clone()
                                    drop_state=drop_state.clone()
                                />
                                {is_last.then(|| view! {
                                    <DropIndicatorLine
                                        target=DropTarget::Item { key, position: DropPosition::After }
                                        state=drop_state.clone()
                                        keys=keys
                                    />
                                })}
                            }
                        }).collect::<Vec<_>>()
                    }}
                </div>
            </div>
            <div style="flex: 1;">
                <p style="margin: 0 0 0.5em 0; font-weight: bold;">"Events:"</p>
                <pre style="
                    width: 100%;
                    height: 10em;
                    overflow: auto;
                    padding: var(--typography-code-padding);
                    border: none;
                    border-radius: var(--typography-code-border-radius);
                    background-color: var(--typography-code-background-color);
                    color: var(--typography-code-color);
                ">
                    { move || string.get() }
                </pre>
            </div>
        </div>
    }
}
