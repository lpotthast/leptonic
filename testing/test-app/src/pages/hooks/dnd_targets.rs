use leptonic::{
    CapturedElement, IntoAttrs, flag,
    hooks::dnd::{
        DragEndEvent, DragItem, DropActivateEvent, DropEnterEvent, DropEvent, DropExitEvent,
        DropItem, DropOperation, DropOperationQuery, UseDragInput, UseDragReturn, UseDropInput,
        UseDropReturn, use_drag, use_drop,
    },
};
use leptos::{ev, prelude::*, web_sys};
use leptos_router::hooks::use_query_map;

/// The custom event (on `#test-page-hook-dnd-targets`, `detail`: the action) that changes the
/// page during a drag, when the drag manager blocks clicks: `add-target` mounts "Drop here 3",
/// `remove-target` unmounts "Drop here 2", `hide-target` hides it and `hide-draggable` the
/// draggable with `aria-hidden`, `add-input` mounts the text field "Text field 2".
pub const ACTION_EVENT: &str = "dnd-action";

/// Drag and drop between single elements (react-aria's `dnd.test.js` "keyboard", "screen reader"
/// and "native drag and drop" setups): a draggable, elements that aren't drop targets, and drop
/// targets "Drop here" (1) and "Drop here 2" (2). `#test-dnd-targets-log` logs the events.
///
/// Query parameters (`?ancestor&op=copy`, ...):
/// - `ancestor`: the draggable is inside the drop target "Ancestor" (0).
/// - `hidden-tree`: a drop target "Hidden target" (9) inside an `aria-hidden` element.
/// - `disabled-drag` / `disabled-drop`: the draggable / "Drop here" is disabled.
/// - `cancel-2`: "Drop here 2" doesn't take the drag (`get_drop_operation`: cancel).
/// - `op=copy|link|move`: the drop targets' preferred operation.
/// - `allowed=copy|link|move`: the only operation the drag allows.
///
/// Every drop target's `get_drop_operation` reads the log signal, so a drop target registered
/// during a drag must not subscribe to the signals they read.
#[component]
pub fn PageHookDndTargets() -> impl IntoView {
    let query = use_query_map();
    let has = move |name: &str| query.with_untracked(|q| q.get(name).is_some());
    let operation = move |name: &str| {
        query.with_untracked(|q| match q.get(name).as_deref() {
            Some("copy") => Some(DropOperation::Copy),
            Some("link") => Some(DropOperation::Link),
            Some("move") => Some(DropOperation::Move),
            _ => None,
        })
    };
    let ancestor = has("ancestor");
    let hidden_tree = has("hidden-tree");
    let disabled_drag = has("disabled-drag");
    let disabled_drop = has("disabled-drop");
    let cancel_2 = has("cancel-2");
    let preferred = operation("op");
    let allowed = operation("allowed");

    let log = RwSignal::new(Vec::<String>::new());
    let added = RwSignal::new(false);
    let removed = RwSignal::new(false);
    let target_hidden = RwSignal::new(false);
    let draggable_hidden = RwSignal::new(false);
    let input_added = RwSignal::new(false);
    let on_action = move |e: web_sys::CustomEvent| match e.detail().as_string().as_deref() {
        Some("add-target") => added.set(true),
        Some("remove-target") => removed.set(true),
        Some("hide-target") => target_hidden.set(true),
        Some("hide-draggable") => draggable_hidden.set(true),
        Some("add-input") => input_added.set(true),
        _ => {}
    };
    let droppable = move |label: &'static str, index: usize, is_disabled: bool, cancel: bool| {
        view! { <Droppable label index log is_disabled cancel preferred /> }
    };
    let draggable = move || view! { <Draggable log is_disabled=disabled_drag allowed /> };

    view! {
        <div
            id="test-page-hook-dnd-targets"
            {..leptos::tachys::html::event::on(
                ev::Custom::<web_sys::CustomEvent>::new(ACTION_EVENT),
                on_action,
            )}
        >
            <h1>"Drag and drop targets"</h1>
            <button>"Before"</button>
            <div
                aria-hidden=move || draggable_hidden.get().then_some("true")
                on:keydown=move |e: web_sys::KeyboardEvent| {
                    if e.key() == "Enter" {
                        log.update(|l| l.push("parent keydown Enter".to_owned()));
                    }
                }
            >
                {if ancestor {
                    view! {
                        <Droppable
                            label="Ancestor"
                            index=0
                            log
                            is_disabled=false
                            cancel=false
                            preferred
                        >
                            {draggable()}
                        </Droppable>
                    }
                        .into_any()
                } else {
                    draggable().into_any()
                }}
            </div>
            <button>"Not a drop target"</button>
            {droppable("Drop here", 1, disabled_drop, false)}
            <input aria-label="Text field" />
            <Show when=move || !removed.get()>
                <div aria-hidden=move || {
                    target_hidden.get().then_some("true")
                }>{droppable("Drop here 2", 2, false, cancel_2)}</div>
            </Show>
            <span>"Text"</span>
            {hidden_tree
                .then(|| {
                    view! {
                        <div aria-hidden="true">{droppable("Hidden target", 9, false, false)}</div>
                    }
                })}
            <Show when=move || added.get()>{droppable("Drop here 3", 3, false, false)}</Show>
            <Show when=move || input_added.get()>
                <input aria-label="Text field 2" />
            </Show>
            <ol id="test-dnd-targets-log">
                {move || log.get().into_iter().map(|e| view! { <li>{e}</li> }).collect_view()}
            </ol>
        </div>
    }
}

#[component]
fn Draggable(
    log: RwSignal<Vec<String>>,
    is_disabled: bool,
    allowed: Option<DropOperation>,
) -> impl IntoView {
    let push = move |entry: String| log.update(|l| l.push(entry));
    let UseDragReturn {
        drag_props,
        is_dragging,
        ..
    } = use_drag(UseDragInput {
        on_drag_start: Some(Callback::new(move |_| push("dragstart".to_owned()))),
        on_drag_end: Some(Callback::new(move |e: DragEndEvent| {
            push(format!("dragend {:?}", e.drop_operation));
        })),
        items: Signal::stored(vec![DragItem::text("hello world")]),
        allowed_drop_operations: allowed.map(|op| Signal::stored(vec![op])),
        preview: None,
        on_drag_move: None,
        has_drag_button: false,
        is_disabled: Signal::stored(is_disabled),
    });
    view! {
        <div role="button" tabindex="0" {..drag_props.into_attrs()} data-dragging=flag(is_dragging)>
            "Drag me"
        </div>
    }
}

#[component]
fn Droppable(
    label: &'static str,
    index: usize,
    log: RwSignal<Vec<String>>,
    is_disabled: bool,
    /// Doesn't take any drag.
    cancel: bool,
    /// The operation to use when the drag allows it.
    preferred: Option<DropOperation>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let push = move |entry: String| log.update(|l| l.push(entry));
    let element = CapturedElement::new();
    let UseDropReturn {
        drop_props,
        is_drop_target,
        ..
    } = use_drop(UseDropInput {
        on_drop_enter: Some(Callback::new(move |_: DropEnterEvent| {
            push(format!("dropenter {index}"));
        })),
        on_drop_exit: Some(Callback::new(move |_: DropExitEvent| {
            push(format!("dropexit {index}"));
        })),
        on_drop_activate: Some(Callback::new(move |_: DropActivateEvent| {
            push(format!("dropactivate {index}"));
        })),
        on_drop: Some(Callback::new(move |e: DropEvent| {
            let text = e
                .items
                .iter()
                .find_map(|item| match item {
                    DropItem::Text(text) => text.get_text("text/plain").map(ToOwned::to_owned),
                    _ => None,
                })
                .unwrap_or_default();
            push(format!("drop {index} {text} {:?}", e.drop_operation));
        })),
        element,
        get_drop_operation: Some(Callback::new(move |q: DropOperationQuery| {
            // Reads a signal (see the page's docs).
            log.track();
            if cancel {
                return DropOperation::Cancel;
            }
            preferred
                .filter(|op| q.allowed_operations.contains(op))
                .or_else(|| q.allowed_operations.first().copied())
                .unwrap_or(DropOperation::Cancel)
        })),
        get_drop_operation_for_point: None,
        on_drop_move: None,
        has_drop_button: false,
        is_disabled: Signal::stored(is_disabled),
    });
    view! {
        <div
            role="button"
            tabindex="0"
            {..drop_props.into_attrs()}
            data-drop-target=flag(is_drop_target)
        >
            {label}
            {children.map(|children| children())}
        </div>
    }
}
