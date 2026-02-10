use crate::hooks::{DragItem, DropEffect};
use leptos::attr;
use leptos::attr::{Attr, Attribute};
use leptos::ev;
use leptos::ev::{on, On, SharedEventCallback};
use leptos::prelude::*;
use uuid::Uuid;
use web_sys::DragEvent;
// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/dnd/src/useDrop.ts

/// Input parameters for the `use_droppable` hook.
#[derive(Clone)]
pub struct UseDroppableInput {
    /// Whether the drop zone is disabled.
    pub is_disabled: Signal<bool>,

    /// The acceptable MIME types.
    pub accepted_types: Vec<String>,

    /// Callback to determine if a drag is acceptable.
    pub get_drop_operation: Option<Callback<DropOperationEvent, DropEffect>>,

    /// Callback when drag enters the drop zone.
    pub on_drop_enter: Option<Callback<DropEnterEvent>>,

    /// Callback when drag is over the drop zone.
    pub on_drop_move: Option<Callback<DropMoveEvent>>,

    /// Callback when drag exits the drop zone.
    pub on_drop_exit: Option<Callback<DropExitEvent>>,

    /// Callback when an item is dropped.
    pub on_drop: Option<Callback<DropEvent>>,
}

impl Default for UseDroppableInput {
    fn default() -> Self {
        Self {
            is_disabled: Signal::derive(|| false),
            accepted_types: vec![],
            get_drop_operation: None,
            on_drop_enter: None,
            on_drop_move: None,
            on_drop_exit: None,
            on_drop: None,
        }
    }
}

/// Event for determining drop operation.
#[derive(Debug, Clone)]
pub struct DropOperationEvent {
    /// The types of data available.
    pub types: Vec<String>,
    /// The allowed drop effects.
    pub allowed_operations: DropEffect,
}

/// Event fired when drag enters a drop zone.
#[derive(Debug, Clone)]
pub struct DropEnterEvent {
    /// The types of data available.
    pub types: Vec<String>,
    /// The x coordinate.
    pub x: i32,
    /// The y coordinate.
    pub y: i32,
}

/// Event fired when drag moves over a drop zone.
#[derive(Debug, Clone)]
pub struct DropMoveEvent {
    /// The types of data available.
    pub types: Vec<String>,
    /// The x coordinate.
    pub x: i32,
    /// The y coordinate.
    pub y: i32,
}

/// Event fired when drag exits a drop zone.
#[derive(Debug, Clone)]
pub struct DropExitEvent {
    /// The types of data available.
    pub types: Vec<String>,
}

/// Event fired when an item is dropped.
#[derive(Debug, Clone)]
pub struct DropEvent {
    /// The dropped items.
    pub items: Vec<DragItem>,
    /// The drop effect that was performed.
    pub drop_effect: DropEffect,
    /// The x coordinate.
    pub x: i32,
    /// The y coordinate.
    pub y: i32,
}

/// The return value of the `use_droppable` hook.
pub struct UseDroppableReturn {
    /// Props for the droppable element.
    pub drop_props: UseDroppableAttrs,

    /// The ID of the droppable element.
    pub droppable_id: String,

    /// Whether a drag is currently over this drop zone.
    pub is_drop_target: Signal<bool>,
}

/// Attributes for the droppable element.
pub type UseDroppableAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaDropeffect, Signal<&'static str>>,
    On<ev::dragenter, SharedEventCallback<DragEvent>>,
    On<ev::dragover, SharedEventCallback<DragEvent>>,
    On<ev::dragleave, SharedEventCallback<DragEvent>>,
    On<ev::drop, SharedEventCallback<DragEvent>>,
);

/// Provides the behavior and accessibility for a drop target.
///
/// # Example
///
/// ```ignore
/// let droppable = use_droppable(UseDroppableInput {
///     accepted_types: vec!["text/plain".to_string()],
///     on_drop: Some(Callback::new(|e: DropEvent| {
///         for item in e.items {
///             tracing::info!("Dropped: {}", item.data);
///         }
///     })),
///     ..Default::default()
/// });
///
/// view! {
///     <div
///         {..droppable.drop_props}
///         class:drop-target=move || droppable.is_drop_target.get()
///     >
///         "Drop here"
///     </div>
/// }
/// ```
#[allow(clippy::too_many_lines)]
pub fn use_droppable(input: UseDroppableInput) -> UseDroppableReturn {
    let UseDroppableInput {
        is_disabled: disabled,
        accepted_types,
        get_drop_operation,
        on_drop_enter,
        on_drop_move,
        on_drop_exit,
        on_drop,
    } = input;

    let droppable_id = format!("droppable-{}", Uuid::new_v4());

    let (is_drop_target, set_is_drop_target) = signal(false);
    let (drag_counter, set_drag_counter) = signal(0i32);

    let aria_dropeffect = Signal::derive(move || {
        if disabled.get() {
            "none"
        } else if is_drop_target.get() {
            "copy"
        } else {
            "move"
        }
    });

    let accepted_types_clone = accepted_types.clone();
    let is_type_accepted = move |types: &[String]| -> bool {
        if accepted_types_clone.is_empty() {
            return true;
        }
        for t in types {
            if accepted_types_clone.contains(t) {
                return true;
            }
        }
        false
    };

    let get_types_from_event = |e: &DragEvent| -> Vec<String> {
        e.data_transfer()
            .map(|dt| {
                let types = dt.types();
                let mut result = Vec::new();
                for i in 0..types.length() {
                    let t = types.get(i);
                    if let Some(s) = t.as_string() {
                        result.push(s);
                    }
                }
                result
            })
            .unwrap_or_default()
    };

    let get_items_from_event = |e: &DragEvent| -> Vec<DragItem> {
        e.data_transfer()
            .map(|dt| {
                let types = dt.types();
                let mut items = Vec::new();
                for i in 0..types.length() {
                    let t = types.get(i);
                    if let Some(kind) = t.as_string() {
                        if let Ok(data) = dt.get_data(&kind) {
                            items.push(DragItem { kind, data });
                        }
                    }
                }
                items
            })
            .unwrap_or_default()
    };

    let accepted_types_for_enter = accepted_types.clone();
    let handle_drag_enter = move |e: DragEvent| {
        if disabled.get_untracked() {
            return;
        }

        let types = get_types_from_event(&e);

        // Check if any type is accepted
        let accepted = if accepted_types_for_enter.is_empty() {
            true
        } else {
            types.iter().any(|t| accepted_types_for_enter.contains(t))
        };

        if !accepted {
            return;
        }

        e.prevent_default();

        let new_count = drag_counter.get_untracked() + 1;
        set_drag_counter.set(new_count);

        if new_count == 1 {
            set_is_drop_target.set(true);

            if let Some(on_enter) = on_drop_enter {
                on_enter.run(DropEnterEvent {
                    types,
                    x: e.client_x(),
                    y: e.client_y(),
                });
            }
        }
    };

    let accepted_types_for_over = accepted_types.clone();
    let handle_drag_over = move |e: DragEvent| {
        if disabled.get_untracked() {
            return;
        }

        let types = get_types_from_event(&e);

        // Check if any type is accepted
        let accepted = if accepted_types_for_over.is_empty() {
            true
        } else {
            types.iter().any(|t| accepted_types_for_over.contains(t))
        };

        if !accepted {
            return;
        }

        e.prevent_default();

        // Determine drop effect
        if let Some(dt) = e.data_transfer() {
            let drop_effect = if let Some(get_op) = get_drop_operation {
                let allowed = match dt.effect_allowed().as_str() {
                    "copy" => DropEffect::Copy,
                    "move" => DropEffect::Move,
                    "link" => DropEffect::Link,
                    _ => DropEffect::All,
                };

                get_op.run(DropOperationEvent {
                    types: types.clone(),
                    allowed_operations: allowed,
                })
            } else {
                DropEffect::Copy
            };

            dt.set_drop_effect(match drop_effect {
                DropEffect::Move => "move",
                DropEffect::Link => "link",
                DropEffect::None => "none",
                DropEffect::Copy | DropEffect::All => "copy",
            });
        }

        if let Some(on_move) = on_drop_move {
            on_move.run(DropMoveEvent {
                types,
                x: e.client_x(),
                y: e.client_y(),
            });
        }
    };

    let handle_drag_leave = move |e: DragEvent| {
        if disabled.get_untracked() {
            return;
        }

        let new_count = drag_counter.get_untracked() - 1;
        set_drag_counter.set(new_count);

        if new_count == 0 {
            set_is_drop_target.set(false);

            if let Some(on_exit) = on_drop_exit {
                let types = get_types_from_event(&e);
                on_exit.run(DropExitEvent { types });
            }
        }
    };

    let handle_drop = move |e: DragEvent| {
        if disabled.get_untracked() {
            return;
        }

        let types = get_types_from_event(&e);

        // Check if any type is accepted
        let accepted = if accepted_types.is_empty() {
            true
        } else {
            is_type_accepted(&types)
        };

        if !accepted {
            return;
        }

        e.prevent_default();
        set_is_drop_target.set(false);
        set_drag_counter.set(0);

        let drop_effect =
            e.data_transfer()
                .map_or(DropEffect::None, |dt| match dt.drop_effect().as_str() {
                    "copy" => DropEffect::Copy,
                    "move" => DropEffect::Move,
                    "link" => DropEffect::Link,
                    _ => DropEffect::None,
                });

        if let Some(on_drop) = on_drop {
            let items = get_items_from_event(&e);
            on_drop.run(DropEvent {
                items,
                drop_effect,
                x: e.client_x(),
                y: e.client_y(),
            });
        }
    };

    UseDroppableReturn {
        drop_props: (
            Attr(attr::Id, droppable_id.clone()),
            Attr(attr::Role, "button"),
            Attr(attr::AriaDropeffect, aria_dropeffect),
            on(ev::dragenter, handle_drag_enter).into_cloneable(),
            on(ev::dragover, handle_drag_over).into_cloneable(),
            on(ev::dragleave, handle_drag_leave).into_cloneable(),
            on(ev::drop, handle_drop).into_cloneable(),
        ),
        droppable_id,
        is_drop_target: is_drop_target.into(),
    }
}
