use leptonic::{
    atoms::grid_list::{
        GridList, GridListHeader, GridListItem, GridListItemDescription, GridListSection,
    },
    hooks::collections::{Key, Selection, SelectionMode, use_collection},
};
use leptos::prelude::*;

/// A section of the file list: its key, header text and files (key, name, description).
struct Folder {
    key: &'static str,
    header: &'static str,
    files: &'static [(&'static str, &'static str, &'static str)],
}

const FOLDERS: [Folder; 2] = [
    Folder {
        key: "recent",
        header: "Recent",
        files: &[
            ("report", "Report.pdf", "Edited today"),
            ("budget", "Budget.xlsx", "Edited yesterday"),
        ],
    },
    Folder {
        key: "shared",
        header: "Shared with you",
        files: &[
            ("roadmap", "Roadmap.pptx", "Shared by Ada"),
            ("notes", "Notes.txt", "Shared by Grace"),
        ],
    },
];

#[component]
pub fn GridListSectionsDemo() -> impl IntoView {
    // The rows, in sections with a header each.
    let files = use_collection(|b| {
        for folder in &FOLDERS {
            b.section(folder.key, |s| {
                s.header(format!("{}-header", folder.key), folder.header);
                for (key, name, _) in folder.files {
                    s.item(*key, *name);
                }
            });
        }
    });
    let selection = RwSignal::new(Selection::default());

    view! {
        <GridList
            collection=files
            selection_mode=SelectionMode::Multiple
            selection=selection
            set_selection=selection
            aria_label="Files"
            classes="demo-grid-list"
        >
            {FOLDERS
                .iter()
                .map(|folder| view! {
                    <GridListSection key=folder.key>
                        // Shows the section's header text from the collection, and labels the group.
                        <GridListHeader classes="demo-grid-list-header"/>
                        {folder
                            .files
                            .iter()
                            .map(|(key, name, description)| view! {
                                <GridListItem key=*key classes="demo-grid-list-item">
                                    <span class="demo-grid-list-cell">
                                        <span class="demo-grid-list-name">{*name}</span>
                                        <GridListItemDescription classes="demo-grid-list-description">
                                            {*description}
                                        </GridListItemDescription>
                                    </span>
                                </GridListItem>
                            })
                            .collect_view()}
                    </GridListSection>
                })
                .collect_view()}
        </GridList>
        <p class="demo-status">
            {move || {
                selection.with(|selection| match selection {
                    Selection::All => "Selected: all.".to_owned(),
                    Selection::Keys(keys) if keys.is_empty() => "Selected: none.".to_owned(),
                    Selection::Keys(keys) => {
                        let mut keys: Vec<String> = keys.iter().map(Key::to_string).collect();
                        keys.sort();
                        format!("Selected: {}.", keys.join(", "))
                    }
                })
            }}
        </p>
    }
}
