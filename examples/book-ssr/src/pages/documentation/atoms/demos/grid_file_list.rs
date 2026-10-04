use std::collections::HashSet;

use itertools::Itertools;
use leptonic::{
    atoms::grid_list::{GridList, GridListItem},
    hooks::{EscapeKeyBehavior, Selection, SelectionBehavior, SelectionMode},
};
use leptos::prelude::*;

fn format_selection(sel: &Selection<String>) -> String {
    match sel {
        Selection::Keys(keys) => {
            if keys.is_empty() {
                "None".to_string()
            } else {
                let mut sorted: Vec<_> = keys.iter().collect();
                sorted.sort();
                sorted.into_iter().cloned().join(", ")
            }
        }
        Selection::All => "All".to_string(),
    }
}

fn file_icon(kind: &str) -> &'static str {
    match kind {
        "pdf" => "\u{1F4C4}",
        "img" => "\u{1F5BC}",
        "xls" | "ppt" => "\u{1F4CA}",
        "zip" => "\u{1F4E6}",
        _ => "\u{1F4C1}",
    }
}

#[component]
pub fn GridFileListDemo() -> impl IntoView {
    let items = [
        ("doc-1", "Document.pdf", "pdf"),
        ("img-1", "Photo.jpg", "img"),
        ("sheet-1", "Spreadsheet.xlsx", "xls"),
        ("pres-1", "Presentation.pptx", "ppt"),
        ("arch-1", "Archive.zip", "zip"),
    ];

    let all_keys: Signal<Vec<String>> = Signal::stored(
        items
            .iter()
            .map(|(k, _, _)| (*k).to_string())
            .collect::<Vec<_>>(),
    );

    let (selected, set_selected) = signal(Selection::<String>::default());
    let (last_action, set_last_action) = signal::<Option<String>>(None);

    let disabled_keys: Signal<HashSet<String>> = Signal::stored(
        ["arch-1".to_string()]
            .into_iter()
            .collect::<HashSet<String>>(),
    );

    view! {
        <div class="demo-frame">
            <GridList
                all_keys
                disabled_keys
                selection_mode=SelectionMode::Multiple
                selection_behavior=SelectionBehavior::Toggle
                selected_keys=selected
                on_selection_change=Callback::new(move |sel| set_selected.set(sel))
                escape_key_behavior=EscapeKeyBehavior::ClearSelection
                on_action=Callback::new(move |key: String| {
                    set_last_action.set(Some(key));
                })
                label="Files".to_string()
                classes="demo-grid-list"
            >
                {items
                    .iter()
                    .enumerate()
                    .map(|(idx, (key, label, icon))| {
                        let key = (*key).to_string();
                        let label = *label;
                        let icon = *icon;
                        view! {
                            <GridListItem<String>
                                item_key=key
                                row_index=idx
                                text_value=label.to_string()
                                classes="demo-grid-list-item"
                            >
                                <span class="demo-grid-list-icon">{file_icon(icon)}</span>
                                <span>{label}</span>
                            </GridListItem<String>>
                        }
                    })
                    .collect_view()}
            </GridList>

            <p class="demo-caption">
                "\"Archive.zip\" is disabled — it is skipped during keyboard navigation and cannot be selected."
            </p>

            <div class="demo-state-display">
                <div>
                    <strong>"Selected: "</strong>
                    {move || format_selection(&selected.get())}
                </div>
                <div class="demo-mt-quarter">
                    <strong>"Last action: "</strong>
                    {move || last_action.get().unwrap_or_else(|| "None".to_string())}
                </div>
            </div>
        </div>
    }
}
