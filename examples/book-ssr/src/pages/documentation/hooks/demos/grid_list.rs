use leptonic::{
    hooks::{
        GridListData, IntoAttrs, SelectionBehavior, SelectionMode, UseGridListInput,
        UseGridListItemInput, UseGridListReturn,
        collections::{
            CollectionOptions, EscapeKeyBehavior, Key, Selection, SelectionOptions,
            UseListStateInput, use_list_collection, use_list_state,
        },
        use_grid_list, use_grid_list_item,
    },
    utils::{CapturedElement, classes::Classes},
};
use leptos::prelude::*;

const FILES: [(&str, &str); 5] = [
    ("doc", "Document.pdf"),
    ("photo", "Photo.jpg"),
    ("sheet", "Spreadsheet.xlsx"),
    ("slides", "Presentation.pptx"),
    ("archive", "Archive.zip"),
];

fn describe(selection: &Selection) -> String {
    match selection {
        Selection::All => "all".to_owned(),
        Selection::Keys(keys) if keys.is_empty() => "none".to_owned(),
        Selection::Keys(keys) => {
            let mut keys: Vec<String> = keys.iter().map(ToString::to_string).collect();
            keys.sort();
            keys.join(", ")
        }
    }
}

#[component]
pub fn GridListDemo() -> impl IntoView {
    let selected = RwSignal::new(String::from("none"));
    let last_action = RwSignal::new(None::<Key>);

    // The rows and their selection.
    let collection = use_list_collection(
        Signal::stored(FILES.to_vec()),
        |(key, _)| Key::from(*key),
        |(_, name)| (*name).to_owned(),
    );
    let state = use_list_state(UseListStateInput {
        collection,
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            selection_behavior: SelectionBehavior::Replace,
            on_selection_change: Some(Callback::new(move |selection| {
                selected.set(describe(&selection));
            })),
            ..SelectionOptions::default()
        },
    });

    let UseGridListReturn { props, data } = use_grid_list(UseGridListInput {
        aria_label: "Files".into(),
        options: CollectionOptions {
            escape_key_behavior: EscapeKeyBehavior::ClearSelection,
            ..CollectionOptions::default()
        },
        on_action: Some(Callback::new(move |key| last_action.set(Some(key)))),
        ..UseGridListInput::new(state, CapturedElement::new())
    });

    view! {
        <div {..props.into_attrs()} class="demo-grid-list">
            {FILES.map(|(key, name)| view! { <FileRow list=data.clone() key=Key::from(key) name/> }).collect_view()}
        </div>

        <div class="demo-state-display">
            <div>
                <strong>"Focused: "</strong>
                {move || state.selection.focused_key().map_or_else(|| "none".to_owned(), |key| key.to_string())}
            </div>
            <div><strong>"Selected: "</strong>{selected}</div>
            <div>
                <strong>"Last action: "</strong>
                {move || last_action.get().map_or_else(|| "none".to_owned(), |key| key.to_string())}
            </div>
        </div>
    }
}

#[component]
fn FileRow(list: GridListData, key: Key, name: &'static str) -> impl IntoView {
    let row = use_grid_list_item(UseGridListItemInput::new(list, key));
    let is_selected = row.is_selected;
    let (row_attrs, row_styles) = row.row_props.into_parts();

    view! {
        <div
            {..row_attrs}
            class=Classes::from("demo-grid-list-item")
                .add_reactive("selected", is_selected)
                .add_reactive("focused", row.is_focused)
            style=row_styles
        >
            <div {..row.grid_cell_props.into_attrs()} class="demo-grid-list-cell">
                <span class="demo-grid-list-icon">{move || if is_selected.get() { "\u{2713}" } else { "" }}</span>
                <span>{name}</span>
            </div>
        </div>
    }
}
