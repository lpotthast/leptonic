use std::collections::HashSet;

use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn GridListDemo() -> impl IntoView {
    let items = vec![
        ("file-1", "Document.pdf"),
        ("file-2", "Photo.jpg"),
        ("file-3", "Spreadsheet.xlsx"),
        ("file-4", "Presentation.pptx"),
        ("file-5", "Archive.zip"),
    ];

    let all_keys = Signal::stored(items.iter().map(|(k, _)| k.to_string()).collect::<Vec<_>>());

    let (list_selected, set_list_selected) = signal(Selection::<String>::default());
    let (last_action, set_last_action) = signal::<Option<String>>(None);

    let grid_list = use_grid_list(UseGridListInput {
        label: Some("Files".to_string()),
        all_keys,
        disabled_keys: Signal::derive(HashSet::new),
        selection_mode: SelectionMode::Multiple,
        selection_behavior: SelectionBehavior::Toggle,
        selected_keys: Some(list_selected.into()),
        on_selection_change: Some(Callback::new(move |sel| set_list_selected.set(sel))),
        escape_key_behavior: EscapeKeyBehavior::ClearSelection,
        on_action: Some(Callback::new(move |key: String| {
            set_last_action.set(Some(key));
        })),
        ..Default::default()
    });

    let list_focused_key = grid_list.focused_key;

    view! {
        <div
            {..grid_list.props.into_attrs()}
            style="margin: 1em 0; border: 1px solid #ccc; border-radius: 4px; overflow: hidden;"
        >
            {items.into_iter().enumerate().map(|(idx, (key, label))| {
                let item = use_grid_list_item(UseGridListItemInput {
                    state: grid_list.state,
                    key: key.to_string(),
                    row_index: idx,
                    is_disabled: false.into(),
                    text_value: Some(label.to_string()),
                });

                let is_selected = item.is_selected;
                let is_focused = item.is_focused;
                let (row_props, row_styles) = item.row_props.into_parts();

                let row_styles = row_styles
                    .add("display", "flex")
                    .add("align-items", "center")
                    .add("padding", "8px 12px")
                    .add("cursor", "pointer")
                    .add("user-select", "none");

                view! {
                    <div
                        {..row_props}
                        style=row_styles
                        style:background=move || if is_selected.get() { "#e3f2fd" } else { "transparent" }
                        style:outline=move || if is_focused.get() { "2px solid #1976d2" } else { "none" }
                        style:outline-offset="-2px"
                    >
                        <div {..item.gridcell_props.into_attrs()} style="display: flex; align-items: center; gap: 8px; width: 100%;">
                            <span style="width: 20px; text-align: center;">
                                {move || if is_selected.get() { "\u{2713}" } else { "" }}
                            </span>
                            <span>{label}</span>
                        </div>
                    </div>
                }
            }).collect_view()}
        </div>

        <div style="margin-top: 1em;">
            <strong>"Focused: "</strong>
            { move || {
                list_focused_key.get().unwrap_or_else(|| "None".to_string())
            }}
        </div>

        <div style="margin-top: 0.5em;">
            <strong>"Selected: "</strong>
            { move || {
                match list_selected.get() {
                    Selection::Keys(keys) => {
                        if keys.is_empty() {
                            "None".to_string()
                        } else {
                            let mut sorted: Vec<_> = keys.into_iter().collect();
                            sorted.sort();
                            sorted.join(", ")
                        }
                    }
                    Selection::All => "All".to_string(),
                }
            }}
        </div>

        <div style="margin-top: 0.5em;">
            <strong>"Last action: "</strong>
            { move || {
                last_action.get().unwrap_or_else(|| "None".to_string())
            }}
        </div>
    }
}
