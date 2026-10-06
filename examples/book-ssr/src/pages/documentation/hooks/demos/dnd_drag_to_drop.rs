use leptonic::{
    components::prelude::*,
    hooks::{
        DragEndEvent, DragItem, DragType, DropEvent, DropItem, DropOperation, DropOperationQuery,
        IntoAttrs, UseDragInput, UseDragReturn, UseDropInput, UseDropReturn, use_drag, use_drop,
    },
    utils::CapturedElement,
};
use leptos::prelude::*;

const URL: &str = "https://leptos.dev";

/// Two cards and two drop targets. The inbox takes anything; the bookmarks only take links, and link them.
#[component]
pub fn DragToDropDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);
    let last_drag = RwSignal::new(String::from("none"));

    // A note: plain text, which may be moved, copied or linked (the default).
    let note = use_drag(UseDragInput {
        on_drag_end: Some(Callback::new(move |e: DragEndEvent| {
            last_drag.set(format!("Note, {:?}", e.drop_operation));
        })),
        is_disabled: disabled.into(),
        ..UseDragInput::new(Callback::new(|()| vec![DragItem::text("Water the plants")]))
    });
    // A link: a URL in two representations. Links can only be copied or linked, not moved.
    let link = use_drag(UseDragInput {
        get_allowed_drop_operations: Some(Callback::new(|()| {
            vec![DropOperation::Copy, DropOperation::Link]
        })),
        on_drag_end: Some(Callback::new(move |e: DragEndEvent| {
            last_drag.set(format!("Link, {:?}", e.drop_operation));
        })),
        is_disabled: disabled.into(),
        ..UseDragInput::new(Callback::new(|()| {
            vec![
                DragItem::new()
                    .with("text/uri-list", URL)
                    .with("text/plain", URL),
            ]
        }))
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

        <Checkbox state=disabled>"Disable dragging"</Checkbox>

        <div class="demo-state-display">
            <strong>"Last drag ended: "</strong>{last_drag}
        </div>
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
    let dropped = RwSignal::new(Vec::<String>::new());
    let UseDropReturn {
        drop_props,
        is_drop_target,
        ..
    } = use_drop(UseDropInput {
        get_drop_operation,
        on_drop: Some(Callback::new(move |e: DropEvent| {
            let operation = e.drop_operation;
            dropped.update(|dropped| {
                dropped.extend(
                    e.items
                        .iter()
                        .map(|item| format!("{} ({operation:?})", describe(item))),
                );
            });
        })),
        ..UseDropInput::new(CapturedElement::new())
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
                {move || dropped.get().into_iter().map(|entry| view! { <li>{entry}</li> }).collect_view()}
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
