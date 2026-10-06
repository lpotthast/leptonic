// Upstream: react-aria/src/dnd/useDrag.ts @ 99e6102368
use std::rc::Rc;

use leptos::{
    attr::{
        self, Attr,
        custom::{CustomAttr, custom_attribute},
    },
    ev,
    ev::{On, SharedEventCallback},
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
        global_drop_effect, set_global_allowed_drop_operations, set_global_drop_effect,
        use_drag_modality, write_to_data_transfer,
    },
};
use crate::{
    hooks::{IntoAttrs, UseButtonInput, interactions::use_press::PressEvent},
    utils::{
        EventHandler,
        event_listeners::{Listener, listen},
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
// - Drags started before React 17 cleanup semantics: not applicable (unmounting a dragged element
//   always ends its drag).
//
// =============================================================================

/// Input of [`use_drag`].
#[derive(Clone)]
pub struct UseDragInput {
    /// The dragged data.
    pub get_items: Callback<(), Vec<DragItem>>,
    /// The operations the drag allows, in order of preference. Defaults to move, copy, link.
    pub get_allowed_drop_operations: Option<Callback<(), Vec<DropOperation>>>,
    /// What to show while dragging.
    pub preview: Option<Callback<Vec<DragItem>, Option<DragPreview>>>,
    pub on_drag_start: Option<Callback<DragStartEvent>>,
    pub on_drag_move: Option<Callback<DragMoveEvent>>,
    pub on_drag_end: Option<Callback<DragEndEvent>>,
    /// Keyboard and screen reader drags start from a separate drag button (`drag_button`).
    pub has_drag_button: bool,
    pub is_disabled: Signal<bool>,
}

impl UseDragInput {
    pub fn new(get_items: Callback<(), Vec<DragItem>>) -> Self {
        Self {
            get_items,
            get_allowed_drop_operations: None,
            preview: None,
            on_drag_start: None,
            on_drag_move: None,
            on_drag_end: None,
            has_drag_button: false,
            is_disabled: Signal::stored(false),
        }
    }
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
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
    On<ev::Capture<ev::keydown>, SharedEventCallback<KeyboardEvent>>,
    On<ev::Capture<ev::keyup>, SharedEventCallback<KeyboardEvent>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::dragstart, SharedEventCallback<DragEvent>>,
    On<ev::drag, SharedEventCallback<DragEvent>>,
    On<ev::dragend, SharedEventCallback<DragEvent>>,
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
        get_items,
        get_allowed_drop_operations,
        preview,
        on_drag_start,
        on_drag_move,
        on_drag_end,
        has_drag_button,
        is_disabled,
    } = input;

    let position = StoredValue::new((0.0_f64, 0.0_f64));
    let dragging_element: StoredValue<Option<SendWrapper<web_sys::Element>>> =
        StoredValue::new(None);
    let (is_dragging, set_is_dragging) = signal(false);
    // Also called after the drag ended or in an animation frame, when this element may be gone
    // (hence `try_*`).
    let set_dragging = move |element: Option<web_sys::Element>| {
        set_is_dragging.try_set(element.is_some());
        dragging_element.try_set_value(element.map(SendWrapper::new));
    };
    let drop_guard: StoredValue<Option<SendWrapper<Listener>>> = StoredValue::new(None);
    let modality_on_pointer_down: StoredValue<Option<PointerModality>> = StoredValue::new(None);

    let allowed_operations = move || {
        get_allowed_drop_operations.map_or_else(
            || {
                vec![
                    DropOperation::Move,
                    DropOperation::Copy,
                    DropOperation::Link,
                ]
            },
            |get| get.run(()),
        )
    };

    let start_dragging = move |target: web_sys::Element| {
        if let Some(on_start) = on_drag_start {
            let rect = target.get_bounding_client_rect();
            on_start.run(DragStartEvent {
                x: rect.x() + rect.width() / 2.0,
                y: rect.y() + rect.height() / 2.0,
            });
        }
        drag_manager::begin_dragging(DragTarget {
            element: target.clone(),
            items: get_items.run(()),
            allowed_drop_operations: allowed_operations(),
            // The drag manager ends the drag after the drop, when this element may be gone.
            on_drag_end: Some(Rc::new(move |e: DragEndEvent| {
                set_dragging(None);
                if let Some(on_end) = on_drag_end {
                    on_end.try_run(e);
                }
            })),
        });
        set_dragging(Some(target));
    };

    let on_dragstart = move |e: DragEvent| {
        if e.default_prevented() {
            return;
        }
        e.stop_propagation();
        if modality_on_pointer_down.get_value() == Some(PointerModality::Virtual) {
            e.prevent_default();
            if let Some(target) = e
                .target()
                .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
            {
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
        let items = get_items.run(());
        let Some(data_transfer) = e.data_transfer() else {
            return;
        };
        let _ = data_transfer.clear_data();
        write_to_data_transfer(&data_transfer, &items);

        let allowed = get_allowed_drop_operations.map_or(DropOperations::ALL, |get| {
            DropOperations::from_operations(&get.run(()))
        });
        set_global_allowed_drop_operations(allowed);
        data_transfer.set_effect_allowed(allowed.as_effect_allowed());

        if let Some(preview) = preview
            && let Some(DragPreview { element, offset }) = preview.run(items)
            && let Some(current) = e
                .current_target()
                .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        {
            let size = element.get_bounding_client_rect();
            let rect = current.get_bounding_client_rect();
            let mut default_x = e.client_x() - rect.x();
            let mut default_y = e.client_y() - rect.y();
            if default_x > size.width() || default_y > size.height() {
                default_x = size.width() / 2.0;
                default_y = size.height() / 2.0;
            }
            let (offset_x, offset_y) = offset.unwrap_or((default_x, default_y));
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
        let target = e
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok());
        request_animation_frame(move || set_dragging(target));
    };

    let on_drag = move |e: DragEvent| {
        e.stop_propagation();
        let current = (e.client_x(), e.client_y());
        #[allow(clippy::float_cmp)]
        if current == position.get_value() {
            return;
        }
        if let Some(on_move) = on_drag_move {
            on_move.run(DragMoveEvent {
                x: current.0,
                y: current.1,
            });
        }
        position.set_value(current);
    };

    let on_dragend = move |e: DragEvent| {
        e.stop_propagation();
        if let Some(on_end) = on_drag_end {
            let effect = global_drop_effect()
                .map(ToOwned::to_owned)
                .or_else(|| e.data_transfer().map(|dt| dt.drop_effect()));
            on_end.run(DragEndEvent {
                x: e.client_x(),
                y: e.client_y(),
                drop_operation: DropOperation::from_drop_effect(
                    effect.as_deref().unwrap_or("none"),
                ),
            });
        }
        set_dragging(None);
        drop_guard.set_value(None);
        set_global_allowed_drop_operations(DropOperations::NONE);
        set_global_drop_effect(None);
    };

    // Unmounting the dragged element ends its drag.
    on_cleanup(move || {
        let dragged = dragging_element.try_get_value().flatten();
        if dragged.is_some_and(|el| !el.is_connected()) {
            if let Some(on_end) = on_drag_end {
                on_end.run(DragEndEvent {
                    x: 0.0,
                    y: 0.0,
                    drop_operation: DropOperation::from_drop_effect(
                        global_drop_effect().unwrap_or("none"),
                    ),
                });
            }
            set_global_allowed_drop_operations(DropOperations::NONE);
            set_global_drop_effect(None);
        }
    });

    let modality = use_drag_modality();
    let description = use_description(Signal::derive(move || {
        let modality = modality.get();
        Some(
            if is_dragging.get() {
                messages::end_drag(modality)
            } else {
                messages::drag_description(modality)
            }
            .to_owned(),
        )
    }));

    let on_pointerdown = move |e: PointerEvent| {
        let virtual_pointer = is_virtual_pointer_event(&e)
            || (e.width() < 1 && e.height() < 1 && is_ios() && is_webkit());
        let centered = e
            .current_target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
            .is_some_and(|el| {
                let rect = el.get_bounding_client_rect();
                let offset_x = e.client_x() - rect.x();
                let offset_y = e.client_y() - rect.y();
                (offset_x - rect.width() / 2.0).abs() <= 0.5
                    && (offset_y - rect.height() / 2.0).abs() <= 0.5
            });
        modality_on_pointer_down.set_value(Some(if virtual_pointer || centered {
            PointerModality::Virtual
        } else {
            PointerModality::Other
        }));
    };
    let on_target_itself =
        |e: &web_sys::Event| e.target().is_some() && e.target() == e.current_target();
    let on_keydown_capture = move |e: KeyboardEvent| {
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
            if let Some(target) = e
                .target()
                .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
            {
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
            if let Some(target) = e
                .target()
                .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
            {
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
        is_dragging: is_dragging.into(),
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
