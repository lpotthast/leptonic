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
    } = use_drag(UseDragInput::new(Callback::new(|()| {
        vec![DragItem::text("Water the plants")]
    })));

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
        ..UseDropInput::new(CapturedElement::new())
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
