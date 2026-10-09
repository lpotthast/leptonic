// Upstream: react-aria/src/dnd/useDrop.ts @ 99e6102368
// Upstream: react-aria/test/dnd/dnd.test.js @ 99e6102368
// Upstream: react-aria/test/dnd/dnd.ssr.test.js @ 99e6102368
use std::{cell::OnceCell, rc::Rc};

use leptos::{
    attr::{self, Attr},
    ev,
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
        event_target_element, global_allowed_drop_operations, read_from_data_transfer,
        restore_dnd_state, set_global_drop_effect, snapshot_dnd_state,
    },
};
use crate::{
    CapturedElement, EventHandler, IntoAttrs, OnEvent,
    hooks::button::UseButtonInput,
    utils::{
        dom_ext::{EventAccessors, node_contains},
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
    crate::ElementCaptureAttr,
    OnEvent<ev::dragenter>,
    OnEvent<ev::dragover>,
    OnEvent<ev::dragleave>,
    OnEvent<ev::drop>,
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

/// A native drag event at a drop target. What several handlers of one event read from it is read
/// once: the drop target's position and the dragged data's types.
struct NativeDragEvent<'a> {
    event: &'a DragEvent,
    origin: OnceCell<(f64, f64)>,
    types: OnceCell<DragTypes>,
}

impl<'a> NativeDragEvent<'a> {
    fn new(event: &'a DragEvent) -> Self {
        Self {
            event,
            origin: OnceCell::new(),
            types: OnceCell::new(),
        }
    }

    /// The pointer's position relative to the drop target.
    fn relative(&self) -> (f64, f64) {
        let (left, top) = *self.origin.get_or_init(|| {
            let rect = self
                .event
                .expect_current_target()
                .unchecked_into::<web_sys::Element>()
                .get_bounding_client_rect();
            (rect.x(), rect.y())
        });
        (self.event.client_x() - left, self.event.client_y() - top)
    }

    /// The types of the dragged data.
    fn types(&self) -> &DragTypes {
        self.types.get_or_init(|| {
            self.event
                .data_transfer()
                .map(|dt| DragTypes::from_data_transfer(&dt))
                .unwrap_or_default()
        })
    }
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

    let fire_drop_enter = move |e: &NativeDragEvent<'_>| {
        set_drop_target.set(true);
        if let Some(on_enter) = on_drop_enter {
            let (x, y) = e.relative();
            on_enter.run(DropEnterEvent { x, y });
        }
    };
    let fire_drop_exit = move |e: &NativeDragEvent<'_>| {
        set_drop_target.set(false);
        if let Some(on_exit) = on_drop_exit {
            let (x, y) = e.relative();
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
    // The drop operation for the drag `e`: `operation` (decided before), else
    // `get_drop_operation`'s (or the first allowed one), then `get_drop_operation_for_point`'s.
    // The app's callbacks run untracked (react-aria calls them during events).
    let decide = move |e: &NativeDragEvent<'_>,
                       allowed: DropOperations,
                       operation: Option<DropOperation>|
          -> DropOperation {
        let mut operation = operation.unwrap_or_else(|| match get_drop_operation {
            Some(get) => allowed.restrict(untrack(|| {
                get.run(DropOperationQuery {
                    types: e.types().clone(),
                    allowed_operations: allowed.to_vec(),
                })
            })),
            None => allowed.first(),
        });
        if let Some(get) = get_drop_operation_for_point {
            let (x, y) = e.relative();
            operation = allowed.restrict(untrack(|| {
                get.run(DropOperationPointQuery {
                    types: e.types().clone(),
                    allowed_operations: allowed.to_vec(),
                    x,
                    y,
                })
            }));
        }
        operation
    };

    let on_dragover = move |e: DragEvent| {
        e.prevent_default();
        e.stop_propagation();
        let allowed = allowed_operations(&e);
        let (x, y) = (e.client_x(), e.client_y());
        let (same, previous, previous_allowed) = state.with_value(|s| {
            #[allow(clippy::float_cmp)]
            let same = s.x == x && s.y == y && s.allowed_operations == allowed;
            (same, s.drop_effect, s.allowed_operations)
        });
        if same {
            if let Some(dt) = e.data_transfer() {
                dt.set_drop_effect(previous);
            }
            return;
        }
        let event = NativeDragEvent::new(&e);
        // The operation changes with the allowed operations (e.g. a modifier key was pressed),
        // or at another point.
        let unchanged =
            (allowed == previous_allowed).then(|| DropOperation::from_drop_effect(previous));
        let effect = decide(&event, allowed, unchanged).as_drop_effect();
        state.update_value(|s| {
            s.x = x;
            s.y = y;
            s.allowed_operations = allowed;
            s.drop_effect = effect;
        });
        if let Some(dt) = e.data_transfer() {
            dt.set_drop_effect(effect);
        }
        if effect == "none" && previous != "none" {
            fire_drop_exit(&event);
        } else if effect != "none" && previous == "none" {
            fire_drop_enter(&event);
        }
        if effect != "none"
            && let Some(on_move) = on_drop_move
        {
            let (x, y) = event.relative();
            on_move.run(DropMoveEvent { x, y });
        }
        clear_activate_timer();
        if effect != "none"
            && let Some(on_activate) = on_drop_activate
        {
            let (x, y) = event.relative();
            let timer = set_timeout_with_handle(
                move || on_activate.run(DropActivateEvent { x, y }),
                std::time::Duration::from_millis(DROP_ACTIVATE_TIMEOUT_MS),
            )
            .ok();
            state.update_value(|s| s.drop_activate_timer = timer);
        }
    };

    let on_dragenter = move |e: DragEvent| {
        e.prevent_default();
        e.stop_propagation();
        let Some(target) = event_target_element(&e) else {
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
        let event = NativeDragEvent::new(&e);
        let allowed = allowed_operations(&e);
        let operation = decide(&event, allowed, None);
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
            fire_drop_enter(&event);
        }
    };

    let on_dragleave = move |e: DragEvent| {
        e.prevent_default();
        e.stop_propagation();
        let target = event_target_element(&e);
        let current: web_sys::Node = e.expect_current_target().unchecked_into();
        let on_drop_target_itself = target
            .as_ref()
            .is_some_and(|t| *t.unchecked_ref::<web_sys::Node>() == current);
        state.update_value(|s| {
            s.drag_over_elements
                .retain(|el| Some(&**el) != target.as_ref());
            // Leaving the drop target itself: forget elements dragged over that were removed.
            if on_drop_target_itself {
                s.drag_over_elements.retain(|el| {
                    node_contains(Some(&current), Some(el.unchecked_ref())).unwrap_or(false)
                });
            }
        });
        if state.with_value(|s| !s.drag_over_elements.is_empty()) {
            return;
        }
        if state.with_value(|s| s.drop_effect != "none") {
            fire_drop_exit(&NativeDragEvent::new(&e));
        }
        clear_activate_timer();
    };

    let handle_drop = move |e: DragEvent| {
        e.prevent_default();
        e.stop_propagation();
        let event = NativeDragEvent::new(&e);
        let effect = state.with_value(|s| s.drop_effect);
        set_global_drop_effect(Some(effect));
        if let Some(on_drop) = on_drop {
            let items = e
                .data_transfer()
                .map(|dt| read_from_data_transfer(&dt))
                .unwrap_or_default();
            let (x, y) = event.relative();
            on_drop.run(DropEvent {
                x,
                y,
                items,
                drop_operation: DropOperation::from_drop_effect(effect),
            });
        }
        let snapshot = snapshot_dnd_state();
        state.update_value(|s| s.drag_over_elements.clear());
        fire_drop_exit(&event);
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
                    Some(get) => untrack(|| {
                        get.run(DropOperationQuery {
                            types: types.clone(),
                            allowed_operations: allowed.to_vec(),
                        })
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

    // Only an enabled drop target describes how to drop on it (react-aria: no drop props when
    // disabled).
    let virtual_drop = use_virtual_drop();
    let description = Signal::derive(move || {
        if is_disabled.get() {
            None
        } else {
            virtual_drop.get()
        }
    });
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
                description
            },
            element,
            on_dragenter: enabled(EventHandler::new(on_dragenter)),
            on_dragover: enabled(EventHandler::new(on_dragover)),
            on_dragleave: enabled(EventHandler::new(on_dragleave)),
            on_drop: enabled(EventHandler::new(handle_drop)),
        },
        drop_button: UseButtonInput {
            aria_describedby: if has_drop_button {
                description
            } else {
                Signal::stored(None)
            },
            is_disabled,
            ..UseButtonInput::default()
        },
        is_drop_target: Signal::derive(move || is_drop_target.get() && !is_disabled.get()),
    }
}
