use leptonic::hooks::FocusMode;
use leptonic::hooks::KeyboardNavigationBehavior;
use leptonic::hooks::collections::CollectionOptions;
use leptonic::hooks::collections::ListLayout;
use leptonic::{
    atoms::button::Button,
    hooks::{
        GridListData, IntoAttrs, Key, SelectionBehavior, SelectionMode, UseGridListInput,
        UseGridListItemInput, UseGridListReturn,
        collections::{Selection, SelectionOptions, UseListStateInput},
        use_grid_list, use_grid_list_item, use_list_collection, use_list_state,
    },
    utils::CapturedElement,
};
use leptos::prelude::*;

const FILES: [(&str, &str); 5] = [
    ("doc", "Document.pdf"),
    ("photo", "Photo.jpg"),
    ("sheet", "Spreadsheet.xlsx"),
    ("slides", "Presentation.pptx"),
    ("archive", "Archive.zip"),
];

#[component]
pub fn GridListDemo() -> impl IntoView {
    // App state: the files, the selected ones, and the file opened last.
    let files = RwSignal::new(FILES.to_vec());
    let selection = RwSignal::new(Selection::default());
    let opened = RwSignal::new(None::<Key>);

    // The rows follow the files: removing one updates the collection, and focus moves to a neighbor.
    let collection = use_list_collection(
        files.into(),
        |(key, _)| Key::from(*key),
        |(_, name)| (*name).to_owned(),
    );
    let state = use_list_state(UseListStateInput {
        collection,
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            selection_behavior: Signal::stored(SelectionBehavior::Replace),
            selection: Some(selection.into()),
            ..SelectionOptions::default()
        },
    });
    let UseGridListReturn { props, data } = use_grid_list(UseGridListInput {
        aria_label: "Files".into(),
        on_action: Some(Callback::new(move |key| opened.set(Some(key)))),
        state,
        element: CapturedElement::new(),
        id: None,
        aria_labelledby: Signal::stored(None),
        layout: ListLayout::Stack,
        keyboard_delegate: None,
        options: CollectionOptions::default(),
        keyboard_navigation_behavior: KeyboardNavigationBehavior::default(),
        should_select_on_press_up: false,
        tree: None,
    });
    let remove = Callback::new(move |key: &'static str| {
        files.update(|files| files.retain(|(k, _)| *k != key));
    });

    let status = move || {
        let selected = selection.with(|selection| match selection {
            Selection::All => "all".to_owned(),
            Selection::Keys(keys) if keys.is_empty() => "none".to_owned(),
            Selection::Keys(keys) => {
                let mut keys: Vec<String> = keys.iter().map(ToString::to_string).collect();
                keys.sort();
                keys.join(", ")
            }
        });
        let opened = opened
            .get()
            .map_or_else(|| "none".to_owned(), |key| key.to_string());
        format!("Selected: {selected}. Opened: {opened}.")
    };

    view! {
        <div {..props.into_attrs()} class="demo-grid-list">
            <For each=move || files.get() key=|(key, _)| *key let:file>
                <FileRow list=data.clone() key=file.0 name=file.1 remove/>
            </For>
        </div>
        <p class="demo-status">{status}</p>
        <div class="demo-controls">
            <Button on_press=move |_| files.set(FILES.to_vec()) classes="demo-btn">"Restore files"</Button>
        </div>
    }
}

/// A row: `use_grid_list_item` sets `role="row"`, `aria-selected` and the single `role="gridcell"`, which holds
/// the name and a button. ArrowRight moves focus to the button, ArrowLeft back to the row.
#[component]
fn FileRow(
    list: GridListData,
    key: &'static str,
    name: &'static str,
    remove: Callback<&'static str>,
) -> impl IntoView {
    let row = use_grid_list_item(UseGridListItemInput {
        list,
        key: Key::from(key),
        focus_mode: FocusMode::Row,
        allows_arrow_navigation: false,
        on_context_menu: None,
    });
    let (row_attrs, row_styles) = row.row_props.into_parts();

    view! {
        <div {..row_attrs} class="demo-grid-list-row" style=row_styles>
            <div {..row.grid_cell_props.into_attrs()} class="demo-grid-list-cell">
                <span class="demo-grid-list-name">{name}</span>
                <Button
                    on_press=move |_| remove.run(key)
                    aria_label=format!("Remove {name}")
                    classes="demo-grid-list-action"
                >
                    <span aria-hidden="true">"\u{2715}"</span>
                </Button>
            </div>
        </div>
    }
}
