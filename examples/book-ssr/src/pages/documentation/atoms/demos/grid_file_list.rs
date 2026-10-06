use std::collections::HashSet;

use leptonic::{
    atoms::grid_list::{GridList, GridListItem},
    components::prelude::{Button, ButtonVariant, Icon},
    hooks::{
        Key, SelectionBehavior, SelectionMode, use_list_collection,
        collections::Selection,
    },
    prelude::icondata,
};
use leptos::prelude::*;

/// A file: key, name and icon.
type File = (&'static str, &'static str, icondata::Icon);

const FILES: [File; 5] = [
    ("doc", "Document.pdf", icondata::BsFileEarmarkPdf),
    ("photo", "Photo.jpg", icondata::BsFileEarmarkImage),
    ("sheet", "Spreadsheet.xlsx", icondata::BsFileEarmarkSpreadsheet),
    ("slides", "Presentation.pptx", icondata::BsFileEarmarkSlides),
    ("archive", "Archive.zip", icondata::BsFileEarmarkZip),
];

#[component]
pub fn GridFileListDemo() -> impl IntoView {
    // The rows: a key and a text value (for type-ahead) per file.
    let files = use_list_collection(
        Signal::stored(FILES.to_vec()),
        |(key, _, _)| Key::from(*key),
        |(_, name, _)| (*name).to_owned(),
    );
    // App state: the selected files, and the last thing done with a file.
    let selection = RwSignal::new(Selection::default());
    let last_action = RwSignal::new(None::<String>);

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
        let action = last_action.get().unwrap_or_else(|| "none".to_owned());
        format!("Selected: {selected}. Last action: {action}.")
    };

    view! {
        <GridList
            collection=files
            disabled_keys=Signal::stored(HashSet::from([Key::from("archive")]))
            selection_mode=SelectionMode::Multiple
            selection_behavior=SelectionBehavior::Replace
            selection=selection
            set_selection=selection
            on_action=move |key: Key| last_action.set(Some(format!("opened {key}")))
            aria_label="Files"
            classes="demo-grid-list"
        >
            {FILES
                .map(|(key, name, icon)| view! {
                    <GridListItem key=key classes="demo-grid-list-item">
                        <span class="demo-grid-list-cell">
                            <Icon icon=icon classes="demo-grid-list-icon"/>
                            <span class="demo-grid-list-name">{name}</span>
                            // An interactive child: ArrowRight moves focus to it, ArrowLeft back to the row.
                            <Button
                                variant=ButtonVariant::Flat
                                on_press=move |_| last_action.set(Some(format!("downloaded {key}")))
                                attr:aria-label=format!("Download {name}")
                            >
                                <Icon icon=icondata::BsDownload/>
                            </Button>
                        </span>
                    </GridListItem>
                })
                .collect_view()}
        </GridList>
        <p class="demo-status">{status}</p>
    }
}
