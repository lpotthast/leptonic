// Upstream: react-aria/src/dnd/useDrop.ts @ 99e6102368
use std::rc::Rc;

use leptos::{
    attr::{self, Attr},
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use web_sys::DragEvent;

use super::{
    drag_manager::{self, DropTargetOptions},
    types::{
        DragTypes, DropActivateEvent, DropEnterEvent, DropEvent, DropExitEvent, DropMoveEvent,
        DropOperation, DropOperations,
    },
    use_virtual_drop::use_virtual_drop,
    utils::{
        global_allowed_drop_operations, read_from_data_transfer, restore_dnd_state,
        set_global_drop_effect, snapshot_dnd_state,
    },
};
use crate::{
    hooks::{IntoAttrs, UseButtonInput},
    utils::{
        CapturedElement, EventAccessors, EventHandler, node_contains,
        platform::device::{is_ipad, is_mac},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The drop target element is captured by the returned props (`element`).
// - The drop button comes back as a `UseButtonInput` for `use_button`.
// - `get_drop_operation` and `get_drop_operation_for_point` take a query struct.
//
// =============================================================================

/// How long a drag hovers a drop target before `on_drop_activate`.
const DROP_ACTIVATE_TIMEOUT_MS: u64 = 800;

/// The data a drop target decides the drop operation from.
#[derive(Debug, Clone)]
pub struct DropOperationQuery {
    pub types: DragTypes,
    /// In order of preference.
    pub allowed_operations: Vec<DropOperation>,
}

/// [`DropOperationQuery`] at a point in the drop target.
#[derive(Debug, Clone)]
pub struct DropOperationPointQuery {
    pub types: DragTypes,
    pub allowed_operations: Vec<DropOperation>,
    /// Relative to the drop target.
    pub x: f64,
    pub y: f64,
}

/// Input of [`use_drop`].
#[derive(Clone)]
pub struct UseDropInput {
    /// The drop target element; the hook's props capture it.
    pub element: CapturedElement,
    /// The drop operation for dragged data. Defaults to the first allowed operation.
    pub get_drop_operation: Option<Callback<DropOperationQuery, DropOperation>>,
    /// The drop operation at a point (native drags).
    pub get_drop_operation_for_point: Option<Callback<DropOperationPointQuery, DropOperation>>,
    pub on_drop_enter: Option<Callback<DropEnterEvent>>,
    pub on_drop_move: Option<Callback<DropMoveEvent>>,
    pub on_drop_activate: Option<Callback<DropActivateEvent>>,
    pub on_drop_exit: Option<Callback<DropExitEvent>>,
    pub on_drop: Option<Callback<DropEvent>>,
    /// Keyboard and screen reader drops go to a separate drop button (`drop_button`).
    pub has_drop_button: bool,
    pub is_disabled: Signal<bool>,
}

/// Return value of [`use_drop`].
pub struct UseDropReturn {
    pub drop_props: UseDropProps,
    /// For `use_button`, with `has_drop_button`.
    pub drop_button: UseButtonInput,
    pub is_drop_target: Signal<bool>,
}

/// Props for the drop target element.
#[derive(Debug)]
pub struct UseDropProps {
    /// How to drop during keyboard drags (without a drop button).
    pub aria_describedby: Signal<Option<String>>,
    pub element: CapturedElement,
    pub on_dragenter: EventHandler<DragEvent>,
    pub on_dragover: EventHandler<DragEvent>,
    pub on_dragleave: EventHandler<DragEvent>,
    pub on_drop: EventHandler<DragEvent>,
}

pub type UseDropAttrs = (
    Attr<attr::AriaDescribedby, Signal<Option<String>>>,
    crate::utils::ElementCaptureAttr,
    On<ev::dragenter, SharedEventCallback<DragEvent>>,
    On<ev::dragover, SharedEventCallback<DragEvent>>,
    On<ev::dragleave, SharedEventCallback<DragEvent>>,
    On<ev::drop, SharedEventCallback<DragEvent>>,
);

impl IntoAttrs for UseDropProps {
    type Attrs = UseDropAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::AriaDescribedby, self.aria_describedby),
            self.element.attr(),
            self.on_dragenter.into_on(ev::dragenter),
            self.on_dragover.into_on(ev::dragover),
            self.on_dragleave.into_on(ev::dragleave),
            self.on_drop.into_on(ev::drop),
        )
    }
}

#[derive(Default)]
struct DropState {
    x: f64,
    y: f64,
    drag_over_elements: Vec<SendWrapper<web_sys::Element>>,
    drop_effect: &'static str,
    allowed_operations: DropOperations,
    drop_activate_timer: Option<leptos::prelude::TimeoutHandle>,
}

fn relative(e: &DragEvent, x: f64, y: f64) -> (f64, f64) {
    e.expect_current_target()
        .dyn_into::<web_sys::Element>()
        .ok()
        .map_or((x, y), |el| {
            let rect = el.get_bounding_client_rect();
            (x - rect.x(), y - rect.y())
        })
}

/// The operations a native drag allows: those of its source, restricted by modifier keys.
fn allowed_operations(e: &DragEvent) -> DropOperations {
    let mut allowed = e.data_transfer().map_or(DropOperations::ALL, |dt| {
        DropOperations::from_effect_allowed(&dt.effect_allowed())
    });
    let global = global_allowed_drop_operations();
    if global != DropOperations::NONE {
        allowed.0 &= global.0;
    }
    let mut modifiers = 0;
    if is_mac() {
        if e.alt_key() {
            modifiers |= DropOperations::COPY;
        }
        if e.ctrl_key() && !is_ipad() {
            modifiers |= DropOperations::LINK;
        }
        if e.meta_key() {
            modifiers |= DropOperations::MOVE;
        }
    } else {
        if e.alt_key() {
            modifiers |= DropOperations::LINK;
        }
        if e.shift_key() {
            modifiers |= DropOperations::MOVE;
        }
        if e.ctrl_key() {
            modifiers |= DropOperations::COPY;
        }
    }
    if modifiers != 0 {
        DropOperations(allowed.0 & modifiers)
    } else {
        allowed
    }
}

/// A drop target for native drags and keyboard or screen reader drags.
#[allow(clippy::too_many_lines)]
pub fn use_drop(input: UseDropInput) -> UseDropReturn {
    let UseDropInput {
        element,
        get_drop_operation,
        get_drop_operation_for_point,
        on_drop_enter,
        on_drop_move,
        on_drop_activate,
        on_drop_exit,
        on_drop,
        has_drop_button,
        is_disabled,
    } = input;
    let (is_drop_target, set_drop_target) = signal(false);
    let state: StoredValue<DropState> = StoredValue::new(DropState {
        drop_effect: "none",
        allowed_operations: DropOperations::ALL,
        ..DropState::default()
    });

    let fire_drop_enter = move |e: &DragEvent| {
        set_drop_target.set(true);
        if let Some(on_enter) = on_drop_enter {
            let (x, y) = relative(e, e.client_x(), e.client_y());
            on_enter.run(DropEnterEvent { x, y });
        }
    };
    let fire_drop_exit = move |e: &DragEvent| {
        set_drop_target.set(false);
        if let Some(on_exit) = on_drop_exit {
            let (x, y) = relative(e, e.client_x(), e.client_y());
            on_exit.run(DropExitEvent { x, y });
        }
    };
    let clear_activate_timer = move || {
        state.update_value(|s| {
            if let Some(timer) = s.drop_activate_timer.take() {
                timer.clear();
            }
        });
    };
    let decide = move |e: &DragEvent, allowed: DropOperations| -> DropOperation {
        let types = e
            .data_transfer()
            .map(|dt| DragTypes::from_data_transfer(&dt))
            .unwrap_or_default();
        let mut operation = allowed
            .to_vec()
            .first()
            .copied()
            .unwrap_or(DropOperation::Cancel);
        if let Some(get) = get_drop_operation {
            operation = allowed.restrict(get.run(DropOperationQuery {
                types: types.clone(),
                allowed_operations: allowed.to_vec(),
            }));
        }
        if let Some(get) = get_drop_operation_for_point {
            let (x, y) = relative(e, e.client_x(), e.client_y());
            operation = allowed.restrict(get.run(DropOperationPointQuery {
                types,
                allowed_operations: allowed.to_vec(),
                x,
                y,
            }));
        }
        operation
    };

    let on_dragover = move |e: DragEvent| {
        e.prevent_default();
        e.stop_propagation();
        let allowed = allowed_operations(&e);
        let (x, y) = (e.client_x(), e.client_y());
        let (same, previous) = state.with_value(|s| {
            #[allow(clippy::float_cmp)]
            let same = s.x == x && s.y == y && s.allowed_operations == allowed;
            (same, s.drop_effect)
        });
        if same {
            if let Some(dt) = e.data_transfer() {
                dt.set_drop_effect(previous);
            }
            return;
        }
        state.update_value(|s| {
            s.x = x;
            s.y = y;
        });

        let allowed_changed = state.with_value(|s| s.allowed_operations != allowed);
        if allowed_changed {
            let mut operation = allowed
                .to_vec()
                .first()
                .copied()
                .unwrap_or(DropOperation::Cancel);
            if let Some(get) = get_drop_operation {
                let types = e
                    .data_transfer()
                    .map(|dt| DragTypes::from_data_transfer(&dt))
                    .unwrap_or_default();
                operation = allowed.restrict(get.run(DropOperationQuery {
                    types,
                    allowed_operations: allowed.to_vec(),
                }));
            }
            state.update_value(|s| s.drop_effect = operation.as_drop_effect());
        }
        if let Some(get) = get_drop_operation_for_point {
            let types = e
                .data_transfer()
                .map(|dt| DragTypes::from_data_transfer(&dt))
                .unwrap_or_default();
            let (rel_x, rel_y) = relative(&e, x, y);
            let operation = allowed.restrict(get.run(DropOperationPointQuery {
                types,
                allowed_operations: allowed.to_vec(),
                x: rel_x,
                y: rel_y,
            }));
            state.update_value(|s| s.drop_effect = operation.as_drop_effect());
        }
        let effect = state.with_value(|s| s.drop_effect);
        state.update_value(|s| s.allowed_operations = allowed);
        if let Some(dt) = e.data_transfer() {
            dt.set_drop_effect(effect);
        }
        if effect == "none" && previous != "none" {
            fire_drop_exit(&e);
        } else if effect != "none" && previous == "none" {
            fire_drop_enter(&e);
        }
        if effect != "none"
            && let Some(on_move) = on_drop_move
        {
            let (rel_x, rel_y) = relative(&e, x, y);
            on_move.run(DropMoveEvent { x: rel_x, y: rel_y });
        }
        clear_activate_timer();
        if effect != "none"
            && let Some(on_activate) = on_drop_activate
        {
            let (rel_x, rel_y) = relative(&e, x, y);
            let timer = set_timeout_with_handle(
                move || on_activate.run(DropActivateEvent { x: rel_x, y: rel_y }),
                std::time::Duration::from_millis(DROP_ACTIVATE_TIMEOUT_MS),
            )
            .ok();
            state.update_value(|s| s.drop_activate_timer = timer);
        }
    };

    let on_dragenter = move |e: DragEvent| {
        e.prevent_default();
        e.stop_propagation();
        let Some(target) = e
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        else {
            return;
        };
        let first = state.with_value(|s| s.drag_over_elements.is_empty());
        state.update_value(|s| {
            if !s.drag_over_elements.iter().any(|el| **el == target) {
                s.drag_over_elements.push(SendWrapper::new(target));
            }
        });
        if !first {
            return;
        }
        let allowed = allowed_operations(&e);
        let operation = decide(&e, allowed);
        state.update_value(|s| {
            s.x = e.client_x();
            s.y = e.client_y();
            s.allowed_operations = allowed;
            s.drop_effect = operation.as_drop_effect();
        });
        if let Some(dt) = e.data_transfer() {
            dt.set_drop_effect(operation.as_drop_effect());
        }
        if operation != DropOperation::Cancel {
            fire_drop_enter(&e);
        }
    };

    let on_dragleave = move |e: DragEvent| {
        e.prevent_default();
        e.stop_propagation();
        let target = e
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok());
        let current = e
            .current_target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok());
        state.update_value(|s| {
            s.drag_over_elements
                .retain(|el| Some(&**el) != target.as_ref());
            // Leaving the drop target itself: forget elements dragged over that were removed.
            if target.is_some() && target == current {
                s.drag_over_elements.retain(|el| {
                    node_contains(
                        current.as_ref().map(JsCast::unchecked_ref::<web_sys::Node>),
                        Some(el.unchecked_ref()),
                    )
                    .unwrap_or(false)
                });
            }
        });
        if state.with_value(|s| !s.drag_over_elements.is_empty()) {
            return;
        }
        if state.with_value(|s| s.drop_effect != "none") {
            fire_drop_exit(&e);
        }
        clear_activate_timer();
    };

    let handle_drop = move |e: DragEvent| {
        e.prevent_default();
        e.stop_propagation();
        let effect = state.with_value(|s| s.drop_effect);
        set_global_drop_effect(Some(effect));
        if let Some(on_drop) = on_drop {
            let items = e
                .data_transfer()
                .map(|dt| read_from_data_transfer(&dt))
                .unwrap_or_default();
            let (x, y) = relative(&e, e.client_x(), e.client_y());
            on_drop.run(DropEvent {
                x,
                y,
                items,
                drop_operation: DropOperation::from_drop_effect(effect),
            });
        }
        let snapshot = snapshot_dnd_state();
        state.update_value(|s| s.drag_over_elements.clear());
        fire_drop_exit(&e);
        clear_activate_timer();
        if snapshot.dragging_collection.is_none() {
            set_global_drop_effect(None);
        } else {
            restore_dnd_state(snapshot);
        }
    };

    // Keyboard and screen reader drags.
    let registration: StoredValue<Option<u64>> = StoredValue::new(None);
    let unregister = move || {
        if let Some(id) = registration.try_get_value().flatten() {
            drag_manager::unregister_drop_target(id);
            registration.set_value(None);
        }
    };
    Effect::new(move || {
        unregister();
        let Some(el) = element.get() else {
            return;
        };
        if is_disabled.get() {
            return;
        }
        let id = drag_manager::register_drop_target(DropTargetOptions {
            element: Some((*el).clone()),
            get_drop_operation: Some(Rc::new(
                move |types: &DragTypes, allowed: &[DropOperation]| match get_drop_operation {
                    Some(get) => get.run(DropOperationQuery {
                        types: types.clone(),
                        allowed_operations: allowed.to_vec(),
                    }),
                    None => allowed.first().copied().unwrap_or(DropOperation::Cancel),
                },
            )),
            on_drop_enter: Some(Rc::new(move |e: DropEnterEvent, _drag| {
                set_drop_target.set(true);
                if let Some(on_enter) = on_drop_enter {
                    on_enter.run(e);
                }
            })),
            on_drop_exit: Some(Rc::new(move |e: DropExitEvent| {
                set_drop_target.set(false);
                if let Some(on_exit) = on_drop_exit {
                    on_exit.run(e);
                }
            })),
            on_drop: Some(Rc::new(move |e: DropEvent, _target| {
                if let Some(on_drop) = on_drop {
                    on_drop.run(e);
                }
            })),
            on_drop_activate: Some(Rc::new(move |e: DropActivateEvent, _target| {
                if let Some(on_activate) = on_drop_activate {
                    on_activate.run(e);
                }
            })),
            ..DropTargetOptions::default()
        });
        registration.set_value(Some(id));
    });
    on_cleanup(move || {
        // A pending activation would run the callback of a disposed drop target.
        state.try_update_value(|s| {
            if let Some(timer) = s.drop_activate_timer.take() {
                timer.clear();
            }
        });
        unregister();
    });

    let virtual_drop = use_virtual_drop();
    let enabled = move |handler: EventHandler<DragEvent>| {
        EventHandler::new(move |e: DragEvent| {
            if !is_disabled.get_untracked() {
                handler.call(e);
            }
        })
    };

    UseDropReturn {
        drop_props: UseDropProps {
            aria_describedby: if has_drop_button {
                Signal::stored(None)
            } else {
                Signal::derive(move || {
                    if is_disabled.get() {
                        None
                    } else {
                        virtual_drop.get()
                    }
                })
            },
            element,
            on_dragenter: enabled(EventHandler::new(on_dragenter)),
            on_dragover: enabled(EventHandler::new(on_dragover)),
            on_dragleave: enabled(EventHandler::new(on_dragleave)),
            on_drop: enabled(EventHandler::new(handle_drop)),
        },
        drop_button: UseButtonInput {
            aria_describedby: if has_drop_button {
                virtual_drop
            } else {
                Signal::stored(None)
            },
            is_disabled,
            ..UseButtonInput::default()
        },
        is_drop_target: Signal::derive(move || is_drop_target.get() && !is_disabled.get()),
    }
}
