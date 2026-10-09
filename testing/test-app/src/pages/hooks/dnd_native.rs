use std::{future::Future, pin::Pin};

use leptonic::{
    CapturedElement, IntoAttrs, flag,
    hooks::dnd::{
        DragEndEvent, DragItem, DragPreview, DragType, DropActivateEvent, DropEnterEvent,
        DropEvent, DropExitEvent, DropItem, DropMoveEvent, DropOperation, DropOperationQuery,
        UseDragInput, UseDragReturn, UseDropInput, UseDropReturn, use_drag, use_drop,
    },
};
use leptos::{html, prelude::*};
use leptos_router::hooks::use_query_map;
use send_wrapper::SendWrapper;

use crate::pages::Section;

/// The types the drop target's `get_drop_operation` reports having (`DragTypes` can only be asked
/// whether it has a type).
const REPORTED_TYPES: [&str; 6] = [
    "test",
    "text/plain",
    "text/html",
    "image/jpeg",
    "application/octet-stream",
    "application/vnd.react-aria.items+json",
];

/// Native drags between single elements, also driven by the keyboard (react-aria's `dnd.test.js`
/// `Draggable`/`Droppable` setups). Elements are found by `data-name`; `#test-dnd-native-log`
/// logs what they report, prefixed by their name: `<source> start`, `<source> move`,
/// `<source> end <operation>`, `<target> enter`, `<target> move <x> <y>`, `<target> activate`,
/// `<target> exit`, `<target> drop <operation>` followed by one `<target> item <item>` per
/// dropped item (`text <type>=<data>, ...`, `file <type> <name>: <content>`, `directory <name>:
/// [<entries>]`), and `<target> operation <allowed>: <types>` for every `get_drop_operation` call.
///
/// Sections (`?only=<name>`):
/// - `basic`: the drag source "source" ("Drag me") and the drop target "target" ("Drop here",
///   with a child element "child").
/// - `nested-drag`: the drag source "child" inside the drag source "parent".
/// - `nested-drop`: the drag source "source" and the drop target "inner" inside the drop target
///   "outer" (nearer to the source than "outer").
/// - `removable`: the drag source "source" and a button "Remove source" unmounting it.
///
/// Query parameters:
/// - `coords`: enter, exit, activate and drop also log their position (`<x> <y>`).
/// - `items=custom|multiple|types|multiple-types`: the dragged items (default: one `text/plain`
///   "hello world"), as in react-aria's "drag data" tests.
/// - `allowed=<operation>`: the only operation drags allow (`copy`, `link`, `move`).
/// - `op=<operation>`: what the drop targets' `get_drop_operation` returns (`cancel`, `copy`,
///   `link`, `move`; default: no `get_drop_operation`).
/// - `preview=<width>x<height>` (and `preview-offset=<x>,<y>`): drags show the preview element
///   "Drag preview" of that size.
#[component]
pub fn PageHookDndNative() -> impl IntoView {
    let query = use_query_map();
    let config = query.with_untracked(|q| Config {
        coords: q.get("coords").is_some(),
        items: q.get("items").unwrap_or_default(),
        allowed: q.get("allowed").as_deref().and_then(operation),
        operation: q.get("op").as_deref().and_then(operation),
        preview: q.get("preview").and_then(|size| {
            let (width, height) = size.split_once('x')?;
            Some((width.parse().ok()?, height.parse().ok()?))
        }),
        preview_offset: q.get("preview-offset").and_then(|offset| {
            let (x, y) = offset.split_once(',')?;
            Some(leptonic::Point {
                x: x.parse().ok()?,
                y: y.parse().ok()?,
            })
        }),
    });
    let log = RwSignal::new(Vec::<String>::new());
    let removed = RwSignal::new(false);
    // Owned by the page: the removable source reports its end after it was unmounted.
    let on_end = Callback::new(move |e: DragEndEvent| {
        log.update(|l| l.push(format!("source end {:?}", e.drop_operation)));
    });
    let config = StoredValue::new(config);
    let source = move |name: &'static str, label: &'static str| {
        view! { <Source name label log config=config.get_value() on_end /> }
    };
    let target = move |name: &'static str, label: &'static str, children: Option<AnyView>| {
        view! { <Target name label log config=config.get_value() inner=children /> }
    };

    view! {
        <div id="test-page-hook-dnd-native">
            <h1>"Native drag and drop"</h1>
            <button>"Before"</button>
            <Section name="basic">
                {source("source", "Drag me")}
                {target(
                    "target",
                    "Drop here",
                    Some(view! { <span data-name="child">"Child"</span> }.into_any()),
                )}
            </Section>
            <Section name="nested-drag">
                <NestedSources log config=config.get_value() on_end />
            </Section>
            <Section name="nested-drop">
                <div style="padding-top: 60px">
                    {target(
                        "outer",
                        "Drop here 1",
                        Some(target("inner", "Drop here 2", None).into_any()),
                    )}
                </div>
                {source("source", "Drag me")}
            </Section>
            <Section name="removable">
                <Show when=move || !removed.get()>{source("source", "Drag me")}</Show>
                <button on:click=move |_| removed.set(true)>"Remove source"</button>
            </Section>
            <ol id="test-dnd-native-log">
                {move || log.get().into_iter().map(|e| view! { <li>{e}</li> }).collect_view()}
            </ol>
        </div>
    }
}

/// What the query configures.
#[derive(Debug, Clone, Default)]
struct Config {
    coords: bool,
    items: String,
    allowed: Option<DropOperation>,
    operation: Option<DropOperation>,
    preview: Option<(f64, f64)>,
    preview_offset: Option<leptonic::Point>,
}

fn operation(name: &str) -> Option<DropOperation> {
    match name {
        "cancel" => Some(DropOperation::Cancel),
        "copy" => Some(DropOperation::Copy),
        "link" => Some(DropOperation::Link),
        "move" => Some(DropOperation::Move),
        _ => None,
    }
}

fn items(kind: &str) -> Vec<DragItem> {
    match kind {
        "custom" => vec![DragItem::new().with("test", "test data")],
        "multiple" => vec![
            DragItem::new().with("test", "item 1"),
            DragItem::new().with("test", "item 2"),
        ],
        "types" => vec![
            DragItem::new()
                .with("test", "test data")
                .with("text/plain", "test data"),
        ],
        "multiple-types" => vec![
            DragItem::new()
                .with("test", "item 1")
                .with("text/plain", "item 1"),
            DragItem::new()
                .with("test", "item 2")
                .with("text/plain", "item 2"),
        ],
        _ => vec![DragItem::text("hello world")],
    }
}

fn operations(operations: &[DropOperation]) -> String {
    operations
        .iter()
        .map(|op| format!("{op:?}").to_lowercase())
        .collect::<Vec<_>>()
        .join(",")
}

/// A drag source logging as `name`.
#[component]
fn Source(
    name: &'static str,
    label: &'static str,
    log: RwSignal<Vec<String>>,
    config: Config,
    on_end: Callback<DragEndEvent>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let push = move |entry: String| log.update(|l| l.push(entry));
    let preview_ref = NodeRef::<html::Div>::new();
    let preview_offset = config.preview_offset;
    let UseDragReturn {
        drag_props,
        is_dragging,
        ..
    } = use_drag(UseDragInput {
        items: Signal::stored(items(&config.items)),
        allowed_drop_operations: config.allowed.map(|op| Signal::stored(vec![op])),
        preview: config.preview.map(|_| {
            Callback::new(move |_: Vec<DragItem>| {
                preview_ref.get_untracked().map(|element| DragPreview {
                    element: SendWrapper::new(element.into()),
                    offset: preview_offset,
                })
            })
        }),
        on_drag_start: Some(Callback::new(move |_| push(format!("{name} start")))),
        on_drag_move: Some(Callback::new(move |_| push(format!("{name} move")))),
        on_drag_end: Some(if name == "source" {
            on_end
        } else {
            Callback::new(move |e: DragEndEvent| push(format!("{name} end {:?}", e.drop_operation)))
        }),
        has_drag_button: false,
        is_disabled: Signal::stored(false),
    });
    view! {
        <div
            role="button"
            tabindex="0"
            data-name=name.to_owned()
            {..drag_props.into_attrs()}
            data-dragging=flag(is_dragging)
        >
            {label}
            {children.map(|children| children())}
        </div>
        {config
            .preview
            .map(|(width, height)| {
                view! {
                    <div
                        node_ref=preview_ref
                        class="test-dnd-native-preview"
                        style=format!(
                            "position: fixed; left: -1000px; top: 0; width: {width}px; height: {height}px",
                        )
                    >
                        "Drag preview"
                    </div>
                }
            })}
    }
}

/// "Parent drag" with the drag source "Child drag" inside.
#[component]
fn NestedSources(
    log: RwSignal<Vec<String>>,
    config: Config,
    on_end: Callback<DragEndEvent>,
) -> impl IntoView {
    let child_config = config.clone();
    view! {
        <Source name="parent" label="Parent drag" log config on_end>
            <Source name="child" label="Child drag" log config=child_config on_end />
        </Source>
    }
}

/// How a dropped item is logged.
fn describe(item: DropItem) -> Pin<Box<dyn Future<Output = String>>> {
    Box::pin(async move {
        match item {
            DropItem::Text(text) => {
                let entries: Vec<String> = text
                    .types()
                    .map(|kind| format!("{kind}={}", text.get_text(kind).unwrap_or_default()))
                    .collect();
                format!("text {}", entries.join(", "))
            }
            DropItem::File(file) => {
                let content = file.get_text().await.unwrap_or_default();
                format!("file {} {}: {content}", file.kind, file.name)
            }
            DropItem::Directory(directory) => {
                let mut entries = Vec::new();
                for entry in directory.get_entries().await.unwrap_or_default() {
                    entries.push(describe(entry).await);
                }
                format!("directory {}: [{}]", directory.name, entries.join(", "))
            }
        }
    })
}

/// A drop target logging as `name`.
#[component]
fn Target(
    name: &'static str,
    label: &'static str,
    log: RwSignal<Vec<String>>,
    config: Config,
    inner: Option<AnyView>,
) -> impl IntoView {
    let push = move |entry: String| log.update(|l| l.push(entry));
    let coords = config.coords;
    let at = move |x: f64, y: f64| {
        if coords {
            format!(" {x:.0} {y:.0}")
        } else {
            String::new()
        }
    };
    let element = CapturedElement::new();
    let UseDropReturn {
        drop_props,
        is_drop_target,
        ..
    } = use_drop(UseDropInput {
        element,
        get_drop_operation: config.operation.map(|operation| {
            Callback::new(move |q: DropOperationQuery| {
                let types: Vec<&str> = REPORTED_TYPES
                    .into_iter()
                    .filter(|kind| q.types.has(&DragType::from(*kind)))
                    .collect();
                push(format!(
                    "{name} operation {}: {}",
                    operations(&q.allowed_operations),
                    types.join(" ")
                ));
                operation
            })
        }),
        get_drop_operation_for_point: None,
        on_drop_enter: Some(Callback::new(move |e: DropEnterEvent| {
            push(format!("{name} enter{}", at(e.x, e.y)));
        })),
        on_drop_move: Some(Callback::new(move |e: DropMoveEvent| {
            push(format!("{name} move {:.0} {:.0}", e.x, e.y));
        })),
        on_drop_activate: Some(Callback::new(move |e: DropActivateEvent| {
            push(format!("{name} activate{}", at(e.x, e.y)));
        })),
        on_drop_exit: Some(Callback::new(move |e: DropExitEvent| {
            push(format!("{name} exit{}", at(e.x, e.y)));
        })),
        on_drop: Some(Callback::new(move |e: DropEvent| {
            push(format!(
                "{name} drop{} {:?}",
                at(e.x, e.y),
                e.drop_operation
            ));
            leptos::task::spawn_local(async move {
                for item in e.items {
                    let item = describe(item).await;
                    push(format!("{name} item {item}"));
                }
            });
        })),
        has_drop_button: false,
        is_disabled: Signal::stored(false),
    });
    view! {
        <div
            role="button"
            tabindex="0"
            data-name=name.to_owned()
            {..drop_props.into_attrs()}
            data-drop-target=flag(is_drop_target)
        >
            {label}
            {inner}
        </div>
    }
}
