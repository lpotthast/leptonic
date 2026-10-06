use std::{collections::HashSet, sync::Arc};

use leptonic::{
    atoms::table::{Table, TableBody, TableCell, TableHeader, TableRow},
    hooks::{
        SelectionMode, SortDescriptor, SortDirection, TableCollection, TableOptions,
        collections::{Key, Selection},
    },
};
use leptos::prelude::*;

use super::listbox::describe_selection;

#[derive(Debug, Clone, PartialEq)]
struct File {
    name: &'static str,
    kind: &'static str,
    date: &'static str,
}

/// react-aria-components' `DynamicTable` rows.
const FILES: [File; 4] = [
    File {
        name: "Games",
        kind: "File folder",
        date: "6/7/2020",
    },
    File {
        name: "Program Files",
        kind: "File folder",
        date: "4/7/2021",
    },
    File {
        name: "bootmgr",
        kind: "System file",
        date: "11/20/2010",
    },
    File {
        name: "log.txt",
        kind: "Text Document",
        date: "1/18/2016",
    },
];

fn sorted(files: &[File], sort: Option<&SortDescriptor>) -> Vec<File> {
    let mut files = files.to_vec();
    if let Some(sort) = sort {
        let field = |f: &File| match sort.column.as_str() {
            Some("type") => f.kind.to_lowercase(),
            Some("date") => f.date.to_owned(),
            _ => f.name.to_lowercase(),
        };
        files.sort_by_key(field);
        if sort.direction == SortDirection::Descending {
            files.reverse();
        }
    }
    files
}

/// A sortable, multi-select table of files with selection checkboxes ("log.txt" is disabled), and
/// a table with a column group.
#[component]
pub fn PageAtomTable() -> impl IntoView {
    let files = RwSignal::new(FILES.to_vec());
    let sort = RwSignal::new(Some(SortDescriptor {
        column: Key::from("name"),
        direction: SortDirection::Ascending,
    }));
    let rows = Memo::new(move |_| files.with(|f| sort.with(|s| sorted(f, s.as_ref()))));
    let table = Memo::new(move |_| {
        Arc::new(TableCollection::build_with(
            TableOptions {
                show_selection_checkboxes: true,
            },
            |t| {
                t.column("name", "Name").row_header().allows_sorting();
                t.column("type", "Type").allows_sorting();
                t.column("date", "Date Modified").allows_sorting();
                rows.with(|rows| {
                    for file in rows {
                        t.row(file.name, file.name, |r| {
                            r.cell(file.name);
                            r.cell(file.kind);
                            r.cell(file.date);
                        });
                    }
                });
            },
        ))
    });
    let selection = RwSignal::new(String::new());
    let sort_text = move || {
        sort.with(|s| {
            s.as_ref().map_or_else(String::new, |s| {
                format!(
                    "{} {}",
                    s.column,
                    match s.direction {
                        SortDirection::Ascending => "ascending",
                        SortDirection::Descending => "descending",
                    }
                )
            })
        })
    };

    view! {
        <div id="test-page-atom-table">
            <h1>"Table"</h1>
            <button id="test-table-before">"Before"</button>
            <Table
                table=table
                selection_mode=SelectionMode::Multiple
                disabled_keys=Signal::stored(HashSet::from([Key::from("log.txt")]))
                default_sort_descriptor=sort.get_untracked().expect("initial sort")
                on_sort_change=Callback::new(move |d: SortDescriptor| sort.set(Some(d)))
                on_selection_change=Callback::new(move |s: Selection| {
                    selection.set(describe_selection(&s));
                })
                aria_label="Files"
            >
                <TableHeader />
                <TableBody>
                    <For
                        each=move || rows.get()
                        key=|file| file.name
                        children=|file: File| {
                            view! {
                                <TableRow key=file.name>
                                    <TableCell column="name">{file.name}</TableCell>
                                    <TableCell column="type">{file.kind}</TableCell>
                                    <TableCell column="date">{file.date}</TableCell>
                                </TableRow>
                            }
                        }
                    />
                </TableBody>
            </Table>
            <div>"Selection: " <span id="test-table-selection">{selection}</span></div>
            <div>"Sort: " <span id="test-table-sort">{sort_text}</span></div>
            <button
                id="test-table-remove-games"
                on:click=move |_| files.update(|f| f.retain(|file| file.name != "Games"))
            >
                "Remove Games"
            </button>
            <button id="test-table-before-contacts">"Before"</button>
            <ContactsTable />
        </div>
    }
}

/// A table with a column group: | Name | Contact (Email, Phone) | Notes |.
#[component]
fn ContactsTable() -> impl IntoView {
    let table = Memo::new(|_| {
        Arc::new(TableCollection::build(|t| {
            t.column("name", "Name");
            t.column_group("contact", "Contact", |g| {
                g.column("email", "Email");
                g.column("phone", "Phone");
            });
            t.column("notes", "Notes");
            for (name, email, phone, notes) in [
                ("Alice", "alice@example.com", "555-0100", "Admin"),
                ("Bob", "bob@example.com", "555-0101", "On leave"),
            ] {
                t.row(name, name, |r| {
                    r.cell(name);
                    r.cell(email);
                    r.cell(phone);
                    r.cell(notes);
                });
            }
        }))
    });
    view! {
        <Table table=table aria_label="Contacts">
            <TableHeader />
            <TableBody>
                {[
                    ("Alice", "alice@example.com", "555-0100", "Admin"),
                    ("Bob", "bob@example.com", "555-0101", "On leave"),
                ]
                    .map(|(name, email, phone, notes)| {
                        view! {
                            <TableRow key=name>
                                <TableCell column="name">{name}</TableCell>
                                <TableCell column="email">{email}</TableCell>
                                <TableCell column="phone">{phone}</TableCell>
                                <TableCell column="notes">{notes}</TableCell>
                            </TableRow>
                        }
                    })
                    .collect_view()}
            </TableBody>
        </Table>
    }
}
