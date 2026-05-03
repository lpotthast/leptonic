use leptonic::hooks::*;
use leptos::prelude::*;
use ringbuf::{
    HeapRb,
    traits::{Consumer, RingBuffer},
};

#[component]
pub fn DragToDropDemo() -> impl IntoView {
    let (events, set_events) = signal(HeapRb::<Oco<'static, str>>::new(20));
    let (dropped_count, set_dropped_count) = signal(0);

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

    let draggable = use_draggable(UseDraggableInput {
        get_items: Callback::new(|()| {
            vec![
                DragItem::text("Package"),
                DragItem::json(r#"{"type": "package", "id": 1}"#),
            ]
        }),
        get_allowed_drop_operations: Callback::new(|()| AllowedDropOperations::COPY),
        on_drag_start: Some(Callback::new(move |_| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Borrowed("Draggable: DragStart"));
            });
        })),
        on_drag_end: Some(Callback::new(move |e: DragEndEvent| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!(
                    "Draggable: DragEnd ({:?})",
                    e.drop_effect
                )));
            });
        })),
        ..Default::default()
    });

    let droppable = use_droppable(UseDroppableInput {
        accepted_types: vec!["text/plain".to_string(), "application/json".to_string()],
        on_drop_enter: Some(Callback::new(move |_| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Borrowed("Droppable: DropEnter"));
            });
        })),
        on_drop_exit: Some(Callback::new(move |_| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Borrowed("Droppable: DropExit"));
            });
        })),
        on_drop: Some(Callback::new(move |e: DropEvent| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!(
                    "Droppable: Drop ({} items)",
                    e.items.len()
                )));
            });
            set_dropped_count.update(|c| *c += 1);
        })),
        ..Default::default()
    });

    view! {
        <div style="display: flex; gap: 2em; margin: 1em 0; align-items: center;">
            <div
                {..draggable.drag_props.into_attrs()}
                style=move || format!(
                    "padding: 1em 2em; border-radius: 8px; cursor: grab; user-select: none; \
                    background: {}; color: white; display: flex; align-items: center; gap: 0.5em;",
                    if draggable.is_dragging.get() { "var(--brand-color)" } else { "#666" }
                )
            >
                <span style="font-size: 1.5em;">"📦"</span>
                "Package"
            </div>

            <div style="font-size: 2em; color: #ccc;">"→"</div>

            <div
                {..droppable.drop_props.into_attrs()}
                style=move || format!(
                    "padding: 2em; border-radius: 8px; min-width: 150px; text-align: center; \
                    border: 2px dashed {}; background: {};",
                    if droppable.is_drop_target.get() { "var(--brand-color)" } else { "#ccc" },
                    if droppable.is_drop_target.get() { "rgba(var(--brand-color-rgb), 0.1)" } else { "transparent" }
                )
            >
                <span style="font-size: 2em;">"📥"</span>
                <p style="margin: 0.5em 0 0 0;">"Inbox"</p>
                <p style="margin: 0.25em 0 0 0; font-size: 0.9em; color: #666;">
                    { move || dropped_count.get() } " received"
                </p>
            </div>
        </div>

        <p>"Events:"</p>
        <pre style="
            width: 100%;
            height: 6em;
            overflow: auto;
            padding: var(--typography-code-padding);
            border: none;
            border-radius: var(--typography-code-border-radius);
            background-color: var(--typography-code-background-color);
            color: var(--typography-code-color);
        ">
            { move || string.get() }
        </pre>
    }
}
