// Upstream: react-aria/src/dnd/useDrag.ts @ 99e6102368
// Upstream: react-aria/src/dnd/DragPreview.tsx @ 99e6102368
// Upstream: react-aria/test/dnd/dnd.test.js @ 99e6102368
// Upstream: react-aria/test/dnd/dnd.ssr.test.js @ 99e6102368
use std::{
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
};

use leptos::{
    attr::{
        self, Attr,
        custom::{CustomAttr, custom_attribute},
    },
    ev,
    prelude::*,
};
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use web_sys::{DragEvent, KeyboardEvent, MouseEvent, PointerEvent};

use super::{
    drag_manager::{self, DragTarget},
    messages,
    types::{
        DragEndEvent, DragItem, DragMoveEvent, DragPreview, DragStartEvent, DropOperation,
        DropOperations,
    },
    utils::{
        event_target_element, global_drop_effect, set_global_allowed_drop_operations,
        set_global_drop_effect, use_drag_modality, write_to_data_transfer,
    },
};
use crate::{
    EventHandler, IntoAttrs, OnEvent,
    hooks::{button::UseButtonInput, interactions::use_press::PressEvent},
    utils::{
        dom_ext::EventAccessors,
        event_listeners::{Listener, listen},
        intl_strings::{DndStrings, use_localized_strings},
        key::{KeyboardEventKey, KeyboardKey},
        platform::{browser::is_webkit, device::is_ios},
        pointer_type::PointerType,
        use_description::use_description,
        virtual_click::{is_virtual_click, is_virtual_pointer_event},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The drag button comes back as a `UseButtonInput` for `use_button`.
// - The drag preview is an element the `preview` callback returns (react-aria renders a
//   `DragPreview` component into a ref).
// - The dragged data and the allowed operations are signals (`items`, `allowed_drop_operations`),
//   read when a drag starts (react-aria: the `getItems`/`getAllowedDropOperations` functions).
//
// ## DIFFERENT BEHAVIOR
// - A pointer counts as virtual (a screen reader's) when `is_virtual_pointer_event` says so OR it
//   hits the element's center (TalkBack) OR it is iOS VoiceOver's zero-size pointer. react-aria
//   overwrites the first check with the pointer type when the second fails, so a virtual pointer
//   off center starts a native drag there.
// - A keyboard or screen reader drag starts (`on_drag_start`) only if no other one is in
//   progress, e.g. after a second Enter before the first drag's session was set up (react-aria
//   calls `onDragStart`, then throws).
// - A native drag ends once: unmounting the dragged element ends it (as in react-aria), and a
//   `dragend` the browser still sends to the removed element afterwards is ignored; a `dragend`
//   before the frame that marks the element as dragging keeps it from being marked.
// - Cleanup ends an active native drag before checking DOM attachment: Leptos disposes the
//   owner before removing its element, unlike React's unmount cleanup ordering.
//
// =============================================================================

/// Input of [`use_drag`].
#[derive(Clone)]
pub struct UseDragInput {
    /// The dragged data, read when a drag starts (e.g. a `Signal::derive`, computed only then).
    pub items: Signal<Vec<DragItem>>,
    /// The operations the drag allows, in order of preference. `None`: all of them (move, copy,
    /// link).
    pub allowed_drop_operations: Option<Signal<Vec<DropOperation>>>,
    /// What to show while dragging.
    pub preview: Option<Callback<Vec<DragItem>, Option<DragPreview>>>,
    pub on_drag_start: Option<Callback<DragStartEvent>>,
    pub on_drag_move: Option<Callback<DragMoveEvent>>,
    pub on_drag_end: Option<Callback<DragEndEvent>>,
    /// Keyboard and screen reader drags start from a separate drag button (`drag_button`).
    pub has_drag_button: bool,
    pub is_disabled: Signal<bool>,
}

/// Return value of [`use_drag`].
pub struct UseDragReturn {
    pub drag_props: UseDragProps,
    /// For `use_button`, with `has_drag_button`.
    pub drag_button: UseButtonInput,
    pub is_dragging: Signal<bool>,
}

/// Props for the draggable element.
#[derive(Debug)]
pub struct UseDragProps {
    pub draggable: Signal<&'static str>,
    /// How to start a drag (without a drag button).
    pub aria_describedby: Signal<Option<String>>,
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub on_keydown_capture: EventHandler<KeyboardEvent>,
    pub on_keyup_capture: EventHandler<KeyboardEvent>,
    pub on_click: EventHandler<MouseEvent>,
    pub on_dragstart: EventHandler<DragEvent>,
    pub on_drag: EventHandler<DragEvent>,
    pub on_dragend: EventHandler<DragEvent>,
}

pub type UseDragAttrs = (
    CustomAttr<&'static str, Signal<&'static str>>,
    Attr<attr::AriaDescribedby, Signal<Option<String>>>,
    OnEvent<ev::pointerdown>,
    OnEvent<ev::Capture<ev::keydown>>,
    OnEvent<ev::Capture<ev::keyup>>,
    OnEvent<ev::click>,
    OnEvent<ev::dragstart>,
    OnEvent<ev::drag>,
    OnEvent<ev::dragend>,
);

impl IntoAttrs for UseDragProps {
    type Attrs = UseDragAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            custom_attribute("draggable", self.draggable),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            self.on_pointerdown.into_on(ev::pointerdown),
            self.on_keydown_capture.into_on(ev::capture(ev::keydown)),
            self.on_keyup_capture.into_on(ev::capture(ev::keyup)),
            self.on_click.into_on(ev::click),
            self.on_dragstart.into_on(ev::dragstart),
            self.on_drag.into_on(ev::drag),
            self.on_dragend.into_on(ev::dragend),
        )
    }
}

/// The native drag of an element, shared with handlers that may run after the element's owner was
/// disposed.
#[derive(Debug, Default)]
struct NativeDrag {
    /// Bumped by every `dragstart` and every end, so that the frame a `dragstart` schedules
    /// doesn't mark a drag as dragging that ended in the meantime.
    generation: AtomicU64,
    /// A drag started and hasn't ended.
    active: AtomicBool,
}

impl NativeDrag {
    /// A drag starts: its generation.
    fn start(&self) -> u64 {
        self.active.store(true, Ordering::Relaxed);
        self.generation.fetch_add(1, Ordering::Relaxed) + 1
    }

    /// Whether no drag started or ended since the one of `generation` started.
    fn is_current(&self, generation: u64) -> bool {
        self.generation.load(Ordering::Relaxed) == generation
    }

    /// The drag ends: `true` for the first end after a start only.
    fn end(&self) -> bool {
        self.generation.fetch_add(1, Ordering::Relaxed);
        self.active.swap(false, Ordering::Relaxed)
    }
}

/// How the pointer that may start a drag was used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PointerModality {
    Virtual,
    Other,
}

/// A draggable element: drags start natively with the mouse or touch, and with the keyboard
/// (Enter) or a screen reader through the drag manager.
#[allow(clippy::too_many_lines)]
pub fn use_drag(input: UseDragInput) -> UseDragReturn {
    let UseDragInput {
        items,
        allowed_drop_operations,
        preview,
        on_drag_start,
        on_drag_move,
        on_drag_end,
        has_drag_button,
        is_disabled,
    } = input;
    let strings = use_localized_strings::<DndStrings>();

    let position = StoredValue::new((0.0_f64, 0.0_f64));
    let native_drag = Arc::new(NativeDrag::default());
    let (is_dragging, set_is_dragging) = signal(false);
    // Also called after the drag ended or in an animation frame, when this element may be gone
    // (hence `try_*`).
    let set_dragging = move |element: Option<web_sys::Element>| {
        set_is_dragging.try_set(element.is_some());
    };
    let drop_guard: StoredValue<Option<SendWrapper<Listener>>> = StoredValue::new(None);
    let modality_on_pointer_down: StoredValue<Option<PointerModality>> = StoredValue::new(None);

    let allowed_operations = move || {
        allowed_drop_operations.map_or_else(
            || {
                vec![
                    DropOperation::Move,
                    DropOperation::Copy,
                    DropOperation::Link,
                ]
            },
            |allowed| allowed.get_untracked(),
        )
    };

    let start_dragging = move |target: web_sys::Element| {
        let started = drag_manager::begin_dragging(DragTarget {
            strings: strings.get_untracked(),
            element: target.clone(),
            items: items.get_untracked(),
            allowed_drop_operations: allowed_operations(),
            // The drag manager ends the drag after the drop, when this element may be gone.
            on_drag_end: Some(Rc::new(move |e: DragEndEvent| {
                set_dragging(None);
                if let Some(on_end) = on_drag_end {
                    on_end.try_run(e);
                }
            })),
        });
        // Another keyboard drag is still being set up (e.g. a second Enter in the same frame).
        if started.is_err() {
            return;
        }
        if let Some(on_start) = on_drag_start {
            let rect = target.get_bounding_client_rect();
            on_start.run(DragStartEvent {
                x: rect.x() + rect.width() / 2.0,
                y: rect.y() + rect.height() / 2.0,
            });
        }
        set_dragging(Some(target));
    };

    let native = native_drag.clone();
    let on_dragstart = move |e: DragEvent| {
        if e.default_prevented() {
            return;
        }
        e.stop_propagation();
        if modality_on_pointer_down.get_value() == Some(PointerModality::Virtual) {
            e.prevent_default();
            if let Some(target) = event_target_element(&e) {
                start_dragging(target);
            }
            modality_on_pointer_down.set_value(None);
            return;
        }
        if let Some(on_start) = on_drag_start {
            on_start.run(DragStartEvent {
                x: e.client_x(),
                y: e.client_y(),
            });
        }
        let items = items.get_untracked();
        let Some(data_transfer) = e.data_transfer() else {
            return;
        };
        let _ = data_transfer.clear_data();
        write_to_data_transfer(&data_transfer, &items);

        let allowed = allowed_drop_operations.map_or(DropOperations::ALL, |allowed| {
            DropOperations::from_operations(&allowed.get_untracked())
        });
        set_global_allowed_drop_operations(allowed);
        data_transfer.set_effect_allowed(allowed.as_effect_allowed());

        if let Some(preview) = preview
            && let Some(DragPreview { element, offset }) = preview.run(items)
        {
            let current: web_sys::Element = e.expect_current_target().unchecked_into();
            let size = element.get_bounding_client_rect();
            let rect = current.get_bounding_client_rect();
            let mut default_x = e.client_x() - rect.x();
            let mut default_y = e.client_y() - rect.y();
            if default_x > size.width() || default_y > size.height() {
                default_x = size.width() / 2.0;
                default_y = size.height() / 2.0;
            }
            let (offset_x, offset_y) = offset.map_or((default_x, default_y), |p| (p.x, p.y));
            let offset_x = offset_x.clamp(0.0, size.width());
            let offset_y = offset_y.clamp(0.0, size.height());
            // An even height keeps the preview sharp on some displays.
            let height = 2.0 * (size.height() / 2.0).round();
            if let Some(html) = element.dyn_ref::<web_sys::HtmlElement>() {
                let _ = html.style().set_property("height", &format!("{height}px"));
            }
            #[allow(clippy::cast_possible_truncation)]
            data_transfer.set_drag_image(
                &element,
                offset_x.round() as i32,
                offset_y.round() as i32,
            );
        }

        // Only `use_drop` targets may receive the drop: they offer the accessible alternative.
        if let Some(window) = leptos_use::use_window().as_ref() {
            let guard = listen(window.unchecked_ref(), "drop", false, move |e| {
                e.prevent_default();
                e.stop_propagation();
                crate::utils::dev_warn!(
                    "Drags initiated from use_drag may only be dropped on a target created with use_drop. This ensures that a keyboard and screen reader accessible alternative is available."
                );
            });
            drop_guard.set_value(Some(SendWrapper::new(guard)));
        }

        position.set_value((e.client_x(), e.client_y()));
        // Wait a frame (the browser renders the drag image first), unless the drag ended by then.
        let generation = native.start();
        let target = event_target_element(&e);
        let native = native.clone();
        request_animation_frame(move || {
            if native.is_current(generation) {
                set_dragging(target);
            }
        });
    };

    // Chromium keeps sending `drag` and `dragend` to a drag source that was removed during its
    // drag (moved to another list, a virtualized row scrolled away), after this hook's owner was
    // disposed: these handlers only use `try_*` accessors.
    let on_drag = move |e: DragEvent| {
        e.stop_propagation();
        let current = (e.client_x(), e.client_y());
        #[allow(clippy::float_cmp)]
        if position
            .try_get_value()
            .is_none_or(|previous| previous == current)
        {
            return;
        }
        if let Some(on_move) = on_drag_move {
            on_move.try_run(DragMoveEvent {
                x: current.0,
                y: current.1,
            });
        }
        position.try_set_value(current);
    };

    let native = native_drag.clone();
    let on_dragend = move |e: DragEvent| {
        e.stop_propagation();
        // Unmounting the dragged element may have ended the drag already.
        if !native.end() {
            return;
        }
        if let Some(on_end) = on_drag_end {
            let effect = global_drop_effect()
                .map(ToOwned::to_owned)
                .or_else(|| e.data_transfer().map(|dt| dt.drop_effect()));
            on_end.try_run(DragEndEvent {
                x: e.client_x(),
                y: e.client_y(),
                drop_operation: DropOperation::from_drop_effect(
                    effect.as_deref().unwrap_or("none"),
                ),
            });
        }
        set_dragging(None);
        drop_guard.try_set_value(None);
        set_global_allowed_drop_operations(DropOperations::NONE);
        set_global_drop_effect(None);
    };

    // Unmounting the dragged element ends its native drag (browsers may never send its
    // `dragend`, https://bugzilla.mozilla.org/show_bug.cgi?id=460801).
    let native = native_drag;
    on_cleanup(move || {
        if native.end() {
            if let Some(on_end) = on_drag_end {
                on_end.try_run(DragEndEvent {
                    x: 0.0,
                    y: 0.0,
                    drop_operation: DropOperation::from_drop_effect(
                        global_drop_effect().unwrap_or("none"),
                    ),
                });
            }
            set_dragging(None);
            drop_guard.try_set_value(None);
            set_global_allowed_drop_operations(DropOperations::NONE);
            set_global_drop_effect(None);
        }
    });

    let modality = use_drag_modality();
    let description = use_description(Signal::derive(move || {
        let modality = modality.get();
        let strings = strings.read();
        Some(if is_dragging.get() {
            messages::end_drag(&strings, modality)
        } else {
            messages::drag_description(&strings, modality)
        })
    }));

    let on_pointerdown = move |e: PointerEvent| {
        if is_disabled.get_untracked() {
            return;
        }
        let virtual_pointer = is_virtual_pointer_event(&e)
            || (e.width() < 1 && e.height() < 1 && is_ios() && is_webkit());
        let centered = {
            let rect = e
                .expect_current_target()
                .unchecked_into::<web_sys::Element>()
                .get_bounding_client_rect();
            let offset_x = e.client_x() - rect.x();
            let offset_y = e.client_y() - rect.y();
            (offset_x - rect.width() / 2.0).abs() <= 0.5
                && (offset_y - rect.height() / 2.0).abs() <= 0.5
        };
        modality_on_pointer_down.set_value(Some(if virtual_pointer || centered {
            PointerModality::Virtual
        } else {
            PointerModality::Other
        }));
    };
    let on_target_itself = |e: &KeyboardEvent| {
        event_target_element(e).is_some_and(|target| {
            *target.unchecked_ref::<web_sys::EventTarget>() == e.expect_current_target()
        })
    };
    let on_keydown_capture = move |e: KeyboardEvent| {
        if is_disabled.get_untracked() {
            return;
        }
        if on_target_itself(&e) && e.typed_key() == KeyboardKey::Enter {
            e.prevent_default();
            e.stop_propagation();
        }
    };
    let on_keyup_capture = move |e: KeyboardEvent| {
        if is_disabled.get_untracked() {
            return;
        }
        if on_target_itself(&e) && e.typed_key() == KeyboardKey::Enter {
            e.prevent_default();
            e.stop_propagation();
            if let Some(target) = event_target_element(&e) {
                start_dragging(target);
            }
        }
    };
    let on_click = move |e: MouseEvent| {
        if is_disabled.get_untracked() {
            return;
        }
        if is_virtual_click(&e)
            || modality_on_pointer_down.get_value() == Some(PointerModality::Virtual)
        {
            e.prevent_default();
            e.stop_propagation();
            if let Some(target) = event_target_element(&e) {
                start_dragging(target);
            }
        }
    };

    let on_press = Callback::new(move |e: PressEvent| {
        if !matches!(e.pointer_type, PointerType::Keyboard | PointerType::Virtual) {
            return;
        }
        if let Some(target) = e.target.dyn_ref::<web_sys::Element>() {
            start_dragging(target.clone());
        }
    });

    UseDragReturn {
        drag_props: UseDragProps {
            draggable: Signal::derive(move || if is_disabled.get() { "false" } else { "true" }),
            aria_describedby: if has_drag_button {
                Signal::stored(None)
            } else {
                Signal::derive(move || {
                    if is_disabled.get() {
                        None
                    } else {
                        description.get()
                    }
                })
            },
            on_pointerdown: without_drag_button(has_drag_button, EventHandler::new(on_pointerdown)),
            on_keydown_capture: without_drag_button(
                has_drag_button,
                EventHandler::new(on_keydown_capture),
            ),
            on_keyup_capture: without_drag_button(
                has_drag_button,
                EventHandler::new(on_keyup_capture),
            ),
            on_click: without_drag_button(has_drag_button, EventHandler::new(on_click)),
            on_dragstart: EventHandler::new(move |e: DragEvent| {
                if is_disabled.get_untracked() {
                    return;
                }
                on_dragstart(e);
            }),
            on_drag: EventHandler::new(on_drag),
            on_dragend: EventHandler::new(on_dragend),
        },
        drag_button: UseButtonInput {
            aria_describedby: description,
            on_press: Some(on_press),
            is_disabled,
            ..UseButtonInput::default()
        },
        is_dragging: Signal::derive(move || is_dragging.get() && !is_disabled.get()),
    }
}

/// `handler`, unless the element has a drag button (which starts keyboard drags instead).
fn without_drag_button<E: 'static>(
    has_drag_button: bool,
    handler: EventHandler<E>,
) -> EventHandler<E> {
    if has_drag_button {
        EventHandler::empty()
    } else {
        handler
    }
}
