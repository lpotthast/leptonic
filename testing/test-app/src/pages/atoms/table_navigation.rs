use std::sync::Arc;

use leptonic::{
    I18nProvider, Locale,
    atoms::{
        button::Button,
        table::{Table, TableBody, TableCell, TableHeader, TableRow},
    },
    hooks::{
        collections::{Selection, SelectionMode},
        grid::CellFocusMode,
        gridlist::KeyboardNavigationBehavior,
        table::TableCollection,
    },
    leptos_styles::Styles,
};
use leptos::prelude::*;

use super::listbox::describe_selection;

/// The rows of react-aria-components' `TabModeTable` (without its tag groups): key, name, type,
/// the label of the notes input, and whether the notes cell also has a button.
const TAB_ROWS: [(&str, &str, &str, &str, bool); 3] = [
    ("1", "Games", "File folder", "Games notes", true),
    (
        "2",
        "Program Files",
        "File folder",
        "Program Files notes",
        false,
    ),
    ("3", "bootmgr", "System file", "bootmgr notes", false),
];

/// react-aria-components' `TabModeTable`: `KeyboardNavigationBehavior::Tab`, a notes cell with
/// a text input (and a button in the first row). Selections show in
/// `#test-tn-{id}-selection`.
#[component]
fn TabModeTable(
    label: &'static str,
    id: &'static str,
    notes_focus_mode: CellFocusMode,
    #[prop(optional)] allows_arrow_navigation: bool,
) -> impl IntoView {
    let table = Memo::new(|_| {
        Arc::new(TableCollection::build(|t| {
            t.column("name", "Name").row_header();
            t.column("type", "Type");
            t.column("notes", "Notes");
            for (key, name, kind, _, _) in TAB_ROWS {
                t.row(key, name, |r| {
                    r.cell(name);
                    r.cell(kind);
                    r.cell("");
                });
            }
        }))
    });
    let selection = RwSignal::new(String::new());
    view! {
        <Table
            table=table
            aria_label=label
            keyboard_navigation_behavior=KeyboardNavigationBehavior::Tab
            selection_mode=SelectionMode::Multiple
            on_selection_change=Callback::new(move |s: Selection| {
                selection.set(describe_selection(&s));
            })
        >
            <TableHeader />
            <TableBody>
                {TAB_ROWS
                    .map(|(key, name, kind, notes, with_button)| {
                        view! {
                            <TableRow key=key>
                                <TableCell column="name">{name}</TableCell>
                                <TableCell column="type">{kind}</TableCell>
                                <TableCell
                                    column="notes"
                                    focus_mode=notes_focus_mode
                                    allows_arrow_navigation=allows_arrow_navigation
                                >
                                    <input aria-label=notes />
                                    {with_button
                                        .then(|| view! { <button>"Button next to input"</button> })}
                                </TableCell>
                            </TableRow>
                        }
                    })
                    .collect_view()}
            </TableBody>
        </Table>
        <div>"Selection: " <span id=format!("test-tn-{id}-selection")>{selection}</span></div>
    }
}

/// react-aria-components' `ArrowModeTable`: arrow key navigation into cells with two buttons
/// each ("R1C2 first", "R1C2 last", ...).
#[component]
fn ArrowModeTable(label: &'static str, focus_mode: CellFocusMode) -> impl IntoView {
    let table = Memo::new(|_| {
        Arc::new(TableCollection::build(|t| {
            t.column("name", "Name").row_header();
            t.column("c2", "Col 2");
            t.column("c3", "Col 3");
            for row in ["Row 1", "Row 2"] {
                t.row(row, row, |r| {
                    r.cell(row);
                    r.cell("");
                    r.cell("");
                });
            }
        }))
    });
    let buttons = move |row: usize, column: usize| {
        let first = format!("R{row}C{column} first");
        let last = format!("R{row}C{column} last");
        move || {
            view! {
                <button aria-label=first.clone()>"first"</button>
                <button aria-label=last.clone()>"last"</button>
            }
        }
    };
    view! {
        <Table table=table aria_label=label>
            <TableHeader />
            <TableBody>
                {[1_usize, 2]
                    .map(|row| {
                        view! {
                            <TableRow key=format!("Row {row}")>
                                <TableCell column="name">{format!("Row {row}")}</TableCell>
                                <TableCell column="c2" focus_mode=focus_mode>
                                    {buttons(row, 2)()}
                                </TableCell>
                                <TableCell column="c3" focus_mode=focus_mode>
                                    {buttons(row, 3)()}
                                </TableCell>
                            </TableRow>
                        }
                    })
                    .collect_view()}
            </TableBody>
        </Table>
    }
}

/// A table of three files, in row focus mode.
#[component]
fn FilesTable(label: &'static str) -> impl IntoView {
    const FILES: [(&str, &str, &str); 3] = [
        ("Games", "File folder", "6/7/2020"),
        ("Program Files", "File folder", "4/7/2021"),
        ("bootmgr", "System file", "11/20/2010"),
    ];
    let table = Memo::new(|_| {
        Arc::new(TableCollection::build(|t| {
            t.column("name", "Name").row_header();
            t.column("type", "Type");
            t.column("date", "Date Modified");
            for (name, kind, date) in FILES {
                t.row(name, name, |r| {
                    r.cell(name);
                    r.cell(kind);
                    r.cell(date);
                });
            }
        }))
    });
    view! {
        <Table table=table aria_label=label selection_mode=SelectionMode::Multiple>
            <TableHeader />
            <TableBody>
                {FILES
                    .map(|(name, kind, date)| {
                        view! {
                            <TableRow key=name>
                                <TableCell column="name">{name}</TableCell>
                                <TableCell column="type">{kind}</TableCell>
                                <TableCell column="date">{date}</TableCell>
                            </TableRow>
                        }
                    })
                    .collect_view()}
            </TableBody>
        </Table>
    }
}

/// 30 rows ("Row 1", ...) in a table scrolling at 200px: PageUp/PageDown.
#[component]
fn PagedTable() -> impl IntoView {
    let table = Memo::new(|_| {
        Arc::new(TableCollection::build(|t| {
            t.column("name", "Name").row_header();
            t.column("value", "Value");
            for i in 1..=30 {
                let name = format!("Row {i}");
                t.row(name.clone(), name.clone(), |r| {
                    r.cell(name);
                    r.cell(i.to_string());
                });
            }
        }))
    });
    view! {
        <Table
            table=table
            aria_label="Paged table"
            styles=Styles::new()
                .add_unchecked("display", "block")
                .add_unchecked("height", "200px")
                .add_unchecked("overflow", "auto")
        >
            <TableHeader />
            <TableBody>
                {(1..=30)
                    .map(|i| {
                        view! {
                            <TableRow key=format!("Row {i}")>
                                <TableCell column="name">{format!("Row {i}")}</TableCell>
                                <TableCell column="value">{i.to_string()}</TableCell>
                            </TableRow>
                        }
                    })
                    .collect_view()}
            </TableBody>
        </Table>
    }
}

/// react-aria-components' `TableCellColSpanWithVariousSpansExample`, with unique cell texts:
/// `R{row}C{column}` (the first column labels the rows), spans as "R{row} span {n}".
#[component]
fn ColSpanTable() -> impl IntoView {
    // Per row: (first column, text, span) of each cell.
    let rows: Vec<Vec<(usize, String, usize)>> = vec![
        vec![
            (1, "R1C1".into(), 1),
            (2, "R1 span 2".into(), 2),
            (4, "R1C4".into(), 1),
        ],
        (1..=4).map(|c| (c, format!("R2C{c}"), 1)).collect(),
        vec![(1, "R3 span 4".into(), 4)],
        (1..=4).map(|c| (c, format!("R4C{c}"), 1)).collect(),
        vec![(1, "R5 span 3".into(), 3), (4, "R5C4".into(), 1)],
        (1..=4).map(|c| (c, format!("R6C{c}"), 1)).collect(),
        vec![(1, "R7C1".into(), 1), (2, "R7 span 3".into(), 3)],
    ];
    let collection_rows = rows.clone();
    let table = Memo::new(move |_| {
        Arc::new(TableCollection::build(|t| {
            t.column("c1", "Col 1").row_header();
            t.column("c2", "Col 2");
            t.column("c3", "Col 3");
            t.column("c4", "Col 4");
            for (i, cells) in collection_rows.iter().enumerate() {
                let key = format!("row{}", i + 1);
                t.row(key.clone(), key, |r| {
                    for (_, text, span) in cells {
                        if *span > 1 {
                            r.cell(text.clone()).col_span(*span);
                        } else {
                            r.cell(text.clone());
                        }
                    }
                });
            }
        }))
    });
    view! {
        <Table table=table aria_label="Table with various colspans">
            <TableHeader />
            <TableBody>
                {rows
                    .into_iter()
                    .enumerate()
                    .map(|(i, cells)| {
                        view! {
                            <TableRow key=format!("row{}", i + 1)>
                                {cells
                                    .into_iter()
                                    .map(|(column, text, _)| {
                                        view! {
                                            <TableCell column=format!("c{column}")>
                                                {text.clone()}
                                            </TableCell>
                                        }
                                    })
                                    .collect_view()}
                            </TableRow>
                        }
                    })
                    .collect_view()}
            </TableBody>
        </Table>
    }
}

/// A table without rows (with a selection checkbox column).
#[component]
fn EmptyTable() -> impl IntoView {
    let table = Memo::new(|_| {
        Arc::new(TableCollection::build(|t| {
            t.column("name", "Name").row_header().allows_sorting();
            t.column("type", "Type");
        }))
    });
    view! {
        <Table
            table=table
            show_selection_checkboxes=true
            aria_label="Empty table"
            selection_mode=SelectionMode::Multiple
        >
            <TableHeader />
            <TableBody />
        </Table>
    }
}

/// A selectable table whose "Actions" cells (`CellFocusMode::Child`) hold two `Button` atoms
/// (crudkit's action column); presses are logged to `#test-tn-actions-log`.
#[component]
fn ActionsTable() -> impl IntoView {
    let table = Memo::new(|_| {
        Arc::new(TableCollection::build(|t| {
            t.column("name", "Name").row_header();
            t.column("actions", "Actions");
            for name in ["Alice", "Bob"] {
                t.row(name, name, |r| {
                    r.cell(name);
                    r.cell("");
                });
            }
        }))
    });
    let log = RwSignal::new(Vec::<String>::new());
    let pressed = move |what: String| Callback::new(move |_| log.update(|l| l.push(what.clone())));
    view! {
        <Table table=table aria_label="Actions table" selection_mode=SelectionMode::Multiple>
            <TableHeader />
            <TableBody>
                {["Alice", "Bob"]
                    .map(|name| {
                        view! {
                            <TableRow key=name>
                                <TableCell column="name">{name}</TableCell>
                                <TableCell column="actions" focus_mode=CellFocusMode::Child>
                                    <Button
                                        attr:aria-label=format!("Edit {name}")
                                        on_press=pressed(format!("edit {name}"))
                                    >
                                        "Edit"
                                    </Button>
                                    <Button
                                        attr:aria-label=format!("Delete {name}")
                                        on_press=pressed(format!("delete {name}"))
                                    >
                                        "Delete"
                                    </Button>
                                </TableCell>
                            </TableRow>
                        }
                    })
                    .collect_view()}
            </TableBody>
        </Table>
        <div>"Presses: " <span id="test-tn-actions-log">{move || log.get().join(", ")}</span></div>
    }
}

/// Keyboard navigation of the table atoms: `KeyboardNavigationBehavior::Tab` with text inputs
/// in cells (react-aria-components' `TabModeTable`), arrow navigation into cells
/// (`ArrowModeTable`), right-to-left, PageUp/PageDown, column spans and an empty table. Each
/// table follows a "Before" button (`#test-tn-before-{name}`).
#[component]
pub fn PageAtomTableNavigation() -> impl IntoView {
    let rtl: Locale = "ar-AE".parse().expect("a valid locale");
    view! {
        <div id="test-page-atom-table-navigation">
            <h1>"Table navigation"</h1>
            <button id="test-tn-before-tab">"Before"</button>
            <TabModeTable label="Tab mode table" id="tab" notes_focus_mode=CellFocusMode::Cell />
            <button id="test-tn-after-tab">"After"</button>
            <button id="test-tn-before-child">"Before"</button>
            <TabModeTable
                label="Tab mode child table"
                id="child"
                notes_focus_mode=CellFocusMode::Child
            />
            <button id="test-tn-before-arrows">"Before"</button>
            <TabModeTable
                label="Tab mode arrows table"
                id="arrows"
                notes_focus_mode=CellFocusMode::Child
                allows_arrow_navigation=true
            />
            <button id="test-tn-before-arrow-mode">"Before"</button>
            <ArrowModeTable label="Arrow mode table" focus_mode=CellFocusMode::Child />
            <button id="test-tn-before-arrow-cell">"Before"</button>
            <ArrowModeTable label="Arrow cell table" focus_mode=CellFocusMode::Cell />
            <button id="test-tn-before-rtl">"Before"</button>
            <div dir="rtl">
                <I18nProvider locale=rtl>
                    <FilesTable label="RTL table" />
                </I18nProvider>
            </div>
            <button id="test-tn-before-paged">"Before"</button>
            <PagedTable />
            <button id="test-tn-before-colspan">"Before"</button>
            <ColSpanTable />
            <button id="test-tn-before-empty">"Before"</button>
            <EmptyTable />
            <button id="test-tn-after-empty">"After"</button>
            <button id="test-tn-before-actions">"Before"</button>
            <ActionsTable />
        </div>
    }
}
