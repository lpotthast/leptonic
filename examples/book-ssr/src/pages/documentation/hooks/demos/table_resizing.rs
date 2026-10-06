use std::sync::Arc;

use leptonic::{
    hooks::{
        ColumnBound, ColumnSize, IntoAttrs, TableCollection, TableColumnResizeState, TableData,
        UseTableCellInput, UseTableColumnHeaderInput, UseTableColumnResizeInput,
        UseTableColumnResizeStateInput, UseTableInput, UseTableReturn, UseTableRowInput,
        UseTableStateInput, collections::Key, use_grid_row_group, use_table, use_table_cell,
        use_table_column_header, use_table_column_resize, use_table_column_resize_state,
        use_table_header_row, use_table_row, use_table_state,
    },
    utils::{
        CapturedElement,
        css::{computed_px, computed_size},
        style::WidthProperty,
    },
};
use leptos::{html, prelude::*};
use leptos_use::use_element_size;

#[derive(Debug, Clone, Copy)]
struct File {
    name: &'static str,
    kind: &'static str,
    size: &'static str,
    modified: &'static str,
}

const FILES: [File; 4] = [
    File {
        name: "quarterly-report-final-v3.pdf",
        kind: "PDF document",
        size: "2.4 MB",
        modified: "Yesterday, 16:20",
    },
    File {
        name: "team-photo.jpg",
        kind: "JPEG image",
        size: "5.1 MB",
        modified: "3 Oct 2026, 09:12",
    },
    File {
        name: "budget.ods",
        kind: "Spreadsheet",
        size: "88 KB",
        modified: "28 Sep 2026, 11:47",
    },
    File {
        name: "notes.md",
        kind: "Markdown text",
        size: "4 KB",
        modified: "Today, 08:05",
    },
];

const COLUMNS: [(&str, &str); 4] = [
    ("name", "Name"),
    ("kind", "Kind"),
    ("size", "Size"),
    ("modified", "Modified"),
];

#[component]
pub fn TableResizingHookDemo() -> impl IntoView {
    let table = Memo::new(|_| {
        Arc::new(TableCollection::build(|t| {
            // Twice the share of the space left by the fixed columns, but at least 140 pixels.
            t.column("name", "Name")
                .row_header()
                .allows_resizing()
                .default_width(ColumnSize::Fr(2.0))
                .min_width(ColumnBound::Px(140.0));
            t.column("kind", "Kind")
                .allows_resizing()
                .default_width(ColumnSize::Px(140.0));
            // Not resizable: it keeps its 90 pixels.
            t.column("size", "Size").default_width(ColumnSize::Px(90.0));
            // At most 40% of the table width.
            t.column("modified", "Modified")
                .allows_resizing()
                .max_width(ColumnBound::Percent(40.0));
            for file in FILES {
                t.row(file.name, file.name, |r| {
                    r.cell(file.name);
                    r.cell(file.kind);
                    r.cell(file.size);
                    r.cell(file.modified);
                });
            }
        }))
    });
    let state = use_table_state(UseTableStateInput::new(table));
    let UseTableReturn { props, data } = use_table(UseTableInput {
        aria_label: "Files".into(),
        ..UseTableInput::new(state, CapturedElement::new())
    });

    // The columns share the width of the scroll container around the table.
    let container = NodeRef::<html::Div>::new();
    let width = use_element_size(container).width;
    let resize = use_table_column_resize_state(UseTableColumnResizeStateInput::new(state, width));

    let headers = COLUMNS
        .map(|(key, _)| view! { <ColumnHeader table=data.clone() resize column=Key::from(key)/> })
        .collect_view();
    let widths = move || {
        resize.column_widths.with(|widths| {
            COLUMNS
                .map(|(key, text)| format!("{text} {}px", widths.width(&Key::from(key))))
                .join(" \u{b7} ")
        })
    };
    let resizing = move || {
        resize.resizing_column.with(|column| {
            column
                .as_ref()
                .map_or_else(|| "no".to_owned(), ToString::to_string)
        })
    };

    view! {
        <div node_ref=container class="demo-table-scroll">
            <table {..props.into_attrs()} class="demo-table demo-resizable-table">
                <thead {..use_grid_row_group().row_group_props.into_attrs()}>
                    <tr {..use_table_header_row().into_attrs()}>{headers}</tr>
                </thead>
                <tbody {..use_grid_row_group().row_group_props.into_attrs()}>
                    {FILES.map(|file| view! { <FileRow table=data.clone() file/> }).collect_view()}
                </tbody>
            </table>
        </div>
        <p class="demo-status">"Widths: "{widths}". Resizing: "{resizing}"."</p>
    }
}

/// A column header, as wide as the resize state says. Resizable columns get a resizer.
#[component]
fn ColumnHeader(table: TableData, resize: TableColumnResizeState, column: Key) -> impl IntoView {
    let (text, allows_resizing) = table.state.table.with_untracked(|t| {
        t.column(&column).map_or_else(Default::default, |c| {
            (c.text_value.to_string(), c.allows_resizing)
        })
    });
    let resizer = allows_resizing
        .then(|| view! { <Resizer table=table.clone() resize column=column.clone()/> });
    let is_resizing = {
        let column = column.clone();
        move || {
            resize
                .resizing_column
                .with(|c| c.as_ref() == Some(&column))
                .then_some("")
        }
    };
    let width = {
        let column = column.clone();
        move || WidthProperty.declare(computed_size(computed_px(resize.column_width(&column))))
    };
    let header = use_table_column_header(UseTableColumnHeaderInput::new(table, column));
    let (attrs, styles) = header.column_header_props.into_parts();

    view! {
        <th {..attrs} style=styles.add_reactive(width) data-resizing=is_resizing>
            {text}
            {resizer}
        </th>
    }
}

/// The handle at the end of a column header: drag it, or focus it and press Enter to resize with the arrow keys.
/// The visually hidden range input inside it holds the focus and tells screen readers the column width.
#[component]
fn Resizer(table: TableData, resize: TableColumnResizeState, column: Key) -> impl IntoView {
    let resizer = use_table_column_resize(UseTableColumnResizeInput::new(
        resize,
        table,
        column,
        CapturedElement::new(),
    ));
    let is_resizing = resizer.is_resizing;
    let (attrs, styles) = resizer.resizer_props.into_parts();
    let (input_attrs, input_styles) = resizer.input_props.into_parts();

    view! {
        <div
            role="presentation"
            {..attrs}
            style=styles
            class="demo-column-resizer"
            data-resizing=move || is_resizing.get().then_some("")
        >
            <input {..input_attrs} style=input_styles/>
        </div>
    }
}

#[component]
fn FileRow(table: TableData, file: File) -> impl IntoView {
    let key = Key::from(file.name);
    let row = use_table_row(UseTableRowInput {
        table: table.clone(),
        key: key.clone(),
    });
    let (attrs, styles) = row.row_props.into_parts();
    let cells = [file.name, file.kind, file.size, file.modified]
        .into_iter()
        .enumerate()
        .map(|(index, text)| view! { <Cell table=table.clone() key=Key::cell(&key, index)>{text}</Cell> })
        .collect_view();

    view! { <tr {..attrs} style=styles>{cells}</tr> }
}

#[component]
fn Cell(table: TableData, key: Key, children: Children) -> impl IntoView {
    let cell = use_table_cell(UseTableCellInput::new(table, key));
    let (attrs, styles) = cell.grid_cell_props.into_parts();

    view! { <td {..attrs} style=styles>{children()}</td> }
}
