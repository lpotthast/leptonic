#![cfg_attr(feature = "ssr", allow(dead_code, unused_imports))]

use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    oco::Oco,
    prelude::*,
};
use uuid::Uuid;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::DragEvent;

use crate::{
    hooks::{
        AllowedDropOperations, DragItem, DragTypes, DropEffect, IntoAttrs, read_from_data_transfer,
    },
    utils::{
        EventHandler,
        aria::AriaDescribedby,
        dom_ext::{EventAccessors, node_contains},
        element_capture::{CapturedElement, ElementCaptureAttr},
        platform::device,
        use_description::use_description,
    },
};
// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/dnd/src/useDrop.ts

//
// ## ACCESSIBILITY
// - useVirtualDrop description uses a single keyboard-focused string
//   ("Press Enter to drop") since DragManager only supports keyboard.
//   React-aria varies text by modality (keyboard/touch/virtual).
//
// ## ELEMENT ACCESS
// - Uses `CapturedElement` (spread attribute that captures the DOM element
//   during view build/hydrate) instead of React refs for DragManager
//   registration. React-aria uses `useRef` + `useEffect` with the ref's
//   `.current`. In Leptos SSR+hydration, `document.getElementById` cannot
//   be used because UUID-based IDs generated at render time differ between
//   server and client, and Leptos does not re-apply static attributes
//   during hydration.
//
// ## OMITTED
// - `hasDropButton` / `dropButtonProps` (explicit focusable drop affordance)
// - `globalAllowedDropOperations` intersection (WebKit/Chrome Android
//   workaround requiring coordination with use_draggable)
// - Chrome Android global drop effect tracking (dropEffect always "none"
//   in onDragEnd on Android)
//

/// Timeout in milliseconds before `on_drop_activate` fires.
#[cfg(not(feature = "ssr"))]
const DROP_ACTIVATE_TIMEOUT_MS: i32 = 800;

/// ARIA description for keyboard drop targets.
const DROP_DESCRIPTION: &str = "Press Enter to drop";

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

    /// Callback when a drag hovers over the drop zone for 800ms.
    /// Used for activation behaviors like expanding tree nodes or opening folders.
    pub on_drop_activate: Option<Callback<DropActivateEvent>>,

    /// Position-aware drop operation callback. When provided, this takes
    /// precedence over `get_drop_operation` during dragover events.
    /// Used by `use_droppable_collection` to route through hit-testing delegates.
    pub get_drop_operation_for_point: Option<Callback<DropOperationForPointEvent, DropEffect>>,
}

/// Event for position-aware drop operation determination.
#[derive(Debug, Clone)]
pub struct DropOperationForPointEvent {
    /// The types of data available.
    pub types: DragTypes,
    /// The allowed drop operations from the drag source.
    pub allowed_operations: AllowedDropOperations,
    /// The x coordinate (client-relative).
    pub x: f64,
    /// The y coordinate (client-relative).
    pub y: f64,
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
            on_drop_activate: None,
            get_drop_operation_for_point: None,
        }
    }
}

/// Event for determining drop operation.
#[derive(Debug, Clone)]
pub struct DropOperationEvent {
    /// The types of data available.
    pub types: DragTypes,
    /// The allowed drop operations from the drag source.
    pub allowed_operations: AllowedDropOperations,
}

/// Event fired when drag enters a drop zone.
#[derive(Debug, Clone)]
pub struct DropEnterEvent {
    /// The types of data available.
    pub types: DragTypes,
    /// The x coordinate relative to the drop zone element.
    pub x: f64,
    /// The y coordinate relative to the drop zone element.
    pub y: f64,
}

/// Event fired when drag moves over a drop zone.
#[derive(Debug, Clone)]
pub struct DropMoveEvent {
    /// The types of data available.
    pub types: DragTypes,
    /// The x coordinate relative to the drop zone element.
    pub x: f64,
    /// The y coordinate relative to the drop zone element.
    pub y: f64,
}

/// Event fired when drag exits a drop zone.
#[derive(Debug, Clone)]
pub struct DropExitEvent {
    /// The types of data available.
    pub types: DragTypes,
    /// The x coordinate relative to the drop zone element.
    pub x: f64,
    /// The y coordinate relative to the drop zone element.
    pub y: f64,
}

/// Event fired when an item is dropped.
#[derive(Debug, Clone)]
pub struct DropEvent {
    /// The dropped items.
    pub items: Vec<DragItem>,
    /// The drop effect that was performed.
    pub drop_effect: DropEffect,
    /// The x coordinate relative to the drop zone element.
    pub x: f64,
    /// The y coordinate relative to the drop zone element.
    pub y: f64,
}

/// Event fired when a drag hovers over a drop zone for 800ms.
#[derive(Debug, Clone)]
pub struct DropActivateEvent {
    /// The x coordinate relative to the drop zone element.
    pub x: f64,
    /// The y coordinate relative to the drop zone element.
    pub y: f64,
}

/// The return value of the `use_droppable` hook.
pub struct UseDroppableReturn {
    /// Props for the droppable element.
    pub drop_props: UseDroppableProps,

    /// The ID of the droppable element.
    pub droppable_id: String,

    /// Whether a drag is currently over this drop zone.
    pub is_drop_target: Signal<bool>,
}

/// Props from `use_droppable` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseDroppableProps {
    pub id: String,
    pub aria_describedby: Signal<AriaDescribedby>,
    pub element_capture: ElementCaptureAttr,
    pub on_dragenter: EventHandler<DragEvent>,
    pub on_dragover: EventHandler<DragEvent>,
    pub on_dragleave: EventHandler<DragEvent>,
    pub on_drop: EventHandler<DragEvent>,
}

impl IntoAttrs for UseDroppableProps {
    type Attrs = UseDroppableAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            self.element_capture,
            self.on_dragenter.into_on(ev::dragenter),
            self.on_dragover.into_on(ev::dragover),
            self.on_dragleave.into_on(ev::dragleave),
            self.on_drop.into_on(ev::drop),
        )
    }
}

/// Attributes for the droppable element.
pub type UseDroppableAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::AriaDescribedby, Signal<AriaDescribedby>>,
    ElementCaptureAttr,
    On<ev::dragenter, SharedEventCallback<DragEvent>>,
    On<ev::dragover, SharedEventCallback<DragEvent>>,
    On<ev::dragleave, SharedEventCallback<DragEvent>>,
    On<ev::drop, SharedEventCallback<DragEvent>>,
);

/// Compute element-relative coordinates from a drag event.
fn element_relative_coords(e: &DragEvent) -> (f64, f64) {
    let (cx, cy) = (e.client_x(), e.client_y());
    if let Some(ct) = e.current_target() {
        if let Some(el) = ct.dyn_ref::<web_sys::HtmlElement>() {
            let rect = el.get_bounding_client_rect();
            return (cx - rect.x(), cy - rect.y());
        }
    }
    (cx, cy)
}

/// Compute allowed drop operations, taking platform-specific modifier keys into account.
fn get_allowed_operations(e: &DragEvent) -> AllowedDropOperations {
    let base = e.data_transfer().map_or(AllowedDropOperations::ALL, |dt| {
        AllowedDropOperations::from_effect_allowed(&dt.effect_allowed())
    });
    let mut modifiers = AllowedDropOperations::NONE;
    if device::is_mac() {
        if e.alt_key() {
            modifiers = modifiers | AllowedDropOperations::COPY;
        }
        if e.ctrl_key() {
            modifiers = modifiers | AllowedDropOperations::LINK;
        }
        if e.meta_key() {
            modifiers = modifiers | AllowedDropOperations::MOVE;
        }
    } else {
        if e.alt_key() {
            modifiers = modifiers | AllowedDropOperations::LINK;
        }
        if e.shift_key() {
            modifiers = modifiers | AllowedDropOperations::MOVE;
        }
        if e.ctrl_key() {
            modifiers = modifiers | AllowedDropOperations::COPY;
        }
    }
    if modifiers == AllowedDropOperations::NONE {
        base
    } else {
        base & modifiers
    }
}

/// Convert a `DropEffect` to its `DataTransfer.dropEffect` string.
fn drop_effect_str(effect: DropEffect) -> &'static str {
    match effect {
        DropEffect::Move => "move",
        DropEffect::Link => "link",
        DropEffect::None => "none",
        DropEffect::Copy | DropEffect::All => "copy",
    }
}

/// Provides the behavior and accessibility for a drop target.
///
/// Registers with the `DragManager` so this target can participate in
/// keyboard-initiated virtual drag sessions.
///
/// # Example
///
/// ```ignore
/// let droppable = use_droppable(UseDroppableInput {
///     accepted_types: vec!["text/plain".to_string()],
///     on_drop: Some(Callback::new(|e: DropEvent| {
///         for item in e.items {
///             tracing::info!("Dropped: {}", item.data());
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
#[allow(clippy::too_many_lines, clippy::useless_conversion)]
pub fn use_droppable(input: UseDroppableInput) -> UseDroppableReturn {
    cfg_if::cfg_if! {
        if #[cfg(feature = "ssr")] {
            let _ = input;
            let droppable_id = format!("droppable-{}", Uuid::new_v4());
            let (is_drop_target, _) = signal(false);
            let captured_element = CapturedElement::new();
            UseDroppableReturn {
                drop_props: UseDroppableProps {
                    id: droppable_id.clone(),
                    aria_describedby: Signal::stored(AriaDescribedby::none()),
                    element_capture: captured_element.attr(),
                    on_dragenter: EventHandler::new(|_: DragEvent| {}),
                    on_dragover: EventHandler::new(|_: DragEvent| {}),
                    on_dragleave: EventHandler::new(|_: DragEvent| {}),
                    on_drop: EventHandler::new(|_: DragEvent| {}),
                },
                droppable_id,
                is_drop_target: is_drop_target.into(),
            }
        } else {
            let UseDroppableInput {
                is_disabled: disabled,
                accepted_types,
                get_drop_operation,
                on_drop_enter,
                on_drop_move,
                on_drop_exit,
                on_drop,
                on_drop_activate,
                get_drop_operation_for_point,
            } = input;

    let _ = &on_drop_activate;

    let droppable_id = format!("droppable-{}", Uuid::new_v4());
    let captured_element = CapturedElement::new();

    let (is_drop_target, set_is_drop_target) = signal(false);

    // --- ARIA description (replaces deprecated aria-dropeffect) ---
    let description = use_description(Oco::Borrowed(DROP_DESCRIPTION));
    let no_description = AriaDescribedby::none();
    let aria_describedby = Signal::derive(move || {
        if is_drop_target.get() {
            description.clone()
        } else {
            no_description.clone()
        }
    });

    // --- Element set for tracking drag enter/leave (replaces i32 counter) ---
    // Uses Vec<JsValue> since web_sys::Element doesn't implement Hash/Eq.
    // Empty Vec is SSR-safe (no JS calls at creation).
    let drag_over_elements: StoredValue<Vec<JsValue>, LocalStorage> =
        StoredValue::new_local(Vec::new());

    // --- Dragover deduplication state ---
    // Stores (x, y, allowed_ops, drop_effect_str) from last dragover.
    // None = no dragover has occurred yet.
    let last_dedup: StoredValue<
        Option<(f64, f64, AllowedDropOperations, &'static str)>,
        LocalStorage,
    > = StoredValue::new_local(None);

    // --- Drop activate timer ---
    // Stores (timeout_handle, closure) to keep the Closure alive and allow cancellation.
    #[cfg(not(feature = "ssr"))]
    let activate_timer: StoredValue<
        Option<(i32, wasm_bindgen::closure::Closure<dyn FnMut()>)>,
        LocalStorage,
    > = StoredValue::new_local(None);

    let clear_activate_timer = move || {
        #[cfg(not(feature = "ssr"))]
        activate_timer.update_value(|timer| {
            if let Some((id, _closure)) = timer.take() {
                if let Some(window) = web_sys::window() {
                    window.clear_timeout_with_handle(id);
                }
            }
        });
    };

    #[cfg(not(feature = "ssr"))]
    let accepted_types_for_dm = accepted_types.clone();
    let accepted_types_clone = accepted_types.clone();
    let is_type_accepted = move |types: &DragTypes| -> bool {
        if accepted_types_clone.is_empty() {
            return true;
        }
        match types {
            DragTypes::UnknownFiles => true,
            DragTypes::Known(known) => known.iter().any(|t| accepted_types_clone.contains(t)),
        }
    };

    let get_drag_types_from_event = |e: &DragEvent| -> DragTypes {
        #[cfg(not(feature = "ssr"))]
        {
            DragTypes::from_drag_event(e)
        }
        #[cfg(feature = "ssr")]
        {
            let _ = e;
            DragTypes::Known(std::collections::HashSet::new())
        }
    };

    let accepted_types_for_enter = accepted_types.clone();
    let handle_drag_enter = move |e: DragEvent| {
        e.prevent_default();
        e.stop_propagation();

        if disabled.get_untracked() {
            return;
        }

        let types = get_drag_types_from_event(&e);

        // Check if any type is accepted
        let accepted = if accepted_types_for_enter.is_empty() {
            true
        } else {
            match &types {
                DragTypes::UnknownFiles => true,
                DragTypes::Known(known) => {
                    known.iter().any(|t| accepted_types_for_enter.contains(t))
                }
            }
        };

        if !accepted {
            return;
        }

        let target_el: JsValue = e.expect_target().into();
        let became_non_empty = std::cell::Cell::new(false);
        drag_over_elements.update_value(|elements| {
            if !elements.contains(&target_el) {
                became_non_empty.set(elements.is_empty());
                elements.push(target_el.clone());
            }
        });

        if !became_non_empty.get() {
            return;
        }

        set_is_drop_target.set(true);

        if let Some(on_enter) = on_drop_enter {
            let (rel_x, rel_y) = element_relative_coords(&e);
            on_enter.run(DropEnterEvent {
                types,
                x: rel_x,
                y: rel_y,
            });
        }
    };

    let accepted_types_for_over = accepted_types.clone();
    let handle_drag_over = move |e: DragEvent| {
        e.prevent_default();
        e.stop_propagation();

        if disabled.get_untracked() {
            return;
        }

        let types = get_drag_types_from_event(&e);

        // Check if any type is accepted
        let accepted = if accepted_types_for_over.is_empty() {
            true
        } else {
            match &types {
                DragTypes::UnknownFiles => true,
                DragTypes::Known(known) => {
                    known.iter().any(|t| accepted_types_for_over.contains(t))
                }
            }
        };

        if !accepted {
            return;
        }

        // Compute allowed operations with modifier keys
        let allowed = get_allowed_operations(&e);

        // Element-relative coordinates (computed early for point-aware callback)
        let client_x = f64::from(e.client_x());
        let client_y = f64::from(e.client_y());

        // Determine drop effect: position-aware callback takes precedence
        let drop_effect = if let Some(get_op_for_point) = get_drop_operation_for_point {
            get_op_for_point.run(DropOperationForPointEvent {
                types: types.clone(),
                allowed_operations: allowed,
                x: client_x,
                y: client_y,
            })
        } else if let Some(get_op) = get_drop_operation {
            get_op.run(DropOperationEvent {
                types: types.clone(),
                allowed_operations: allowed,
            })
        } else {
            DropEffect::Copy
        };

        let de_str = drop_effect_str(drop_effect);

        // Always set dropEffect on DataTransfer (browser needs it even if dedup).
        if let Some(dt) = e.data_transfer() {
            dt.set_drop_effect(de_str);
        }

        // Element-relative coordinates
        let (rel_x, rel_y) = element_relative_coords(&e);

        // Deduplication: skip if nothing changed since last dragover.
        let prev = last_dedup.get_value();
        #[allow(clippy::float_cmp)]
        let is_dup = prev.is_some_and(|(px, py, pa, pe)| {
            px == rel_x && py == rel_y && pa == allowed && pe == de_str
        });

        if is_dup {
            return;
        }

        let prev_drop_effect = prev.map(|(_, _, _, pe)| pe);
        last_dedup.set_value(Some((rel_x, rel_y, allowed, de_str)));

        // Drop operation change detection: fire enter/exit when switching
        // between valid and "none".
        if let Some(prev_de) = prev_drop_effect {
            if prev_de == "none" && de_str != "none" {
                // Became valid — fire enter.
                set_is_drop_target.set(true);
                if let Some(on_enter) = on_drop_enter {
                    on_enter.run(DropEnterEvent {
                        types: types.clone(),
                        x: rel_x,
                        y: rel_y,
                    });
                }
            } else if prev_de != "none" && de_str == "none" {
                // Became invalid — fire exit.
                set_is_drop_target.set(false);
                if let Some(on_exit) = on_drop_exit {
                    on_exit.run(DropExitEvent {
                        types: types.clone(),
                        x: rel_x,
                        y: rel_y,
                    });
                }
            }
        }

        if let Some(on_move) = on_drop_move {
            on_move.run(DropMoveEvent {
                types,
                x: rel_x,
                y: rel_y,
            });
        }

        // Drop activate timer: reset on every non-dedup dragover.
        clear_activate_timer();
        #[cfg(not(feature = "ssr"))]
        if let Some(on_activate) = on_drop_activate {
            if de_str != "none" {
                let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                    on_activate.run(DropActivateEvent { x: rel_x, y: rel_y });
                })
                    as Box<dyn FnMut()>);

                if let Some(window) = web_sys::window() {
                    if let Ok(id) = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                        closure.as_ref().unchecked_ref(),
                        DROP_ACTIVATE_TIMEOUT_MS,
                    ) {
                        activate_timer.set_value(Some((id, closure)));
                    }
                }
            }
        }
    };

    let handle_drag_leave = move |e: DragEvent| {
        e.prevent_default();
        e.stop_propagation();

        if disabled.get_untracked() {
            return;
        }

        let target_el: JsValue = e.expect_target().into();
        let current_target = e.current_target();

        let became_empty = std::cell::Cell::new(false);
        drag_over_elements.update_value(|elements| {
            elements.retain(|el| *el != target_el);

            // Clean up elements no longer in the DOM.
            if let Some(ref ct) = current_target {
                let ct_node: &web_sys::Node = ct.unchecked_ref();
                elements.retain(|el| {
                    node_contains(Some(ct_node), el.dyn_ref::<web_sys::Node>()).unwrap_or(false)
                });
            }

            became_empty.set(elements.is_empty());
        });

        if !became_empty.get() {
            return;
        }

        set_is_drop_target.set(false);
        last_dedup.set_value(None);
        clear_activate_timer();

        if let Some(on_exit) = on_drop_exit {
            let types = get_drag_types_from_event(&e);
            let (rel_x, rel_y) = element_relative_coords(&e);
            on_exit.run(DropExitEvent {
                types,
                x: rel_x,
                y: rel_y,
            });
        }
    };

    let handle_drop = move |e: DragEvent| {
        e.prevent_default();
        e.stop_propagation();

        if disabled.get_untracked() {
            return;
        }

        let types = get_drag_types_from_event(&e);

        // Check if any type is accepted
        let accepted = if accepted_types.is_empty() {
            true
        } else {
            is_type_accepted(&types)
        };

        if !accepted {
            return;
        }

        set_is_drop_target.set(false);
        drag_over_elements.update_value(Vec::clear);
        last_dedup.set_value(None);
        clear_activate_timer();

        let (rel_x, rel_y) = element_relative_coords(&e);

        let drop_effect =
            e.data_transfer()
                .map_or(DropEffect::None, |dt| match dt.drop_effect().as_str() {
                    "copy" => DropEffect::Copy,
                    "move" => DropEffect::Move,
                    "link" => DropEffect::Link,
                    _ => DropEffect::None,
                });

        if let Some(on_drop) = on_drop {
            let items = e
                .data_transfer()
                .map(|dt| read_from_data_transfer(&dt))
                .unwrap_or_default();
            on_drop.run(DropEvent {
                items,
                drop_effect,
                x: rel_x,
                y: rel_y,
            });
        }

        // Fire on_drop_exit after on_drop, matching react-aria behavior.
        if let Some(on_exit) = on_drop_exit {
            on_exit.run(DropExitEvent {
                types,
                x: rel_x,
                y: rel_y,
            });
        }
    };

    // --- Register with DragManager for virtual drag support ---
    #[cfg(not(feature = "ssr"))]
    {
        let dm_id = droppable_id.clone();
        let dm_accepted_types = accepted_types_for_dm;
        let dm_get_drop_operation = get_drop_operation;
        let dm_on_drop_enter = on_drop_enter;
        let dm_on_drop_exit = on_drop_exit;
        let dm_on_drop = on_drop;
        let dm_on_drop_activate = on_drop_activate;

        Effect::new(move || {
            if disabled.get() {
                return;
            }
            let Some(el) = captured_element.get() else {
                return;
            };
            let element: web_sys::Element = (*el).clone();
            // Ensure DOM id matches our generated id for consistency.
            element.set_id(&dm_id);

            super::drag_manager::register_drop_target(super::drag_manager::RegisteredDropTarget {
                id: dm_id.clone(),
                element,
                accepted_types: dm_accepted_types.clone(),
                get_drop_operation: dm_get_drop_operation,
                on_drop_enter: dm_on_drop_enter,
                on_drop_exit: dm_on_drop_exit,
                on_drop: dm_on_drop,
                on_drop_activate: dm_on_drop_activate,
                on_key_down: None,
                prevent_focus_on_drop: false,
                activate_button: None,
            });

            // Unregister on cleanup. Capture just the String ID (Send+Sync).
            let cleanup_id = dm_id.clone();
            on_cleanup(move || {
                super::drag_manager::unregister_drop_target(&cleanup_id);
            });
        });
    }

    // Cleanup activate timer on unmount.
    on_cleanup(move || {
        clear_activate_timer();
    });

            UseDroppableReturn {
                drop_props: UseDroppableProps {
                    id: droppable_id.clone(),
                    aria_describedby,
                    element_capture: captured_element.attr(),
                    on_dragenter: EventHandler::new(handle_drag_enter),
                    on_dragover: EventHandler::new(handle_drag_over),
                    on_dragleave: EventHandler::new(handle_drag_leave),
                    on_drop: EventHandler::new(handle_drop),
                },
                droppable_id,
                is_drop_target: is_drop_target.into(),
            }
        }
    }
}
