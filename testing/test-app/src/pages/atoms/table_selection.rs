use std::sync::Arc;

use leptonic::{
    atoms::table::{Table, TableBody, TableCell, TableHeader, TableRow},
    hooks::{
        SelectionBehavior, SelectionMode, TableCollection, TableOptions,
        collections::{EscapeKeyBehavior, Key, Selection},
    },
};
use leptos::prelude::*;

use super::listbox::describe_selection;

/// react-aria-components' `TestTable` rows: key, name, type, date.
const FILES: [(&str, &str, &str, &str); 3] = [
    ("1", "Games", "File folder", "6/7/2020"),
    ("2", "Program Files", "File folder", "4/7/2021"),
    ("3", "bootmgr", "System file", "11/20/2010"),
];

fn files_table(show_selection_checkboxes: bool) -> Memo<Arc<TableCollection>> {
    Memo::new(move |_| {
        Arc::new(TableCollection::build_with(
            TableOptions {
                show_selection_checkboxes,
            },
            |t| {
                t.column("name", "Name").row_header();
                t.column("type", "Type");
                t.column("date", "Date Modified");
                for (key, name, kind, date) in FILES {
                    t.row(key, name, |r| {
                        r.cell(name);
                        r.cell(kind);
                        r.cell(date);
                    });
                }
            },
        ))
    })
}

fn rows() -> impl IntoView {
    FILES
        .map(|(key, name, kind, date)| {
            view! {
                <TableRow key=key>
                    <TableCell column="name">{name}</TableCell>
                    <TableCell column="type">{kind}</TableCell>
                    <TableCell column="date">{date}</TableCell>
                </TableRow>
            }
        })
        .collect_view()
}

/// The selection display of the table `id`: `#test-ts-{id}-selection`, and the number of
/// selection changes, `#test-ts-{id}-changes`.
fn selection_log(id: &'static str) -> (Callback<Selection>, impl IntoView) {
    let selection = RwSignal::new(String::new());
    let changes = RwSignal::new(0_usize);
    let on_change = Callback::new(move |s: Selection| {
        selection.set(describe_selection(&s));
        changes.update(|c| *c += 1);
    });
    let view = view! {
        <div>
            "Selection: " <span id=format!("test-ts-{id}-selection")>{selection}</span>
            " Changes: " <span id=format!("test-ts-{id}-changes")>{move || changes.get().to_string()}</span>
        </div>
    };
    (on_change, view)
}

/// The files table with `selection_behavior` replace (react-aria-components'
/// `selectionBehavior="replace"` tests).
#[component]
fn ReplaceTable(label: &'static str, id: &'static str, selection_mode: SelectionMode) -> impl IntoView {
    let (on_change, log) = selection_log(id);
    view! {
        <Table
            table=files_table(false)
            aria_label=label
            selection_mode=selection_mode
            selection_behavior=SelectionBehavior::Replace
            on_selection_change=on_change
        >
            <TableHeader />
            <TableBody>{rows()}</TableBody>
        </Table>
        {log}
    }
}

/// The files table with selection checkboxes, `escape_key_behavior` none.
#[component]
fn EscapeTable() -> impl IntoView {
    let (on_change, log) = selection_log("escape");
    view! {
        <Table
            table=files_table(true)
            aria_label="Escape table"
            selection_mode=SelectionMode::Multiple
            escape_key_behavior=EscapeKeyBehavior::None
            on_selection_change=on_change
        >
            <TableHeader />
            <TableBody>{rows()}</TableBody>
        </Table>
        {log}
    }
}

/// The files table selecting on press up (or down) in single selection mode.
#[component]
fn PressTable(label: &'static str, id: &'static str, should_select_on_press_up: bool) -> impl IntoView {
    let (on_change, log) = selection_log(id);
    view! {
        <Table
            table=files_table(false)
            aria_label=label
            selection_mode=SelectionMode::Single
            should_select_on_press_up=should_select_on_press_up
            on_selection_change=on_change
        >
            <TableHeader />
            <TableBody>{rows()}</TableBody>
        </Table>
        {log}
    }
}

/// The files table with row actions (no selection): `#test-ts-action` shows the last one.
#[component]
fn ActionTable() -> impl IntoView {
    let action = RwSignal::new(String::new());
    let count = RwSignal::new(0_usize);
    view! {
        <Table
            table=files_table(false)
            aria_label="Action table"
            on_row_action=Callback::new(move |key: Key| {
                action.set(key.to_string());
                count.update(|c| *c += 1);
            })
        >
            <TableHeader />
            <TableBody>{rows()}</TableBody>
        </Table>
        <div>
            "Action: " <span id="test-ts-action">{action}</span>
            " Count: " <span id="test-ts-action-count">{move || count.get().to_string()}</span>
        </div>
    }
}

/// Changing columns: the files table with a selection column, whose "type" column can be hidden
/// (`#test-ts-hide-type`), renamed (`#test-ts-rename-type`) and moved after "date"
/// (`#test-ts-move-type`); "date" can be made sortable (`#test-ts-sort-date`), and the selection
/// mode switched between multiple and single (`#test-ts-single`). Rows render a cell per
/// current column (react-aria-components' `HidingColumnsExample`).
#[component]
fn ColumnsTable() -> impl IntoView {
    let hide_type = RwSignal::new(false);
    let rename_type = RwSignal::new(false);
    let move_type = RwSignal::new(false);
    let sort_date = RwSignal::new(false);
    let single = RwSignal::new(false);
    let columns = Memo::new(move |_| {
        let mut columns = vec![("name", "Name")];
        if !hide_type.get() {
            columns.push(("type", if rename_type.get() { "Kind" } else { "Type" }));
        }
        columns.push(("date", "Date Modified"));
        if move_type.get() && columns.len() == 3 {
            columns.swap(1, 2);
        }
        columns
    });
    let field = |file: (&'static str, &'static str, &'static str, &'static str), column: &str| match column {
        "name" => file.1,
        "type" => file.2,
        _ => file.3,
    };
    let table = Memo::new(move |_| {
        let columns = columns.get();
        let sortable = sort_date.get();
        Arc::new(TableCollection::build_with(
            TableOptions {
                show_selection_checkboxes: true,
            },
            |t| {
                for (key, text) in &columns {
                    let column = t.column(*key, *text);
                    if *key == "name" {
                        column.row_header();
                    } else if *key == "date" && sortable {
                        column.allows_sorting();
                    }
                }
                for file in FILES {
                    t.row(file.0, file.1, |r| {
                        for (key, _) in &columns {
                            r.cell(field(file, key));
                        }
                    });
                }
            },
        ))
    });
    let toggle = |signal: RwSignal<bool>| move |_| signal.update(|v| *v = !*v);
    view! {
        <Table
            table=table
            aria_label="Columns table"
            selection_mode=Signal::derive(move || {
                if single.get() { SelectionMode::Single } else { SelectionMode::Multiple }
            })
        >
            <TableHeader />
            <TableBody>
                {FILES
                    .map(|file| {
                        view! {
                            <TableRow key=file.0>
                                <For
                                    each=move || columns.get()
                                    key=|(key, _)| *key
                                    children=move |(key, _)| {
                                        view! { <TableCell column=key>{field(file, key)}</TableCell> }
                                    }
                                />
                            </TableRow>
                        }
                    })
                    .collect_view()}
            </TableBody>
        </Table>
        <button id="test-ts-hide-type" on:click=toggle(hide_type)>"Hide type"</button>
        <button id="test-ts-rename-type" on:click=toggle(rename_type)>"Rename type"</button>
        <button id="test-ts-move-type" on:click=toggle(move_type)>"Move type"</button>
        <button id="test-ts-sort-date" on:click=toggle(sort_date)>"Sort date"</button>
        <button id="test-ts-single" on:click=toggle(single)>"Single"</button>
    }
}

/// Selection behaviors and row actions of the table atoms (react-aria-components'
/// `Table.test.js`): replace selection, Escape without clearing, selecting on press up, row
/// actions, and columns that change. Each table follows a "Before" button
/// (`#test-ts-before-{name}`).
#[component]
pub fn PageAtomTableSelection() -> impl IntoView {
    view! {
        <div id="test-page-atom-table-selection">
            <h1>"Table selection"</h1>
            <button id="test-ts-before-replace">"Before"</button>
            <ReplaceTable label="Replace table" id="replace" selection_mode=SelectionMode::Multiple />
            <button id="test-ts-before-single-replace">"Before"</button>
            <ReplaceTable
                label="Single replace table"
                id="single-replace"
                selection_mode=SelectionMode::Single
            />
            <button id="test-ts-before-escape">"Before"</button>
            <EscapeTable />
            <button id="test-ts-before-press-down">"Before"</button>
            <PressTable label="Press down table" id="press-down" should_select_on_press_up=false />
            <button id="test-ts-before-press-up">"Before"</button>
            <PressTable label="Press up table" id="press-up" should_select_on_press_up=true />
            <button id="test-ts-before-action">"Before"</button>
            <ActionTable />
            <button id="test-ts-before-columns">"Before"</button>
            <ColumnsTable />
        </div>
    }
}
