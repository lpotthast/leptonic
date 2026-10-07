use leptonic::{
    hooks::{
        DragItem, DropEvent, DropItem, IntoAttrs, UseDragInput, UseDragReturn, UseDropInput,
        UseDropReturn, use_drag, use_drop,
    },
    utils::CapturedElement,
};
use leptos::prelude::*;

/// A card that can be dragged onto a drop target with the mouse, touch or the keyboard.
#[component]
pub fn DndQuickStartDemo() -> impl IntoView {
    let dropped = RwSignal::new(None::<String>);

    let UseDragReturn {
        drag_props,
        is_dragging,
        ..
    } = use_drag(UseDragInput {
        items: Signal::stored(vec![DragItem::text("Water the plants")]),
        allowed_drop_operations: None,
        preview: None,
        on_drag_start: None,
        on_drag_move: None,
        on_drag_end: None,
        has_drag_button: false,
        is_disabled: Signal::stored(false),
    });

    let UseDropReturn {
        drop_props,
        is_drop_target,
        ..
    } = use_drop(UseDropInput {
        on_drop: Some(Callback::new(move |e: DropEvent| {
            let text = e.items.iter().find_map(|item| match item {
                DropItem::Text(text) => text.get_text("text/plain").map(str::to_owned),
                DropItem::File(_) | DropItem::Directory(_) => None,
            });
            dropped.set(text);
        })),
        element: CapturedElement::new(),
        get_drop_operation: None,
        get_drop_operation_for_point: None,
        on_drop_enter: None,
        on_drop_move: None,
        on_drop_activate: None,
        on_drop_exit: None,
        has_drop_button: false,
        is_disabled: Signal::stored(false),
    });

    view! {
        <div class="demo-dnd-board">
            // Focusable, so that Enter starts a keyboard drag.
            <div
                {..drag_props.into_attrs()}
                role="button"
                tabindex="0"
                class="demo-dnd-card"
                data-dragging=move || is_dragging.get().then_some("")
            >
                "Water the plants"
            </div>
            // Keyboard drags move focus to the drop target.
            <div
                {..drop_props.into_attrs()}
                role="button"
                tabindex="0"
                class="demo-dnd-target"
                data-drop-target=move || is_drop_target.get().then_some("")
            >
                "Drop here"
            </div>
        </div>

        <p class="demo-status">
            {move || match dropped.get() {
                Some(text) => format!("Dropped: {text}"),
                None => "Nothing dropped yet".to_owned(),
            }}
        </p>
    }
}
