use leptonic::{
    atoms::{button::Button, checkbox::{CheckboxButton, CheckboxField}},
    hooks::{
        ClipboardAction, DragItem, DropItem, IntoAttrs, UseClipboardInput, UseClipboardReturn,
        use_clipboard,
    },
};
use leptos::prelude::*;

fn default_items() -> Vec<String> {
    vec!["Milk".to_owned(), "Bread".to_owned(), "Apples".to_owned()]
}

fn items_text(count: usize) -> String {
    if count == 1 {
        "1 item".to_owned()
    } else {
        format!("{count} items")
    }
}

#[component]
pub fn ClipboardDemo() -> impl IntoView {
    let items = RwSignal::new(default_items());
    let last = RwSignal::new(String::from("none yet"));
    let disabled = RwSignal::new(false);

    let UseClipboardReturn { clipboard_props } = use_clipboard(UseClipboardInput {
        // One clipboard item per list entry, as plain text.
        get_items: Some(Callback::new(move |_: ClipboardAction| {
            items
                .get_untracked()
                .into_iter()
                .map(DragItem::text)
                .collect()
        })),
        on_copy: Some(Callback::new(move |()| {
            last.set(format!(
                "copied {}",
                items_text(items.with_untracked(Vec::len))
            ));
        })),
        // Called after the items were written to the clipboard: remove them.
        on_cut: Some(Callback::new(move |()| {
            let count = items.with_untracked(Vec::len);
            items.set(Vec::new());
            last.set(format!("cut {}", items_text(count)));
        })),
        // Text pasted from this list arrives as one item per entry, text from elsewhere as one item: add every line.
        on_paste: Some(Callback::new(move |pasted: Vec<DropItem>| {
            let mut lines = Vec::new();
            for item in &pasted {
                if let DropItem::Text(text) = item
                    && let Some(text) = text.get_text("text/plain")
                {
                    lines.extend(
                        text.lines()
                            .map(str::trim)
                            .filter(|line| !line.is_empty())
                            .map(ToOwned::to_owned),
                    );
                }
            }
            last.set(format!("pasted {}", items_text(lines.len())));
            items.update(|items| items.extend(lines));
        })),
        is_disabled: disabled.into(),
    });

    view! {
        // Focusable, so that the clipboard shortcuts reach it.
        <div
            {..clipboard_props.into_attrs()}
            tabindex="0"
            role="group"
            aria-label="Shopping list"
            class="demo-clipboard-list"
        >
            <strong>"Shopping list"</strong>
            <Show
                when=move || items.with(|items| !items.is_empty())
                fallback=|| view! { <p class="demo-clipboard-empty">"Empty. Paste some lines."</p> }
            >
                <ul>
                    {move || items.get().into_iter().map(|item| view! { <li>{item}</li> }).collect_view()}
                </ul>
            </Show>
        </div>
        <p class="demo-status">"Last action: "{last}"."</p>
        <div class="demo-controls">
            <Button on_press=move |_| items.set(default_items()) classes="demo-btn">"Reset list"</Button>
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>
    }
}
