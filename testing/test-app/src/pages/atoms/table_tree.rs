use std::{collections::HashSet, sync::Arc};

use leptonic::{
    I18nProvider, Locale,
    atoms::table::{Table, TableBody, TableCell, TableExpandButton, TableHeader, TableRow},
    hooks::{
        collections::{Key, Selection, SelectionMode},
        table::TableCollection,
    },
};
use leptos::prelude::*;

use super::listbox::describe_selection;

/// A row of react-aria-components' `Treeble.test.js` example: key, name, type, date and child
/// rows.
struct File {
    key: &'static str,
    name: &'static str,
    kind: &'static str,
    date: &'static str,
    children: &'static [File],
}

const fn leaf(
    key: &'static str,
    name: &'static str,
    kind: &'static str,
    date: &'static str,
) -> File {
    File {
        key,
        name,
        kind,
        date,
        children: &[],
    }
}

const FILES: &[File] = &[
    File {
        key: "games",
        name: "Games",
        kind: "Folder",
        date: "6/7/2023",
        children: &[
            leaf("mario", "Mario Kart", "Game", "8/27/1992"),
            leaf("tetris", "Tetris", "Game", "1/27/1988"),
            leaf("pacman", "Pac-Man", "Game", "5/22/1980"),
        ],
    },
    File {
        key: "apps",
        name: "Applications",
        kind: "Folder",
        date: "4/7/2025",
        children: &[
            leaf("ps", "Photoshop", "Application", "2/19/1990"),
            leaf("premiere", "Premiere", "Application", "9/24/2003"),
            leaf("lightroom", "Lightroom", "Application", "10/18/2017"),
        ],
    },
    leaf(
        "report",
        "2024 Financial Report",
        "PDF Document",
        "12/30/2024",
    ),
    leaf("job", "Job Posting", "Text Document", "1/18/2025"),
];

fn add_rows(builder: &mut leptonic::hooks::collections::CollectionBuilder, files: &[File]) {
    for file in files {
        let row = builder.row(file.key, file.name, |r| {
            r.cell(file.name);
            r.cell(file.kind);
            r.cell(file.date);
        });
        if !file.children.is_empty() {
            let _ = row.children(|b| add_rows(b, file.children));
        }
    }
}

/// Every row in collection order: parents before their children.
fn flattened(files: &'static [File]) -> Vec<&'static File> {
    files
        .iter()
        .flat_map(|file| std::iter::once(file).chain(flattened(file.children)))
        .collect()
}

/// How a [`Treeble`] holds its expanded rows.
#[derive(Clone, Copy)]
enum Expansion {
    Default(&'static [&'static str]),
    /// Controlled without a setter.
    Controlled(&'static [&'static str]),
    /// Controlled, with the app's signal as the setter.
    Bound(RwSignal<HashSet<Key>>),
}

fn keys(keys: &[&str]) -> HashSet<Key> {
    keys.iter().map(|k| Key::from(*k)).collect()
}

/// The `Treeble` example (tree column "name"), with the expanded rows and the selection logged
/// to `#test-tt-{id}-expanded` and `#test-tt-{id}-selection`.
#[component]
fn Treeble(
    id: &'static str,
    label: &'static str,
    expansion: Expansion,
    #[prop(optional)] selection_mode: SelectionMode,
) -> impl IntoView {
    let table = Memo::new(|_| {
        Arc::new(TableCollection::build(|t| {
            t.column("name", "Name").row_header();
            t.column("type", "Type");
            t.column("date", "Date Modified");
            t.rows(|b| add_rows(b, FILES));
        }))
    });
    let expanded_log = RwSignal::new(String::new());
    let selection_log = RwSignal::new(String::new());
    let describe = |keys: &HashSet<Key>| {
        let mut keys: Vec<String> = keys.iter().map(ToString::to_string).collect();
        keys.sort();
        keys.join(",")
    };
    let rows = || {
        view! {
            <TableHeader />
            <TableBody>
                {flattened(FILES)
                    .into_iter()
                    .map(|file| {
                        view! {
                            <TableRow key=file.key>
                                <TableCell column="name">
                                    <TableExpandButton>">"</TableExpandButton>
                                    {file.name}
                                </TableCell>
                                <TableCell column="type">{file.kind}</TableCell>
                                <TableCell column="date">{file.date}</TableCell>
                            </TableRow>
                        }
                    })
                    .collect_view()}
            </TableBody>
        }
    };
    let on_expanded_change =
        Callback::new(move |keys: HashSet<Key>| expanded_log.set(describe(&keys)));
    let on_selection_change =
        Callback::new(move |s: Selection| selection_log.set(describe_selection(&s)));
    let table_view = match expansion {
        Expansion::Bound(expanded) => view! {
            <Table
                table=table
                tree_column=Key::from("name")
                expanded_keys=expanded
                set_expanded_keys=expanded
                on_expanded_change=on_expanded_change
                selection_mode=selection_mode
                on_selection_change=on_selection_change
                aria_label=label
            >
                {rows()}
            </Table>
        }
        .into_any(),
        Expansion::Controlled(expanded) => view! {
            <Table
                table=table
                tree_column=Key::from("name")
                expanded_keys=keys(expanded)
                on_expanded_change=on_expanded_change
                selection_mode=selection_mode
                on_selection_change=on_selection_change
                aria_label=label
            >
                {rows()}
            </Table>
        }
        .into_any(),
        Expansion::Default(default_expanded_keys) => view! {
            <Table
                table=table
                tree_column=Key::from("name")
                default_expanded_keys=keys(default_expanded_keys)
                on_expanded_change=on_expanded_change
                selection_mode=selection_mode
                on_selection_change=on_selection_change
                aria_label=label
            >
                {rows()}
            </Table>
        }
        .into_any(),
    };
    view! {
        <div id=format!("test-tt-{id}")>{table_view}</div>
        <div>"Expanded: " <span id=format!("test-tt-{id}-expanded")>{expanded_log}</span></div>
        <div>"Selection: " <span id=format!("test-tt-{id}-selection")>{selection_log}</span></div>
    }
}

/// Tree tables (react-aria-components' `Treeble.test.js`): `#test-tt-files` collapsed,
/// `#test-tt-rtl` the same in ar-AE (right to left), `#test-tt-default` with Games expanded by
/// default and multiple selection, `#test-tt-controlled` with Games expanded (controlled, no
/// setter), `#test-tt-bound` bound to the app's expanded keys (Games and the leaf "report"),
/// which "Collapse all" (`#test-tt-collapse-all`) empties. Each follows a "Before" button
/// (`#test-tt-before-{id}`).
#[component]
pub fn PageAtomTableTree() -> impl IntoView {
    let rtl: Locale = "ar-AE".parse().expect("a locale");
    let bound = RwSignal::new(keys(&["games", "report"]));
    view! {
        <div id="test-page-atom-table-tree">
            <h1>"Tree tables"</h1>
            <button id="test-tt-before-files">"Before"</button>
            <Treeble id="files" label="Files" expansion=Expansion::Default(&[]) />
            <button id="test-tt-before-rtl">"Before"</button>
            <div dir="rtl">
                <I18nProvider locale=rtl>
                    <Treeble id="rtl" label="Files RTL" expansion=Expansion::Default(&[]) />
                </I18nProvider>
            </div>
            <button id="test-tt-before-default">"Before"</button>
            <Treeble
                id="default"
                label="Default expanded"
                expansion=Expansion::Default(&["games"])
                selection_mode=SelectionMode::Multiple
            />
            <button id="test-tt-before-controlled">"Before"</button>
            <Treeble
                id="controlled"
                label="Controlled"
                expansion=Expansion::Controlled(&["games"])
            />
            <button id="test-tt-collapse-all" on:click=move |_| bound.set(HashSet::new())>
                "Collapse all"
            </button>
            <button id="test-tt-before-bound">"Before"</button>
            <Treeble id="bound" label="Bound" expansion=Expansion::Bound(bound) />
        </div>
    }
}
