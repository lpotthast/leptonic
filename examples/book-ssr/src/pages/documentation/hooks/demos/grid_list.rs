use std::collections::HashSet;

use leptonic::{hooks::*, utils::classes::Classes};
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
        <div {..grid_list.props.into_attrs()} class="demo-grid-list demo-my-1">
            {items
                .into_iter()
                .enumerate()
                .map(|(idx, (key, label))| {
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

                    view! {
                        <div
                            {..row_props}
                            class=Classes::from("demo-grid-list-item")
                                .add_reactive("selected", is_selected)
                                .add_reactive("focused", is_focused)
                            style=row_styles
                        >
                            <div {..item.gridcell_props.into_attrs()} class="demo-grid-list-cell">
                                <span class="demo-grid-list-icon">
                                    {move || if is_selected.get() { "\u{2713}" } else { "" }}
                                </span>
                                <span>{label}</span>
                            </div>
                        </div>
                    }
                })
                .collect_view()}
        </div>

        <div class="demo-mt-1">
            <strong>"Focused: "</strong>
            {move || { list_focused_key.get().unwrap_or_else(|| "None".to_string()) }}
        </div>

        <div class="demo-mt-half">
            <strong>"Selected: "</strong>
            {move || {
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

        <div class="demo-mt-half">
            <strong>"Last action: "</strong>
            {move || { last_action.get().unwrap_or_else(|| "None".to_string()) }}
        </div>
    }
}
