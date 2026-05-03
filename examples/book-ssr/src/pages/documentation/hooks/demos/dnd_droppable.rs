use leptonic::hooks::*;
use leptos::prelude::*;
use ringbuf::{
    HeapRb,
    traits::{Consumer, RingBuffer},
};

#[component]
pub fn DroppableDemo() -> impl IntoView {
    let (events, set_events) = signal(HeapRb::<Oco<'static, str>>::new(20));
    let (dropped_items, set_dropped_items) = signal(Vec::<String>::new());

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

    let droppable = use_droppable(UseDroppableInput {
        accepted_types: vec!["text/plain".to_string()],
        on_drop_enter: Some(Callback::new(move |e: DropEnterEvent| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!("DropEnter: types={:?}", e.types)));
            });
        })),
        on_drop_move: Some(Callback::new(move |e: DropMoveEvent| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!("DropMove: x={}, y={}", e.x, e.y)));
            });
        })),
        on_drop_exit: Some(Callback::new(move |_e: DropExitEvent| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned("DropExit".to_string()));
            });
        })),
        on_drop: Some(Callback::new(move |e: DropEvent| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!(
                    "Drop: {} items, effect={:?}",
                    e.items.len(),
                    e.drop_effect
                )));
            });
            for item in e.items {
                if item.kind() == "text/plain" {
                    set_dropped_items.update(|items| items.push(item.data().to_owned()));
                }
            }
        })),
        ..Default::default()
    });

    view! {
        <div style="display: flex; gap: 1em; margin: 1em 0;">
            <div
                {..droppable.drop_props.into_attrs()}
                style=move || format!(
                    "padding: 2em; border-radius: 8px; min-width: 200px; min-height: 100px; \
                    border: 2px dashed {}; background: {};",
                    if droppable.is_drop_target.get() { "var(--brand-color)" } else { "#ccc" },
                    if droppable.is_drop_target.get() { "rgba(var(--brand-color-rgb), 0.1)" } else { "transparent" }
                )
            >
                <p style="margin: 0; text-align: center; color: #666;">"Drop zone"</p>
                <ul style="margin: 0.5em 0 0 0; padding-left: 1.5em;">
                    {move || dropped_items.get().into_iter().map(|item| {
                        view! { <li>{ item }</li> }
                    }).collect::<Vec<_>>()}
                </ul>
            </div>
            <div style="flex: 1;">
                <p style="margin: 0 0 0.5em 0;">"is_drop_target: " { move || droppable.is_drop_target.get().to_string() }</p>
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
