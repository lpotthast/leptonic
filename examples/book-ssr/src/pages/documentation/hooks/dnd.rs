use indoc::indoc;
use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptos::prelude::*;
use ringbuf::traits::{Consumer, Observer, RingBuffer};
use ringbuf::HeapRb;

use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;

#[component]
pub fn PageUseDnd() -> impl IntoView {
    view! {
        <Article>
            <h1 id="dnd" class="anchor">
                "Drag and Drop Hooks"
                <AnchorLink href="#dnd" description="Direct link to article header"/>
            </h1>

            <p>"Hooks for creating accessible drag and drop interactions with full keyboard support and ARIA announcements."</p>

            <h2 id="use_draggable" class="anchor">
                "use_draggable"
                <AnchorLink href="#use_draggable" description="Direct link to use_draggable"/>
            </h2>

            <p>"Makes an element draggable with proper accessibility attributes."</p>

            <DraggableDemo />

            <Code>
                {indoc!(r#"
                    let draggable = use_draggable(UseDraggableInput {
                        get_items: Callback::new(|_| vec![DragItem::text("Hello")]),
                        on_drag_start: Some(Callback::new(|e: DragStartEvent| {
                            // Handle drag start
                        })),
                        on_drag_end: Some(Callback::new(|e: DragEndEvent| {
                            // Handle drag end
                        })),
                        ..Default::default()
                    });

                    view! {
                        <div {..draggable.drag_props.into_attrs()}>"Drag me"</div>
                    }
                "#)}
            </Code>

            <h3>"Return Value"</h3>
            <ul>
                <li><code>"drag_props"</code>" - Props to spread on the draggable element"</li>
                <li><code>"draggable_id"</code>" - Unique ID for the draggable element"</li>
                <li><code>"is_dragging"</code>" - Signal indicating if this element is being dragged"</li>
            </ul>

            <h2 id="use_droppable" class="anchor">
                "use_droppable"
                <AnchorLink href="#use_droppable" description="Direct link to use_droppable"/>
            </h2>

            <p>"Makes an element a drop target that can accept dragged items."</p>

            <DroppableDemo />

            <Code>
                {indoc!(r#"
                    let droppable = use_droppable(UseDroppableInput {
                        accepted_types: vec!["text/plain".to_string()],
                        on_drop: Some(Callback::new(|e: DropEvent| {
                            for item in e.items {
                                // Handle dropped item
                            }
                        })),
                        ..Default::default()
                    });

                    view! {
                        <div
                            {..droppable.drop_props.into_attrs()}
                            class:drop-target=move || droppable.is_drop_target.get()
                        >
                            "Drop here"
                        </div>
                    }
                "#)}
            </Code>

            <h3>"Return Value"</h3>
            <ul>
                <li><code>"drop_props"</code>" - Props to spread on the drop zone element"</li>
                <li><code>"droppable_id"</code>" - Unique ID for the drop zone"</li>
                <li><code>"is_drop_target"</code>" - Signal indicating if something is being dragged over this zone"</li>
            </ul>

            <h2 id="drag-to-drop" class="anchor">
                "Drag to Drop"
                <AnchorLink href="#drag-to-drop" description="Direct link to drag to drop"/>
            </h2>

            <p>"Combine use_draggable and use_droppable to create drag-and-drop interactions."</p>

            <DragToDropDemo />

            <h2 id="use_drag_and_drop" class="anchor">
                "use_drag_and_drop"
                <AnchorLink href="#use_drag_and_drop" description="Direct link to use_drag_and_drop"/>
            </h2>

            <p>"A higher-level hook for creating sortable lists with drag-and-drop reordering."</p>

            <ReorderDemo />

            <Code>
                {indoc!(r#"
                    // In the parent component - create shared state:
                    let dnd = use_drag_and_drop(UseDragAndDropInput {
                        drag_options: Some(DraggableOptions {
                            get_items: Callback::new(|key: String| vec![DragItem::text(key)]),
                            allowed_drop_effect: DropEffect::Move,
                            ..Default::default()
                        }),
                        drop_options: Some(DroppableOptions {
                            accepted_types: vec!["text/plain".to_string()],
                            ..Default::default()
                        }),
                        on_reorder: Some(Callback::new(|e: ReorderEvent| {
                            // Handle reorder: e.keys moved to e.target
                        })),
                        ..Default::default()
                    });

                    // Pass dnd.state to each item component...

                    // In each item component - call hooks at component level:
                    #[component]
                    fn Item(key: String, state: DragAndDropState) -> impl IntoView {
                        let drag = use_draggable_item(key.clone(), state.clone());
                        let drop = use_droppable_item(key, state);

                        // Then spread the props:
                        if let (Some(drag), Some(drop)) = (drag, drop) {
                            view! { <div {..drag.drag_props.into_attrs()} {..drop.drop_props.into_attrs()}>...</div> }
                        }
                    }
                "#)}
            </Code>

            <h3>"Callbacks"</h3>
            <ul>
                <li><code>"on_reorder"</code>" - Called when an item is reordered within the same collection"</li>
                <li><code>"on_insert"</code>" - Called when an item is inserted from another collection"</li>
                <li><code>"on_remove"</code>" - Called when an item is moved out of this collection"</li>
            </ul>

            <h2 id="drop-positions" class="anchor">
                "Drop Positions"
                <AnchorLink href="#drop-positions" description="Direct link to drop positions"/>
            </h2>

            <p>"When reordering, the drop target includes position information:"</p>

            <ul>
                <li><code>"DropPosition::Before"</code>" - Insert before the target item"</li>
                <li><code>"DropPosition::After"</code>" - Insert after the target item"</li>
                <li><code>"DropPosition::On"</code>" - Drop onto the target item (for nested structures like trees)"</li>
            </ul>

            <h2 id="drop-effects" class="anchor">
                "Drop Effects"
                <AnchorLink href="#drop-effects" description="Direct link to drop effects"/>
            </h2>

            <ul>
                <li><code>"DropEffect::Copy"</code>" - Create a copy of the dragged item"</li>
                <li><code>"DropEffect::Move"</code>" - Move the item (remove from source)"</li>
                <li><code>"DropEffect::Link"</code>" - Create a link/reference"</li>
                <li><code>"DropEffect::All"</code>" - Allow all effects"</li>
                <li><code>"DropEffect::None"</code>" - Disallow dropping"</li>
            </ul>

            <h2 id="drag-item" class="anchor">
                "DragItem"
                <AnchorLink href="#drag-item" description="Direct link to DragItem"/>
            </h2>

            <p>"Data that can be transferred during a drag operation:"</p>

            <Code>
                {indoc!(r#"
                    // Plain text
                    DragItem::text("Hello, World!")

                    // JSON data
                    DragItem::json("{\"id\": 1, \"name\": \"Item\"}")

                    // Custom MIME type
                    DragItem::custom("application/x-my-type", "custom data")
                "#)}
            </Code>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Draggable items with custom data (text, JSON, or custom MIME types)"</li>
                <li>"Drop zones with type filtering"</li>
                <li>"Collection reordering support"</li>
                <li>"Insert/remove between collections"</li>
                <li>"Visual feedback via is_dragging and is_drop_target signals"</li>
                <li>"ARIA attributes for accessibility (aria-grabbed, aria-dropeffect)"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Drag and Drop", link: "#dnd" },
                Toc::Leaf { title: "use_draggable", link: "#use_draggable" },
                Toc::Leaf { title: "use_droppable", link: "#use_droppable" },
                Toc::Leaf { title: "Drag to Drop", link: "#drag-to-drop" },
                Toc::Leaf { title: "use_drag_and_drop", link: "#use_drag_and_drop" },
                Toc::Leaf { title: "Drop Positions", link: "#drop-positions" },
                Toc::Leaf { title: "Drop Effects", link: "#drop-effects" },
                Toc::Leaf { title: "DragItem", link: "#drag-item" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}

/// Demo: Basic use_draggable
#[component]
fn DraggableDemo() -> impl IntoView {
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
        get_items: Callback::new(|_| vec![DragItem::text("Hello from draggable!")]),
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

        <p>"Last " { move || events.with(|events| events.occupied_len()) } " events:"</p>
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

/// Demo: Basic use_droppable
#[component]
fn DroppableDemo() -> impl IntoView {
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
                if item.kind == "text/plain" {
                    set_dropped_items.update(|items| items.push(item.data));
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

        <p>"Last " { move || events.with(|events| events.occupied_len()) } " events:"</p>
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

/// Demo: Combined drag-to-drop
#[component]
fn DragToDropDemo() -> impl IntoView {
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
        get_items: Callback::new(|_| {
            vec![
                DragItem::text("Package"),
                DragItem::json(r#"{"type": "package", "id": 1}"#),
            ]
        }),
        allowed_drop_effect: DropEffect::Copy,
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

/// Demo: List reordering with use_drag_and_drop
#[component]
fn ReorderDemo() -> impl IntoView {
    let (items, set_items) = signal(vec![
        ("item-1".to_string(), "First Item".to_string()),
        ("item-2".to_string(), "Second Item".to_string()),
        ("item-3".to_string(), "Third Item".to_string()),
        ("item-4".to_string(), "Fourth Item".to_string()),
        ("item-5".to_string(), "Fifth Item".to_string()),
    ]);

    let (events, set_events) = signal(HeapRb::<Oco<'static, str>>::new(10));

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

    // Collection-level hook - creates shared state for items
    let dnd = use_drag_and_drop(UseDragAndDropInput {
        drag_options: Some(DraggableOptions {
            get_items: Callback::new(|key: String| vec![DragItem::text(key)]),
            allowed_drop_effect: DropEffect::Move,
            on_drag_start: Some(Callback::new(move |_| {
                set_events.update(|events| {
                    events.push_overwrite(Oco::Borrowed("DragStart"));
                });
            })),
            on_drag_end: Some(Callback::new(move |e: DragEndEvent| {
                set_events.update(|events| {
                    events.push_overwrite(Oco::Owned(format!("DragEnd: {:?}", e.drop_effect)));
                });
            })),
            ..Default::default()
        }),
        drop_options: Some(DroppableOptions {
            accepted_types: vec!["text/plain".to_string()],
            ..Default::default()
        }),
        on_reorder: Some(Callback::new(move |e: ReorderEvent| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!(
                    "Reorder: {:?} -> {} ({:?})",
                    e.keys, e.target.key, e.target.position
                )));
            });

            // Perform the reorder
            set_items.update(|items| {
                if let Some(dragged_key) = e.keys.first() {
                    let from_idx = items.iter().position(|(k, _)| k == dragged_key);
                    let to_idx = items.iter().position(|(k, _)| k == &e.target.key);

                    if let (Some(from), Some(to)) = (from_idx, to_idx) {
                        if from != to {
                            let item = items.remove(from);
                            let insert_at = if from < to { to } else { to + 1 };
                            items.insert(insert_at.min(items.len()), item);
                        }
                    }
                }
            });
        })),
        ..Default::default()
    });

    view! {
        <div style="display: flex; gap: 2em; margin: 1em 0;">
            <div style="flex: 1;">
                <p style="margin: 0 0 0.5em 0; font-weight: bold;">"Drag items to reorder:"</p>
                <div style="display: flex; flex-direction: column; gap: 0.25em;">
                    {move || items.get().into_iter().map(|(key, label)| {
                        // Pass state to each item - hooks are called at component level
                        view! {
                            <ReorderItem
                                key=key
                                label=label
                                state=dnd.state.clone()
                                is_dragging=dnd.is_dragging
                            />
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
            <div style="flex: 1;">
                <p style="margin: 0 0 0.5em 0; font-weight: bold;">"Events:"</p>
                <pre style="
                    width: 100%;
                    height: 10em;
                    overflow: auto;
                    padding: var(--typography-code-padding);
                    border: none;
                    border-radius: var(--typography-code-border-radius);
                    background-color: var(--typography-code-background-color);
                    color: var(--typography-code-color);
                ">
                    { move || string.get() }
                </pre>
            </div>
        </div>
    }
}

/// Individual reorderable item that calls hooks at component level.
///
/// This is the key fix: hooks (`use_draggable_item` and `use_droppable_item`) are called
/// directly in the component body, not inside callbacks. This ensures event handlers
/// are properly attached during SSR hydration.
#[component]
fn ReorderItem(
    key: String,
    label: String,
    state: DragAndDropState,
    #[allow(unused_variables)] is_dragging: Signal<bool>,
) -> impl IntoView {
    // Hooks called at component level - this is critical for SSR hydration!
    let drag = use_draggable_item(key.clone(), state.clone());
    let drop = use_droppable_item(key, state);

    let is_this_dragging = drag
        .as_ref()
        .map(|d| d.is_dragging)
        .unwrap_or_else(|| Signal::derive(|| false));
    let is_drop_target = drop
        .as_ref()
        .map(|d| d.is_drop_target)
        .unwrap_or_else(|| Signal::derive(|| false));

    let style = move || {
        let dragging = is_this_dragging.get();
        let target = is_drop_target.get();
        format!(
            "padding: 0.75em 1em; border-radius: 8px; cursor: grab; user-select: none; \
            display: flex; align-items: center; gap: 0.5em; \
            background: {}; border: 2px solid {}; transition: all 0.15s; {}",
            if dragging {
                "var(--brand-color)"
            } else if target {
                "rgba(var(--brand-color-rgb), 0.1)"
            } else {
                "white"
            },
            if dragging {
                "var(--brand-color)"
            } else if target {
                "var(--brand-color)"
            } else {
                "#ddd"
            },
            if dragging {
                "color: white; opacity: 0.8; transform: scale(1.02);"
            } else {
                ""
            }
        )
    };

    // We need to conditionally spread the props
    match (drag, drop) {
        (Some(drag), Some(drop)) => view! {
            <div {..drag.drag_props.into_attrs()} {..drop.drop_props.into_attrs()} style=style>
                <span style="color: #999;">"⋮⋮"</span>
                { label }
            </div>
        }
        .into_any(),
        (Some(drag), None) => view! {
            <div {..drag.drag_props.into_attrs()} style=style>
                <span style="color: #999;">"⋮⋮"</span>
                { label }
            </div>
        }
        .into_any(),
        (None, Some(drop)) => view! {
            <div {..drop.drop_props.into_attrs()} style=style>
                <span style="color: #999;">"⋮⋮"</span>
                { label }
            </div>
        }
        .into_any(),
        (None, None) => view! {
            <div style=style>
                <span style="color: #999;">"⋮⋮"</span>
                { label }
            </div>
        }
        .into_any(),
    }
}
