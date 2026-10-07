use leptonic::{
    atoms::checkbox::Checkbox,
    hooks::{
        DragEndEvent, DragItem, DragType, DropEvent, DropItem, DropOperation, DropOperationQuery,
        IntoAttrs, UseDragInput, UseDragReturn, UseDropInput, UseDropReturn, use_drag, use_drop,
    },
    utils::CapturedElement,
};
use leptos::prelude::*;
use ringbuf::{
    HeapRb,
    traits::{Consumer, RingBuffer},
};

const URL: &str = "https://leptos.dev";

/// Two cards and two drop targets. The inbox takes anything; the bookmarks only take links, and link them.
#[component]
pub fn DragToDropDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);
    let last_drag = RwSignal::new(String::from("none yet"));

    // A note: plain text, which may be moved, copied or linked (the default).
    let note = use_drag(UseDragInput {
        on_drag_end: Some(Callback::new(move |e: DragEndEvent| {
            last_drag.set(format!("Note ({:?})", e.drop_operation));
        })),
        is_disabled: disabled.into(),
        items: Signal::stored(vec![DragItem::text("Water the plants")]),
        allowed_drop_operations: None,
        preview: None,
        on_drag_start: None,
        on_drag_move: None,
        has_drag_button: false,
    });
    // A link: a URL in two representations. Links can only be copied or linked, not moved.
    let link = use_drag(UseDragInput {
        allowed_drop_operations: Some(Signal::stored(vec![
            DropOperation::Copy,
            DropOperation::Link,
        ])),
        on_drag_end: Some(Callback::new(move |e: DragEndEvent| {
            last_drag.set(format!("Link ({:?})", e.drop_operation));
        })),
        is_disabled: disabled.into(),
        items: Signal::stored(vec![
            DragItem::new()
                .with("text/uri-list", URL)
                .with("text/plain", URL),
        ]),
        preview: None,
        on_drag_start: None,
        on_drag_move: None,
        has_drag_button: false,
    });

    view! {
        <div class="demo-dnd-board">
            <div class="demo-dnd-cards">
                <Card drag=note title="Note" content="Water the plants"/>
                <Card drag=link title="Link" content=URL/>
            </div>
            <div class="demo-dnd-targets">
                // Takes any data, with the first operation the drag allows.
                <Target title="Inbox" hint="Accepts anything" get_drop_operation=None/>
                // Takes links only, and links them.
                <Target
                    title="Bookmarks"
                    hint="Accepts links"
                    get_drop_operation=Some(Callback::new(|q: DropOperationQuery| {
                        if q.types.has(&DragType::from("text/uri-list"))
                            && q.allowed_operations.contains(&DropOperation::Link)
                        {
                            DropOperation::Link
                        } else {
                            DropOperation::Cancel
                        }
                    }))
                />
            </div>
        </div>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Disabled"
            </Checkbox>
        </div>

        <p class="demo-status">"Last drag ended: "{last_drag}</p>
    }
}

#[component]
fn Card(drag: UseDragReturn, title: &'static str, content: &'static str) -> impl IntoView {
    let is_dragging = drag.is_dragging;
    view! {
        // Focusable, so that Enter starts a keyboard drag.
        <div
            {..drag.drag_props.into_attrs()}
            role="button"
            tabindex="0"
            class="demo-dnd-card"
            data-dragging=move || is_dragging.get().then_some("")
        >
            <strong>{title}</strong>
            <span>{content}</span>
        </div>
    }
}

#[component]
fn Target(
    title: &'static str,
    hint: &'static str,
    get_drop_operation: Option<Callback<DropOperationQuery, DropOperation>>,
) -> impl IntoView {
    // The last 50 drops.
    let dropped = RwSignal::new(HeapRb::<String>::new(50));
    let UseDropReturn {
        drop_props,
        is_drop_target,
        ..
    } = use_drop(UseDropInput {
        get_drop_operation,
        on_drop: Some(Callback::new(move |e: DropEvent| {
            let operation = e.drop_operation;
            dropped.update(|dropped| {
                for item in &e.items {
                    dropped.push_overwrite(format!("{} ({operation:?})", describe(item)));
                }
            });
        })),
        element: CapturedElement::new(),
        get_drop_operation_for_point: None,
        on_drop_enter: None,
        on_drop_move: None,
        on_drop_activate: None,
        on_drop_exit: None,
        has_drop_button: false,
        is_disabled: Signal::stored(false),
    });

    view! {
        <div class="demo-dnd-target-column">
            // The props capture the element; keyboard drags focus it.
            <div
                {..drop_props.into_attrs()}
                role="button"
                tabindex="0"
                class="demo-dnd-target"
                data-drop-target=move || is_drop_target.get().then_some("")
            >
                <strong>{title}</strong>
                <span class="demo-caption">{hint}</span>
            </div>
            <ul class="demo-dnd-dropped" aria-label=format!("Dropped on {title}")>
                {move || dropped.with(|dropped| dropped.iter().cloned().map(|entry| view! { <li>{entry}</li> }).collect_view())}
            </ul>
        </div>
    }
}

/// What was dropped: the link or text of text items, the names of files and directories.
fn describe(item: &DropItem) -> String {
    match item {
        DropItem::Text(text) => text
            .get_text("text/uri-list")
            .or_else(|| text.get_text("text/plain"))
            .unwrap_or("(other data)")
            .to_owned(),
        DropItem::File(file) => format!("File {}", file.name),
        DropItem::Directory(directory) => format!("Folder {}", directory.name),
    }
}
