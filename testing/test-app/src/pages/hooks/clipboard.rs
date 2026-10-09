use leptonic::{
    IntoAttrs,
    hooks::{
        clipboard::{ClipboardAction, UseClipboardInput, use_clipboard},
        dnd::{DragItem, DropItem},
    },
};
use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

/// `use_clipboard` on a focusable "Copy" button (react-aria's `useClipboard.test.js` setup).
/// `#test-clipboard-log` logs `copy`, `cut` and `paste <items>` (each item `type=text` pairs,
/// sorted by type, items separated by ` | `).
///
/// Query parameters:
/// - `items=none|custom|multiple|types|multiple-types|action`: the items to cut or copy (default:
///   one `text/plain` "hello world"; `action`: the type is the action, `copy` or `cut`).
/// - `cut`, `paste`: the button handles cut / paste.
#[component]
pub fn PageHookClipboard() -> impl IntoView {
    let query = use_query_map();
    let (items, cut, paste) = query.with_untracked(|q| {
        (
            q.get("items").unwrap_or_default(),
            q.get("cut").is_some(),
            q.get("paste").is_some(),
        )
    });
    let log = RwSignal::new(Vec::<String>::new());
    let push = move |entry: String| log.update(|l| l.push(entry));

    let get_items = match items.as_str() {
        "none" => None,
        kind => {
            let kind = kind.to_owned();
            Some(Callback::new(move |action: ClipboardAction| {
                match kind.as_str() {
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
                    "action" => vec![DragItem::new().with(
                        match action {
                            ClipboardAction::Copy => "copy",
                            ClipboardAction::Cut => "cut",
                        },
                        "test data",
                    )],
                    _ => vec![DragItem::text("hello world")],
                }
            }))
        }
    };
    let clipboard = use_clipboard(UseClipboardInput {
        get_items,
        on_copy: Some(Callback::new(move |()| push("copy".to_owned()))),
        on_cut: cut.then(|| Callback::new(move |()| push("cut".to_owned()))),
        on_paste: paste.then(|| {
            Callback::new(move |items: Vec<DropItem>| {
                let items: Vec<String> = items
                    .iter()
                    .map(|item| match item {
                        DropItem::Text(text) => {
                            let mut types: Vec<&str> = text.types().collect();
                            types.sort_unstable();
                            types
                                .iter()
                                .map(|t| format!("{t}={}", text.get_text(t).unwrap_or_default()))
                                .collect::<Vec<_>>()
                                .join(" ")
                        }
                        DropItem::File(_) => "file".to_owned(),
                        DropItem::Directory(_) => "directory".to_owned(),
                    })
                    .collect();
                push(format!("paste {}", items.join(" | ")));
            })
        }),
        is_disabled: Signal::stored(false),
    });
    view! {
        <div id="test-page-hook-clipboard">
            <h1>"Clipboard"</h1>
            <button>"Before"</button>
            <div role="button" tabindex="0" {..clipboard.clipboard_props.into_attrs()}>
                "Copy"
            </div>
            <ol id="test-clipboard-log">
                {move || log.get().into_iter().map(|e| view! { <li>{e}</li> }).collect_view()}
            </ol>
        </div>
    }
}
