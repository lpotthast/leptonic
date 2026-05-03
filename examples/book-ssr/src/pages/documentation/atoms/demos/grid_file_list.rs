use std::collections::HashSet;

use itertools::Itertools;
use leptonic::{
    atoms::grid_list::{GridList, GridListItem},
    hooks::{EscapeKeyBehavior, Selection, SelectionBehavior, SelectionMode},
};
use leptos::prelude::*;
use leptos_styles::{
    Style::{
        AlignItems, Background, Border, BorderBottom, BorderRadius, Cursor, Display, FontFamily,
        FontSize, Gap, MarginTop, MaxWidth, Outline, OutlineOffset, Overflow, Padding, UserSelect,
    },
    Styles,
};

fn list_item_style() -> Styles {
    Styles::builder()
        .with(Display, "flex")
        .with(AlignItems, "center")
        .with(Padding, "10px 14px")
        .with(Cursor, "pointer")
        .with(UserSelect, "none")
        .with("transition", "background-color 0.15s, outline-color 0.15s")
        .with(Outline, "2px solid transparent")
        .with(OutlineOffset, "-2px")
        .with(BorderBottom, "1px solid #e0e0e0")
        .with(Gap, "10px")
        .build()
}

fn state_display_style() -> Styles {
    Styles::from([
        (MarginTop, "1em"),
        (Padding, "0.75em 1em"),
        (Background, "#f5f5f5"),
        (BorderRadius, "6px"),
        (FontSize, "0.9em"),
        (FontFamily, "monospace"),
    ])
}

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

    let list_container_style = Styles::from([
        (Border, "1px solid #ccc"),
        (BorderRadius, "6px"),
        (Overflow, "hidden"),
        (MaxWidth, "360px"),
    ]);

    view! {
        <div style="padding: 1.5em; border: 1px solid #ddd; border-radius: 8px; margin: 1em 0;">
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
                styles=list_container_style
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
                                styles=list_item_style()
                            >
                                <span style="width: 24px; text-align: center; font-size: 1.1em;">
                                    {file_icon(icon)}
                                </span>
                                <span>{label}</span>
                            </GridListItem<String>>
                        }
                    })
                    .collect_view()}
            </GridList>

            <p style="font-size: 0.85em; color: #666; margin-top: 0.5em;">
                "\"Archive.zip\" is disabled — it is skipped during keyboard navigation and cannot be selected."
            </p>

            <div style=state_display_style()>
                <div>
                    <strong>"Selected: "</strong>
                    {move || format_selection(&selected.get())}
                </div>
                <div style="margin-top: 0.25em;">
                    <strong>"Last action: "</strong>
                    {move || last_action.get().unwrap_or_else(|| "None".to_string())}
                </div>
            </div>
        </div>

        <style>
            "article [role='row'][data-focused='true'] { outline-color: #1976d2 !important; }"
            "article [role='row'][data-selected='true'] { background-color: #e3f2fd; }"
            "article [role='row'][data-disabled='true'] { opacity: 0.4; cursor: not-allowed !important; }"
        </style>
    }
}
