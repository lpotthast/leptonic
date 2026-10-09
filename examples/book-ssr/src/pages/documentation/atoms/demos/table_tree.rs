use std::{collections::HashSet, sync::Arc};

use leptonic::{
    atoms::table::{Table, TableBody, TableCell, TableExpandButton, TableHeader, TableRow},
    hooks::{
        collections::{CollectionBuilder, Key},
        table::TableCollection,
    },
};
use leptos::prelude::*;

/// A file or folder: key, name, kind, size and the folder's content.
struct File {
    key: &'static str,
    name: &'static str,
    kind: &'static str,
    size: &'static str,
    children: &'static [File],
}

const fn file(
    key: &'static str,
    name: &'static str,
    kind: &'static str,
    size: &'static str,
) -> File {
    File {
        key,
        name,
        kind,
        size,
        children: &[],
    }
}

const FILES: &[File] = &[
    File {
        key: "documents",
        name: "Documents",
        kind: "Folder",
        size: "\u{2014}",
        children: &[
            file("report", "Report.pdf", "PDF", "2.4 MB"),
            File {
                key: "invoices",
                name: "Invoices",
                kind: "Folder",
                size: "\u{2014}",
                children: &[
                    file("march", "March.pdf", "PDF", "120 KB"),
                    file("april", "April.pdf", "PDF", "118 KB"),
                ],
            },
        ],
    },
    File {
        key: "photos",
        name: "Photos",
        kind: "Folder",
        size: "\u{2014}",
        children: &[
            file("beach", "Beach.jpg", "Image", "3.1 MB"),
            file("mountains", "Mountains.jpg", "Image", "2.8 MB"),
        ],
    },
    file("notes", "Notes.txt", "Text", "4 KB"),
];

/// Adds `files` as rows, each folder's content as its child rows.
fn add_rows(builder: &mut CollectionBuilder, files: &[File]) {
    for file in files {
        let row = builder.row(file.key, file.name, |r| {
            r.cell(file.name);
            r.cell(file.kind);
            r.cell(file.size);
        });
        if !file.children.is_empty() {
            let _ = row.children(|b| add_rows(b, file.children));
        }
    }
}

/// Every row in collection order: each folder before its content.
fn flattened(files: &'static [File]) -> Vec<&'static File> {
    files
        .iter()
        .flat_map(|file| std::iter::once(file).chain(flattened(file.children)))
        .collect()
}

#[component]
pub fn TableTreeAtomDemo() -> impl IntoView {
    let table = Memo::new(|_| {
        Arc::new(TableCollection::build(|t| {
            t.column("name", "Name").row_header();
            t.column("kind", "Kind");
            t.column("size", "Size");
            t.rows(|b| add_rows(b, FILES));
        }))
    });
    // App state: the expanded folders.
    let expanded = RwSignal::new(HashSet::from([Key::from("documents")]));

    view! {
        <div class="demo-table-scroll">
            // The "name" column shows the hierarchy. Every row is rendered, in collection order; rows inside a
            // collapsed folder stay in the DOM, `hidden`.
            <Table
                table=table
                tree_column=Key::from("name")
                expanded_keys=expanded
                set_expanded_keys=expanded
                aria_label="Files"
                classes=["demo-table", "demo-atom-table", "demo-tree-table"]
            >
                <TableHeader/>
                <TableBody>
                    {flattened(FILES)
                        .into_iter()
                        .map(|file| view! {
                            <TableRow key=file.key classes="demo-atom-table-row">
                                <TableCell column="name">
                                    // Hidden in rows without child rows.
                                    <TableExpandButton classes="demo-tree-table-expand">
                                        <span aria-hidden="true">"\u{25b8}"</span>
                                    </TableExpandButton>
                                    {file.name}
                                </TableCell>
                                <TableCell column="kind">{file.kind}</TableCell>
                                <TableCell column="size">{file.size}</TableCell>
                            </TableRow>
                        })
                        .collect_view()}
                </TableBody>
            </Table>
        </div>
        <p class="demo-status">
            {move || {
                let mut keys: Vec<String> = expanded.get().iter().map(ToString::to_string).collect();
                keys.sort();
                if keys.is_empty() { "Expanded: none.".to_owned() } else { format!("Expanded: {}.", keys.join(", ")) }
            }}
        </p>
    }
}
