use std::sync::Arc;

use leptonic::hooks::GridFocusMode;
use leptonic::hooks::KeyboardNavigationBehavior;
use leptonic::hooks::collections::CollectionOptions;
use leptonic::{
    hooks::{
        ColumnKind, DisabledBehavior, IntoAttrs, NodeKind, SelectionMode, SortDescriptor,
        SortDirection, TableCollection, TableData, TableOptions, UseTableCellInput,
        UseTableColumnHeaderInput, UseTableHeaderPlaceholderInput, UseTableInput, UseTableReturn,
        UseTableRowInput, UseTableSelectAllCheckboxInput, UseTableSelectionCheckboxInput,
        UseTableStateInput,
        collections::{Key, SelectionOptions},
        use_checkbox, use_grid_row_group, use_table, use_table_cell, use_table_column_header,
        use_table_header_placeholder, use_table_header_row, use_table_row,
        use_table_select_all_checkbox, use_table_selection_checkbox, use_table_state,
    },
    utils::CapturedElement,
};
use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Book {
    title: &'static str,
    author: &'static str,
    stock: u32,
    price: &'static str,
    out_of_print: bool,
}

const BOOKS: [Book; 5] = [
    Book {
        title: "Dune",
        author: "Frank Herbert",
        stock: 12,
        price: "$9.99",
        out_of_print: false,
    },
    Book {
        title: "Emma",
        author: "Jane Austen",
        stock: 4,
        price: "$7.50",
        out_of_print: false,
    },
    Book {
        title: "Neuromancer",
        author: "William Gibson",
        stock: 0,
        price: "\u{2014}",
        out_of_print: true,
    },
    Book {
        title: "Solaris",
        author: "Stanis\u{142}aw Lem",
        stock: 7,
        price: "$11.00",
        out_of_print: false,
    },
    Book {
        title: "Ulysses",
        author: "James Joyce",
        stock: 2,
        price: "$14.25",
        out_of_print: false,
    },
];

/// The books in the order of `sort`. The table hooks only track the sort descriptor; sorting the data is up to you.
fn sorted(sort: Option<&SortDescriptor>) -> Vec<Book> {
    let mut books = BOOKS.to_vec();
    if let Some(sort) = sort {
        match sort.column.as_str() {
            Some("author") => books.sort_by_key(|book| book.author),
            Some("stock") => books.sort_by_key(|book| book.stock),
            _ => books.sort_by_key(|book| book.title),
        }
        if sort.direction == SortDirection::Descending {
            books.reverse();
        }
    }
    books
}

#[component]
pub fn TableHookDemo() -> impl IntoView {
    let sort = RwSignal::new(Some(SortDescriptor {
        column: Key::from("title"),
        direction: SortDirection::Ascending,
    }));
    let books = Memo::new(move |_| sort.with(|sort| sorted(sort.as_ref())));

    // Columns and rows, rebuilt whenever the order of the books changes. The first column holds the selection
    // checkboxes, so the data cells of a row are cells 1 to 4.
    let table = Memo::new(move |_| {
        Arc::new(TableCollection::build_with(
            TableOptions {
                show_selection_checkboxes: true,
            },
            |t| {
                t.column("title", "Title").row_header().allows_sorting();
                t.column("author", "Author").allows_sorting();
                t.column_group("inventory", "Inventory", |g| {
                    g.column("stock", "In stock").allows_sorting();
                    g.column("price", "Price");
                });
                books.with(|books| {
                    for book in books {
                        t.row(book.title, book.title, |r| {
                            r.cell(book.title);
                            r.cell(book.author);
                            r.cell(book.stock.to_string());
                            r.cell(book.price);
                        })
                        .disabled(book.out_of_print);
                    }
                });
            },
        ))
    });

    let state = use_table_state(UseTableStateInput {
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            // Disabled rows stay focusable; they only can't be selected.
            disabled_behavior: DisabledBehavior::Selection,
            ..SelectionOptions::default()
        },
        default_sort_descriptor: sort.get_untracked(),
        on_sort_change: Some(Callback::new(move |descriptor| sort.set(Some(descriptor)))),
        table,
        focus_mode: GridFocusMode::Row,
        sort_descriptor: None,
    });
    let UseTableReturn { props, data } = use_table(UseTableInput {
        aria_label: "Books".into(),
        state,
        element: CapturedElement::new(),
        id: None,
        aria_labelledby: None,
        keyboard_delegate: None,
        options: CollectionOptions::default(),
        keyboard_navigation_behavior: KeyboardNavigationBehavior::default(),
        should_select_on_press_up: false,
        on_row_action: None,
        on_cell_action: None,
    });

    // The header rows: a row for the "Inventory" group (with placeholders above the other columns), then the
    // column headers. The columns don't change, so they are read once.
    let header_rows: Vec<Vec<(Key, NodeKind)>> = table.with_untracked(|t| {
        t.header_rows()
            .iter()
            .map(|row| {
                t.collection()
                    .children(row)
                    .map(|cell| (cell.key.clone(), cell.kind))
                    .collect()
            })
            .collect()
    });
    let header = {
        let data = data.clone();
        header_rows
            .into_iter()
            .map(|cells| {
                let cells = cells
                    .into_iter()
                    .map(|(key, kind)| {
                        if kind == NodeKind::Placeholder {
                            view! { <th {..use_table_header_placeholder(UseTableHeaderPlaceholderInput { table: data.clone(), key: key.clone() }).into_attrs()}></th> }.into_any()
                        } else {
                            view! { <ColumnHeader table=data.clone() column=key/> }.into_any()
                        }
                    })
                    .collect_view();
                view! { <tr {..use_table_header_row().into_attrs()}>{cells}</tr> }
            })
            .collect_view()
    };

    let selection = state.grid.list.selection;
    let selected = move || {
        let mut keys: Vec<String> = selection
            .selected_keys()
            .iter()
            .map(ToString::to_string)
            .collect();
        keys.sort();
        if keys.is_empty() {
            "none".to_owned()
        } else {
            keys.join(", ")
        }
    };
    let sorting = move || {
        state.sort_descriptor.with(|sort| {
            sort.as_ref().map_or_else(
                || "none".to_owned(),
                |sort| {
                    let direction = match sort.direction {
                        SortDirection::Ascending => "ascending",
                        SortDirection::Descending => "descending",
                    };
                    format!("{} {direction}", sort.column)
                },
            )
        })
    };

    view! {
        <div class="demo-table-scroll">
            <table {..props.into_attrs()} class="demo-table demo-hook-table">
                <thead {..use_grid_row_group().row_group_props.into_attrs()}>{header}</thead>
                <tbody {..use_grid_row_group().row_group_props.into_attrs()}>
                    <For each=move || books.get() key=|book| book.title let:book>
                        <BookRow table=data.clone() book/>
                    </For>
                </tbody>
            </table>
        </div>
        <p class="demo-status">"Selected: "{selected}". Sorted by: "{sorting}"."</p>

    }
}

/// A column header. Pressing a sortable one sorts the table; the header of the checkbox column holds the
/// "select all" checkbox.
#[component]
fn ColumnHeader(table: TableData, column: Key) -> impl IntoView {
    let (text, is_checkbox_column) = table.state.table.with_untracked(|t| {
        t.column(&column).map_or_else(Default::default, |column| {
            (
                column.text_value.to_string(),
                column.kind == ColumnKind::SelectionCheckbox,
            )
        })
    });
    let content = if is_checkbox_column {
        let checkbox = use_checkbox(use_table_select_all_checkbox(UseTableSelectAllCheckboxInput { table: table.clone() }));
        let (attrs, styles) = checkbox.input_props.into_parts();
        view! { <input {..attrs} style=styles/> }.into_any()
    } else {
        text.into_any()
    };
    let header = use_table_column_header(UseTableColumnHeaderInput {
        table,
        key: column,
        allows_arrow_navigation: false,
    });
    let (attrs, styles) = header.column_header_props.into_parts();

    view! { <th {..attrs} style=styles>{content}</th> }
}

#[component]
fn BookRow(table: TableData, book: Book) -> impl IntoView {
    let key = Key::from(book.title);
    let row = use_table_row(UseTableRowInput {
        table: table.clone(),
        key: key.clone(),
        on_context_menu: None,
    });
    let allows_selection = row.allows_selection;
    let (attrs, styles) = row.row_props.into_parts();
    let checkbox = use_checkbox(use_table_selection_checkbox(UseTableSelectionCheckboxInput { table: table.clone(), key: key.clone() }));
    let (checkbox_attrs, checkbox_styles) = checkbox.input_props.into_parts();

    view! {
        <tr {..attrs} style=styles data-disabled=move || (!allows_selection.get()).then_some("")>
            <Cell table=table.clone() key=Key::cell(&key, 0)>
                <input {..checkbox_attrs} style=checkbox_styles/>
            </Cell>
            <Cell table=table.clone() key=Key::cell(&key, 1)>{book.title}</Cell>
            <Cell table=table.clone() key=Key::cell(&key, 2)>{book.author}</Cell>
            <Cell table=table.clone() key=Key::cell(&key, 3) number=true>{book.stock}</Cell>
            <Cell table key=Key::cell(&key, 4) number=true>{book.price}</Cell>
        </tr>
    }
}

/// A body cell. The title cells label their rows (`row_header()`), so they get `role="rowheader"`.
#[component]
fn Cell(
    table: TableData,
    key: Key,
    #[prop(optional)] number: bool,
    children: Children,
) -> impl IntoView {
    let cell = use_table_cell(UseTableCellInput {
        table,
        key,
        focus_mode: None,
        allows_arrow_navigation: false,
        should_select_on_press_up: false,
    });
    let (attrs, styles) = cell.grid_cell_props.into_parts();

    view! {
        <td {..attrs} style=styles class=number.then_some("demo-table-number")>
            {children()}
        </td>
    }
}
