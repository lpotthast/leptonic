use leptonic::hooks::*;
use leptos::prelude::*;
use ringbuf::{
    HeapRb,
    traits::{Consumer, RingBuffer},
};

#[component]
pub fn DraggableDemo() -> impl IntoView {
    let (events, set_events) = signal(HeapRb::<Oco<'static, str>>::new(20));

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
        get_items: Callback::new(|()| vec![DragItem::text("Hello from draggable!")]),
        on_drag_start: Some(Callback::new(move |e: DragStartEvent| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!("DragStart: x={}, y={}", e.x, e.y)));
            });
        })),
        on_drag_move: Some(Callback::new(move |e: DragMoveEvent| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!("DragMove: x={}, y={}", e.x, e.y)));
            });
        })),
        on_drag_end: Some(Callback::new(move |e: DragEndEvent| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!(
                    "DragEnd: x={}, y={}, effect={:?}",
                    e.x, e.y, e.drop_effect
                )));
            });
        })),
        ..Default::default()
    });

    view! {
        <div style="display: flex; gap: 1em; margin: 1em 0;">
            <div
                {..draggable.drag_props.into_attrs()}
                style=move || format!(
                    "padding: 1em 2em; border-radius: 8px; cursor: grab; user-select: none; \
                    background: {}; color: white; font-weight: bold;",
                    if draggable.is_dragging.get() { "var(--brand-color)" } else { "#666" }
                )
            >
                "Drag me"
            </div>
            <div style="flex: 1;">
                <p style="margin: 0 0 0.5em 0;">"is_dragging: " { move || draggable.is_dragging.get().to_string() }</p>
            </div>
        </div>

        <p>"Last " { move || events.with(ringbuf::traits::Observer::occupied_len) } " events:"</p>
        <pre style="
            width: 100%;
            height: 8em;
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
